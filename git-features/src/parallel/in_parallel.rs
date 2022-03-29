use std::sync::atomic::{AtomicBool, Ordering};

use crate::parallel::{num_threads, Reduce};

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
    new_thread_state: impl Fn(usize) -> S + Send + Clone,
    consume: impl Fn(I, &mut S) -> O + Send + Clone,
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
                    let new_thread_state = new_thread_state.clone();
                    let consume = consume.clone();
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
    .expect("no panic")
}

/// An experiment to have fine-grained per-item parallelization with built-in aggregation via thread state.
/// This is only good for operations where near-random access isn't detremental, so it's not usually great
/// for file-io as it won't make use of sorted inputs well.
/// Note that `periodic` is not guaranteed to be called in case other threads come up first and finish too fast.
// TODO: better docs
pub fn in_parallel_with_slice<I, S, E>(
    input: &[I],
    thread_limit: Option<usize>,
    new_thread_state: impl FnMut(usize) -> S + Send + Clone,
    consume: impl FnMut(&I, &mut S) -> Result<(), E> + Send + Clone,
    mut periodic: impl FnMut() -> Option<std::time::Duration> + Send,
) -> Result<Vec<S>, E>
where
    I: Send + Sync,
    E: Send + Sync,
    S: Send,
{
    let num_threads = num_threads(thread_limit);
    let num_items = input.len();
    let mut results = Vec::with_capacity(num_threads);
    let stop_everything = &AtomicBool::default();

    crossbeam_utils::thread::scope({
        move |s| {
            s.spawn({
                move |_| loop {
                    if stop_everything.load(Ordering::Relaxed) {
                        break;
                    }

                    match periodic() {
                        Some(duration) => std::thread::sleep(duration),
                        None => {
                            stop_everything.store(true, Ordering::Relaxed);
                            break;
                        }
                    }
                }
            });

            let threads: Vec<_> = (0..num_threads)
                .map(|thread_id| {
                    s.spawn({
                        let mut new_thread_state = new_thread_state.clone();
                        let mut consume = consume.clone();
                        move |_| {
                            let mut state = new_thread_state(thread_id);
                            for input_index in (thread_id..num_items).step_by(num_threads) {
                                if stop_everything.load(Ordering::Relaxed) {
                                    break;
                                }
                                let item = &input[input_index];
                                if let Err(err) = consume(item, &mut state) {
                                    stop_everything.store(true, Ordering::Relaxed);
                                    return Err(err);
                                }
                            }
                            Ok(state)
                        }
                    })
                })
                .collect();
            for thread in threads {
                match thread.join() {
                    Ok(res) => {
                        results.push(res?);
                    }
                    Err(err) => {
                        // a panic happened, stop the world gracefully (even though we panic later)
                        stop_everything.store(true, Ordering::Relaxed);
                        std::panic::resume_unwind(err);
                    }
                }
            }

            stop_everything.store(true, Ordering::Relaxed);
            Ok(results)
        }
    })
    .expect("no panic")
}
