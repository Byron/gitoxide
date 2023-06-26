use bstr::{BStr, ByteSlice, ByteVec};
use std::borrow::Cow;
use std::ops::Range;

/// Undo identifiers like `$Id:<hexsha>$` to `$Id$`. Newlines between dollars are ignored.
pub fn undo(mut input: Cow<'_, BStr>) -> Cow<'_, BStr> {
    fn find_range(input: &[u8]) -> Option<Range<usize>> {
        let mut ofs = 0;
        loop {
            let mut cursor = input.get(ofs..)?;
            let start = cursor.find(b"$Id:")?;
            cursor = cursor.get((start + 4)..)?;
            let maybe_end = cursor.find_byteset(b"$\n")?;
            if cursor[maybe_end] == b'\n' {
                ofs += start + 4 + maybe_end + 1;
                continue;
            } else {
                return Some((ofs + start)..(ofs + start + 4 + maybe_end + 1));
            }
        }
    }

    let mut ofs = 0;
    while let Some(range) = find_range(&input[ofs..]) {
        input
            .to_mut()
            .replace_range((range.start + ofs)..(range.end + ofs), b"$Id$");
        ofs += range.start + 4;
    }
    input
}

/// Substitute all occurrences of `$Id$` with `$Id: <hexsha-of-input>$` if present and return the changed buffer, with `object_hash`
/// being used accordingly.
///
/// ### Deviation
///
/// `Git` also tries to cleanup 'stray' substituted `$Id: <hex>$`, but we don't do that, sticking exactly to what ought to be done.
/// The respective code is up to 16 years old and one might assume that `git` by now handles checking and checkout filters correctly.
pub fn apply(mut input: Cow<'_, BStr>, object_hash: gix_hash::Kind) -> Cow<'_, BStr> {
    let mut buf: [u8; b": $".len() + gix_hash::Kind::longest().len_in_hex()] = std::array::from_fn(|_| 0);
    let mut id = None;
    let mut ofs = 0;
    while let Some(pos) = input[ofs..].find(b"$Id$") {
        let id = id.get_or_insert_with(|| gix_object::compute_hash(object_hash, gix_object::Kind::Blob, &input));

        buf[..2].copy_from_slice(b": ");
        let _ = id.hex_to_buf(&mut buf[2..][..object_hash.len_in_hex()]);
        let replaced_id = &mut buf[..2 + object_hash.len_in_hex() + 1];
        *replaced_id.last_mut().expect("present") = b'$';
        input
            .to_mut()
            .replace_range((ofs + pos + 3)..(ofs + pos + 4), &*replaced_id);
        ofs += pos + 3 + replaced_id.len();
    }
    input
}
