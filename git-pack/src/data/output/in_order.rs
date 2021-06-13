use std::cmp::Ordering;
use std::collections::BTreeMap;

/// An iterator which olds iterated items with a **sequential** ID starting at 0 long enough to dispense them in order.
///
/// Note that this iterator is made specifically to support the signature of the iterator returned
/// by [from_counts_iter(…)][super::entry::from_counts_iter()].
pub struct InOrderIter<T, I> {
    /// The iterator yielding the out-of-order elements we are to yield in order.
    pub inner: I,
    store: BTreeMap<usize, T>,
    next_chunk: usize,
    is_done: bool,
}

impl<T, E, I> From<I> for InOrderIter<T, I>
where
    I: Iterator<Item = Result<(usize, T), E>>,
{
    fn from(iter: I) -> Self {
        InOrderIter {
            inner: iter,
            store: Default::default(),
            next_chunk: 0,
            is_done: false,
        }
    }
}

impl<T, E, I> Iterator for InOrderIter<T, I>
where
    I: Iterator<Item = Result<(usize, T), E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        match self.inner.next() {
            Some(Ok((c, v))) => match self.next_chunk.cmp(&c) {
                Ordering::Equal => {
                    self.next_chunk += 1;
                    return Some(Ok(v));
                }
                Ordering::Less => unreachable!("in a correctly ordered sequence we can never see keys again"),
                Ordering::Greater => todo!("handle out-of-sequence item"),
            },
            Some(Err(e)) => {
                self.is_done = true;
                return Some(Err(e));
            }
            None => {
                debug_assert!(
                    self.store.is_empty(),
                    "When iteration is done we should not have stored items left"
                );
                return None;
            }
        }
    }
}
