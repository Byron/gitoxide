use crate::PriorityQueue;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub(crate) struct Item<K, T> {
    key: K,
    value: T,
}

impl<K: Ord, T> PartialEq<Self> for Item<K, T> {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other).is_eq()
    }
}

impl<K: Ord, T> Eq for Item<K, T> {}

impl<K: Ord, T> PartialOrd<Self> for Item<K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Ord::cmp(self, other).into()
    }
}

impl<K: Ord, T> Ord for Item<K, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, T> PriorityQueue<K, T> {
    /// Insert `value` so that it is ordered according to `key`.
    pub fn insert(&mut self, key: K, value: T) {
        self.0.push(Item { key, value });
    }

    /// Pop the highest-priority item off the queue.
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|t| t.value)
    }

    /// Iterate all items ordered from highest to lowest priority.
    pub fn iter_random(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|t| &t.value)
    }

    /// Return true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<K: Ord, T> FromIterator<(K, T)> for PriorityQueue<K, T> {
    fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
        let mut q = PriorityQueue(BinaryHeap::new());
        for (k, v) in iter {
            q.insert(k, v);
        }
        q
    }
}
