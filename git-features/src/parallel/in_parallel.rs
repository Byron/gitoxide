use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::parallel::{num_threads, Reduce};

/// Runs `left` and `right` in parallel, returning their output when both are done.
pub fn join<O1: Send, O2: Send>(left: impl FnOnce() -> O1 + Send, right: impl FnOnce() -> O2 + Send) -> (O1, O2) {
    crossbeam_utils::thread::scope(|s| {
        let left = s
            .builder()
            .name("gitoxide.join.left".into())
            .spawn(|_| left())
            .expect("valid name");
        let right = s
            .builder()
            .name("gitoxide.join.right".into())
            .spawn(|_| right())
            .expect("valid name");
        (left.join().unwrap(), right.join().unwrap())
    })
    .unwrap()
}

/// Runs `f` with a scope to be used for spawning threads that will not outlive the function call.
/// That way it's possible to handle threads without needing the 'static lifetime for data they interact with.
///
/// Note that the threads should not rely on actual parallelism as threading might be turned off entirely, hence should not
/// connect each other with channels as deadlock would occur in single-threaded mode.
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
                s.builder()
                    .name(format!("gitoxide.in_parallel.produce.{thread_id}"))
                    .spawn({
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
                    })
                    .expect("valid name");
            }
            s.builder()
                .name("gitoxide.in_parallel.feed".into())
                .spawn(move |_| {
                    for item in input {
                        if send_input.send(item).is_err() {
                            break;
                        }
                    }
                })
                .expect("valid name");
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
/// This is only good for operations where near-random access isn't detrimental, so it's not usually great
/// for file-io as it won't make use of sorted inputs well.
/// Note that `periodic` is not guaranteed to be called in case other threads come up first and finish too fast.
// TODO: better docs
pub fn in_parallel_with_slice<I, S, R, E>(
    input: &mut [I],
    thread_limit: Option<usize>,
    new_thread_state: impl FnMut(usize) -> S + Send + Clone,
    consume: impl FnMut(&mut I, &mut S) -> Result<(), E> + Send + Clone,
    mut periodic: impl FnMut() -> Option<std::time::Duration> + Send,
    state_to_rval: impl FnOnce(S) -> R + Send + Clone,
) -> Result<Vec<R>, E>
where
    I: Send,
    E: Send,
    R: Send,
{
    let num_threads = num_threads(thread_limit);
    let mut results = Vec::with_capacity(num_threads);
    let stop_everything = &AtomicBool::default();
    let index = &AtomicUsize::default();

    // TODO: use std::thread::scope() once Rust 1.63 is available.
    crossbeam_utils::thread::scope({
        move |s| {
            s.builder()
                .name("gitoxide.in_parallel_with_slice.watch-interrupts".into())
                .spawn({
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
                })
                .expect("valid name");

            let input_len = input.len();
            struct Input<I>(*mut [I])
            where
                I: Send;

            // SAFETY: I is Send + Sync, so is a *mut [I]
            #[allow(unsafe_code)]
            unsafe impl<I> Send for Input<I> where I: Send {}

            let threads: Vec<_> = (0..num_threads)
                .map(|thread_id| {
                    s.builder()
                        .name(format!("gitoxide.in_parallel_with_slice.produce.{thread_id}"))
                        .spawn({
                            let mut new_thread_state = new_thread_state.clone();
                            let state_to_rval = state_to_rval.clone();
                            let mut consume = consume.clone();
                            let input = Input(input as *mut [I]);
                            move |_| {
                                let mut state = new_thread_state(thread_id);
                                while let Ok(input_index) =
                                    index.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
                                        (x < input_len).then_some(x + 1)
                                    })
                                {
                                    if stop_everything.load(Ordering::Relaxed) {
                                        break;
                                    }
                                    // SAFETY: our atomic counter for `input_index` is only ever incremented, yielding
                                    //         each item exactly once.
                                    let item = {
                                        #[allow(unsafe_code)]
                                        unsafe {
                                            &mut (&mut *input.0)[input_index]
                                        }
                                    };
                                    if let Err(err) = consume(item, &mut state) {
                                        stop_everything.store(true, Ordering::Relaxed);
                                        return Err(err);
                                    }
                                }
                                Ok(state_to_rval(state))
                            }
                        })
                        .expect("valid name")
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
