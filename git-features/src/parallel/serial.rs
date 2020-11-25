use crate::parallel::Reducer;

#[cfg(not(feature = "parallel"))]
/// Runs `left` and then `right`, one after another, returning their output when both are done.
pub fn join<O1: Send, O2: Send>(left: impl FnOnce() -> O1 + Send, right: impl FnOnce() -> O2 + Send) -> (O1, O2) {
    (left(), right())
}

/// Read items from `input` and `consume` them, producing an output to be collected by a `reducer`, whose task is to
/// aggregate these outputs into the final result returned by this function.
///
/// * `new_thread_state(thread_number) -> State` produces thread-local state once per thread to be based to `consume`
/// * `consume(Item, &mut State) -> Output` produces an output given an input along with mutable state.
/// * For `reducer`, see the [`Reducer`] trait
/// * if `thread_limit` has no effect as everything is run on the main thread, but is present to keep the signature
///   similar to the parallel version.
///
/// **This serial version performing all calculations on the current thread.**
pub fn in_parallel<I, S, O, R>(
    input: impl Iterator<Item = I> + Send,
    _thread_limit: Option<usize>,
    new_thread_state: impl Fn(usize) -> S + Send + Sync,
    consume: impl Fn(I, &mut S) -> O + Send + Sync,
    mut reducer: R,
) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
where
    R: Reducer<Input = O>,
    I: Send,
    O: Send,
{
    let mut state = new_thread_state(0);
    for item in input {
        reducer.feed(consume(item, &mut state))?;
    }
    reducer.finalize()
}
