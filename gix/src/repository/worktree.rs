use crate::{worktree, Worktree};

/// Interact with individual worktrees and their information.
impl crate::Repository {
    /// Return a list of all _linked_ worktrees sorted by private git dir path as a lightweight proxy.
    ///
    /// Note that these need additional processing to become usable, but provide a first glimpse a typical worktree information.
    pub fn worktrees(&self) -> std::io::Result<Vec<worktree::Proxy<'_>>> {
        let mut res = Vec::new();
        let iter = match std::fs::read_dir(self.common_dir().join("worktrees")) {
            Ok(iter) => iter,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(res),
            Err(err) => return Err(err),
        };
        for entry in iter {
            let entry = entry?;
            let worktree_git_dir = entry.path();
            if worktree_git_dir.join("gitdir").is_file() {
                res.push(worktree::Proxy {
                    parent: self,
                    git_dir: worktree_git_dir,
                })
            }
        }
        res.sort_by(|a, b| a.git_dir.cmp(&b.git_dir));
        Ok(res)
    }
    /// Return the repository owning the main worktree, typically from a linked worktree.
    ///
    /// Note that it might be the one that is currently open if this repository doesn't point to a linked worktree.
    /// Also note that the main repo might be bare.
    #[allow(clippy::result_large_err)]
    pub fn main_repo(&self) -> Result<crate::Repository, crate::open::Error> {
        crate::ThreadSafeRepository::open_opts(self.common_dir(), self.options.clone()).map(Into::into)
    }

    /// Return the currently set worktree if there is one, acting as platform providing a validated worktree base path.
    ///
    /// Note that there would be `None` if this repository is `bare` and the parent [`Repository`][crate::Repository] was instantiated without
    /// registered worktree in the current working dir, even if no `.git` file or directory exists.
    /// It's merely based on configuration, see [Worktree::dot_git_exists()] for a way to perform more validation.
    pub fn worktree(&self) -> Option<Worktree<'_>> {
        self.work_dir().map(|path| Worktree { parent: self, path })
    }

    /// Return true if this repository is bare, and has no main work tree.
    ///
    /// This is not to be confused with the [`worktree()`][crate::Repository::worktree()] worktree, which may exists if this instance
    /// was opened in a worktree that was created separately.
    pub fn is_bare(&self) -> bool {
        self.config.is_bare && self.work_dir().is_none()
    }

    /// If `id` points to a tree, produce a stream that yields one worktree entry after the other. The index of the tree at `id`
    /// is returned as well as it is an intermediate byproduct that might be useful to callers.
    ///
    /// The entries will look exactly like they would if one would check them out, with filters applied.
    /// The `export-ignore` attribute is used to skip blobs or directories to which it applies.
    #[cfg(feature = "worktree-stream")]
    #[gix_macros::momo]
    pub fn worktree_stream(
        &self,
        id: impl Into<gix_hash::ObjectId>,
    ) -> Result<(gix_worktree_stream::Stream, gix_index::File), crate::repository::worktree_stream::Error> {
        use gix_odb::{FindExt, HeaderExt};
        let id = id.into();
        let header = self.objects.header(id)?;
        if !header.kind().is_tree() {
            return Err(crate::repository::worktree_stream::Error::NotATree {
                id,
                actual: header.kind(),
            });
        }

        // TODO(perf): potential performance improvements could be to use the index at `HEAD` if possible (`index_from_head_tree…()`)
        // TODO(perf): when loading a non-HEAD tree, we effectively traverse the tree twice. This is usually fast though, and sharing
        //             an object cache between the copies of the ODB handles isn't trivial and needs a lock.
        let index = self.index_from_tree(&id)?;
        let mut cache = self
            .attributes_only(&index, gix_worktree::stack::state::attributes::Source::IdMapping)?
            .detach();
        let pipeline =
            gix_filter::Pipeline::new(cache.attributes_collection(), crate::filter::Pipeline::options(self)?);
        let objects = self.objects.clone().into_arc().expect("TBD error handling");
        let stream = gix_worktree_stream::from_tree(
            id,
            {
                let objects = objects.clone();
                move |id, buf| objects.find(id, buf)
            },
            pipeline,
            move |path, mode, attrs| -> std::io::Result<()> {
                let entry = cache.at_entry(path, Some(mode.is_tree()), |id, buf| objects.find_blob(id, buf))?;
                entry.matching_attributes(attrs);
                Ok(())
            },
        );
        Ok((stream, index))
    }

    /// Produce an archive from the `stream` and write it to `out` according to `options`.
    /// Use `blob` to provide progress for each entry written to `out`, and note that it should already be initialized to the amount
    /// of expected entries, with `should_interrupt` being queried between each entry to abort if needed, and on each write to `out`.
    ///
    /// ### Performance
    ///
    /// Be sure that `out` is able to handle a lot of write calls. Otherwise wrap it in a [`BufWriter`][std::io::BufWriter].
    ///
    /// ### Additional progress and fine-grained interrupt handling
    ///
    /// For additional progress reporting, wrap `out` into a writer that counts throughput on each write.
    /// This can also be used to react to interrupts on each write, instead of only for each entry.
    #[cfg(feature = "worktree-archive")]
    pub fn worktree_archive(
        &self,
        mut stream: gix_worktree_stream::Stream,
        out: impl std::io::Write + std::io::Seek,
        blobs: impl gix_features::progress::Count,
        should_interrupt: &std::sync::atomic::AtomicBool,
        options: gix_archive::Options,
    ) -> Result<(), crate::repository::worktree_archive::Error> {
        let mut out = gix_features::interrupt::Write {
            inner: out,
            should_interrupt,
        };
        if options.format == gix_archive::Format::InternalTransientNonPersistable {
            std::io::copy(&mut stream.into_read(), &mut out)?;
            return Ok(());
        }
        gix_archive::write_stream_seek(
            &mut stream,
            |stream| {
                if should_interrupt.load(std::sync::atomic::Ordering::Relaxed) {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Cancelled by user").into());
                }
                let res = stream.next_entry();
                blobs.inc();
                res
            },
            out,
            options,
        )?;
        Ok(())
    }
}
