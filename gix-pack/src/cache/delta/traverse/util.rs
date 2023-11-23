use std::marker::PhantomData;

pub(crate) struct ItemSliceSend<'a, T>
where
    T: Send,
{
    items: *mut T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> ItemSliceSend<'a, T>
where
    T: Send,
{
    pub fn new(items: &'a mut [T]) -> Self {
        ItemSliceSend {
            items: items.as_mut_ptr(),
            phantom: PhantomData,
        }
    }

    /// SAFETY: The index must point into the slice and must not be reused concurrently.
    #[allow(unsafe_code)]
    pub unsafe fn get_mut(&self, index: usize) -> &'a mut T {
        // SAFETY: The children array is alive by the 'a lifetime.
        unsafe { &mut *self.items.add(index) }
    }
}

/// SAFETY: This would be unsafe if this would ever be abused, but it's used internally and only in a way that assure that the pointers
///         don't violate aliasing rules.
impl<T> Clone for ItemSliceSend<'_, T>
where
    T: Send,
{
    fn clone(&self) -> Self {
        ItemSliceSend {
            items: self.items,
            phantom: self.phantom,
        }
    }
}

// SAFETY: T is `Send`, and we only ever access one T at a time. And, ptrs need that assurance, I wonder if it's always right.
#[allow(unsafe_code)]
unsafe impl<T> Send for ItemSliceSend<'_, T> where T: Send {}
