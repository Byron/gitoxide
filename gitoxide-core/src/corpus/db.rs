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
        let sys =
            sysinfo::System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::new().with_frequency()));
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
