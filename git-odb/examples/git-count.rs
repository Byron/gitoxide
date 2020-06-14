use anyhow::{anyhow, Context, Result};
use git_odb as odb;
use rayon::prelude::*;

use std::{
    env,
    io::{stdout, Write},
};

fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let (index, pack) = match (args.next(), args.next()) {
        (Some(index), Some(pack)) => (index, pack),
        _ => {
            return Err(anyhow!(
                "USAGE: {} <index-file> <pack-file>",
                env::current_exe()?.display()
            ))
        }
    };
    let index = odb::pack::index::File::at(index)?;
    let pack = odb::pack::File::at(pack)?;
    use odb::pack::parsed::Object::*;

    writeln!(
        stdout(),
        "pack: kind = {:?}, num_objects = {}",
        pack.kind(),
        pack.num_objects()
    )?;
    writeln!(
        stdout(),
        "index: kind = {:?}, num_objects = {}, version = {}, checksum_of_index = {}, checksum_of_pack = {}",
        index.kind(),
        index.num_objects(),
        index.version(),
        hex::encode(index.checksum_of_index()),
        hex::encode(index.checksum_of_pack()),
    )?;

    let (deltas, commits, trees, blobs, tags) = index
        .iter()
        .collect::<Vec<_>>()
        .par_iter()
        .fold(
            || (0, 0, 0, 0, 0),
            |(mut deltas, mut commits, mut trees, mut blobs, mut tags): (
                u32,
                u32,
                u32,
                u32,
                u32,
            ),
             entry| {
                match pack.entry(entry.offset).object {
                    Commit => commits += 1,
                    Tag => tags += 1,
                    Tree => trees += 1,
                    Blob => blobs += 1,
                    OfsDelta { .. } => deltas += 1,
                    RefDelta { .. } => deltas += 1,
                };
                (deltas, commits, trees, blobs, tags)
            },
        )
        .reduce(
            || (0, 0, 0, 0, 0),
            |l, r| (l.0 + r.0, l.1 + r.1, l.2 + r.2, l.3 + r.3, l.4 + r.4),
        );
    writeln!(
        stdout(),
        "commits: {}, trees: {}, blobs: {}, tags: {}, deltas: {} == {}",
        commits,
        trees,
        blobs,
        tags,
        deltas,
        commits + trees + blobs + tags + deltas
    )?;
    Ok(())
}

fn main() -> Result<()> {
    run().with_context(|| "Failed to count git objects")
}
