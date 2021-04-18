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
