#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

mod archive {

    use gix_hash::oid;
    use gix_object::Data;

    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    #[error("{msg}")]
    pub struct FailedToArchiveError {
        msg: String,
    }

    /// Available formats
    #[derive(PartialEq)]
    pub enum Format {
        Tar,
        Zip(u8),
        TarGz(u8),
        Tgz(u8),
    }

    #[derive(PartialEq)]
    pub struct FromConfig {
        name: String,
        command: String,
    }

    pub struct Options {
        pub format: Format,
        pub prefix: String,
        pub modified_time: std::time::SystemTime,
        pub use_worktree_attributes: bool,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                format: Format::Tar,
                prefix: String::new(),
                modified_time: std::time::SystemTime::now(),
                use_worktree_attributes: false,
            }
        }
    }

    /// Finds files to be added to the archive my means of `_object_id` and `_find` closure and writes to `_destination` according to archive `opts`
    ///
    /// Querying `_find` with `_object_id` should return the data that would go to the worktree.
    /// We do not handle .gitattributes yet (see `gix-attributes` crate) and hard fail if there are substitution placeholders (`$Format:PLACEHOLDERS$`) in the files to be archived.
    /// The substitution capability is based on git/gix-log and is to be implemented.
    ///
    /// .gitattributes integration plans:
    /// gix-archive code needs access to git attributes and access to a way to apply filters that would also be applied during worktree checkouts.
    /// This can be boiled down to applying tree-to-worktree-filters outside of gix-archive. For that, there will be gix-filter,
    /// which is controlled by attributes which can probably be passed in via gix_worktree::fs::Cache or an equivalent closure to abstract that detail.
    /// We care about a set of attributes per paths, partly interesting for us for our own substitution, and partially interesting to the one applying filters.
    /// That will probably also happen via a closure as gix-config also plays a role there.
    ///
    /// In addition to the filters above, gix-archive has the following filters:
    ///  - ignoring paths to be added to the archive
    ///  - substitution (placeholders expansion)
    pub fn write<W, Find, E>(
        _object_id: &oid,
        mut _find: Find,
        mut _destination: W,
        opts: Options,
    ) -> Result<(), FailedToArchiveError>
    where
        W: std::io::Write,
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<Data<'a>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        // Just for tests
        if opts.format != Format::Tar {
            return Err(FailedToArchiveError {
                msg: "Failed to archive".into(),
            });
        }

        Ok(())
    }
}

pub use archive::{write, FailedToArchiveError, Format, Options};
