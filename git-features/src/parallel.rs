pub trait Reducer {
    type Input;
    type Output;
    type Error;
    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error>;
    fn finalize(self) -> Result<Self::Output, Self::Error>;
}

mod serial {
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
}

#[cfg(feature = "parallel")]
mod in_parallel {
    use crate::parallel::{num_threads, Reducer};
    use crossbeam_utils::thread;

    pub fn join<O1: Send, O2: Send>(left: impl FnOnce() -> O1 + Send, right: impl FnOnce() -> O2 + Send) -> (O1, O2) {
        thread::scope(|s| {
            let left = s.spawn(|_| left());
            let right = s.spawn(|_| right());
            (left.join().unwrap(), right.join().unwrap())
        })
        .unwrap()
    }

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
        thread::scope(move |s| {
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
                reducer.feed(item)?;
            }
            reducer.finalize()
        })
        .unwrap()
    }
}

pub fn optimize_chunk_size_and_thread_limit(
    desired_chunk_size: usize,
    num_chunks: Option<usize>,
    thread_limit: Option<usize>,
    available_threads: Option<usize>,
) -> (usize, Option<usize>) {
    #[cfg(not(feature = "num_cpus"))]
    return (desired_chunk_size, thread_limit);

    let available_threads = available_threads.unwrap_or_else(|| num_cpus::get());
    let available_threads = thread_limit
        .map(|l| if l == 0 { available_threads } else { l })
        .unwrap_or(available_threads);

    let desired_chunks_per_thread_at_least = 2;
    let (chunk_size, thread_limit) = num_chunks
        .map(|num_chunks| {
            let items = num_chunks * desired_chunk_size;
            let chunk_size = (items / (available_threads * desired_chunks_per_thread_at_least)).max(1);
            let num_chunks = items / chunk_size;
            let thread_limit = if num_chunks <= available_threads {
                (num_chunks / desired_chunks_per_thread_at_least).max(1)
            } else {
                available_threads
            };
            (chunk_size, thread_limit)
        })
        .unwrap_or((
            if available_threads == 1 {
                desired_chunk_size
            } else {
                let arbitrary_desirable_chunk_size = 5;
                if desired_chunk_size < arbitrary_desirable_chunk_size {
                    arbitrary_desirable_chunk_size
                } else {
                    desired_chunk_size
                }
            },
            available_threads,
        ));
    (chunk_size, Some(thread_limit))
}

#[cfg(not(feature = "parallel"))]
pub use serial::*;

#[cfg(feature = "parallel")]
pub use in_parallel::*;

pub(crate) fn num_threads(thread_limit: Option<usize>) -> usize {
    #[cfg(not(feature = "num_cpus"))]
    return 1;

    let logical_cores = num_cpus::get();
    thread_limit
        .map(|l| if l == 0 { logical_cores } else { l })
        .unwrap_or(logical_cores)
}

pub fn in_parallel_if<I, S, O, R>(
    condition: impl FnOnce() -> bool,
    input: impl Iterator<Item = I> + Send,
    thread_limit: Option<usize>,
    new_thread_state: impl Fn(usize) -> S + Send + Sync,
    consume: impl Fn(I, &mut S) -> O + Send + Sync,
    reducer: R,
) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
where
    R: Reducer<Input = O>,
    I: Send,
    O: Send,
{
    if num_threads(thread_limit) > 1 && condition() {
        in_parallel(input, thread_limit, new_thread_state, consume, reducer)
    } else {
        serial::in_parallel(input, thread_limit, new_thread_state, consume, reducer)
    }
}
