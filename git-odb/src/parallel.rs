pub trait Reducer {
    type Input;
    type Output;
    type Error;
    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error>;
    fn finalize(&mut self) -> Result<Self::Output, Self::Error>;
}

mod serial {
    use crate::parallel::Reducer;

    pub fn in_parallel<I, S, O, R>(
        input: impl Iterator<Item = I> + Send,
        new_thread_state: impl Fn() -> S + Send + Sync + Copy,
        consume: impl Fn(I, &mut S) -> O + Send + Copy,
        mut reducer: R,
    ) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
    where
        R: Reducer<Input = O>,
        I: Send,
        O: Send,
    {
        let mut state = new_thread_state();
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

    pub fn in_parallel<I, S, O, R>(
        input: impl Iterator<Item = I> + Send,
        new_thread_state: impl Fn() -> S + Send + Sync + Copy,
        consume: impl Fn(I, &mut S) -> O + Send + Copy,
        mut reducer: R,
    ) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
    where
        R: Reducer<Input = O>,
        I: Send,
        O: Send,
    {
        let logical_cpus = num_cpus::get();
        thread::scope(move |scope| {
            let receive_result = {
                let (send_input, receive_input) = crossbeam_channel::bounded::<I>(logical_cpus);
                let (send_result, receive_result) = flume::bounded::<O>(logical_cpus);
                for _ in 0..logical_cpus {
                    scope.spawn({
                        let send_result = send_result.clone();
                        let receive_input = receive_input.clone();
                        move |_| {
                            let mut state = new_thread_state();
                            for item in receive_input {
                                send_result.send(consume(item, &mut state)).ok();
                            }
                        }
                    });
                }
                scope.spawn(move |_| {
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
    new_thread_state: impl Fn() -> S + Send + Sync + Copy,
    consume: impl Fn(I, &mut S) -> O + Send + Copy,
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
