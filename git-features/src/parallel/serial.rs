use crate::parallel::Reduce;

#[cfg(not(feature = "parallel"))]
mod not_parallel {
    /// Runs `left` and then `right`, one after another, returning their output when both are done.
    pub fn join<O1, O2>(left: impl FnOnce() -> O1, right: impl FnOnce() -> O2) -> (O1, O2) {
        (left(), right())
    }

    /// A scope for spawning threads.
    pub struct Scope<'env> {
        _marker: std::marker::PhantomData<&'env mut &'env ()>,
    }

    #[allow(unsafe_code)]
    unsafe impl Sync for Scope<'_> {}

    impl<'env> Scope<'env> {
        pub fn spawn<'scope, F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
        where
            F: FnOnce(&Scope<'env>) -> T,
            F: Send + 'env,
            T: Send + 'env,
        {
            ScopedJoinHandle {
                result: f(self),
                _marker: Default::default(),
            }
        }
    }

    /// Runs `f` with a scope to be used for spawning threads that will not outlive the function call.
    /// Note that this implementation will run the spawned functions immediately.
    pub fn threads<'env, F, R>(f: F) -> std::thread::Result<R>
    where
        F: FnOnce(&Scope<'env>) -> R,
    {
        Ok(f(&Scope {
            _marker: Default::default(),
        }))
    }

    /// A handle that can be used to join its scoped thread.
    ///
    /// This struct is created by the [`Scope::spawn`] method and the
    /// [`ScopedThreadBuilder::spawn`] method.
    pub struct ScopedJoinHandle<'scope, T> {
        /// Holds the result of the inner closure.
        result: T,
        _marker: std::marker::PhantomData<&'scope mut &'scope ()>,
    }

    impl<T> ScopedJoinHandle<'_, T> {
        pub fn join(self) -> std::thread::Result<T> {
            Ok(self.result)
        }
    }
}
#[cfg(not(feature = "parallel"))]
pub use not_parallel::{join, threads, Scope, ScopedJoinHandle};

/// Read items from `input` and `consume` them in a single thread, producing an output to be collected by a `reducer`,
/// whose task is to aggregate these outputs into the final result returned by this function.
///
/// * `new_thread_state(thread_number) -> State` produces thread-local state once per thread to be based to `consume`
/// * `consume(Item, &mut State) -> Output` produces an output given an input along with mutable state.
/// * For `reducer`, see the [`Reduce`] trait
/// * if `thread_limit` has no effect as everything is run on the main thread, but is present to keep the signature
///   similar to the parallel version.
///
/// **This serial version performing all calculations on the current thread.**
pub fn in_parallel<I, S, O, R>(
    input: impl Iterator<Item = I>,
    _thread_limit: Option<usize>,
    new_thread_state: impl Fn(usize) -> S,
    consume: impl Fn(I, &mut S) -> O,
    mut reducer: R,
) -> Result<<R as Reduce>::Output, <R as Reduce>::Error>
where
    R: Reduce<Input = O>,
{
    let mut state = new_thread_state(0);
    for item in input {
        drop(reducer.feed(consume(item, &mut state))?);
    }
    reducer.finalize()
}
