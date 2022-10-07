use crate::cache::delta::{Item, Tree};

/// Iteration
impl<T> Tree<T> {
    /// Return an iterator over chunks of roots. Roots are not children themselves, they have no parents.
    pub fn iter_root_chunks(&mut self, chunk_size: usize) -> impl Iterator<Item = Chunk<'_, T>> + '_ {
        let roots = self.root_items.as_mut_slice();
        let children = self.child_items.as_mut_slice();

        roots.chunks_mut(chunk_size).map(move |c| Chunk {
            inner: c.iter_mut(),
            child_items: children as *mut [Item<T>],
        })
    }
}

/// A chunk returned by `iter_root_chunks`, which can be iterated over to get [`Node`]s.
pub struct Chunk<'a, T> {
    inner: std::slice::IterMut<'a, Item<T>>,
    child_items: *mut [Item<T>],
}

// SAFETY: The raw pointer is uniquely materialized in `Node::into_child_iter`.
#[allow(unsafe_code)]
unsafe impl<'a, T> Send for Chunk<'a, T> where T: Send {}

impl<'a, T> Iterator for Chunk<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|item| Node {
            item,
            child_items: self.child_items,
        })
    }
}

/// An item returned by `iter_root_chunks`, allowing access to the `data` stored alongside nodes in a [`Tree`].
pub struct Node<'a, T> {
    item: &'a mut Item<T>,
    child_items: *mut [Item<T>],
}

impl<'a, T> Node<'a, T> {
    /// Returns the offset into the pack at which the `Node`s data is located.
    pub fn offset(&self) -> u64 {
        self.item.offset
    }

    /// Returns the slice into the data pack at which the pack entry is located.
    pub fn entry_slice(&self) -> crate::data::EntryRange {
        self.item.offset..self.item.next_offset
    }

    /// Returns the node data associated with this node.
    pub fn data(&mut self) -> &mut T {
        &mut self.item.data
    }

    /// Returns true if this node has children, e.g. is not a leaf in the tree.
    pub fn has_children(&self) -> bool {
        !self.item.children.is_empty()
    }

    /// Transform this `Node` into an iterator over its children.
    ///
    /// Children are `Node`s referring to pack entries whose base object is this pack entry.
    pub fn into_child_iter(self) -> impl Iterator<Item = Node<'a, T>> + 'a {
        let children = self.child_items;
        self.item.children.iter().map(move |&index| {
            // SAFETY: The children array is alive by the 'a lifetime.
            // SAFETY: The index is a valid index into the children array.
            // SAFETY: The resulting mutable pointer cannot be yielded by any other node.
            #[allow(unsafe_code)]
            Node {
                item: unsafe { &mut *(children as *mut Item<T>).add(index as usize) },
                child_items: children,
            }
        })
    }
}
