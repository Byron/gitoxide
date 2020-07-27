use crate::pack;
use git_features::progress::Progress;
use std::time::{Instant, SystemTime};

pub(crate) fn index_entries_sorted_by_offset_ascending(
    idx: &pack::index::File,
    mut progress: impl Progress,
) -> Vec<pack::index::Entry> {
    progress.init(Some(idx.num_objects), Some("entries"));
    let then = SystemTime::now();

    let mut v = Vec::with_capacity(idx.num_objects as usize);
    for entry in idx.iter() {
        v.push(entry);
        progress.inc();
    }
    v.sort_by_key(|e| e.pack_offset);

    let elapsed = then.elapsed().expect("system time").as_secs_f32();
    progress.info(format!(
        "in {:.02}s ({} entries/s)",
        elapsed,
        idx.num_objects as f32 / elapsed
    ));
    v
}

pub(crate) struct TimeThroughput {
    then: Instant,
    byte_size: usize,
}

impl TimeThroughput {
    pub fn new(byte_size: usize) -> TimeThroughput {
        TimeThroughput {
            then: Instant::now(),
            byte_size,
        }
    }
}

impl Into<String> for TimeThroughput {
    fn into(self) -> String {
        let time_taken = std::time::Instant::now().duration_since(self.then).as_secs_f32();
        format!(
            "finished in {:.2}s at {}/s",
            time_taken,
            bytesize::ByteSize((self.byte_size as f32 / time_taken) as u64)
        )
    }
}
