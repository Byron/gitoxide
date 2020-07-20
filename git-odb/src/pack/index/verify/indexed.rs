use super::{Error, Mode, Outcome};
use crate::{pack, pack::index};
use git_features::progress::{self, Progress};
use std::{fs, io};

impl index::File {
    pub(crate) fn inner_verify_with_indexed_lookup<P, C>(
        &self,
        _thread_limit: Option<usize>,
        _mode: Mode,
        _make_cache: impl Fn() -> C + Send + Sync,
        mut progress: progress::DoOrDiscard<P>,
        pack: &pack::data::File,
    ) -> Result<Outcome, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let indexing_progress = progress.add_child("indexing");
        let r = io::BufReader::with_capacity(
            8192 * 8, // this value directly corresponds to performance, 8k (default) is about 4x slower than 64k
            fs::File::open(pack.path()).map_err(|err| Error::Io(err, pack.path().into(), "open"))?,
        );
        pack::graph::DeltaTree::from_sorted_offsets(self.sorted_offsets().into_iter(), r, indexing_progress)?;

        unimplemented!()
    }
}
