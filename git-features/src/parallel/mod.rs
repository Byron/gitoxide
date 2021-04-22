//! Run computations in parallel, or not based the `parallel` feature toggle.
//!
//! ### in_parallel(…)
//!
//! The [`in_parallel(…)`][in_parallel()] is the typical fan-out-fan-in mode of parallelism, with thread local storage
//! made available to a `consume(…)` function to process input. The result is sent to the [`Reduce`] running in the calling
//! thread to aggregate the results into a single output, which is returned by [`in_parallel()`].
//!
//! Interruptions can be achieved by checking for [`is_interrupted()`][crate::interrupt::is_triggered()] in the input iterator
//! or by letting the reducers [`feed(…)`][Reduce::feed()]` method fail.
//!
//! It gets a boost in usability as it allows threads to borrow variables from the stack, most commonly the repository itself
//! or the data to work on.
//!
//! This mode of operation doesn't lend itself perfectly to being wrapped for `async` as it appears like a single long-running
//! operation which runs as fast as possible, which is cancellable only by merit of stopping the input or stopping the output
//! aggregation.
//!
//! ### `reduce::Stepwise`
//!
//! The [`Stepwise`][reduce::Stepwise] iterator works exactly as [`in_parallel()`] except that the processing of the output produced by
//! `consume(I, &mut State) -> O` is made accessible by the `Iterator` trait's `next()` method. As produced work is not
//! buffered, the owner of the iterator controls the progress made.
//!
//! Getting the final output of the [`Reduce`] is achieved through the consuming [`Stepwise::finalize()`][reduce::Stepwise::finalize()] method, which
//! is functionally equivalent to calling [`in_parallel()`].
//!
//! In an `async` context this means that progress is only made each time `next()` is called on the iterator, while merely dropping
//! the iterator will wind down the computation without any result.
//!
//! #### Maintaining Safety
//!
//! In order to assure that threads don't outlive the data they borrow because their handles are leaked, we enforce
//! the `'static` lifetime for its inputs, making it less intuitive to use. It is, however, possible to produce
//! suitable input iterators as long as they can hold something on the heap.
#[cfg(feature = "parallel")]
mod in_parallel;
mod serial;

#[cfg(not(feature = "parallel"))]
pub use serial::*;

#[cfg(feature = "parallel")]
pub use in_parallel::*;

mod eager_iter;
pub use eager_iter::{EagerIter, EagerIterIf};

/// A no-op returning the input _(`desired_chunk_size`, `Some(thread_limit)`, `thread_limit)_ used
/// when the `parallel` feature toggle is not set.
#[cfg(not(feature = "parallel"))]
pub fn optimize_chunk_size_and_thread_limit(
    desired_chunk_size: usize,
    _num_items: Option<usize>,
    thread_limit: Option<usize>,
    _available_threads: Option<usize>,
) -> (usize, Option<usize>, usize) {
    (desired_chunk_size, thread_limit, num_threads(thread_limit))
}

/// Return the 'optimal' _(`size of chunks`,  `amount of threads as Option`, `amount of threads`)_ to use in [`in_parallel()`] for the given
/// `desired_chunk_size`, `num_items`, `thread_limit` and `available_threads`.
///
/// * `desired_chunk_size` is the amount of items per chunk you think should be used.
/// * `num_items` is the total amount of items in the iteration, if `Some`.
///    Otherwise this knowledge will not affect the output of this function.
/// * `thread_limit` is the amount of threads to use at most, if `Some`.
///    Otherwise this knowledge will not affect the output of this function.
/// * `available_threads` is the total amount of threads available, if `Some`.
///    Otherwise the actual amount of available threads is determined by querying the system.
///
/// `Note` that this implementation is available only if the `parallel` feature toggle is set.
#[cfg(feature = "parallel")]
pub fn optimize_chunk_size_and_thread_limit(
    desired_chunk_size: usize,
    num_items: Option<usize>,
    thread_limit: Option<usize>,
    available_threads: Option<usize>,
) -> (usize, Option<usize>, usize) {
    let available_threads = available_threads.unwrap_or_else(num_cpus::get);
    let available_threads = thread_limit
        .map(|l| if l == 0 { available_threads } else { l })
        .unwrap_or(available_threads);

    let (lower, upper) = (50, 1000);
    let (chunk_size, thread_limit) = num_items
        .map(|num_items| {
            let desired_chunks_per_thread_at_least = 2;
            let items = num_items;
            let chunk_size = (items / (available_threads * desired_chunks_per_thread_at_least))
                .max(1)
                .min(upper);
            let num_chunks = items / chunk_size;
            let thread_limit = if num_chunks <= available_threads {
                (num_chunks / desired_chunks_per_thread_at_least).max(1)
            } else {
                available_threads
            };
            (chunk_size, thread_limit)
        })
        .unwrap_or({
            let chunk_size = if available_threads == 1 {
                desired_chunk_size
            } else if desired_chunk_size < lower {
                lower
            } else {
                desired_chunk_size.min(upper)
            };
            (chunk_size, available_threads)
        });
    (chunk_size, Some(thread_limit), thread_limit)
}

/// Always returns 1, available when the `parallel` feature toggle is unset.
#[cfg(not(feature = "parallel"))]
pub(crate) fn num_threads(_thread_limit: Option<usize>) -> usize {
    1
}

/// Returns the amount of threads the system can effectively use as the amount of its logical cores.
///
/// Only available with the `parallel` feature toggle set.
#[cfg(feature = "parallel")]
pub(crate) fn num_threads(thread_limit: Option<usize>) -> usize {
    let logical_cores = || num_cpus::get();
    thread_limit
        .map(|l| if l == 0 { logical_cores() } else { l })
        .unwrap_or_else(logical_cores)
}

///
pub mod reduce {

    #[cfg(feature = "parallel")]
    mod stepped {
        use crate::parallel::num_threads;

        /// An iterator adaptor to allow running computations using [`in_parallel()`][crate::parallel::in_parallel()] in a step-wise manner, see the [module docs][crate::parallel]
        /// for details.
        pub struct Stepwise<Reduce: super::Reduce> {
            /// This field is first to assure it's dropped first and cause threads that are dropped next to stop their loops
            /// as sending results fails when the receiver is dropped.
            receive_result: std::sync::mpsc::Receiver<Reduce::Input>,
            /// `join()` will be called on these guards to assure every thread tries to send through a closed channel. When
            /// that happens, they break out of their loops.
            _threads: Vec<std::thread::JoinHandle<()>>,
            /// The reducer is called only in the thread using the iterator, dropping it has no side effects.
            reducer: Option<Reduce>,
        }

        impl<Reduce: super::Reduce> Drop for Stepwise<Reduce> {
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
                    std::panic::resume_unwind(thread_err);
                }
            }
        }

        impl<Reduce: super::Reduce> Stepwise<Reduce> {
            /// Instantiate a new iterator and start working in threads.
            /// For a description of parameters, see [`in_parallel()`][crate::parallel::in_parallel()].
            pub fn new<InputIter, ThreadStateFn, ConsumeFn, I, O, S>(
                input: InputIter,
                thread_limit: Option<usize>,
                new_thread_state: ThreadStateFn,
                consume: ConsumeFn,
                reducer: Reduce,
            ) -> Self
            where
                InputIter: Iterator<Item = I> + Send + 'static,
                ThreadStateFn: Fn(usize) -> S + Send + Clone + 'static,
                ConsumeFn: Fn(I, &mut S) -> O + Send + Clone + 'static,
                Reduce: super::Reduce<Input = O> + 'static,
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
                Stepwise {
                    _threads: threads,
                    receive_result,
                    reducer: Some(reducer),
                }
            }

            /// Consume the iterator by finishing its iteration and calling [`Reduce::finalize()`][crate::parallel::Reduce::finalize()].
            pub fn finalize(mut self) -> Result<Reduce::Output, Reduce::Error> {
                for value in self.by_ref() {
                    drop(value?);
                }
                self.reducer
                    .take()
                    .expect("this is the last call before consumption")
                    .finalize()
            }
        }

        impl<Reduce: super::Reduce> Iterator for Stepwise<Reduce> {
            type Item = Result<Reduce::FeedProduce, Reduce::Error>;

            fn next(&mut self) -> Option<<Self as Iterator>::Item> {
                self.receive_result
                    .recv()
                    .ok()
                    .and_then(|input| self.reducer.as_mut().map(|r| r.feed(input)))
            }
        }
    }
    #[cfg(not(feature = "parallel"))]
    mod stepped {
        /// An iterator adaptor to allow running computations using [`in_parallel()`] in a step-wise manner, see the [module docs][crate::parallel]
        /// for details.
        pub struct Stepwise<InputIter, ConsumeFn, ThreadState, Reduce> {
            input: InputIter,
            consume: ConsumeFn,
            thread_state: ThreadState,
            reducer: Reduce,
        }

        impl<InputIter, ConsumeFn, Reduce, I, O, S> Stepwise<InputIter, ConsumeFn, S, Reduce>
        where
            InputIter: Iterator<Item = I> + Send,
            ConsumeFn: Fn(I, &mut S) -> O + Send + Sync,
            Reduce: super::Reduce<Input = O>,
            I: Send,
            O: Send,
        {
            /// Instantiate a new iterator.
            /// For a description of parameters, see [`in_parallel()`].
            pub fn new<ThreadStateFn>(
                input: InputIter,
                _thread_limit: Option<usize>,
                new_thread_state: ThreadStateFn,
                consume: ConsumeFn,
                reducer: Reduce,
            ) -> Self
            where
                ThreadStateFn: Fn(usize) -> S + Send + Sync,
            {
                Stepwise {
                    input,
                    consume,
                    thread_state: new_thread_state(0),
                    reducer,
                }
            }

            /// Consume the iterator by finishing its iteration and calling [`Reduce::finalize()`][crate::parallel::Reduce::finalize()].
            pub fn finalize(mut self) -> Result<Reduce::Output, Reduce::Error> {
                for value in self.by_ref() {
                    drop(value?);
                }
                self.reducer.finalize()
            }
        }

        impl<InputIter, ConsumeFn, ThreadState, Reduce, I, O> Iterator for Stepwise<InputIter, ConsumeFn, ThreadState, Reduce>
        where
            InputIter: Iterator<Item = I> + Send,
            ConsumeFn: Fn(I, &mut ThreadState) -> O + Send + Sync,
            Reduce: super::Reduce<Input = O>,
            I: Send,
            O: Send,
        {
            type Item = Result<Reduce::FeedProduce, Reduce::Error>;

            fn next(&mut self) -> Option<<Self as Iterator>::Item> {
                self.input
                    .next()
                    .map(|input| self.reducer.feed((self.consume)(input, &mut self.thread_state)))
            }
        }
    }
    use std::marker::PhantomData;
    pub use stepped::Stepwise;

    /// An trait for aggregating items commonly produced in threads into a single result, without itself
    /// needing to be thread safe.
    pub trait Reduce {
        /// The type fed to the reducer in the [`feed()`][Reduce::feed()] method.
        ///
        /// It's produced by a function that may run on multiple threads.
        type Input;
        /// The type produced in Ok(…) by [`feed()`][Reduce::feed()].
        /// Most reducers by nature use `()` here as the value is in the aggregation.
        /// However, some may use it to collect statistics only and return their Input
        /// in some form as a result here for [`Stepwise`] to be useful.
        type FeedProduce;
        /// The type produced once by the [`finalize()`][Reduce::finalize()] method.
        ///
        /// For traditional reducers, this is the value produced by the entire operation.
        /// For those made for step-wise iteration this may be aggregated statistics.
        type Output;
        /// The error type to use for all methods of this trait.
        type Error;
        /// Called each time a new `item` was produced in order to aggregate it into the final result.
        ///
        /// If an `Error` is returned, the entire operation will be stopped.
        fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error>;
        /// Called once once all items where passed to `feed()`, producing the final `Output` of the operation or an `Error`.
        fn finalize(self) -> Result<Self::Output, Self::Error>;
    }

    /// An identity reducer for those who want to use [`Stepwise`] or [`in_parallel()`][crate::parallel::in_parallel()]
    /// without the use of non-threaded reduction of products created in threads.
    pub struct IdentityWithResult<Input, Error> {
        _input: PhantomData<Input>,
        _error: PhantomData<Error>,
    }

    impl<Input, Error> Default for IdentityWithResult<Input, Error> {
        fn default() -> Self {
            IdentityWithResult {
                _input: Default::default(),
                _error: Default::default(),
            }
        }
    }

    impl<Input, Error> Reduce for IdentityWithResult<Input, Error> {
        type Input = Result<Input, Self::Error>;
        type FeedProduce = Input;
        type Output = ();
        type Error = Error;

        fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
            item
        }

        fn finalize(self) -> Result<Self::Output, Self::Error> {
            Ok(())
        }
    }
}

pub use reduce::Reduce;

/// Run [`in_parallel()`] only if the given `condition()` returns true when eagerly evaluated.
///
/// For parameters, see the documentation of [`in_parallel()`]
pub fn in_parallel_if<I, S, O, R>(
    condition: impl FnOnce() -> bool,
    input: impl Iterator<Item = I> + Send,
    thread_limit: Option<usize>,
    new_thread_state: impl Fn(usize) -> S + Send + Sync,
    consume: impl Fn(I, &mut S) -> O + Send + Sync,
    reducer: R,
) -> Result<<R as Reduce>::Output, <R as Reduce>::Error>
where
    R: Reduce<Input = O>,
    I: Send,
    O: Send,
{
    if num_threads(thread_limit) > 1 && condition() {
        in_parallel(input, thread_limit, new_thread_state, consume, reducer)
    } else {
        serial::in_parallel(input, thread_limit, new_thread_state, consume, reducer)
    }
}
