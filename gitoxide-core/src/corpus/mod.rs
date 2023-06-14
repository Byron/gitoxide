pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 0..=3;

pub struct Engine<P> {
    progress: P,
    con: rusqlite::Connection,
    gitoxide_version: String,
}

pub struct RunOutcome {
    /// the relative path to the repositories that could not be found on disk
    pub missing_repos_rela_paths: usize,
}

pub mod engine {
    use super::db;
    use crate::corpus::Engine;
    use crate::organize::find_git_repository_workdirs;
    use anyhow::Context;
    use bytesize::ByteSize;
    use rusqlite::params;
    use std::path::{Path, PathBuf};
    use std::time::Instant;

    pub(crate) type Id = u32;

    impl<P> Engine<P>
    where
        P: gix::Progress,
    {
        /// Open the corpus DB or create it.
        pub fn open_or_create(db: PathBuf, gitoxide_version: String, progress: P) -> anyhow::Result<Engine<P>> {
            let con = crate::corpus::db::create(db).context("Could not open or create database")?;
            Ok(Engine {
                progress,
                con,
                gitoxide_version,
            })
        }

        /// Run on the existing set of repositories we have already seen or obtain them from `path` if there is none yet.
        pub fn run(&mut self, corpus_path: PathBuf) -> anyhow::Result<()> {
            let corpus_path = gix::path::realpath(corpus_path)?;
            let corpus_id = self.corpus_id_or_insert(&corpus_path)?;
            let _gitoxide_id = self.gitoxide_version_id_or_insert()?;
            let _runner_id = self.runner_id_or_insert()?;
            let _repos = self.find_repos_or_insert(&corpus_path, corpus_id)?;
            todo!("do run on repos")
        }

        fn find_repos(&mut self, corpus_id: Id) -> anyhow::Result<Vec<db::Repo>> {
            self.progress.set_name("query db-repos");
            self.progress.init(None, gix::progress::count("repos"));

            Ok(self
                .con
                .prepare(
                    "SELECT id, rela_path, odb_size, num_objects, num_references FROM repository WHERE corpus = ?1",
                )?
                .query_map([corpus_id], |r| {
                    Ok(db::Repo {
                        id: r.get(0)?,
                        path: r.get::<_, String>(1)?.into(),
                        odb_size: ByteSize(r.get(2)?),
                        num_objects: r.get(3)?,
                        num_references: r.get(4)?,
                    })
                })?
                .inspect(|_| self.progress.inc())
                .collect::<Result<_, _>>()?)
        }

        fn refresh_repos(&mut self, corpus_path: &Path, corpus_id: Id) -> anyhow::Result<Vec<db::Repo>> {
            let start = Instant::now();
            self.progress.set_name("refresh");
            self.progress.init(None, gix::progress::count("repos"));

            let repos = std::thread::scope({
                let progress = &mut self.progress;
                let con = &mut self.con;
                |scope| -> anyhow::Result<_> {
                    let threads = std::thread::available_parallelism()
                        .map(std::num::NonZeroUsize::get)
                        .ok()
                        .unwrap_or(1);
                    let (path_tx, repo_rx) = {
                        let (path_tx, path_rx) = crossbeam_channel::bounded(threads * 2);
                        let (repo_tx, repo_rx) = std::sync::mpsc::channel::<(PathBuf, anyhow::Result<db::Repo>)>();
                        (0..threads).for_each(|_| {
                            scope.spawn({
                                let path_rx = path_rx.clone();
                                let repo_tx = repo_tx.clone();
                                move || -> anyhow::Result<_> {
                                    for repo_path in path_rx {
                                        let res = (|| {
                                            let repo = gix::open_opts(&repo_path, gix::open::Options::isolated())?;
                                            db::Repo::try_from(&repo)
                                        })();
                                        repo_tx.send((repo_path, res))?;
                                    }
                                    Ok(())
                                }
                            });
                        });
                        (path_tx, repo_rx)
                    };

                    let find_progress = progress.add_child("find");
                    let write_db = scope.spawn(move || -> anyhow::Result<Vec<db::Repo>> {
                        progress.set_name("write to DB");
                        progress.init(None, gix::progress::count("repos"));
                        let start = Instant::now();

                        let mut out = Vec::new();
                        let transaction = con.transaction()?;
                        let mut statement = transaction.prepare("INSERT INTO repository (rela_path, corpus, odb_size, num_objects, num_references) VALUES (?1, ?2, ?3, ?4, ?5)\
                                                        ON CONFLICT DO UPDATE SET rela_path = rela_path, corpus = corpus, odb_size = ?3, num_objects = ?4, num_references = ?5\
                                                        RETURNING id")?;
                        for (repo_path, repo_res) in repo_rx {
                            match repo_res {
                                Ok(mut repo) => {
                                    let rela_path = repo.path.strip_prefix(corpus_path)?;
                                    repo.id = statement.query_row(params![rela_path.to_str().context("only valid UTF8 is allowed for repository paths")?, corpus_id, repo.odb_size.as_u64(), repo.num_objects, repo.num_references], |r| r.get(0))?;
                                    out.push(repo);
                                    progress.inc();
                                }
                                Err(err) => progress.fail(format!("{repo_path:?}: {err:#?}")),
                            }
                        }
                        statement.finalize()?;
                        transaction.commit()?;
                        progress.show_throughput(start);
                        Ok(out)
                    });

                    let repos = gix::interrupt::Iter::new(
                        find_git_repository_workdirs(corpus_path, find_progress, false, Some(threads)),
                        || anyhow::anyhow!("interrupted by user"),
                    );
                    for res in repos {
                        let (repo_path, _kind) = res?;
                        path_tx.send(repo_path)?;
                    }
                    drop(path_tx);
                    write_db.join().expect("no panic")
                }
            })?;

            self.progress.show_throughput(start);
            Ok(repos)
        }

        fn find_repos_or_insert(&mut self, corpus_path: &Path, corpus_id: Id) -> anyhow::Result<Vec<db::Repo>> {
            let start = Instant::now();
            let repos = self.find_repos(corpus_id)?;
            if repos.is_empty() {
                self.refresh_repos(corpus_path, corpus_id)
            } else {
                self.progress.show_throughput(start);
                Ok(repos)
            }
        }
    }
}

pub mod db {
    use crate::corpus::engine::Id;
    use crate::corpus::Engine;
    use anyhow::{bail, Context};
    use bytesize::ByteSize;
    use rusqlite::{params, OptionalExtension};
    use std::path::{Path, PathBuf};
    use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, SystemExt};

    /// a husk of a repository
    pub(crate) struct Repo {
        pub(crate) id: Id,
        /// The full path to the repository on disk, not yet validated to exist.
        pub(crate) path: PathBuf,
        /// The size of the object database, counted quickly by packs only.
        pub(crate) odb_size: ByteSize,
        /// The amount of objects stored in the object database.
        pub(crate) num_objects: u64,
        /// The total amount of references, no matter which type.
        pub(crate) num_references: usize,
    }

    impl Repo {
        pub(crate) fn try_from(repo: &gix::Repository) -> anyhow::Result<Self> {
            let num_references = repo.refs.iter()?.all()?.count();
            let num_objects = repo.objects.packed_object_count()?;
            let odb_size = ByteSize(
                std::fs::read_dir(repo.objects.store_ref().path().join("pack"))
                    .map(|dir| {
                        dir.filter_map(Result::ok)
                            .filter_map(|e| e.metadata().ok())
                            .filter_map(|m| m.is_file().then_some(m.len()))
                            .sum()
                    })
                    .unwrap_or_default(),
            );

            Ok(Repo {
                id: 0,
                path: repo.path().to_owned(),
                odb_size,
                num_objects,
                num_references,
            })
        }
    }

    /// A version to be incremented whenever the database layout is changed, to refresh it automatically.
    const VERSION: usize = 1;

    pub fn create(path: impl AsRef<std::path::Path>) -> anyhow::Result<rusqlite::Connection> {
        let path = path.as_ref();
        let con = rusqlite::Connection::open(path)?;
        let meta_table = r#"
        CREATE TABLE if not exists meta(
            version int
        )"#;
        con.execute_batch(meta_table)?;
        let version: Option<usize> = con.query_row("SELECT version FROM meta", [], |r| r.get(0)).optional()?;
        match version {
            None => {
                con.execute("INSERT into meta(version) values(?)", params![VERSION])?;
            }
            Some(version) if version != VERSION => match con.close() {
                Ok(()) => {
                    bail!("Cannot handle database with version {version}, cannot yet migrate to {VERSION} - maybe migrate by hand?");
                }
                Err((_, err)) => return Err(err.into()),
            },
            _ => {}
        }
        con.execute_batch("PRAGMA synchronous = OFF;")?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists runner(
            id integer PRIMARY KEY,
            vendor text,
            brand text,
            host_name text, -- this is just to help ID the runner
            UNIQUE (vendor, brand)
        )
        "#,
        )?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists corpus(
            id integer PRIMARY KEY,
            root text UNIQUE -- the root path of all repositories we want to consider, as canonicalized path
        )
        "#,
        )?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists repository(
            id integer PRIMARY KEY,
            rela_path text, -- the path to the repository on disk, relative to the corpus root path, without leading `./` or `.\`
            corpus integer,
            odb_size integer, -- the object database size in bytes
            num_references integer, -- the total amount of references
            num_objects integer, -- the total amount of objects
            FOREIGN KEY (corpus) REFERENCES corpus (id)
            UNIQUE (rela_path, corpus)
        )
        "#,
        )?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists gitoxide_version(
            id integer PRIMARY KEY,
            version text UNIQUE -- the unique git version via gix describe
        )
        "#,
        )?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists run(
            repository integer,
            runner integer,
            gitoxide_version integer,
            start_time integer,
            end_time integer, -- or NULL if not yet finished (either successfull or with failure)
            error text, -- or NULL if there was on error
            FOREIGN KEY (repository) REFERENCES repository (id),
            FOREIGN KEY (runner) REFERENCES runner (id),
            FOREIGN KEY (gitoxide_version) REFERENCES gitoxide_version (id)
        )
        "#,
        )?;

        Ok(con)
    }

    /// Utilities
    impl<P> Engine<P> {
        pub(crate) fn runner_id_or_insert(&self) -> anyhow::Result<Id> {
            let sys = sysinfo::System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::new().with_frequency()),
            );
            let cpu = &sys.cpus()[0];
            let vendor = Some(cpu.vendor_id().to_owned());
            let host = sys.host_name();
            let brand = Some(cpu.brand().to_owned());
            Ok(self.con.query_row(
                "INSERT INTO runner (vendor, brand, host_name) VALUES (?1, ?2, ?3) \
                        ON CONFLICT DO UPDATE SET vendor = vendor, brand = brand, host_name = ?3 RETURNING id",
                [vendor.as_deref(), brand.as_deref(), host.as_deref()],
                |r| r.get(0),
            )?)
        }
        pub(crate) fn corpus_id_or_insert(&self, path: &Path) -> anyhow::Result<Id> {
            let path = path.to_str().context("corpus root cannot contain illformed UTF-8")?;
            Ok(self.con.query_row(
                "INSERT INTO corpus (root) VALUES (?1) \
                    ON CONFLICT DO UPDATE SET root = root RETURNING id",
                [path],
                |r| r.get(0),
            )?)
        }
        pub(crate) fn gitoxide_version_id_or_insert(&self) -> anyhow::Result<Id> {
            Ok(self
                    .con
                    .query_row(
                        "INSERT INTO gitoxide_version (version) VALUES (?1) ON CONFLICT DO UPDATE SET version = version RETURNING id",
                        [&self.gitoxide_version],
                        |r| r.get(0),
                    )?)
        }
    }
}
