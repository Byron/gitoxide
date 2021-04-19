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

#[cfg(feature = "parallel")]
mod _static {
    /// An iterator adaptor to allow running computations using [`in_parallel()`] in a step-wise manner, see the [module docs][crate::parallel]
    /// for details.
    struct SteppedReduce<Reducer: crate::parallel::Reducer> {
        /// This field is first to assure it's dropped first and cause threads that are dropped next to stop their loops
        /// as sending results fails when the receiver is dropped.
        receive_result: std::sync::mpsc::Receiver<Reducer::Input>,
        /// `join()` will be called on these guards to assure every thread tries to send through a closed channel. When
        /// that happens, they break out of their loops.
        _threads: Vec<std::thread::JoinHandle<()>>,
        /// The reducer is called only in the thread using the iterator, dropping it has no side effects.
        reducer: Option<Reducer>,
    }

    impl<Reducer: crate::parallel::Reducer> Drop for SteppedReduce<Reducer> {
        fn drop(&mut self) {
            for value in self.by_ref() {
                drop(value);
            }
            let mut last_err = None;
            for handle in std::mem::replace(&mut self._threads, Default::default()) {
                if let Err(err) = handle.join() {
                    last_err = Some(err);
                };
            }
            if let Some(thread_err) = last_err {
                std::panic::panic_any(thread_err);
            }
        }
    }

    impl<Reducer: crate::parallel::Reducer> SteppedReduce<Reducer> {
        /// Instantiate a new iterator and start working in threads.
        /// For a description of parameters, see [`in_parallel()`].
        pub fn new<InputIter, ThreadStateFn, ConsumeFn, I, O, S>(
            input: InputIter,
            thread_limit: Option<usize>,
            new_thread_state: ThreadStateFn,
            consume: ConsumeFn,
            reducer: Reducer,
        ) -> Self
        where
            InputIter: Iterator<Item = I> + Send + 'static,
            ThreadStateFn: Fn(usize) -> S + Send + Sync + Clone + 'static,
            ConsumeFn: Fn(I, &mut S) -> O + Send + Sync + Clone + 'static,
            Reducer: crate::parallel::Reducer<Input = O> + 'static,
            I: Send + 'static,
            O: Send + 'static,
        {
            let num_threads = 4; // FIXME num_threads()
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
            SteppedReduce {
                _threads: threads,
                receive_result,
                reducer: Some(reducer),
            }
        }

        /// Consume the iterator by finishing its iteration and calling [`Reducer::finalize()`][crate::parallel::Reducer::finalize()].
        pub fn finalize(mut self) -> Result<Reducer::Output, Reducer::Error> {
            for value in self.by_ref() {
                drop(value?);
            }
            self.reducer
                .take()
                .expect("this is the last call before consumption")
                .finalize()
        }
    }

    impl<Reducer: crate::parallel::Reducer> Iterator for SteppedReduce<Reducer> {
        type Item = Result<Reducer::FeedProduce, Reducer::Error>;

        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            self.receive_result
                .recv()
                .ok()
                .and_then(|input| self.reducer.as_mut().map(|r| r.feed(input)))
        }
    }

    #[derive(Default)]
    struct Adder {
        count: usize,
    }

    impl git_features::parallel::Reducer for Adder {
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
        struct ArcIter(std::sync::Arc<Vec<usize>>, usize);
        impl Iterator for ArcIter {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                let n = self.0.get(self.1).copied();
                self.1 += 1;
                n
            }
        }

        let mut iter = SteppedReduce::new(
            ArcIter(seq.clone(), 0).enumerate(),
            None,
            {
                let seq = std::sync::Arc::clone(&seq);
                move |_n| seq.len()
            },
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
