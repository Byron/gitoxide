use std::io;

use anyhow::bail;

use crate::OutputFormat;

#[cfg_attr(not(feature = "serde"), allow(unused_variables))]
pub fn info(
    repo: gix::Repository,
    format: OutputFormat,
    out: impl io::Write,
    mut err: impl io::Write,
) -> anyhow::Result<()> {
    if format == OutputFormat::Human {
        writeln!(err, "Only JSON is implemented - using that instead")?;
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    pub struct Statistics {
        pub path: std::path::PathBuf,
        pub object_hash: String,
        pub use_multi_pack_index: bool,
        pub structure: Vec<gix::odb::store::structure::Record>,
        pub metrics: gix::odb::store::Metrics,
    }

    let store = repo.objects.store_ref();
    let stats = Statistics {
        path: store.path().into(),
        object_hash: store.object_hash().to_string(),
        use_multi_pack_index: store.use_multi_pack_index(),
        structure: store.structure()?,
        metrics: store.metrics(),
    };

    #[cfg(feature = "serde")]
    {
        serde_json::to_writer_pretty(out, &stats)?;
    }

    Ok(())
}

pub mod statistics {
    use crate::OutputFormat;

    pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 0..=3;

    #[derive(Debug, Copy, Clone)]
    pub struct Options {
        pub format: OutputFormat,
        pub thread_limit: Option<usize>,
    }
}

#[cfg_attr(not(feature = "serde"), allow(unused_variables))]
pub fn statistics(
    repo: gix::Repository,
    mut progress: impl gix::Progress,
    out: impl io::Write,
    mut err: impl io::Write,
    statistics::Options { format, thread_limit }: statistics::Options,
) -> anyhow::Result<()> {
    use bytesize::ByteSize;
    use gix::odb::{find, HeaderExt};

    if format == OutputFormat::Human {
        writeln!(err, "Only JSON is implemented - using that instead")?;
    }

    progress.init(None, gix::progress::count("objects"));
    progress.set_name("counting".into());
    let counter = progress.counter();
    let start = std::time::Instant::now();

    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[derive(Default)]
    struct Statistics {
        total_objects: usize,
        loose_objects: usize,
        packed_objects: usize,
        packed_delta_objects: usize,
        total_delta_chain_length: u64,
        trees: usize,
        trees_size: ByteSize,
        tags: usize,
        tags_size: ByteSize,
        commits: usize,
        commits_size: ByteSize,
        blobs: usize,
        blobs_size: ByteSize,
    }

    impl Statistics {
        fn count(&mut self, kind: gix::object::Kind, size: u64) {
            use gix::object::Kind::*;
            match kind {
                Commit => {
                    self.commits += 1;
                    self.commits_size += size;
                }
                Tree => {
                    self.trees += 1;
                    self.trees_size += size;
                }
                Tag => {
                    self.tags += 1;
                    self.tags_size += size;
                }
                Blob => {
                    self.blobs += 1;
                    self.blobs_size += size;
                }
            }
        }
        fn consume(&mut self, item: gix::odb::find::Header) {
            match item {
                find::Header::Loose { size, kind } => {
                    self.loose_objects += 1;
                    self.count(kind, size)
                }
                find::Header::Packed(packed) => {
                    self.packed_objects += 1;
                    self.packed_delta_objects += usize::from(packed.num_deltas > 0);
                    self.total_delta_chain_length += packed.num_deltas as u64;
                    self.count(packed.kind, packed.object_size);
                }
            }
        }
    }

    #[derive(Default)]
    struct Reduce {
        stats: Statistics,
    }

    impl gix::parallel::Reduce for Reduce {
        type Input = Result<Vec<gix::odb::find::Header>, anyhow::Error>;
        type FeedProduce = ();
        type Output = Statistics;
        type Error = anyhow::Error;

        fn feed(&mut self, items: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
            for item in items? {
                self.stats.consume(item);
            }
            Ok(())
        }

        fn finalize(mut self) -> Result<Self::Output, Self::Error> {
            self.stats.total_objects = self.stats.loose_objects + self.stats.packed_objects;
            Ok(self.stats)
        }
    }

    let cancelled = || anyhow::anyhow!("Cancelled by user");
    let object_ids = repo.objects.store_ref().iter()?.filter_map(Result::ok);
    let chunk_size = 1_000;
    let stats = if gix::parallel::num_threads(thread_limit) > 1 {
        gix::parallel::in_parallel(
            gix::interrupt::Iter::new(
                gix::features::iter::Chunks {
                    inner: object_ids,
                    size: chunk_size,
                },
                cancelled,
            ),
            thread_limit,
            move |_| (repo.objects.clone().into_inner(), counter),
            |ids, (handle, counter)| {
                let ids = ids?;
                counter.fetch_add(ids.len(), std::sync::atomic::Ordering::Relaxed);
                let out = ids
                    .into_iter()
                    .map(|id| handle.header(id))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(out)
            },
            Reduce::default(),
        )?
    } else {
        let mut stats = Statistics::default();

        for (count, id) in object_ids.enumerate() {
            if count % chunk_size == 0 && gix::interrupt::is_triggered() {
                return Err(cancelled());
            }
            stats.consume(repo.objects.header(id)?);
            progress.inc();
        }
        stats
    };

    progress.show_throughput(start);

    #[cfg(feature = "serde")]
    {
        serde_json::to_writer_pretty(out, &stats)?;
    }

    Ok(())
}

pub fn entries(repo: gix::Repository, format: OutputFormat, mut out: impl io::Write) -> anyhow::Result<()> {
    if format != OutputFormat::Human {
        bail!("Only human output format is supported at the moment");
    }

    for object in repo.objects.iter()? {
        let object = object?;
        writeln!(out, "{object}")?;
    }

    Ok(())
}
