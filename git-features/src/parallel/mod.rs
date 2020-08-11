#[cfg(feature = "parallel")]
mod in_parallel;
mod serial;

#[cfg(not(feature = "parallel"))]
pub use serial::*;

#[cfg(feature = "parallel")]
pub use in_parallel::*;

mod eager;
pub use eager::{EagerIter, EagerIterIf};

#[cfg(not(feature = "parallel"))]
pub fn optimize_chunk_size_and_thread_limit(
    desired_chunk_size: usize,
    _num_items: Option<usize>,
    thread_limit: Option<usize>,
    _available_threads: Option<usize>,
) -> (usize, Option<usize>, usize) {
    return (desired_chunk_size, thread_limit, num_threads(thread_limit));
}

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
        .unwrap_or((
            if available_threads == 1 {
                desired_chunk_size
            } else if desired_chunk_size < lower {
                lower
            } else {
                desired_chunk_size.min(upper)
            },
            available_threads,
        ));
    (chunk_size, Some(thread_limit), thread_limit)
}

#[cfg(not(feature = "parallel"))]
pub(crate) fn num_threads(_thread_limit: Option<usize>) -> usize {
    return 1;
}

#[cfg(feature = "parallel")]
pub(crate) fn num_threads(thread_limit: Option<usize>) -> usize {
    let logical_cores = num_cpus::get();
    thread_limit
        .map(|l| if l == 0 { logical_cores } else { l })
        .unwrap_or(logical_cores)
}

pub trait Reducer {
    type Input;
    type Output;
    type Error;
    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error>;
    fn finalize(self) -> Result<Self::Output, Self::Error>;
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
