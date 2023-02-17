use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use gix_features::{interrupt, parallel::in_parallel, progress, progress::Progress};
use gix_hash::oid;

use crate::fs;

pub mod checkout;
pub(crate) mod entry;

/// Note that interruption still produce an `Ok(…)` value, so the caller should look at `should_interrupt` to communicate the outcome.
/// `dir` is the directory into which to checkout the `index`.
/// `git_dir` is the `.git` directory for reading additional per-repository configuration files.
#[allow(clippy::too_many_arguments)]
pub fn checkout<Find, E>(
    index: &mut gix_index::State,
    dir: impl Into<std::path::PathBuf>,
    find: Find,
    files: &mut impl Progress,
    bytes: &mut impl Progress,
    should_interrupt: &AtomicBool,
    options: checkout::Options,
) -> Result<checkout::Outcome, checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E> + Send + Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    let paths = index.take_path_backing();
    let res = checkout_inner(index, &paths, dir, find, files, bytes, should_interrupt, options);
    index.return_path_backing(paths);
    res
}
#[allow(clippy::too_many_arguments)]
fn checkout_inner<Find, E>(
    index: &mut gix_index::State,
    paths: &gix_index::PathStorage,
    dir: impl Into<std::path::PathBuf>,
    find: Find,
    files: &mut impl Progress,
    bytes: &mut impl Progress,
    should_interrupt: &AtomicBool,
    options: checkout::Options,
) -> Result<checkout::Outcome, checkout::Error<E>>
where
    Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E> + Send + Clone,
    E: std::error::Error + Send + Sync + 'static,
{
    let num_files = AtomicUsize::default();
    let dir = dir.into();
    let case = if options.fs.ignore_case {
        gix_glob::pattern::Case::Fold
    } else {
        gix_glob::pattern::Case::Sensitive
    };
    let (chunk_size, thread_limit, num_threads) = gix_features::parallel::optimize_chunk_size_and_thread_limit(
        100,
        index.entries().len().into(),
        options.thread_limit,
        None,
    );

    let state = fs::cache::State::for_checkout(options.overwrite_existing, options.attribute_globals.clone().into());
    let attribute_files = state.build_attribute_list(index, paths, case);
    let mut ctx = chunk::Context {
        buf: Vec::new(),
        path_cache: fs::Cache::new(dir, state, case, Vec::with_capacity(512), attribute_files),
        find,
        options,
        num_files: &num_files,
    };

    let chunk::Outcome {
        mut collisions,
        mut errors,
        mut bytes_written,
        delayed,
    } = if num_threads == 1 {
        let entries_with_paths = interrupt::Iter::new(index.entries_mut_with_paths_in(paths), should_interrupt);
        chunk::process(entries_with_paths, files, bytes, &mut ctx)?
    } else {
        let entries_with_paths = interrupt::Iter::new(index.entries_mut_with_paths_in(paths), should_interrupt);
        in_parallel(
            gix_features::iter::Chunks {
                inner: entries_with_paths,
                size: chunk_size,
            },
            thread_limit,
            {
                let ctx = ctx.clone();
                move |_| (progress::Discard, progress::Discard, ctx.clone())
            },
            |chunk, (files, bytes, ctx)| chunk::process(chunk.into_iter(), files, bytes, ctx),
            chunk::Reduce {
                files,
                bytes,
                num_files: &num_files,
                aggregate: Default::default(),
                marker: Default::default(),
            },
        )?
    };

    for (entry, entry_path) in delayed {
        bytes_written += chunk::checkout_entry_handle_result(
            entry,
            entry_path,
            &mut errors,
            &mut collisions,
            files,
            bytes,
            &mut ctx,
        )? as u64;
    }

    Ok(checkout::Outcome {
        files_updated: num_files.load(Ordering::Relaxed),
        collisions,
        errors,
        bytes_written,
    })
}

mod chunk {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use bstr::BStr;
    use gix_features::progress::Progress;
    use gix_hash::oid;

    use crate::{
        fs, index,
        index::{checkout, entry},
        os,
    };

    mod reduce {
        use std::{
            marker::PhantomData,
            sync::atomic::{AtomicUsize, Ordering},
        };

        use gix_features::progress::Progress;

        use crate::index::checkout;

        pub struct Reduce<'a, 'entry, P1, P2, E> {
            pub files: &'a mut P1,
            pub bytes: &'a mut P2,
            pub num_files: &'a AtomicUsize,
            pub aggregate: super::Outcome<'entry>,
            pub marker: PhantomData<E>,
        }

        impl<'a, 'entry, P1, P2, E> gix_features::parallel::Reduce for Reduce<'a, 'entry, P1, P2, E>
        where
            P1: Progress,
            P2: Progress,
            E: std::error::Error + Send + Sync + 'static,
        {
            type Input = Result<super::Outcome<'entry>, checkout::Error<E>>;
            type FeedProduce = ();
            type Output = super::Outcome<'entry>;
            type Error = checkout::Error<E>;

            fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
                let item = item?;
                let super::Outcome {
                    bytes_written,
                    delayed,
                    errors,
                    collisions,
                } = item;
                self.aggregate.bytes_written += bytes_written;
                self.aggregate.delayed.extend(delayed);
                self.aggregate.errors.extend(errors);
                self.aggregate.collisions.extend(collisions);

                self.bytes.set(self.aggregate.bytes_written as usize);
                self.files.set(self.num_files.load(Ordering::Relaxed));

                Ok(())
            }

            fn finalize(self) -> Result<Self::Output, Self::Error> {
                Ok(self.aggregate)
            }
        }
    }
    pub use reduce::Reduce;

    #[derive(Default)]
    pub struct Outcome<'a> {
        pub collisions: Vec<checkout::Collision>,
        pub errors: Vec<checkout::ErrorRecord>,
        pub delayed: Vec<(&'a mut gix_index::Entry, &'a BStr)>,
        pub bytes_written: u64,
    }

    #[derive(Clone)]
    pub struct Context<'a, Find: Clone> {
        pub find: Find,
        pub path_cache: fs::Cache,
        pub buf: Vec<u8>,
        pub options: checkout::Options,
        /// We keep these shared so that there is the chance for printing numbers that aren't looking like
        /// multiple of chunk sizes. Purely cosmetic. Otherwise it's the same as `files`.
        pub num_files: &'a AtomicUsize,
    }

    pub fn process<'entry, Find, E>(
        entries_with_paths: impl Iterator<Item = (&'entry mut gix_index::Entry, &'entry BStr)>,
        files: &mut impl Progress,
        bytes: &mut impl Progress,
        ctx: &mut Context<'_, Find>,
    ) -> Result<Outcome<'entry>, checkout::Error<E>>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E> + Clone,
        E: std::error::Error + Send + Sync + 'static,
    {
        let mut delayed = Vec::new();
        let mut collisions = Vec::new();
        let mut errors = Vec::new();
        let mut bytes_written = 0;

        for (entry, entry_path) in entries_with_paths {
            // TODO: write test for that
            if entry.flags.contains(gix_index::entry::Flags::SKIP_WORKTREE) {
                files.inc();
                continue;
            }

            // Symlinks always have to be delayed on windows as they have to point to something that exists on creation.
            // And even if not, there is a distinction between file and directory symlinks, hence we have to check what the target is
            // before creating it.
            // And to keep things sane, we just do the same on non-windows as well which is similar to what git does and adds some safety
            // around writing through symlinks (even though we handle this).
            // This also means that we prefer content in files over symlinks in case of collisions, which probably is for the better, too.
            if entry.mode == gix_index::entry::Mode::SYMLINK {
                delayed.push((entry, entry_path));
                continue;
            }

            bytes_written +=
                checkout_entry_handle_result(entry, entry_path, &mut errors, &mut collisions, files, bytes, ctx)?
                    as u64;
        }

        Ok(Outcome {
            bytes_written,
            errors,
            collisions,
            delayed,
        })
    }

    pub fn checkout_entry_handle_result<Find, E>(
        entry: &mut gix_index::Entry,
        entry_path: &BStr,
        errors: &mut Vec<checkout::ErrorRecord>,
        collisions: &mut Vec<checkout::Collision>,
        files: &mut impl Progress,
        bytes: &mut impl Progress,
        Context {
            find,
            path_cache,
            buf,
            options,
            num_files,
        }: &mut Context<'_, Find>,
    ) -> Result<usize, checkout::Error<E>>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Result<gix_object::BlobRef<'a>, E> + Clone,
        E: std::error::Error + Send + Sync + 'static,
    {
        let res = entry::checkout(
            entry,
            entry_path,
            entry::Context { find, path_cache, buf },
            options.clone(),
        );
        files.inc();
        num_files.fetch_add(1, Ordering::SeqCst);
        match res {
            Ok(object_size) => {
                bytes.inc_by(object_size);
                Ok(object_size)
            }
            Err(index::checkout::Error::Io(err)) if os::indicates_collision(&err) => {
                // We are here because a file existed or was blocked by a directory which shouldn't be possible unless
                // we are on a file insensitive file system.
                files.fail(format!("{}: collided ({:?})", entry_path, err.kind()));
                collisions.push(checkout::Collision {
                    path: entry_path.into(),
                    error_kind: err.kind(),
                });
                Ok(0)
            }
            Err(err) => {
                if options.keep_going {
                    errors.push(checkout::ErrorRecord {
                        path: entry_path.into(),
                        error: Box::new(err),
                    });
                    Ok(0)
                } else {
                    Err(err)
                }
            }
        }
    }
}
