use std::{convert::TryInto, time::Duration};

use gix_utils::backoff::Exponential;

const EXPECTED_TILL_SECOND: &[usize] = &[
    1usize, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529, 576,
    625, 676, 729, 784, 841, 900, 961, 1000, 1000,
];

#[test]
fn random_exponential_produces_values_in_the_correct_range() {
    let mut num_identities = 0;
    for (actual, expected) in Exponential::default_with_random().zip(EXPECTED_TILL_SECOND) {
        let actual: usize = actual.as_millis().try_into().unwrap();
        if actual == *expected {
            num_identities += 1;
        }
        assert!(
            actual * 1000 >= (expected - 1) * 750,
            "value too small: {actual} < {expected}"
        );
        assert!(
            actual * 1000 <= (expected + 1) * 1250,
            "value too big: {actual} > {expected}"
        );
    }
    assert!(
        num_identities < EXPECTED_TILL_SECOND.len(),
        "too many untransformed values: {num_identities}"
    );
}

#[test]
fn how_many_iterations_for_a_second_of_waittime() {
    let max = Duration::from_millis(1000);
    assert_eq!(Exponential::default().until_no_remaining(max).count(), 14);
    assert_eq!(
        Exponential::default()
            .until_no_remaining(max)
            .reduce(|acc, n| acc + n)
            .unwrap(),
        Duration::from_millis(1015),
        "a little overshoot"
    );
}

#[test]
fn output_with_default_settings() {
    assert_eq!(
        Exponential::default().take(33).collect::<Vec<_>>(),
        EXPECTED_TILL_SECOND
            .iter()
            .map(|n| Duration::from_millis(*n as u64))
            .collect::<Vec<_>>()
    );
}
