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
pub struct SteppedReduce<Reducer: crate::parallel::Reducer> {
    /// This field is first to assure it's dropped first and cause threads that are dropped next to stop their loops
    /// as sending results fails when the receiver is dropped.
    receive_result: std::sync::mpsc::Receiver<Reducer::Input>,
    /// `join()` will be called on these guards to assure every thread tries to send through a closed channel. When
    /// that happens, they break out of their loops.
    _threads: Vec<std::thread::JoinHandle<()>>,
    /// The reducer is called only in the thread using the iterator, dropping it has no side effects.
    reducer: Option<Reducer>,
}

impl<Reducer: crate::parallel::Reducer> Drop for SteppedReduce<Reducer> {
    fn drop(&mut self) {
        let (_, sink) = std::sync::mpsc::channel();
        drop(std::mem::replace(&mut self.receive_result, sink));

        let mut last_err = None;
        for handle in std::mem::take(&mut self._threads) {
            if let Err(err) = handle.join() {
                last_err = Some(err);
            };
        }
        if let Some(thread_err) = last_err {
            std::panic::panic_any(thread_err);
        }
    }
}

impl<Reducer: crate::parallel::Reducer> SteppedReduce<Reducer> {
    /// Instantiate a new iterator and start working in threads.
    /// For a description of parameters, see [`in_parallel()`].
    pub fn new<InputIter, ThreadStateFn, ConsumeFn, I, O, S>(
        input: InputIter,
        thread_limit: Option<usize>,
        new_thread_state: ThreadStateFn,
        consume: ConsumeFn,
        reducer: Reducer,
    ) -> Self
    where
        InputIter: Iterator<Item = I> + Send + 'static,
        ThreadStateFn: Fn(usize) -> S + Send + Clone + 'static,
        ConsumeFn: Fn(I, &mut S) -> O + Send + Clone + 'static,
        Reducer: crate::parallel::Reducer<Input = O> + 'static,
        I: Send + 'static,
        O: Send + 'static,
    {
        let num_threads = num_threads(thread_limit);
        let mut threads = Vec::with_capacity(num_threads + 1);
        let receive_result = {
            let (send_input, receive_input) = crossbeam_channel::bounded::<I>(num_threads);
            let (send_result, receive_result) = std::sync::mpsc::sync_channel::<O>(num_threads);
            for thread_id in 0..num_threads {
                let handle = std::thread::spawn({
                    let send_result = send_result.clone();
                    let receive_input = receive_input.clone();
                    let new_thread_state = new_thread_state.clone();
                    let consume = consume.clone();
                    move || {
                        let mut state = new_thread_state(thread_id);
                        for item in receive_input {
                            if send_result.send(consume(item, &mut state)).is_err() {
                                break;
                            }
                        }
                    }
                });
                threads.push(handle);
            }
            threads.push(std::thread::spawn(move || {
                for item in input {
                    if send_input.send(item).is_err() {
                        break;
                    }
                }
            }));
            receive_result
        };
        SteppedReduce {
            _threads: threads,
            receive_result,
            reducer: Some(reducer),
        }
    }

    /// Consume the iterator by finishing its iteration and calling [`Reducer::finalize()`][crate::parallel::Reducer::finalize()].
    pub fn finalize(mut self) -> Result<Reducer::Output, Reducer::Error> {
        for value in self.by_ref() {
            drop(value?);
        }
        self.reducer
            .take()
            .expect("this is the last call before consumption")
            .finalize()
    }
}

impl<Reducer: crate::parallel::Reducer> Iterator for SteppedReduce<Reducer> {
    type Item = Result<Reducer::FeedProduce, Reducer::Error>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.receive_result
            .recv()
            .ok()
            .and_then(|input| self.reducer.as_mut().map(|r| r.feed(input)))
    }
}
