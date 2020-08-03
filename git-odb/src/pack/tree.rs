use quick_error::quick_error;
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvariantIncreasingPackOffset(last_pack_offset: u64, pack_offset: u64) {
            display("Pack offsets must only increment. The previous pack offset was {}, the current one is {}", last_pack_offset, pack_offset)
        }
        InvariantNonEmpty {
            display("Is there ever a need to create empty indices? If so, please post a PR.")
        }
        InvariantBasesBeforeDeltasNeedThem(delta_pack_offset: u64, base_pack_offset: u64) {
            display("The delta at pack offset {} could not find its base at {} - it should have been seen already", delta_pack_offset, base_pack_offset)
        }
    }
}

pub(crate) struct Item<D> {
    offset: u64,
    is_root: bool,
    pub data: D,
    // TODO: figure out average amount of children per node and use smallvec instead
    children: Vec<usize>,
}

/// A tree that allows one-time iteration over all nodes and their children, consuming it in the process,
/// while being shareable among threads without a lock.
/// It does this by making the run-time guarantee that iteration only happens once.
pub(crate) struct Tree<D> {
    items: UnsafeCell<Vec<Item<D>>>,
    last_added_offset: u64,
    // assure we truly create only one iterator, ever, to avoid violating access rules
    iterator_active: AtomicBool,
    one_past_last_seen_root: usize,
}

/// SAFETY: We solemnly swear…that this is sync because without the unsafe cell, it is also sync.
/// But that's really the only reason why I would dare to know.
#[allow(unsafe_code)]
unsafe impl<T> Sync for Tree<T> {}

impl<D> Tree<D> {
    pub fn new(num_objects: usize) -> Result<Self, Error> {
        if num_objects == 0 {
            return Err(Error::InvariantNonEmpty);
        }
        Ok(Tree {
            items: UnsafeCell::new(Vec::with_capacity(num_objects)),
            last_added_offset: 0,
            iterator_active: AtomicBool::new(false),
            one_past_last_seen_root: 0,
        })
    }

    fn assert_is_incrementing(&mut self, offset: u64) -> Result<u64, Error> {
        if offset > self.last_added_offset {
            self.last_added_offset = offset;
            Ok(offset)
        } else {
            Err(Error::InvariantIncreasingPackOffset(self.last_added_offset, offset))
        }
    }

    pub fn add_root(&mut self, offset: u64, data: D) -> Result<(), Error> {
        assert!(
            !self.iterator_active.load(Ordering::SeqCst),
            "Cannot mutate after the iterator was created as it assumes exclusive access"
        );
        // SAFETY: Because we passed the assertion above which implies no other access is possible as per
        // standard borrow check rules.
        #[allow(unsafe_code)]
        let items = unsafe { &mut *(self.items.get()) };
        let offset = self.assert_is_incrementing(offset)?;
        items.push(Item {
            offset,
            data,
            is_root: true,
            children: Default::default(),
        });
        self.one_past_last_seen_root = items.len();
        Ok(())
    }

    pub fn add_child(&mut self, base_offset: u64, offset: u64, data: D) -> Result<(), Error> {
        assert!(
            !self.iterator_active.load(Ordering::SeqCst),
            "Cannot mutate after the iterator was created as it assumes exclusive access"
        );
        // SAFETY: Because we passed the assertion above which implies no other access is possible as per
        // standard borrow check rules.
        #[allow(unsafe_code)]
        let items = unsafe { &mut *(self.items.get()) };
        let offset = self.assert_is_incrementing(offset)?;
        let base_index = items
            .binary_search_by_key(&base_offset, |e| e.offset)
            .map_err(|_| Error::InvariantBasesBeforeDeltasNeedThem(offset, base_offset))?;
        let child_index = items.len();
        items[base_index].children.push(child_index);
        items.push(Item {
            is_root: false,
            offset,
            data,
            children: Default::default(),
        });
        Ok(())
    }

    pub fn into_items(self) -> Vec<Item<D>> {
        self.items.into_inner()
    }

    /// Return an iterator over chunks of roots. Roots are not children themselves, they have no parents.
    pub fn iter_root_chunks(&mut self, size: usize) -> Chunks<D> {
        // We would love to consume the tree, of course, but if we can't hand out items that borrow from ourselves,
        // it's nothing we can use effectively. Thus it's better to check at runtime…
        assert!(
            !self.iterator_active.load(Ordering::SeqCst),
            "Can only create a single iterator to avoid aliasing mutable tree nodes"
        );
        self.iterator_active.store(true, Ordering::SeqCst);
        Chunks {
            tree: self,
            size,
            cursor: 0,
        }
    }

    #[allow(unsafe_code)]
    /// SAFETY: Called from node with is guaranteed to not be aliasing with any other node.
    /// Note that we are iterating nodes that we are affecting here, but that only affects the
    /// 'is_root' field fo the item, not the data field that we are writing here.
    /// For all details see `from_node_take_entry()`.
    unsafe fn from_node_put_data(&self, index: usize, data: D) {
        debug_assert!(
            self.iterator_active.load(Ordering::SeqCst),
            "Must only be called after an iterator was created"
        );
        let items_mut: &mut Vec<Item<D>> = &mut *(self.items.get());
        items_mut.get_unchecked_mut(index).data = data;
    }

    #[allow(unsafe_code)]
    /// SAFETY: A tree is a data structure without cycles, and we assure of that by verifying all input.
    /// A node as identified by index can only be traversed once using the Chunks iterator.
    /// When the iterator is created, this instance cannot be mutated anymore nor can it be read.
    /// That iterator is only handed out once.
    /// `Node` instances produced by it consume themselves when iterating their children, allowing them to be
    /// used only once, recursively. Again, traversal is always forward and consuming, making it impossible to
    /// alias multiple nodes in the tree.
    /// It's safe for multiple threads to hold different chunks, as they are guaranteed to be non-overlapping and unique.
    /// If the tree is accessed after iteration, it will panic as no mutation is allowed anymore, nor is
    unsafe fn from_node_take_entry(&self, index: usize) -> (D, Vec<usize>)
    where
        D: Default,
    {
        debug_assert!(
            self.iterator_active.load(Ordering::SeqCst),
            "Must only be called after an iterator was created"
        );
        let items_mut: &mut Vec<Item<D>> = &mut *(self.items.get());
        let item = items_mut.get_unchecked_mut(index);
        let children = std::mem::take(&mut item.children);
        let data = std::mem::take(&mut item.data);
        (data, children)
    }

    #[allow(unsafe_code)]
    /// SAFETY: As `take_entry(…)` - but this one only takes if the data of Node is a root
    unsafe fn from_iter_take_entry_if_root(&self, index: usize) -> Option<(D, Vec<usize>)>
    where
        D: Default,
    {
        debug_assert!(
            self.iterator_active.load(Ordering::SeqCst),
            "Must only be called after an iterator was created"
        );
        let items_mut: &mut Vec<Item<D>> = &mut *(self.items.get());
        let item = items_mut.get_unchecked_mut(index);
        if item.is_root {
            let children = std::mem::take(&mut item.children);
            let data = std::mem::take(&mut item.data);
            Some((data, children))
        } else {
            None
        }
    }

    #[allow(unsafe_code)]
    /// SAFETY: It's called when dropping the iterator, and it's impossible to have multiple of these
    /// at the same time. The flag indicating this is thread-safe, so there can't be a race either.
    fn from_iter_dropped(&self) {
        self.iterator_active.store(false, Ordering::SeqCst);
    }
}

pub struct Node<'a, D> {
    tree: &'a Tree<D>,
    index: usize,
    // TODO: figure out average amount of children per node and use smallvec instead
    children: Vec<usize>,
    pub data: D,
}

impl<'a, D> Node<'a, D>
where
    D: Default,
{
    pub fn store_changes_then_into_child_iter(self) -> impl Iterator<Item = Node<'a, D>> {
        #[allow(unsafe_code)]
        // SAFETY: The index is valid as it was controlled by `add_child(…)`, then see `take_entry(…)`
        unsafe {
            self.tree.from_node_put_data(self.index, self.data)
        };
        let Self { tree, children, .. } = self;
        children.into_iter().map(move |index| {
            // SAFETY: The index is valid as it was controlled by `add_child(…)`, then see `take_entry(…)`
            #[allow(unsafe_code)]
            let (data, children) = unsafe { tree.from_node_take_entry(index) };
            Node {
                tree,
                data,
                children,
                index,
            }
        })
    }
}

pub struct Chunks<'a, D> {
    tree: &'a Tree<D>,
    size: usize,
    cursor: usize,
}

impl<'a, D> Iterator for Chunks<'a, D>
where
    D: Default,
{
    type Item = Vec<Node<'a, D>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.tree.one_past_last_seen_root {
            return None;
        }
        let mut items_remaining = self.size;
        let mut res = Vec::with_capacity(self.size);

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

impl<'a, D> Drop for Chunks<'a, D> {
    fn drop(&mut self) {
        self.tree.from_iter_dropped()
    }
}
