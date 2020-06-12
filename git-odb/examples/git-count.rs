use anyhow::{anyhow, Context, Result};
extern crate git_odb as odb;

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

    let (mut deltas, mut commits, mut trees, mut blobs, mut tags) = (0, 0, 0, 0, 0);
    for entry in index.iter() {
        match pack.entry(entry.offset).object {
            Commit => commits += 1,
            Tag => tags += 1,
            Tree => trees += 1,
            Blob => blobs += 1,
            OfsDelta { .. } => deltas += 1,
            RefDelta { .. } => deltas += 1,
        }
    }
    writeln!(
        stdout(),
        "commits: {}, trees: {}, blobs: {}, tags: {}, deltas: {}",
        commits,
        trees,
        blobs,
        tags,
        deltas,
    )
    .map_err(Into::into)
}

fn main() -> Result<()> {
    run().with_context(|| "Failed to count git objects")
}
