impl crate::Repository {
    // TODO: tests
    /// Similar to [`open_mailmap_into()`][crate::Repository::open_mailmap_into()], but ignores all errors and returns at worst
    /// an empty mailmap, e.g. if there is no mailmap or if there were errors loading them.
    ///
    /// This represents typical usage within git, which also works with what's there without considering a populated mailmap
    /// a reason to abort an operation, considering it optional.
    pub fn open_mailmap(&self) -> gix_mailmap::Snapshot {
        let mut out = gix_mailmap::Snapshot::default();
        self.open_mailmap_into(&mut out).ok();
        out
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
    pub fn open_mailmap_into(&self, target: &mut gix_mailmap::Snapshot) -> Result<(), crate::mailmap::load::Error> {
        let mut err = None::<crate::mailmap::load::Error>;
        let mut buf = Vec::new();
        let mut blob_id = self
            .config
            .resolved
            .raw_value("mailmap", None, "blob")
            .ok()
            .and_then(|spec| {
                // TODO: actually resolve this as spec (once we can do that)
                gix_hash::ObjectId::from_hex(spec.as_ref())
                    .map_err(|e| err.get_or_insert(e.into()))
                    .ok()
            });
        match self.work_dir() {
            None => {
                blob_id = blob_id.or_else(|| {
                    self.head().ok().and_then(|mut head| {
                        let commit = head.peel_to_commit_in_place().ok()?;
                        let tree = commit.tree().ok()?;
                        tree.find_entry(".mailmap").map(|e| e.object_id())
                    })
                });
            }
            Some(root) => {
                if let Ok(mut file) = gix_features::fs::open_options_no_follow()
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
                    target.merge(gix_mailmap::parse_ignore_errors(&buf));
                }
            }
        }

        if let Some(blob) = blob_id.and_then(|id| self.find_object(id).map_err(|e| err.get_or_insert(e.into())).ok()) {
            target.merge(gix_mailmap::parse_ignore_errors(&blob.data));
        }

        let configured_path = self
            .config
            .resolved
            .value::<gix_config::Path<'_>>("mailmap", None, "file")
            .ok()
            .and_then(|path| {
                let install_dir = self.install_dir().ok()?;
                let home = self.config.home_dir();
                match path.interpolate(gix_config::path::interpolate::Context {
                    git_install_dir: Some(install_dir.as_path()),
                    home_dir: home.as_deref(),
                    home_for_user: if self.options.git_dir_trust.expect("trust is set") == gix_sec::Trust::Full {
                        Some(gix_config::path::interpolate::home_for_user)
                    } else {
                        None
                    },
                }) {
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
            target.merge(gix_mailmap::parse_ignore_errors(&buf));
        }

        err.map_or(Ok(()), Err)
    }
}
