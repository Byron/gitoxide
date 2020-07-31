use crate::{hash, pack, pack::index::V2_SIGNATURE};
use byteorder::{BigEndian, WriteBytesExt};
use git_object::owned;
use std::io;

pub(crate) fn to_write(
    out: impl io::Write,
    _entries_sorted_by_oid: Vec<(u64, owned::Id, u32)>,
    kind: pack::index::Kind,
) -> io::Result<owned::Id> {
    use io::Write;

    // Write header
    let mut out = hash::Write::new(out, kind.hash());
    out.write_all(V2_SIGNATURE)?;
    out.write_u32::<BigEndian>(kind as u32)?;

    // todo: write fanout
    Ok(out.hash.digest().into())
}
