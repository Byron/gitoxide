use crate::{hash, loose, pack, pack::index::write::types::ObjectKind};
use git_object::{owned, HashKind};
use std::io;

pub fn base(entry: &mut pack::index::write::types::TreeEntry, decompressed: &[u8], hash: HashKind) -> git_object::Kind {
    let base_kind = entry.kind.to_kind().expect("base object as source of iteration");
    let id = compute_hash(base_kind, &decompressed, hash);
    entry.id = id;
    base_kind
}

pub fn child(entry: &mut pack::index::write::types::TreeEntry, base_kind: git_object::Kind) {
    entry.kind = ObjectKind::Base(base_kind);
}

fn compute_hash(kind: git_object::Kind, bytes: &[u8], hash_kind: HashKind) -> owned::Id {
    let mut write = hash::Write::new(io::sink(), hash_kind);
    loose::object::header::encode(kind, bytes.len() as u64, &mut write).expect("write to sink and hash cannot fail");
    write.hash.update(bytes);
    owned::Id::from(write.hash.digest())
}
