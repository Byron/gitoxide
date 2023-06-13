pub struct Engine<P> {
    progress: P,
    con: rusqlite::Connection,
}

pub mod engine {
    use crate::corpus::Engine;
    use anyhow::Context;
    use std::path::PathBuf;

    impl<P> Engine<P>
    where
        P: gix::Progress,
    {
        /// Open the corpus DB or create it.
        pub fn open_or_create(db: PathBuf, progress: P) -> anyhow::Result<Engine<P>> {
            let con = crate::corpus::db::create(db).context("Could not open or create database")?;
            Ok(Engine { progress, con })
        }

        /// Run on the existing set of repositories we have already seen or obtain them from `path` if there is none yet.
        pub fn run(&self, _path: PathBuf) -> anyhow::Result<()> {
            todo!()
        }
    }
}

pub mod db {
    use anyhow::bail;
    use rusqlite::{params, OptionalExtension};

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
        CREATE TABLE if not exists gix_version(
            version text UNIQUE -- the unique git version via gix describe
        )
        "#,
        )?;
        con.execute_batch(
            r#"
        CREATE TABLE if not exists run(
            repository integer,
            runner integer,
            gix_version integer,
            start_time integer,
            end_time integer, -- or NULL if not yet finished (either successfull or with failure)
            error text, -- or NULL if there was on error
            FOREIGN KEY (repository) REFERENCES repository (rowid),
            FOREIGN KEY (runner) REFERENCES runner (rowid),
            FOREIGN KEY (gix_version) REFERENCES gix_version (rowid)
        )
        "#,
        )?;

        Ok(con)
    }
}
