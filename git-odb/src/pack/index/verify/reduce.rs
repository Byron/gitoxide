use crate::pack::{data::decode, index::verify};
use git_features::{parallel, progress::Progress};
use std::time::Instant;

fn add_decode_result(lhs: &mut decode::Outcome, rhs: decode::Outcome) {
    lhs.num_deltas += rhs.num_deltas;
    lhs.decompressed_size += rhs.decompressed_size;
    lhs.compressed_size += rhs.compressed_size;
    lhs.object_size += rhs.object_size;
}

fn div_decode_result(lhs: &mut decode::Outcome, div: usize) {
    lhs.num_deltas = (lhs.num_deltas as f32 / div as f32) as u32;
    lhs.decompressed_size /= div as u64;
    lhs.compressed_size /= div;
    lhs.object_size /= div as u64;
}

pub struct Reducer<'a, P> {
    pub progress: &'a std::sync::Mutex<P>,
    pub then: Instant,
    pub entries_seen: u32,
    pub chunks_seen: usize,
    pub stats: verify::Outcome,
}

impl<'a, P> Reducer<'a, P>
where
    P: Progress,
{
    pub fn from_progress(progress: &'a std::sync::Mutex<P>, pack_data_len_in_bytes: usize) -> Self {
        Reducer {
            progress: &progress,
            then: Instant::now(),
            entries_seen: 0,
            chunks_seen: 0,
            stats: verify::Outcome {
                average: decode::Outcome::default_from_kind(git_object::Kind::Tree),
                objects_per_chain_length: Default::default(),
                total_compressed_entries_size: 0,
                total_decompressed_entries_size: 0,
                total_object_size: 0,
                pack_size: pack_data_len_in_bytes as u64,
            },
        }
    }
}

impl<'a, P> parallel::Reducer for Reducer<'a, P>
where
    P: Progress,
{
    type Input = Result<Vec<decode::Outcome>, verify::Error>;
    type Output = verify::Outcome;
    type Error = verify::Error;

    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        let chunk_stats: Vec<_> = input?;
        let num_entries_in_chunk = chunk_stats.len();
        self.entries_seen += num_entries_in_chunk as u32;
        self.chunks_seen += 1;

        let mut chunk_average = chunk_stats.into_iter().fold(
            decode::Outcome::default_from_kind(git_object::Kind::Tree),
            |mut average, stats| {
                *self.stats.objects_per_chain_length.entry(stats.num_deltas).or_insert(0) += 1;
                self.stats.total_decompressed_entries_size += stats.decompressed_size;
                self.stats.total_compressed_entries_size += stats.compressed_size as u64;
                self.stats.total_object_size += stats.object_size as u64;
                add_decode_result(&mut average, stats);
                average
            },
        );
        div_decode_result(&mut chunk_average, num_entries_in_chunk);
        add_decode_result(&mut self.stats.average, chunk_average);

        self.progress.lock().unwrap().set(self.entries_seen);
        Ok(())
    }

    fn finalize(mut self) -> Result<Self::Output, Self::Error> {
        self.progress.lock().unwrap().done("finished");
        div_decode_result(&mut self.stats.average, self.chunks_seen);
        let elapsed_s = Instant::now().duration_since(self.then).as_secs_f32();
        let objects_per_second = (self.entries_seen as f32 / elapsed_s) as u32;
        self.progress.lock().unwrap().info(format!(
            "Verified {} objects in {:.2}s ({} objects/s, ~{}/s)",
            self.entries_seen,
            elapsed_s,
            objects_per_second,
            bytesize::ByteSize(self.stats.average.object_size * objects_per_second as u64)
        ));
        Ok(self.stats)
    }
}
