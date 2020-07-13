#![forbid(unsafe_code)]

use anyhow::{anyhow, Context as AnyhowContext, Result};
use bytesize::ByteSize;
use git_features::progress::Progress;
use git_object::Kind;
use git_odb::pack::{self, index};
use std::str::FromStr;
use std::{io, path::Path};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum OutputFormat {
    Human,
    #[cfg(feature = "serde1")]
    Json,
}

impl OutputFormat {
    pub fn variants() -> &'static [&'static str] {
        &[
            "human",
            #[cfg(feature = "serde1")]
            "json",
        ]
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lc = s.to_ascii_lowercase();
        Ok(match s_lc.as_str() {
            "human" => OutputFormat::Human,
            #[cfg(feature = "serde1")]
            "json" => OutputFormat::Json,
            _ => return Err(format!("Invalid output format: '{}'", s)),
        })
    }
}

/// A general purpose context for many operations provided here
pub struct Context<W1: io::Write, W2: io::Write> {
    /// If set, provide statistics to `out` in the given format
    pub output_statistics: Option<OutputFormat>,
    /// A stream to which to output operation results
    pub out: W1,
    /// A stream to which to errors
    pub err: W2,
    /// If set, don't use more than this amount of threads.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
}

impl Default for Context<Vec<u8>, Vec<u8>> {
    fn default() -> Self {
        Context {
            output_statistics: None,
            thread_limit: None,
            out: Vec::new(),
            err: Vec::new(),
        }
    }
}

pub fn init() -> Result<()> {
    git_repository::init::repository().with_context(|| "Repository initialization failed")
}

enum EitherCache {
    Left(pack::cache::DecodeEntryNoop),
    Right(pack::cache::DecodeEntryLRU),
}

impl pack::cache::DecodeEntry for EitherCache {
    fn put(&mut self, offset: u64, data: &[u8], kind: Kind, compressed_size: usize) {
        match self {
            EitherCache::Left(v) => v.put(offset, data, kind, compressed_size),
            EitherCache::Right(v) => v.put(offset, data, kind, compressed_size),
        }
    }

    fn get(&mut self, offset: u64, out: &mut Vec<u8>) -> Option<(Kind, usize)> {
        match self {
            EitherCache::Left(v) => v.get(offset, out),
            EitherCache::Right(v) => v.get(offset, out),
        }
    }
}

pub fn verify_pack_or_pack_index<P, W1, W2>(
    path: impl AsRef<Path>,
    progress: Option<P>,
    Context {
        mut out,
        mut err,
        output_statistics,
        thread_limit,
    }: Context<W1, W2>,
) -> Result<(git_object::Id, Option<index::verify::Outcome>)>
where
    P: Progress,
    <P as Progress>::SubProgress: Send,
    W1: io::Write,
    W2: io::Write,
{
    let path = path.as_ref();
    let ext = path.extension().and_then(|ext| ext.to_str()).ok_or_else(|| {
        anyhow!(
            "Cannot determine file type on path without extension '{}', expecting default extensions 'idx' and 'pack'",
            path.display()
        )
    })?;
    let res = match ext {
        "pack" => {
            let pack = git_odb::pack::File::at(path).with_context(|| "Could not open pack file")?;
            pack.verify_checksum().map(|id| (id, None))?
        }
        "idx" => {
            let idx = git_odb::pack::index::File::at(path).with_context(|| "Could not open pack index file")?;
            let packfile_path = path.with_extension("pack");
            let pack = git_odb::pack::File::at(&packfile_path)
                .or_else(|e| {
                    writeln!(
                        err,
                        "Could not find matching pack file at '{}' - only index file will be verified, error was: {}",
                        packfile_path.display(),
                        e
                    )
                    .ok();
                    Err(e)
                })
                .ok();
            idx.verify_checksum_of_index(pack.as_ref(), thread_limit, progress, || -> EitherCache {
                if output_statistics.is_some() {
                    // turn off acceleration as we need to see entire chains all the time
                    EitherCache::Left(pack::cache::DecodeEntryNoop)
                } else {
                    EitherCache::Right(pack::cache::DecodeEntryLRU::default())
                }
            })?
        }
        ext => return Err(anyhow!("Unknown extension {:?}, expecting 'idx' or 'pack'", ext)),
    };
    if let Some(stats) = res.1.as_ref() {
        match output_statistics {
            Some(OutputFormat::Human) => drop(print_statistics(&mut out, stats)),
            #[cfg(feature = "serde1")]
            Some(OutputFormat::Json) => drop(serde_json::to_writer_pretty(out, stats)),
            _ => {}
        };
    }
    Ok(res)
}

fn print_statistics(out: &mut impl io::Write, stats: &index::verify::Outcome) -> io::Result<()> {
    writeln!(out, "objects per delta chain length")?;
    let mut chain_length_to_object: Vec<_> = stats.objects_per_chain_length.iter().map(|(a, b)| (*a, *b)).collect();
    chain_length_to_object.sort_by_key(|e| e.0);
    let mut total_object_count = 0;
    for (chain_length, object_count) in chain_length_to_object.into_iter() {
        total_object_count += object_count;
        writeln!(out, "\t{:>2}: {}", chain_length, object_count)?;
    }
    writeln!(out, "\t->: {}", total_object_count)?;

    let pack::decode::DecodeEntryOutcome {
        kind: _,
        num_deltas,
        decompressed_size,
        compressed_size,
        object_size,
    } = stats.average;

    let width = 30;
    writeln!(out, "\naverages")?;
    #[rustfmt::skip]
    writeln!(
        out,
        "\t{:<width$} {};\n\t{:<width$} {};\n\t{:<width$} {};\n\t{:<width$} {};",
        "delta chain length:", num_deltas,
        "decompressed entry [B]:", decompressed_size,
        "compressed entry [B]:", compressed_size,
        "decompressed object size [B]:", object_size,
        width = width
    )?;

    writeln!(out, "\ncompression")?;
    #[rustfmt::skip]
    writeln!(
        out, "\t{:<width$}: {}\n\t{:<width$}: {}\n\t{:<width$}: {}\n\t{:<width$}: {}",
        "compressed entries size", ByteSize(stats.total_compressed_entries_size),
        "decompressed entries size", ByteSize(stats.total_decompressed_entries_size),
        "total object size", ByteSize(stats.total_object_size),
        "pack size", ByteSize(stats.pack_size),
        width = width
    )?;
    let compression_ratio = stats.total_decompressed_entries_size as f64 / stats.total_compressed_entries_size as f64;
    let delta_compression_ratio = stats.total_object_size as f64 / stats.total_compressed_entries_size as f64;
    #[rustfmt::skip]
    writeln!(
        out,
        "\n\t{:<width$}: {:.2}\n\t{:<width$}: {:.2}\n\t{:<width$}: {:.2}\n\t{:<width$}: {:.3}%",
        "compression ratio", compression_ratio,
        "delta compression ratio", delta_compression_ratio,
        "delta gain", delta_compression_ratio / compression_ratio,
        "pack overhead", (1.0 - (stats.total_compressed_entries_size as f64 / stats.pack_size as f64)) * 100.0,
        width = width
    )?;
    Ok(())
}
