use std::convert::Infallible;

use git_odb::data::output::InOrderIter;

#[test]
fn in_order_stays_in_order() {
    assert_eq!(
        InOrderIter::from(vec![Ok::<_, Infallible>((0usize, 'a')), Ok((1, 'b')), Ok((2, 'c'))].into_iter())
            .collect::<Result<Vec<_>, _>>()
            .expect("infallible"),
        vec!['a', 'b', 'c']
    )
}

#[test]
fn out_of_order_items_are_held_until_the_sequence_is_complete() {
    assert_eq!(
        InOrderIter::from(
            vec![
                Ok::<_, Infallible>((2usize, 'c')),
                Ok((1, 'b')),
                Ok((0, 'a')),
                Ok((3, 'd'))
            ]
            .into_iter()
        )
        .collect::<Result<Vec<_>, _>>()
        .expect("infallible"),
        vec!['a', 'b', 'c', 'd']
    )
}

#[test]
fn in_sequence_errors_immediately_trigger_a_fuse() {
    let mut iter = InOrderIter::from(vec![Ok::<_, &'static str>((0usize, 'a')), Err("err"), Ok((1, 'b'))].into_iter());
    assert_eq!(iter.next(), Some(Ok('a')));
    assert_eq!(iter.next(), Some(Err("err")));
    assert_eq!(
        iter.next(),
        None,
        "fuse should have triggered so we don't see anything else"
    );
}

#[test]
fn out_of_sequence_errors_immediately_trigger_a_fuse() {
    let mut iter = InOrderIter::from(vec![Ok::<_, &'static str>((1usize, 'b')), Err("err"), Ok((0, 'a'))].into_iter());
    assert_eq!(iter.next(), Some(Err("err")));
    assert_eq!(
        iter.next(),
        None,
        "fuse should have triggered so we don't see anything else"
    );
}
