use std::marker::PhantomData;

/// SAFETY: This type is used to allow access to a size-optimized vec of items that form a
/// tree, and we need to access it concurrently with each thread taking its own root node,
/// and working its way through all the reachable leaves.
///
/// The tree was built by decoding a pack whose entries refer to its bases only by OFS_DELTA -
/// they are pointing backwards only which assures bases have to be listed first, and that each entry
/// only has a single parent.
///
/// REF_DELTA entries aren't supported here, and cause immediate failure - they are expected to have
/// been resolved before as part of the thin-pack handling.
///
/// If we somehow would allow REF_DELTA entries to point to an in-pack object, then in theory malicious packs could
/// cause all kinds of graphs as they can point anywhere in the pack, but they still can't link an entry to
/// more than one base. And that's what one would really have to do for two threads to encounter the same child.
///
/// Thus I believe it's impossible for this data structure to end up in a place where it violates its assumption.
pub(in crate::cache::delta::traverse) struct ItemSliceSync<'a, T>
where
    T: Send,
{
    items: *mut T,
    #[cfg(debug_assertions)]
    len: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> ItemSliceSync<'a, T>
where
    T: Send,
{
    pub(in crate::cache::delta::traverse) fn new(items: &'a mut [T]) -> Self {
        ItemSliceSync {
            items: items.as_mut_ptr(),
            #[cfg(debug_assertions)]
            len: items.len(),
            phantom: PhantomData,
        }
    }

    // SAFETY: The index must point into the slice and must not be reused concurrently.
    #[allow(unsafe_code)]
    pub(in crate::cache::delta::traverse) unsafe fn get_mut(&self, index: usize) -> &'a mut T {
        #[cfg(debug_assertions)]
        if index >= self.len {
            panic!("index out of bounds: the len is {} but the index is {index}", self.len);
        }
        // SAFETY: The index is within the slice
        // SAFETY: The children array is alive by the 'a lifetime.
        unsafe { &mut *self.items.add(index) }
    }
}

// SAFETY: T is `Send`, and we only use the pointer for creating new pointers.
#[allow(unsafe_code)]
unsafe impl<T> Send for ItemSliceSync<'_, T> where T: Send {}
// SAFETY: T is `Send`, and as long as the user follows the contract of
// `get_mut()`, we only ever access one T at a time.
#[allow(unsafe_code)]
unsafe impl<T> Sync for ItemSliceSync<'_, T> where T: Send {}
