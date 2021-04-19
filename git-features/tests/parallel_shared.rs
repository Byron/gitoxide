//! Tests that are working similarly in parallel and serial mode
use git_features::parallel;

#[derive(Default)]
struct Adder {
    count: usize,
}

impl parallel::Reducer for Adder {
    type Input = usize;
    type FeedProduce = usize;
    type Output = usize;
    type Error = ();

    fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
        self.count += item;
        Ok(item)
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        Ok(self.count)
    }
}

#[test]
fn in_parallel() {
    let res = parallel::in_parallel(
        std::iter::from_fn(|| Some(1)).take(100),
        None,
        |_n| (),
        |input, _state| input,
        Adder::default(),
    )
    .expect("successful computation");
    assert_eq!(res, 100);
}

#[test]
fn stepped_reduce_next() {
    let mut iter = unsafe {
        parallel::SteppedReduce::new(
            std::iter::from_fn(|| Some(1)).take(100),
            None,
            |_n| (),
            |input, _state| input,
            Adder::default(),
        )
    };

    let mut aggregate = 0;
    for value in iter.by_ref() {
        aggregate += value.expect("success");
    }
    assert_eq!(aggregate, 100);
}

#[test]
fn stepped_reduce_ref_input_and_consume() {
    let seq = vec![0usize, 1, 2];
    let mut iter = unsafe {
        parallel::SteppedReduce::new(
            seq.iter().enumerate(),
            None,
            |_n| seq.len(),
            |(idx, ref_val), _state| seq[idx] * *ref_val,
            Adder::default(),
        )
    };

    let mut aggregate = 0;
    for value in iter.by_ref() {
        aggregate += value.expect("success");
    }
    assert_eq!(aggregate, 5);
}

#[test]
fn stepped_reduce_finalize() {
    let iter = unsafe {
        parallel::SteppedReduce::new(
            std::iter::from_fn(|| Some(1)).take(100),
            None,
            |_n| (),
            |input, _state| input,
            Adder::default(),
        )
    };

    assert_eq!(iter.finalize().expect("success"), 100);
}

mod _static {
    //! Tests that are working similarly in parallel and serial mode
    use git_features::parallel;

    /// An iterator adaptor to allow running computations using [`in_parallel()`] in a step-wise manner, see the [module docs][crate::parallel]
    /// for details.
    pub struct SteppedReduceStatic<InputIter, ConsumeFn, ThreadState, Reducer> {
        input: InputIter,
        consume: ConsumeFn,
        thread_state: ThreadState,
        reducer: Reducer,
    }

    impl<InputIter, ConsumeFn, Reducer, I, O, S> SteppedReduceStatic<InputIter, ConsumeFn, S, Reducer>
    where
        InputIter: Iterator<Item = I>,
        ConsumeFn: Fn(I, &mut S) -> O,
        Reducer: crate::parallel::Reducer<Input = O>,
        I: Send,
        O: Send,
    {
        /// Instantiate a new iterator.
        /// For a description of parameters, see [`in_parallel()`].
        pub fn new<InputIterFn, ThreadStateFn>(
            new_iter: InputIterFn,
            _thread_limit: Option<usize>,
            new_thread_state: ThreadStateFn,
            consume: ConsumeFn,
            reducer: Reducer,
        ) -> Self
        where
            ThreadStateFn: Fn(usize) -> S,
            InputIterFn: FnOnce() -> InputIter,
        {
            SteppedReduceStatic {
                input: new_iter(),
                consume,
                thread_state: new_thread_state(0),
                reducer,
            }
        }

        /// Consume the iterator by finishing its iteration and calling [`Reducer::finalize()`][crate::parallel::Reducer::finalize()].
        pub fn finalize(mut self) -> Result<Reducer::Output, Reducer::Error> {
            for value in self.by_ref() {
                drop(value?);
            }
            self.reducer.finalize()
        }
    }

    impl<InputIter, ConsumeFn, ThreadState, Reducer, I, O> Iterator
        for SteppedReduceStatic<InputIter, ConsumeFn, ThreadState, Reducer>
    where
        InputIter: Iterator<Item = I>,
        ConsumeFn: Fn(I, &mut ThreadState) -> O,
        Reducer: crate::parallel::Reducer<Input = O>,
        I: Send,
        O: Send,
    {
        type Item = Result<Reducer::FeedProduce, Reducer::Error>;

        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            self.input
                .next()
                .map(|input| self.reducer.feed((self.consume)(input, &mut self.thread_state)))
        }
    }

    #[derive(Default)]
    struct Adder {
        count: usize,
    }

    impl parallel::Reducer for Adder {
        type Input = usize;
        type FeedProduce = usize;
        type Output = usize;
        type Error = ();

        fn feed(&mut self, item: Self::Input) -> Result<Self::FeedProduce, Self::Error> {
            self.count += item;
            Ok(item)
        }

        fn finalize(self) -> Result<Self::Output, Self::Error> {
            Ok(self.count)
        }
    }

    #[test]
    fn stepped_reduce_ref_input_and_consume() {
        let seq = std::sync::Arc::new(vec![0usize, 1, 2]);
        let mut iter = SteppedReduceStatic::new(
            {
                let seq = std::sync::Arc::clone(&seq);
                move || seq.iter().map(|v| *v).enumerate()
            },
            None,
            |_n| seq.len(),
            {
                let seq = std::sync::Arc::clone(&seq);
                move |(idx, ref_val): (usize, usize), _state| seq[idx] * ref_val
            },
            Adder::default(),
        );

        let mut aggregate = 0;
        for value in iter.by_ref() {
            aggregate += value.expect("success");
        }
        assert_eq!(aggregate, 5);
    }
}
