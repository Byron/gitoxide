use crate::parallel::{num_threads, Reducer};

/// Runs `left` and `right` in parallel, returning their output when both are done.
pub fn join<O1: Send, O2: Send>(left: impl FnOnce() -> O1 + Send, right: impl FnOnce() -> O2 + Send) -> (O1, O2) {
    crossbeam_utils::thread::scope(|s| {
        let left = s.spawn(|_| left());
        let right = s.spawn(|_| right());
        (left.join().unwrap(), right.join().unwrap())
    })
    .unwrap()
}

/// Read items from `input` and `consume` them in multiple threads,
/// whose output output is collected by a `reducer`. Its task is to
/// aggregate these outputs into the final result returned by this function with the benefit of not having to be thread-safe.
///
/// * if `thread_limit` is `Some`, the given amount of threads will be used. If `None`, all logical cores will be used.
/// * `new_thread_state(thread_number) -> State` produces thread-local state once per thread to be based to `consume`
/// * `consume(Item, &mut State) -> Output` produces an output given an input obtained by `input` along with mutable state initially
///   created by `new_thread_state(…)`.
/// * For `reducer`, see the [`Reducer`] trait
pub fn in_parallel<I, S, O, R>(
    input: impl Iterator<Item = I> + Send,
    thread_limit: Option<usize>,
    new_thread_state: impl Fn(usize) -> S + Send + Sync,
    consume: impl Fn(I, &mut S) -> O + Send + Sync,
    mut reducer: R,
) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
where
    R: Reducer<Input = O>,
    I: Send,
    O: Send,
{
    let num_threads = num_threads(thread_limit);
    let new_thread_state = &new_thread_state;
    let consume = &consume;
    crossbeam_utils::thread::scope(move |s| {
        let receive_result = {
            let (send_input, receive_input) = crossbeam_channel::bounded::<I>(num_threads);
            let (send_result, receive_result) = std::sync::mpsc::sync_channel::<O>(num_threads);
            for thread_id in 0..num_threads {
                s.spawn({
                    let send_result = send_result.clone();
                    let receive_input = receive_input.clone();
                    move |_| {
                        let mut state = new_thread_state(thread_id);
                        for item in receive_input {
                            if send_result.send(consume(item, &mut state)).is_err() {
                                break;
                            }
                        }
                    }
                });
            }
            s.spawn(move |_| {
                for item in input {
                    if send_input.send(item).is_err() {
                        break;
                    }
                }
            });
            receive_result
        };

        for item in receive_result {
            drop(reducer.feed(item)?);
        }
        reducer.finalize()
    })
    .unwrap()
}

/// An iterator adaptor to allow running computations using [`in_parallel()`] in a step-wise manner, see the [module docs][crate::parallel]
/// for details.
#[cfg(feature = "parallel")]
pub struct SteppedReduce<'a, Reducer: crate::parallel::Reducer> {
    /// This field is first to assure it's dropped first and cause threads that are dropped next to stop their loops
    /// as sending results fails.
    receive_result: std::sync::mpsc::Receiver<Reducer::Input>,
    _threads: Vec<thread_scoped::JoinGuard<'a, ()>>,
    reducer: Reducer,
}

#[cfg(feature = "parallel")]
impl<'a, Reducer: crate::parallel::Reducer> SteppedReduce<'a, Reducer> {
    /// Instantiate a new iterator and start working in threads.
    /// For a description of parameters, see [`in_parallel()`].
    ///
    /// # Safety
    ///
    /// Read all about it in the [module documentation][crate::parallel].
    #[allow(unsafe_code)]
    pub unsafe fn new<Input, ThreadStateFn, ConsumeFn, I, O, S>(
        input: Input,
        thread_limit: Option<usize>,
        new_thread_state: ThreadStateFn,
        consume: ConsumeFn,
        reducer: Reducer,
    ) -> Self
    where
        Input: Iterator<Item = I> + Send + 'a,
        ThreadStateFn: Fn(usize) -> S + Send + Sync + Copy + 'a,
        ConsumeFn: Fn(I, &mut S) -> O + Send + Sync + Copy + 'a,
        Reducer: crate::parallel::Reducer<Input = O> + 'a,
        I: Send + 'a,
        O: Send + 'a,
    {
        let num_threads = num_threads(thread_limit);
        let mut threads = Vec::with_capacity(num_threads + 1);
        let receive_result = {
            let (send_input, receive_input) = crossbeam_channel::bounded::<I>(num_threads);
            let (send_result, receive_result) = std::sync::mpsc::sync_channel::<O>(num_threads);
            for thread_id in 0..num_threads {
                #[allow(unsafe_code, unused_unsafe)]
                let handle = unsafe {
                    thread_scoped::scoped({
                        let send_result = send_result.clone();
                        let receive_input = receive_input.clone();
                        move || {
                            let mut state = new_thread_state(thread_id);
                            for item in receive_input {
                                if send_result.send(consume(item, &mut state)).is_err() {
                                    break;
                                }
                            }
                        }
                    })
                };
                threads.push(handle);
            }
            threads.push(
                #[allow(unsafe_code, unused_unsafe)]
                unsafe {
                    thread_scoped::scoped(move || {
                        for item in input {
                            if send_input.send(item).is_err() {
                                break;
                            }
                        }
                    })
                },
            );
            receive_result
        };
        SteppedReduce {
            _threads: threads,
            receive_result,
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

#[cfg(feature = "parallel")]
impl<'a, Reducer: crate::parallel::Reducer> Iterator for SteppedReduce<'a, Reducer> {
    type Item = Result<Reducer::FeedProduce, Reducer::Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.receive_result.recv().ok().map(|input| self.reducer.feed(input))
    }
}
