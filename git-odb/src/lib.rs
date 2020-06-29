#![deny(unsafe_code)]

#[cfg(any(feature = "fast-sha1", feature = "minimal-sha1"))]
mod hash;
mod zlib;

mod parallel {
    pub trait Reducer {
        type Input;
        type Output;
        type Error;
        fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error>;
        fn finalize(&mut self) -> Result<Self::Output, Self::Error>;
    }

    pub fn in_parallel<I, S, O, R>(
        input: impl Iterator<Item = I>,
        new_thread_state: impl Fn() -> S,
        consume: impl Fn(I, &mut S) -> O,
        mut reducer: R,
    ) -> Result<<R as Reducer>::Output, <R as Reducer>::Error>
    where
        R: Reducer<Input = O>,
    {
        let mut state = new_thread_state();
        for item in input {
            reducer.feed(consume(item, &mut state))?;
        }
        reducer.finalize()
    }
}

pub mod loose;
pub mod pack;
