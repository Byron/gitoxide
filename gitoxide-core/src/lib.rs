use anyhow::{anyhow, Context, Result};
use bytesize::ByteSize;
use git_features::progress::Progress;
use git_odb::pack::{self, index};
use std::{io, path::Path};

pub fn init() -> Result<()> {
    git_repository::init::repository().with_context(|| "Repository initialization failed")
}

pub fn verify_pack_or_pack_index<P>(
    path: impl AsRef<Path>,
    progress: Option<P>,
    output_statistics: bool,
    mut out: impl io::Write,
    mut err: impl io::Write,
) -> Result<(git_object::Id, Option<index::PackFileChecksumResult>)>
where
    P: Progress,
    <P as Progress>::SubProgress: Send,
{
    let path = path.as_ref();
    let ext = path.extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow!("Cannot determine file type on path without extension '{}', expecting default extensions 'idx' and 'pack'", path.display()))?;
    let res = match ext {
        "pack" => {
            let pack = git_odb::pack::File::at(path).with_context(|| "Could not open pack file")?;
            pack.verify_checksum().map(|id| (id, None))?
        }
        "idx" => {
            let idx = git_odb::pack::index::File::at(path)
                .with_context(|| "Could not open pack index file")?;
            let packfile_path = path.with_extension("pack");
            let pack = git_odb::pack::File::at(&packfile_path)
                .or_else(|e| {
                    writeln!(err, "Could not find matching pack file at '{}' - only index file will be verified, error was: {}", packfile_path.display(), e).ok();
                    Err(e)
                })
                .ok();
            idx.verify_checksum_of_index(pack.as_ref(), progress)?
        }
        ext => {
            return Err(anyhow!(
                "Unknown extension {:?}, expecting 'idx' or 'pack'",
                ext
            ))
        }
    };
    if let Some(stats) = res.1.as_ref() {
        if output_statistics {
            print_statistics(&mut out, stats).ok();
        }
    }
    writeln!(out, "OK")?;
    Ok(res)
}

fn print_statistics(
    out: &mut impl io::Write,
    stats: &index::PackFileChecksumResult,
) -> io::Result<()> {
    writeln!(out, "objects per delta chain length")?;
    let mut chain_length_to_object: Vec<_> = stats
        .objects_per_chain_length
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect();
    chain_length_to_object.sort_by_key(|e| e.0);
    let mut total_object_count = 0;
    for (chain_length, object_count) in chain_length_to_object.into_iter() {
        total_object_count += object_count;
        writeln!(out, "\t{:>2}: {}", chain_length, object_count)?;
    }
    writeln!(out, "\t->: {}", total_object_count)?;

    let pack::DecodeEntryResult {
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
    let compression_ratio =
        stats.total_decompressed_entries_size as f64 / stats.total_compressed_entries_size as f64;
    let delta_compression_ratio =
        stats.total_object_size as f64 / stats.total_compressed_entries_size as f64;
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
