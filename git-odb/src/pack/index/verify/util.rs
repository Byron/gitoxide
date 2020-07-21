use crate::pack;

pub fn index_entries_sorted_by_offset_ascending(idx: &pack::index::File) -> Vec<pack::index::Entry> {
    let mut v: Vec<_> = idx.iter().collect();
    v.sort_by_key(|e| e.pack_offset);
    v
}
