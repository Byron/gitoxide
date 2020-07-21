use super::{Error, Mode, Outcome};
use crate::{pack, pack::index};
use git_features::{
    parallel::in_parallel_if,
    progress::{self, Progress},
};
use std::time::SystemTime;

impl index::File {
    pub(crate) fn inner_verify_with_indexed_lookup<P, C>(
        &self,
        thread_limit: Option<usize>,
        _mode: Mode,
        _make_cache: impl Fn() -> C + Send + Sync,
        mut root: progress::DoOrDiscard<P>,
        pack: &pack::data::File,
    ) -> Result<Outcome, Error>
    where
        P: Progress,
        <P as Progress>::SubProgress: Send,
        C: pack::cache::DecodeEntry,
    {
        let offsets = {
            let mut indexing_progress = root.add_child("preparing pack offsets");
            indexing_progress.init(Some(self.num_objects), Some("objects"));
            let then = SystemTime::now();
            let iter = self.sorted_offsets().into_iter();
            let elapsed = then.elapsed().expect("system time").as_secs_f32();
            indexing_progress.info(format!(
                "in {:.02}s ({} objects/s)",
                elapsed,
                self.num_objects as f32 / elapsed
            ));
            iter
        };
        let tree = pack::graph::DeltaTree::from_sorted_offsets(offsets, pack.path(), root.add_child("indexing"))?;
        let if_there_are_enough_objects = || self.num_objects > 10_000;

        let reduce_progress = std::sync::Mutex::new({
            let mut p = root.add_child("Checking");
            p.init(Some(self.num_objects()), Some("objects"));
            p
        });
        in_parallel_if(
            if_there_are_enough_objects,
            tree.bases(),
            thread_limit,
            |_index| (),
            |node: pack::graph::Node, state: &mut ()| Ok::<_, Error>(Vec::new()),
            index::verify::Reducer::from_progress(&reduce_progress, pack.data_len()),
        )?;

        unimplemented!()
    }
}
