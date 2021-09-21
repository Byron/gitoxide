use std::{marker::PhantomData, sync::Arc};

use git_features::{parallel, progress::Progress};

use super::Outcome;
use crate::data::output;

pub struct Statistics<E, P> {
    total: Outcome,
    counts: Vec<output::Count>,
    progress: Arc<parking_lot::Mutex<P>>,
    _err: PhantomData<E>,
}

impl<E, P> Statistics<E, P>
where
    P: Progress,
{
    pub fn new(progress: Arc<parking_lot::Mutex<P>>) -> Self {
        Statistics {
            total: Default::default(),
            counts: Default::default(),
            progress,
            _err: PhantomData::default(),
        }
    }
}

impl<E, P> parallel::Reduce for Statistics<E, P>
where
    P: Progress,
{
    type Input = Result<(Vec<output::Count>, Outcome), E>;
    type FeedProduce = ();
    type Output = (Vec<output::Count>, Outcome);
    type Error = E;

    fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
        let (counts, mut stats) = item?;
        stats.total_objects = counts.len();
        self.total.aggregate(stats);
        self.progress.lock().inc_by(counts.len());
        self.counts.extend(counts);
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        Ok((self.counts, self.total))
    }
}
