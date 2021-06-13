///
pub mod count;
#[doc(inline)]
pub use count::Count;

///
pub mod entry;
#[doc(inline)]
pub use entry::Entry;

///
pub mod bytes;

mod in_order {
    /// An iterator which olds iterated items with a **sequential** ID starting at 0 long enough to dispense them in order.
    ///
    /// Note that this iterator is made specifically to support the signature of the iterator returned
    /// by [from_counts_iter(…)][super::entry::from_counts_iter()].
    pub struct InOrderIter<I> {
        inner: I,
    }

    impl<C, T, E, I> From<I> for InOrderIter<I>
    where
        I: Iterator<Item = Result<(C, T), E>>,
        C: Ord,
    {
        fn from(iter: I) -> Self {
            InOrderIter { inner: iter }
        }
    }

    impl<C, T, E, I> Iterator for InOrderIter<I>
    where
        I: Iterator<Item = Result<(C, T), E>>,
        C: Ord,
    {
        type Item = Result<T, E>;

        fn next(&mut self) -> Option<Self::Item> {
            todo!("in order iteration")
        }
    }
}

pub use in_order::InOrderIter;
