use crate::parallel::Reducer;

#[cfg(not(feature = "parallel"))]
/// Runs `left` and then `right`, one after another, returning their output when both are done.
pub fn join<O1: Send, O2: Send>(left: impl FnOnce() -> O1 + Send, right: impl FnOnce() -> O2 + Send) -> (O1, O2) {
    (left(), right())
}

/// Read items from `input` and `consume` them in a single thread, producing an output to be collected by a `reducer`,
/// whose task is to aggregate these outputs into the final result returned by this function.
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
        drop(reducer.feed(consume(item, &mut state))?);
    }
    reducer.finalize()
}

/// An iterator adaptor to allow running computations using [`in_parallel()`] in a step-wise manner, see the [module docs][crate::parallel]
/// for details.
#[cfg(not(feature = "parallel"))]
pub struct SteppedReduce<Input, ConsumeFn, ThreadState, Reducer> {
    input: Input,
    consume: ConsumeFn,
    thread_state: ThreadState,
    reducer: Reducer,
}

#[cfg(not(feature = "parallel"))]
impl<Input, ConsumeFn, Reducer, I, O, S> SteppedReduce<Input, ConsumeFn, S, Reducer>
where
    Input: Iterator<Item = I> + Send,
    ConsumeFn: Fn(I, &mut S) -> O + Send + Sync,
    Reducer: crate::parallel::Reducer<Input = O>,
    I: Send,
    O: Send,
{
    /// Instantiate a new iterator.
    /// For a description of parameters, see [`in_parallel()`].
    ///
    /// # Safety
    ///
    /// Read all about it in the [module documentation][crate::parallel].
    #[allow(unsafe_code)]
    pub unsafe fn new<ThreadStateFn>(
        input: Input,
        _thread_limit: Option<usize>,
        new_thread_state: ThreadStateFn,
        consume: ConsumeFn,
        reducer: Reducer,
    ) -> Self
    where
        ThreadStateFn: Fn(usize) -> S + Send + Sync,
    {
        SteppedReduce {
            input,
            consume,
            thread_state: new_thread_state(0),
            reducer,
        }
    }

    /// Consume the iterator by finishing its iteration and calling [`Reducer::finalize()`][crate::parallel::Reducer::finalize()].
    pub fn finalize(mut self) -> Result<Reducer::Output, Reducer::Error> {
        for value in self.by_ref() {
            drop(value?);
        }
        self.reducer.finalize()
    }
}

#[cfg(not(feature = "parallel"))]
impl<Input, ConsumeFn, ThreadState, Reducer, I, O> Iterator for SteppedReduce<Input, ConsumeFn, ThreadState, Reducer>
where
    Input: Iterator<Item = I> + Send,
    ConsumeFn: Fn(I, &mut ThreadState) -> O + Send + Sync,
    Reducer: crate::parallel::Reducer<Input = O>,
    I: Send,
    O: Send,
{
    type Item = Result<Reducer::FeedProduce, Reducer::Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.input
            .next()
            .map(|input| self.reducer.feed((self.consume)(input, &mut self.thread_state)))
    }
}
