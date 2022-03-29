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
    /// - read the `.mailmap` file without following symlinks from the working tree, if present
    /// - OR read `HEAD:.mailmap` if this repository is bare (i.e. has no working tree), if the `mailmap.blob` is not set.
    /// - read the mailmap as configured in `mailmap.blob`, if set.
    /// - read the file as configured by `mailmap.file`, following symlinks, if set.
    ///
    /// Only the first error will be reported, and as many source mailmaps will be merged into `target` as possible.
    /// Parsing errors will be ignored.
    #[cfg(feature = "git-mailmap")]
    pub fn load_mailmap_into(&self, target: &mut git_mailmap::Snapshot) -> Result<(), crate::mailmap::load::Error> {
        let mut err = None::<crate::mailmap::load::Error>;
        let mut buf = Vec::new();
        let mut blob_id = self
            .config
            .get_raw_value("mailmap", None, "blob")
            .ok()
            .and_then(|spec| {
                // TODO: actually resolve this as spec (once we can do that)
                git_hash::ObjectId::from_hex(spec.as_ref())
                    .map_err(|e| err.get_or_insert(e.into()))
                    .ok()
            });
        match self.work_tree() {
            None => {
                // TODO: replace with ref-spec `HEAD:.mailmap` for less verbose way of getting the blob id
                blob_id = blob_id.or_else(|| {
                    self.head().ok().and_then(|mut head| {
                        let commit = head.peel_to_commit_in_place().ok()?;
                        let tree = commit.tree().ok()?;
                        tree.lookup_path(std::iter::once(".mailmap")).ok()?.map(|e| e.oid)
                    })
                });
            }
            Some(root) => {
                if let Ok(mut file) = git_features::fs::open_options_no_follow()
                    .read(true)
                    .open(root.join(".mailmap"))
                    .map_err(|e| {
                        if e.kind() != std::io::ErrorKind::NotFound {
                            err.get_or_insert(e.into());
                        }
                    })
                {
                    buf.clear();
                    std::io::copy(&mut file, &mut buf)
                        .map_err(|e| err.get_or_insert(e.into()))
                        .ok();
                    target.merge(git_mailmap::parse_ignore_errors(&buf));
                }
            }
        }

        if let Some(blob) = blob_id.and_then(|id| self.find_object(id).map_err(|e| err.get_or_insert(e.into())).ok()) {
            target.merge(git_mailmap::parse_ignore_errors(&blob.data));
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

        if let Some(mut file) =
            configured_path.and_then(|path| std::fs::File::open(path).map_err(|e| err.get_or_insert(e.into())).ok())
        {
            buf.clear();
            std::io::copy(&mut file, &mut buf)
                .map_err(|e| err.get_or_insert(e.into()))
                .ok();
            target.merge(git_mailmap::parse_ignore_errors(&buf));
        }

        err.map(Err).unwrap_or(Ok(()))
    }
}
