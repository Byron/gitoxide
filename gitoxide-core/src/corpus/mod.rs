pub struct Engine<P> {
    progress: P,
    con: rusqlite::Connection,
    gitoxide_version: String,
}

pub mod engine {
    use crate::corpus::Engine;
    use anyhow::Context;
    use std::path::PathBuf;

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
        pub fn run(&self, path: PathBuf) -> anyhow::Result<()> {
            let _corpus_id = self.corpus_id_or_insert(&path)?;
            let _gitoxide_id = self.gitoxide_version_id_or_insert()?;
            let _runner_id = self.runner_id_or_insert()?;
            todo!()
        }
    }
}

pub mod db {
    use crate::corpus::engine::Id;
    use crate::corpus::Engine;
    use anyhow::{bail, Context};
    use rusqlite::{params, OptionalExtension};
    use std::path::Path;
    use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, SystemExt};

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
        con.execute_batch(
            r#"
        CREATE TABLE if not exists runner(
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
            root text UNIQUE -- the root path of all repositories we want to consider, as canonicalized path
        )
        "#,
        )?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists repository(
            rela_path text UNIQUE, -- the path to the repository on disk, relative to the corpus root path, without leading `./` or `.\`
            corpus integer,
            FOREIGN KEY (corpus) REFERENCES corpus (rowid)
        )
        "#,
        )?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists gitoxide_version(
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
            FOREIGN KEY (repository) REFERENCES repository (rowid),
            FOREIGN KEY (runner) REFERENCES runner (rowid),
            FOREIGN KEY (gitoxide_version) REFERENCES gitoxide_version (rowid)
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
            Ok(
                match self
                    .con
                    .query_row(
                        "SELECT rowid FROM runner WHERE vendor = ?1 AND brand = ?2",
                        [vendor.as_deref(), brand.as_deref()],
                        |r| r.get(0),
                    )
                    .optional()?
                {
                    Some(existing) => existing,
                    None => {
                        self.con.execute(
                            "INSERT INTO runner (vendor, brand, host_name) VALUES (?1, ?2, ?3)",
                            [vendor.as_deref(), brand.as_deref(), host.as_deref()],
                        )?;
                        self.con.query_row(
                            "SELECT rowid FROM runner WHERE vendor = ?1 AND brand = ?2",
                            [vendor, brand],
                            |r| r.get(0),
                        )?
                    }
                },
            )
        }
        pub(crate) fn corpus_id_or_insert(&self, path: &Path) -> anyhow::Result<Id> {
            let path = path.to_str().context("corpus root cannot contain illformed UTF-8")?;
            Ok(
                match self
                    .con
                    .query_row("SELECT rowid FROM corpus WHERE root = ?1", [path], |r| r.get(0))
                    .optional()?
                {
                    Some(existing) => existing,
                    None => {
                        self.con.execute("INSERT INTO corpus (root) VALUES (?1)", [path])?;
                        self.con
                            .query_row("SELECT rowid FROM corpus WHERE root = ?1", [path], |r| r.get(0))?
                    }
                },
            )
        }
        pub(crate) fn gitoxide_version_id_or_insert(&self) -> anyhow::Result<Id> {
            Ok(
                match self
                    .con
                    .query_row(
                        "SELECT rowid FROM gitoxide_version WHERE version = ?1",
                        [&self.gitoxide_version],
                        |r| r.get(0),
                    )
                    .optional()?
                {
                    Some(existing) => existing,
                    None => {
                        self.con.execute(
                            "INSERT INTO gitoxide_version (version) VALUES (?1)",
                            [&self.gitoxide_version],
                        )?;
                        self.con.query_row(
                            "SELECT rowid FROM gitoxide_version WHERE version = ?1",
                            [&self.gitoxide_version],
                            |r| r.get(0),
                        )?
                    }
                },
            )
        }
    }
}
