//! Tests that are working similarly in parallel and serial mode
use gix_features::parallel;

mod in_order_iter;

#[derive(Default)]
struct Adder {
    count: usize,
}

impl parallel::Reduce for Adder {
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
fn in_parallel_with_mut_slice_in_chunks() {
    let num_items = 33;
    let mut input: Vec<_> = std::iter::repeat(1).take(num_items).collect();
    let counts = parallel::in_parallel_with_slice(
        &mut input,
        None,
        |_| 0usize,
        |item, acc, _threads_eft, _should_interrupt| {
            *acc += *item;
            *item += 1;
            Ok::<_, ()>(())
        },
        || Some(std::time::Duration::from_millis(10)),
        std::convert::identity,
    )
    .unwrap();
    let expected = std::iter::repeat(1).take(num_items).sum::<usize>();
    assert_eq!(counts.iter().sum::<usize>(), expected);
    assert_eq!(input.iter().sum::<usize>(), expected * 2, "we increment each entry");
}

#[test]
fn stepped_reduce_next() {
    let mut iter = parallel::reduce::Stepwise::new(
        std::iter::from_fn(|| Some(1)).take(100),
        None,
        |_n| (),
        |input, _state| input,
        Adder::default(),
    );

    let mut aggregate = 0;
    for value in iter.by_ref() {
        aggregate += value.expect("success");
    }
    assert_eq!(aggregate, 100);
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

    let mut iter = parallel::reduce::Stepwise::new(
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

#[test]
fn stepped_reduce_finalize() {
    let iter = parallel::reduce::Stepwise::new(
        std::iter::from_fn(|| Some(1)).take(100),
        None,
        |_n| (),
        |input, _state| input,
        Adder::default(),
    );

    assert_eq!(iter.finalize().expect("success"), 100);
}
