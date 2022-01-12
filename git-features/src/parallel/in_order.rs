use std::{cmp::Ordering, collections::BTreeMap};

/// A counter for items that are in sequence, to be able to put them back into original order later.
pub type SequenceId = usize;

/// An iterator which olds iterated items with a **sequential** ID starting at 0 long enough to dispense them in order.
pub struct InOrderIter<T, I> {
    /// The iterator yielding the out-of-order elements we are to yield in order.
    pub inner: I,
    store: BTreeMap<SequenceId, T>,
    next_chunk: SequenceId,
    is_done: bool,
}

impl<T, E, I> From<I> for InOrderIter<T, I>
where
    I: Iterator<Item = Result<(SequenceId, T), E>>,
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
    I: Iterator<Item = Result<(SequenceId, T), E>>,
{
    type Item = Result<T, E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }
        'find_next_in_sequence: loop {
            match self.inner.next() {
                Some(Ok((c, v))) => match c.cmp(&self.next_chunk) {
                    Ordering::Equal => {
                        self.next_chunk += 1;
                        return Some(Ok(v));
                    }
                    Ordering::Less => {
                        unreachable!("in a correctly ordered sequence we can never see keys again, got {}", c)
                    }
                    Ordering::Greater => {
                        let previous = self.store.insert(c, v);
                        assert!(
                            previous.is_none(),
                            "Chunks are returned only once, input is an invalid sequence"
                        );
                        if let Some(v) = self.store.remove(&self.next_chunk) {
                            self.next_chunk += 1;
                            return Some(Ok(v));
                        }
                        continue 'find_next_in_sequence;
                    }
                },
                Some(Err(e)) => {
                    self.is_done = true;
                    self.store.clear();
                    return Some(Err(e));
                }
                None => match self.store.remove(&self.next_chunk) {
                    Some(v) => {
                        self.next_chunk += 1;
                        return Some(Ok(v));
                    }
                    None => {
                        debug_assert!(
                            self.store.is_empty(),
                            "When iteration is done we should not have stored items left"
                        );
                        return None;
                    }
                },
            }
        }
    }
}
