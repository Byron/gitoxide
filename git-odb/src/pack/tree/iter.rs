use crate::{
    pack,
    pack::tree::{Item, Tree},
};

/// All the **unsafe** bits to support parallel iteration with write access
impl<T> Tree<T> {
    #[allow(unsafe_code)]
    /// # SAFETY
    /// Called from node with is guaranteed to not be aliasing with any other node.
    /// Note that we are iterating nodes that we are affecting here, but that only affects the
    /// 'is_root' field fo the item, not the data field that we are writing here.
    /// For all details see `from_node_take_entry()`.
    unsafe fn put_data(&self, index: usize, data: T) {
        let items_mut: &mut Vec<Item<T>> = &mut *(self.items.get());
        items_mut.get_unchecked_mut(index).data = data;
    }

    #[allow(unsafe_code)]
    /// # SAFETY
    /// Similar to `from_node_put_data(…)`
    unsafe fn offset(&self, index: usize) -> u64 {
        let items: &Vec<Item<T>> = &*(self.items.get());
        items.get_unchecked(index).offset
    }

    #[allow(unsafe_code)]
    /// # SAFETY
    /// Similar to `from_node_put_data(…)`
    unsafe fn entry_slice(&self, index: usize) -> std::ops::Range<u64> {
        let items: &Vec<Item<T>> = &*(self.items.get());
        let start = items.get_unchecked(index).offset;
        let end = items
            .get(index + 1)
            .map(|e| e.offset)
            .or(self.pack_entries_end)
            .expect("traversal(…) to have set this value (BUG)");
        start..end
    }

    #[allow(unsafe_code)]
    /// # SAFETY
    /// A tree is a data structure without cycles, and we assure of that by verifying all input.
    /// A node as identified by index can only be traversed once using the Chunks iterator.
    /// When the iterator is created, this instance cannot be mutated anymore nor can it be read.
    /// That iterator is only handed out once.
    /// `Node` instances produced by it consume themselves when iterating their children, allowing them to be
    /// used only once, recursively. Again, traversal is always forward and consuming, making it impossible to
    /// alias multiple nodes in the tree.
    /// It's safe for multiple threads to hold different chunks, as they are guaranteed to be non-overlapping and unique.
    /// If the tree is accessed after iteration, it will panic as no mutation is allowed anymore, nor is
    unsafe fn take_entry(&self, index: usize) -> (T, Vec<usize>)
    where
        T: Default,
    {
        let items_mut: &mut Vec<Item<T>> = &mut *(self.items.get());
        let item = items_mut.get_unchecked_mut(index);
        let children = std::mem::take(&mut item.children);
        let data = std::mem::take(&mut item.data);
        (data, children)
    }

    #[allow(unsafe_code)]
    /// # SAFETY
    /// As `take_entry(…)` - but this one only takes if the data of Node is a root
    unsafe fn from_iter_take_entry_if_root(&self, index: usize) -> Option<(T, Vec<usize>)>
    where
        T: Default,
    {
        let items_mut: &mut Vec<Item<T>> = &mut *(self.items.get());
        let item = items_mut.get_unchecked_mut(index);
        if item.is_root {
            let children = std::mem::take(&mut item.children);
            let data = std::mem::take(&mut item.data);
            Some((data, children))
        } else {
            None
        }
    }
}

/// Iteration
impl<T> Tree<T> {
    /// Return an iterator over chunks of roots. Roots are not children themselves, they have no parents.
    pub fn iter_root_chunks(&mut self, chunk_size: usize) -> Chunks<'_, T> {
        Chunks {
            tree: self,
            chunk_size,
            cursor: 0,
        }
    }
}

/// An item returned by a [`Chunks`] iterator, allowing access to the `data` stored alongside nodes in a [`Tree`]
pub struct Node<'a, T> {
    tree: &'a Tree<T>,
    index: usize,
    children: Vec<usize>,
    /// The custom data attached to each node of the tree whose ownership was transferred into the iteration.
    pub data: T,
}

impl<'a, T> Node<'a, T>
where
    T: Default,
{
    /// Returns the offset into the pack at which the `Node`s data is located.
    pub fn offset(&self) -> u64 {
        #[allow(unsafe_code)]
        // SAFETY: The index is valid as it was controlled by `add_child(…)`, then see `take_entry(…)`
        unsafe {
            self.tree.offset(self.index)
        }
    }

    /// Returns the slice into the data pack at which the pack entry is located.
    pub fn entry_slice(&self) -> pack::data::EntrySlice {
        #[allow(unsafe_code)]
        // SAFETY: The index is valid as it was controlled by `add_child(…)`, then see `take_entry(…)`
        unsafe {
            self.tree.entry_slice(self.index)
        }
    }

    /// Write potentially changed `Node` data back into the [`Tree`] and transform this `Node` into an iterator
    /// over its `Node`s children.
    ///
    /// Children are `Node`s referring to pack entries whose base object is this pack entry.
    pub fn store_changes_then_into_child_iter(self) -> impl Iterator<Item = Node<'a, T>> {
        #[allow(unsafe_code)]
        // SAFETY: The index is valid as it was controlled by `add_child(…)`, then see `take_entry(…)`
        unsafe {
            self.tree.put_data(self.index, self.data)
        };
        let Self { tree, children, .. } = self;
        children.into_iter().map(move |index| {
            // SAFETY: The index is valid as it was controlled by `add_child(…)`, then see `take_entry(…)`
            #[allow(unsafe_code)]
            let (data, children) = unsafe { tree.take_entry(index) };
            Node {
                tree,
                data,
                children,
                index,
            }
        })
    }
}

/// An iterator over chunks of [`Node`]s in a [`Tree`]
pub struct Chunks<'a, T> {
    tree: &'a Tree<T>,
    chunk_size: usize,
    cursor: usize,
}

impl<'a, T> Iterator for Chunks<'a, T>
where
    T: Default,
{
    type Item = Vec<Node<'a, T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.tree.one_past_last_seen_root {
            return None;
        }
        let mut items_remaining = self.chunk_size;
        let mut res = Vec::with_capacity(self.chunk_size);

        while items_remaining > 0 && self.cursor < self.tree.one_past_last_seen_root {
            // SAFETY: The index is valid as the cursor cannot surpass the amount of items. `one_past_last_seen_root`
            // is guaranteed to be self.tree.items.len() at most, or smaller.
            // Then see `take_entry_if_root(…)`
            #[allow(unsafe_code)]
            if let Some((data, children)) = unsafe { self.tree.from_iter_take_entry_if_root(self.cursor) } {
                res.push(Node {
                    tree: self.tree,
                    index: self.cursor,
                    children,
                    data,
                });
                items_remaining -= 1;
            }
            self.cursor += 1;
        }

        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}
