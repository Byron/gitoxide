impl crate::Repository {
    // TODO: tests
    /// Load the index file of this repository's workspace, if present.
    ///
    /// Note that it is loaded into memory each time this method is called, but also is independent of the workspace.
    #[cfg(feature = "git-index")]
    pub fn load_index(&self) -> Option<Result<git_index::File, git_index::file::init::Error>> {
        // TODO: choose better/correct options
        let opts = git_index::decode::Options {
            object_hash: self.object_hash,
            thread_limit: None,
            min_extension_block_in_bytes_for_threading: 1024 * 256,
        };
        match git_index::File::at(self.git_dir().join("index"), opts) {
            Ok(index) => Some(Ok(index)),
            Err(git_index::file::init::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => Some(Err(err)),
        }
    }

    // TODO: tests
    /// Try to merge mailmaps from the following locations into `target`:
    ///
    /// - read `HEAD:.mailmap` if this repository is bare (i.e. has no working tree), if the blob is not configured.
    /// - OR read the `.mailmap` file without following symlinks from the working tree, if present
    /// - AND read the mailmap as configured in `mailmap.blob`, if set.
    /// - AND read the file as configured by `mailmap.file` if set.
    ///
    /// Only the first error will be reported, and as many source mailmaps will be merged into `target` as possible.
    /// Parsing errors will be ignored.
    #[cfg(feature = "git-mailmap")]
    pub fn load_mailmap_into(&self, target: &mut git_mailmap::Snapshot) -> Result<(), crate::mailmap::load::Error> {
        let mut err = None::<crate::mailmap::load::Error>;
        self.config
            .get_raw_value("mailmap", None, "blob")
            .ok()
            .and_then(|spec| {
                // TODO: actually resolve the spec when available
                match git_hash::ObjectId::from_hex(spec.as_ref()) {
                    Ok(id) => Some(id),
                    Err(e) => {
                        err.get_or_insert(e.into());
                        None
                    }
                }
            });
        match self.work_tree() {
            None => {
                todo!("read blob")
            }
            Some(root) => {
                todo!("read mailmap file in tree")
            }
        }

        let configured_path = self
            .config
            .value::<git_config::values::Path<'_>>("mailmap", None, "file")
            .ok()
            .and_then(|path| {
                let install_dir = self.install_directory().ok()?;
                match path.interpolate(Some(install_dir.as_path())) {
                    Ok(path) => Some(path),
                    Err(e) => {
                        err.get_or_insert(e.into());
                        None
                    }
                }
            });

        if let Some(path) = configured_path {
            todo!("read mailmap file as configured")
        }

        err.map(Err).unwrap_or(Ok(()))
    }
}
