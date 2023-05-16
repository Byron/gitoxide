use crate::cache::delta::Item;

pub struct ItemSliceSend<T>(pub *mut [T])
where
    T: Send;

/// SAFETY: This would be unsafe if this would ever be abused, but it's used internally and only in a way that assure that the pointers
///         don't violate aliasing rules.
impl<T> Clone for ItemSliceSend<T>
where
    T: Send,
{
    fn clone(&self) -> Self {
        ItemSliceSend(self.0)
    }
}

// SAFETY: T is `Send`, and we only ever access one T at a time. And, ptrs need that assurance, I wonder if it's always right.
#[allow(unsafe_code)]
unsafe impl<T> Send for ItemSliceSend<T> where T: Send {}

/// An item returned by `iter_root_chunks`, allowing access to the `data` stored alongside nodes in a [`Tree`].
pub struct Node<'a, T: Send> {
    pub item: &'a mut Item<T>,
    pub child_items: ItemSliceSend<Item<T>>,
}

impl<'a, T: Send> Node<'a, T> {
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
                item: &mut unsafe { &mut *children.0 }[index as usize],
                child_items: children.clone(),
            }
        })
    }
}
