use crate::parallel::Reducer;

#[cfg(not(feature = "parallel"))]
pub fn join<O1: Send, O2: Send>(left: impl FnOnce() -> O1 + Send, right: impl FnOnce() -> O2 + Send) -> (O1, O2) {
    (left(), right())
}

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
