use std::marker::PhantomData;

use gix_features::parallel;

use super::Outcome;
use crate::data::output;

pub struct Statistics<E> {
    total: Outcome,
    counts: Vec<output::Count>,
    _err: PhantomData<E>,
}

impl<E> Statistics<E> {
    pub fn new() -> Self {
        Statistics {
            total: Default::default(),
            counts: Default::default(),
            _err: PhantomData,
        }
    }
}

impl<E> parallel::Reduce for Statistics<E> {
    type Input = Result<(Vec<output::Count>, Outcome), E>;
    type FeedProduce = ();
    type Output = (Vec<output::Count>, Outcome);
    type Error = E;

    fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
        let (counts, stats) = item?;
        self.total.aggregate(stats);
        self.counts.extend(counts);
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        Ok((self.counts, self.total))
    }
}
