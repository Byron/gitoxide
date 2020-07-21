use crate::pack;
use git_features::progress::Progress;
use std::time::SystemTime;

pub fn index_entries_sorted_by_offset_ascending(
    idx: &pack::index::File,
    mut progress: impl Progress,
) -> Vec<pack::index::Entry> {
    progress.init(Some(idx.num_objects), Some("entries"));
    let then = SystemTime::now();

    let mut count = 0;
    let mut v: Vec<_> = idx
        .iter()
        .inspect(|_| {
            count += 1;
            progress.set(count);
        })
        .collect();
    v.sort_by_key(|e| e.pack_offset);

    let elapsed = then.elapsed().expect("system time").as_secs_f32();
    progress.info(format!(
        "in {:.02}s ({} entries/s)",
        elapsed,
        idx.num_objects as f32 / elapsed
    ));
    v
}
