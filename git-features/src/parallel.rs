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
    use crate::parallel::Reducer;
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
        new_thread_state: impl Fn(usize) -> S + Send + Sync,
        consume: impl Fn(I, &mut S) -> O + Send + Sync,
        mut reducer: R,
    ) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
    where
        R: Reducer<Input = O>,
        I: Send,
        O: Send,
    {
        let logical_cores = num_cpus::get();
        let new_thread_state = &new_thread_state;
        let consume = &consume;
        thread::scope(move |s| {
            let receive_result = {
                let (send_input, receive_input) = crossbeam_channel::bounded::<I>(logical_cores);
                let (send_result, receive_result) = std::sync::mpsc::sync_channel::<O>(logical_cores);
                for thread_id in 0..logical_cores {
                    s.spawn({
                        let send_result = send_result.clone();
                        let receive_input = receive_input.clone();
                        move |_| {
                            let mut state = new_thread_state(thread_id);
                            for item in receive_input {
                                send_result.send(consume(item, &mut state)).unwrap();
                            }
                        }
                    });
                }
                s.spawn(move |_| {
                    for item in input {
                        send_input.send(item).unwrap();
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

#[cfg(not(feature = "parallel"))]
pub use serial::*;

#[cfg(feature = "parallel")]
pub use in_parallel::*;

pub fn in_parallel_if<I, S, O, R>(
    condition: impl FnOnce() -> bool,
    input: impl Iterator<Item = I> + Send,
    new_thread_state: impl Fn(usize) -> S + Send + Sync,
    consume: impl Fn(I, &mut S) -> O + Send + Sync,
    reducer: R,
) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
where
    R: Reducer<Input = O>,
    I: Send,
    O: Send,
{
    if condition() {
        in_parallel(input, new_thread_state, consume, reducer)
    } else {
        serial::in_parallel(input, new_thread_state, consume, reducer)
    }
}
