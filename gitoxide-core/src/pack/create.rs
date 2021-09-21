use std::{convert::TryFrom, ffi::OsStr, io, path::Path, str::FromStr, sync::Arc, time::Instant};

use anyhow::anyhow;
use git_repository as git;
use git_repository::{
    hash,
    hash::ObjectId,
    interrupt,
    objs::bstr::ByteVec,
    odb::{pack, pack::cache::DecodeEntry},
    prelude::{Finalize, ReferenceAccessExt},
    progress, traverse, Progress,
};

use crate::OutputFormat;
use git_repository::odb::FindExt;

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

#[derive(PartialEq, Debug)]
pub enum ObjectExpansion {
    None,
    TreeTraversal,
    TreeDiff,
}

impl ObjectExpansion {
    pub fn variants() -> &'static [&'static str] {
        &["none", "tree-traversal", "tree-diff"]
    }
}

impl Default for ObjectExpansion {
    fn default() -> Self {
        ObjectExpansion::None
    }
}

impl FromStr for ObjectExpansion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ObjectExpansion::*;
        let slc = s.to_ascii_lowercase();
        Ok(match slc.as_str() {
            "none" => None,
            "tree-traversal" => TreeTraversal,
            "tree-diff" => TreeDiff,
            _ => return Err("invalid value".into()),
        })
    }
}

impl From<ObjectExpansion> for pack::data::output::count::objects::ObjectExpansion {
    fn from(v: ObjectExpansion) -> Self {
        use pack::data::output::count::objects::ObjectExpansion::*;
        match v {
            ObjectExpansion::None => AsIs,
            ObjectExpansion::TreeTraversal => TreeContents,
            ObjectExpansion::TreeDiff => TreeAdditionsComparedToAncestor,
        }
    }
}

/// A general purpose context for many operations provided here
pub struct Context<W> {
    /// The way input objects should be handled
    pub expansion: ObjectExpansion,
    /// If set, use `tread_limit` to accelerate the counting phase at the cost of loosing determinism as the order of objects
    /// during expansion changes with multiple threads unless no expansion is performed. In the latter case, this flag
    /// has no effect.
    /// If unset, counting will only use one thread and thus yield the same sequence of objects in any case.
    /// If the `thread_limit` is 1, the count is always deterministic.
    pub nondeterministic_count: bool,
    /// If true, delta objects may refer to their base as reference, allowing it not to be included in the created back.
    /// Otherwise these have to be recompressed in order to make the pack self-contained.
    pub thin: bool,
    /// If set, don't use more than this amount of threads.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
    /// If set, statistics about the operation will be written to the output stream.
    pub statistics: Option<OutputFormat>,
    /// The output stream for use of additional information
    pub out: W,
}

pub fn create<W>(
    repository_path: impl AsRef<Path>,
    tips: impl IntoIterator<Item = impl AsRef<OsStr>>,
    input: Option<impl io::BufRead + Send + 'static>,
    output_directory: Option<impl AsRef<Path>>,
    mut progress: impl Progress,
    Context {
        expansion,
        nondeterministic_count,
        thin,
        thread_limit,
        statistics,
        mut out,
    }: Context<W>,
) -> anyhow::Result<()>
where
    W: std::io::Write,
{
    let repo = git::discover(repository_path)?;
    progress.init(Some(2), progress::steps());
    let tips = tips.into_iter();
    let make_cancellation_err = || anyhow!("Cancelled by user");
    let (odb, input): (
        _,
        Box<dyn Iterator<Item = Result<ObjectId, input_iteration::Error>> + Send>,
    ) = match input {
        None => {
            use git::bstr::ByteSlice;
            use os_str_bytes::OsStrBytes;
            let mut progress = progress.add_child("traversing");
            let repo = repo.into_easy();
            progress.init(None, progress::count("commits"));
            let tips = tips
                .map(|tip| {
                    ObjectId::from_hex(&Vec::from_os_str_lossy(tip.as_ref())).or_else(|_| {
                        repo.find_reference(tip.as_ref().to_raw_bytes().as_bstr())
                            .map_err(anyhow::Error::from)
                            .and_then(|r| r.into_fully_peeled_id().map(|oid| oid.detach()).map_err(Into::into))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let odb = Arc::new(git::Repository::try_from(repo)?.odb);
            let iter = Box::new(
                traverse::commit::Ancestors::new(tips, traverse::commit::ancestors::State::default(), {
                    let db = Arc::clone(&odb);
                    move |oid, buf| db.find_commit_iter(oid, buf, &mut pack::cache::Never).ok()
                })
                .map(|res| res.map_err(Into::into))
                .inspect(move |_| progress.inc()),
            );
            (odb, iter)
        }
        Some(input) => {
            let mut progress = progress.add_child("iterating");
            progress.init(None, progress::count("objects"));
            let iter = Box::new(
                input
                    .lines()
                    .map(|hex_id| {
                        hex_id
                            .map_err(Into::into)
                            .and_then(|hex_id| ObjectId::from_hex(hex_id.as_bytes()).map_err(Into::into))
                    })
                    .inspect(move |_| progress.inc()),
            );
            (Arc::new(repo.odb), iter)
        }
    };

    let mut stats = Statistics::default();
    let chunk_size = 1000; // What's a good value for this?
    let counts = {
        let mut progress = progress.add_child("counting");
        progress.init(None, progress::count("objects"));
        let may_use_multiple_threads = nondeterministic_count || matches!(expansion, ObjectExpansion::None);
        let thread_limit = if may_use_multiple_threads {
            thread_limit
        } else {
            Some(1)
        };
        let make_cache = move || {
            if may_use_multiple_threads {
                Box::new(pack::cache::Never) as Box<dyn DecodeEntry>
            } else {
                Box::new(pack::cache::lru::MemoryCappedHashmap::new(512 * 1024 * 1024)) as Box<dyn DecodeEntry>
                // todo: Make that configurable
            }
        };
        let progress = progress::ThroughputOnDrop::new(progress);
        let input_object_expansion = expansion.into();
        let total_object_cache_size = 50 * 1024 * 1024; // TODO: make configurable
        let (mut counts, count_stats) = if may_use_multiple_threads {
            pack::data::output::count::objects(
                Arc::clone(&odb),
                make_cache,
                input,
                progress,
                &interrupt::IS_INTERRUPTED,
                pack::data::output::count::objects::Options {
                    thread_limit,
                    chunk_size,
                    input_object_expansion,
                    object_cache_size_in_bytes: total_object_cache_size / thread_limit.unwrap_or(1),
                },
            )?
        } else {
            pack::data::output::count::objects_unthreaded(
                Arc::clone(&odb),
                &mut make_cache(),
                input,
                progress,
                &interrupt::IS_INTERRUPTED,
                input_object_expansion,
                total_object_cache_size,
            )?
        };
        stats.counts = count_stats;
        counts.shrink_to_fit();
        counts
    };

    progress.inc();
    let num_objects = counts.len();
    let mut in_order_entries = {
        let progress = progress.add_child("creating entries");
        pack::data::output::InOrderIter::from(pack::data::output::entry::iter_from_counts(
            counts,
            Arc::clone(&odb),
            || pack::cache::Never,
            progress,
            pack::data::output::entry::iter_from_counts::Options {
                thread_limit,
                mode: pack::data::output::entry::iter_from_counts::Mode::PackCopyAndBaseObjects,
                allow_thin_pack: thin,
                chunk_size,
                version: Default::default(),
            },
        ))
    };

    let mut entries_progress = progress.add_child("consuming");
    entries_progress.init(Some(num_objects), progress::count("entries"));
    let mut write_progress = progress.add_child("writing");
    write_progress.init(None, progress::bytes());
    let start = Instant::now();

    let mut named_tempfile_store: Option<tempfile::NamedTempFile> = None;
    let mut sink_store: std::io::Sink;
    let (mut pack_file, output_directory): (&mut dyn std::io::Write, Option<_>) = match output_directory {
        Some(dir) => {
            named_tempfile_store = Some(tempfile::NamedTempFile::new_in(dir.as_ref())?);
            (named_tempfile_store.as_mut().expect("packfile just set"), Some(dir))
        }
        None => {
            sink_store = std::io::sink();
            (&mut sink_store, None)
        }
    };
    let mut interruptible_output_iter = interrupt::Iter::new(
        pack::data::output::bytes::FromEntriesIter::new(
            in_order_entries.by_ref().inspect(|e| {
                if let Ok(entries) = e {
                    entries_progress.inc_by(entries.len())
                }
            }),
            &mut pack_file,
            num_objects as u32,
            pack::data::Version::default(),
            hash::Kind::default(),
        ),
        make_cancellation_err,
    );
    for io_res in interruptible_output_iter.by_ref() {
        let written = io_res??;
        write_progress.inc_by(written as usize);
    }

    let hash = interruptible_output_iter
        .into_inner()
        .digest()
        .expect("iteration is done");
    let pack_name = format!("{}.pack", hash);
    if let (Some(pack_file), Some(dir)) = (named_tempfile_store.take(), output_directory) {
        pack_file.persist(dir.as_ref().join(pack_name))?;
    } else {
        writeln!(out, "{}", pack_name)?;
    }
    stats.entries = in_order_entries.inner.finalize()?;

    write_progress.show_throughput(start);
    entries_progress.show_throughput(start);

    if let Some(format) = statistics {
        print(stats, format, out)?;
    }
    progress.inc();
    Ok(())
}

fn print(stats: Statistics, format: OutputFormat, out: impl std::io::Write) -> anyhow::Result<()> {
    match format {
        OutputFormat::Human => human_output(stats, out).map_err(Into::into),
        #[cfg(feature = "serde1")]
        OutputFormat::Json => serde_json::to_writer_pretty(out, &stats).map_err(Into::into),
    }
}

fn human_output(
    Statistics {
        counts:
            pack::data::output::count::objects::Outcome {
                input_objects,
                expanded_objects,
                decoded_objects,
                total_objects,
            },
        entries:
            pack::data::output::entry::iter_from_counts::Outcome {
                decoded_and_recompressed_objects,
                missing_objects,
                objects_copied_from_pack,
                ref_delta_objects,
            },
    }: Statistics,
    mut out: impl std::io::Write,
) -> std::io::Result<()> {
    let width = 30;
    writeln!(out, "counting phase")?;
    #[rustfmt::skip]
    writeln!(
        out,
        "\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}",
        "input objects", input_objects,
        "expanded objects", expanded_objects,
        "decoded objects", decoded_objects,
        "total objects", total_objects,
        width = width
    )?;
    writeln!(out, "generation phase")?;
    #[rustfmt::skip]
    writeln!(
        out,
        "\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}",
        "decoded and recompressed", decoded_and_recompressed_objects,
        "pack-to-pack copies", objects_copied_from_pack,
        "ref-delta-objects", ref_delta_objects,
        "missing objects", missing_objects,
        width = width
    )?;
    Ok(())
}

#[derive(Default)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
struct Statistics {
    counts: pack::data::output::count::objects::Outcome,
    entries: pack::data::output::entry::iter_from_counts::Outcome,
}

pub mod input_iteration {
    use git_repository::{hash, traverse};
    use quick_error::quick_error;
    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Iteration(err: traverse::commit::ancestors::Error) {
                display("input objects couldn't be iterated completely")
                from()
                source(err)
            }
            InputLinesIo(err: std::io::Error) {
                display("An error occurred while reading hashes from standard input")
                from()
                source(err)
            }
            HashDecode(err: hash::decode::Error) {
                display("Could not decode hex hash provided on standard input")
                from()
                source(err)
            }
        }
    }
}
