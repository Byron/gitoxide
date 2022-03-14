use crate::parallel::{num_threads, Reduce};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Runs `left` and `right` in parallel, returning their output when both are done.
pub fn join<O1: Send, O2: Send>(left: impl FnOnce() -> O1 + Send, right: impl FnOnce() -> O2 + Send) -> (O1, O2) {
    crossbeam_utils::thread::scope(|s| {
        let left = s.spawn(|_| left());
        let right = s.spawn(|_| right());
        (left.join().unwrap(), right.join().unwrap())
    })
    .unwrap()
}

/// Runs `f` with a scope to be used for spawning threads that will not outlive the function call.
/// That way it's possible to handle threads without needing the 'static lifetime for data they interact with.
///
/// Note that the threads should not rely on actual parallelism as threading might be turned off entirely, hence should not
/// connect each other with channels as deadlock would occour in single-threaded mode.
pub fn threads<'env, F, R>(f: F) -> std::thread::Result<R>
where
    F: FnOnce(&crossbeam_utils::thread::Scope<'env>) -> R,
{
    crossbeam_utils::thread::scope(f)
}

/// Read items from `input` and `consume` them in multiple threads,
/// whose output output is collected by a `reducer`. Its task is to
/// aggregate these outputs into the final result returned by this function with the benefit of not having to be thread-safe.
///
/// * if `thread_limit` is `Some`, the given amount of threads will be used. If `None`, all logical cores will be used.
/// * `new_thread_state(thread_number) -> State` produces thread-local state once per thread to be based to `consume`
/// * `consume(Item, &mut State) -> Output` produces an output given an input obtained by `input` along with mutable state initially
///   created by `new_thread_state(…)`.
/// * For `reducer`, see the [`Reduce`] trait
pub fn in_parallel<I, S, O, R>(
    input: impl Iterator<Item = I> + Send,
    thread_limit: Option<usize>,
    new_thread_state: impl FnMut(usize) -> S + Send + Clone,
    consume: impl FnMut(I, &mut S) -> O + Send + Clone,
    mut reducer: R,
) -> Result<<R as Reduce>::Output, <R as Reduce>::Error>
where
    R: Reduce<Input = O>,
    I: Send,
    O: Send,
{
    let num_threads = num_threads(thread_limit);
    crossbeam_utils::thread::scope(move |s| {
        let receive_result = {
            let (send_input, receive_input) = crossbeam_channel::bounded::<I>(num_threads);
            let (send_result, receive_result) = crossbeam_channel::bounded::<O>(num_threads);
            for thread_id in 0..num_threads {
                s.spawn({
                    let send_result = send_result.clone();
                    let receive_input = receive_input.clone();
                    let mut new_thread_state = new_thread_state.clone();
                    let mut consume = consume.clone();
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

#[allow(missing_docs)] // TODO: docs
pub fn in_parallel_with_mut_slice_in_chunks<I, S, O, E>(
    input: &mut [I],
    chunk_size: usize,
    thread_limit: Option<usize>,
    new_thread_state: impl FnMut(usize) -> S + Send + Clone,
    consume: impl FnMut(&mut [I], &mut S) -> Result<O, E> + Send + Clone,
    mut periodic: impl FnMut() -> std::time::Duration + Send,
) -> Result<Vec<O>, E>
where
    I: Send + Sync,
    O: Send,
    E: Send,
{
    let num_threads = num_threads(thread_limit);
    let num_chunks = ((input.len() / chunk_size) + if input.len() % chunk_size == 0 { 0 } else { 1 }).max(1);
    let input = &*input; // downgrade to immutable
    let results: Vec<parking_lot::Mutex<Option<Result<O, E>>>> =
        std::iter::repeat_with(Default::default).take(num_chunks).collect();
    let stop_periodic = &AtomicBool::default();
    let chunks_left = &AtomicUsize::new(num_chunks);

    crossbeam_utils::thread::scope({
        let results = &results;
        move |s| {
            s.spawn({
                move |_| loop {
                    if stop_periodic.load(Ordering::Relaxed) {
                        break;
                    }

                    std::thread::sleep(periodic());
                }
            });

            let threads: Vec<_> = (0..num_threads)
                .map(|n| {
                    s.spawn({
                        let mut new_thread_state = new_thread_state.clone();
                        let mut consume = consume.clone();
                        move |_| {
                            let mut state = new_thread_state(n);
                            let mut chunk_index = n;
                            while let Some(res) = &results.get(chunk_index) {
                                if let Some(mut guard) = res.try_lock() {
                                    match &mut *guard {
                                        Some(_) => {
                                            // somebody stole our work, enter work-stealing mode.
                                            break;
                                        }
                                        res @ None => {
                                            let chunk_range = {
                                                let start = chunk_size * chunk_index;
                                                start..(start + chunk_size).min(input.len())
                                            };
                                            let chunk = {
                                                let c = &input[chunk_range];
                                                let mut_ptr = c.as_ptr() as *mut _;
                                                // SAFETY: We know that 'chunks' is only non-overlapping slices due to the way we address them
                                                // AND lock their results.
                                                #[allow(unsafe_code)]
                                                unsafe {
                                                    std::slice::from_raw_parts_mut(mut_ptr, c.len())
                                                }
                                            };
                                            let result = consume(chunk, &mut state);
                                            chunks_left.fetch_sub(1, Ordering::SeqCst);
                                            let abort = result.is_err();
                                            *res = Some(result);
                                            if abort {
                                                return;
                                            }
                                        }
                                    }
                                }
                                chunk_index += num_threads;
                            }
                            // TODO: work-stealing logic once we are out of bounds. Pose as prior thread and walk backwards.
                        }
                    })
                })
                .collect();
            for thread in threads {
                thread.join().expect("must not panic");
            }

            stop_periodic.store(true, Ordering::Relaxed);
        }
    })
    .unwrap();

    let mut unwrapped_results = Vec::with_capacity(results.len());
    let mut expect_early_bailout = false;
    for res in results {
        match res.into_inner() {
            Some(res) => unwrapped_results.push(res?),
            None => {
                expect_early_bailout = true;
            }
        }
    }
    assert!(
        !expect_early_bailout,
        "BUG: we shouldn't be here - a thread computation failed and we should have found it and aborted early"
    );
    Ok(unwrapped_results)
}
