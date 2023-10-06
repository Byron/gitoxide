use std::{cmp::Ordering, collections::BinaryHeap};

use crate::PriorityQueue;

pub(crate) struct Item<K, T> {
    key: K,
    value: T,
}

impl<K: Ord + std::fmt::Debug, T: std::fmt::Debug> std::fmt::Debug for Item<K, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}: {:?})", self.key, self.value)
    }
}

impl<K: Ord + std::fmt::Debug, T: std::fmt::Debug> std::fmt::Debug for PriorityQueue<K, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<K: Ord, T> PartialEq<Self> for Item<K, T> {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other).is_eq()
    }
}

impl<K: Ord, T> Eq for Item<K, T> {}

impl<K: Ord, T> PartialOrd<Self> for Item<K, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<K: Ord, T> Ord for Item<K, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K, T> Clone for Item<K, T>
where
    K: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        Item {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl<K, T> Clone for PriorityQueue<K, T>
where
    K: Clone + Ord,
    T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K: Ord, T> PriorityQueue<K, T> {
    /// Create a new instance.
    pub fn new() -> Self {
        PriorityQueue(Default::default())
    }
    /// Insert `value` so that it is ordered according to `key`.
    pub fn insert(&mut self, key: K, value: T) {
        self.0.push(Item { key, value });
    }

    /// Pop the highest-priority item value off the queue.
    pub fn pop_value(&mut self) -> Option<T> {
        self.0.pop().map(|t| t.value)
    }

    /// Pop the highest-priority item key and value off the queue.
    pub fn pop(&mut self) -> Option<(K, T)> {
        self.0.pop().map(|t| (t.key, t.value))
    }

    /// Iterate all items ordered from highest to lowest priority.
    pub fn iter_unordered(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|t| &t.value)
    }

    /// Turn this instance into an iterator over its keys and values in arbitrary order.
    pub fn into_iter_unordered(self) -> impl Iterator<Item = (K, T)> {
        self.0.into_vec().into_iter().map(|item| (item.key, item.value))
    }

    /// Return true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return true the amount of items on the queue.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the greatest item `(K, T)` tuple, as ordered by `K`, if the queue is not empty, without removing it.
    pub fn peek(&self) -> Option<(&K, &T)> {
        self.0.peek().map(|e| (&e.key, &e.value))
    }

    /// Drop all items from the queue, without changing its capacity.
    pub fn clear(&mut self) {
        self.0.clear()
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
