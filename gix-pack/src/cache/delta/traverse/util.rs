use std::marker::PhantomData;

pub(crate) struct ItemSliceSync<'a, T>
where
    T: Send,
{
    items: *mut T,
    len: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> ItemSliceSync<'a, T>
where
    T: Send,
{
    pub fn new(items: &'a mut [T]) -> Self {
        ItemSliceSync {
            items: items.as_mut_ptr(),
            len: items.len(),
            phantom: PhantomData,
        }
    }

    // SAFETY: The index must not be reused concurrently
    #[allow(unsafe_code)]
    pub unsafe fn get_mut(&self, index: usize) -> &'a mut T {
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
