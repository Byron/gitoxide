use crate::OutputFormat;
use anyhow::anyhow;
use git_repository::{
    hash,
    hash::ObjectId,
    interrupt,
    object::bstr::ByteVec,
    odb::{linked, pack},
    prelude::{Finalize, FindExt},
    progress, traverse, Progress,
};
use std::{ffi::OsStr, io, path::Path, str::FromStr, sync::Arc, time::Instant};

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

impl From<ObjectExpansion> for pack::data::output::count::from_objects_iter::ObjectExpansion {
    fn from(v: ObjectExpansion) -> Self {
        use pack::data::output::count::from_objects_iter::ObjectExpansion::*;
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
    repository: impl AsRef<Path>,
    tips: impl IntoIterator<Item = impl AsRef<OsStr>>,
    input: Option<impl io::BufRead + Send + 'static>,
    output_directory: Option<impl AsRef<Path>>,
    mut progress: impl Progress,
    Context {
        expansion,
        nondeterministic_count,
        thread_limit,
        statistics,
        mut out,
    }: Context<W>,
) -> anyhow::Result<()>
where
    W: std::io::Write,
{
    let db = Arc::new(find_db(repository)?);
    progress.init(Some(4), progress::steps());
    let tips = tips.into_iter();
    let make_cancellation_err = || anyhow!("Cancelled by user");
    let input: Box<dyn Iterator<Item = ObjectId> + Send + 'static> = match input {
        None => Box::new({
            let mut progress = progress.add_child("traversing");
            progress.init(None, progress::count("commits"));
            let start = Instant::now();
            let iter = interrupt::Iter::new(
                traverse::commit::Ancestors::new(
                    tips.map(|t| ObjectId::from_hex(&Vec::from_os_str_lossy(t.as_ref())))
                        .collect::<Result<Vec<_>, _>>()?,
                    traverse::commit::ancestors::State::default(),
                    {
                        let db = Arc::clone(&db);
                        move |oid, buf| db.find_existing_commit_iter(oid, buf, &mut pack::cache::Never).ok()
                    },
                )
                .inspect(|_| progress.inc()),
                make_cancellation_err,
            )
            .collect::<Result<Result<Vec<_>, _>, _>>()??
            .into_iter();
            progress.show_throughput(start);
            iter
        }),
        Some(input) => Box::new(input.lines().filter_map(|hex_id| {
            hex_id
                .ok()
                .and_then(|hex_id| ObjectId::from_hex(hex_id.as_bytes()).ok())
        })),
    };
    progress.inc();

    let mut stats = Statistics::default();
    let chunk_size = 200;
    let start = Instant::now();
    let counts = {
        let mut progress = progress.add_child("counting");
        progress.init(None, progress::count("objects"));
        let mut interruptible_counts_iter = interrupt::Iter::new(
            pack::data::output::count::from_objects_iter(
                Arc::clone(&db),
                pack::cache::lru::StaticLinkedList::<64>::default,
                input,
                progress.add_child("threads"),
                pack::data::output::count::from_objects_iter::Options {
                    thread_limit: if nondeterministic_count || matches!(expansion, ObjectExpansion::None) {
                        thread_limit
                    } else {
                        Some(1)
                    },
                    chunk_size,
                    input_object_expansion: expansion.into(),
                },
            ),
            make_cancellation_err,
        );
        let mut counts = Vec::new();
        for c in interruptible_counts_iter.by_ref() {
            let c = c??;
            progress.inc_by(c.len());
            counts.extend(c.into_iter());
        }
        stats.counts = interruptible_counts_iter.into_inner().finalize()?;
        progress.show_throughput(start);
        counts.shrink_to_fit();
        counts
    };

    progress.inc();
    let num_objects = counts.len();
    let mut in_order_entries = {
        let progress = progress.add_child("creating entries");
        pack::data::output::InOrderIter::from(pack::data::output::entry::from_counts_iter(
            counts,
            Arc::clone(&db),
            pack::cache::lru::StaticLinkedList::<64>::default,
            progress,
            git_repository::odb::data::output::entry::from_counts_iter::Options {
                thread_limit,
                chunk_size,
                version: Default::default(),
            },
        ))
    };

    progress.inc();
    let mut entries_progress = progress.add_child("consumed");
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
    Ok(())
}

fn find_db(repository: impl AsRef<Path>) -> anyhow::Result<linked::Store> {
    let path = repository.as_ref();
    Ok(linked::Store::at(
        git_repository::path::discover::existing(path)?
            .into_repository_directory()
            .join("objects"),
    )?)
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
            pack::data::output::count::from_objects_iter::Outcome {
                input_objects,
                expanded_objects,
                decoded_objects,
                total_objects,
            },
        entries:
            pack::data::output::entry::from_counts_iter::Outcome {
                decoded_and_recompressed_objects,
                objects_copied_from_pack,
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
        "\t{:<width$} {}\n\t{:<width$} {}",
        "decoded and recompressed", decoded_and_recompressed_objects,
        "pack-to-pack copies", objects_copied_from_pack,
        width = width
    )?;
    Ok(())
}

#[derive(Default)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
struct Statistics {
    counts: pack::data::output::count::from_objects_iter::Outcome,
    entries: pack::data::output::entry::from_counts_iter::Outcome,
}
