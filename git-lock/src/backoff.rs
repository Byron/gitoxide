use std::time::Duration;

pub fn randomize(backoff_ms: usize) -> usize {
    let new_value = (fastrand::usize(750..=1250) * backoff_ms) / 1000;
    if new_value == 0 {
        backoff_ms
    } else {
        new_value
    }
}

struct Exponential<Fn> {
    multiplier: usize,
    max_multiplier: usize,
    exponent: usize,
    transform: Fn,
}

impl Default for Exponential<fn(usize) -> usize> {
    fn default() -> Self {
        Exponential {
            multiplier: 1,
            max_multiplier: 1000,
            exponent: 1,
            transform: std::convert::identity,
        }
    }
}

impl Exponential<fn(usize) -> usize> {
    fn default_with_random() -> Self {
        Exponential {
            multiplier: 1,
            max_multiplier: 1000,
            exponent: 1,
            transform: randomize,
        }
    }
}

impl<Transform> Exponential<Transform>
where
    Transform: Fn(usize) -> usize,
{
    fn until_no_remaining(&mut self, time: Duration) -> impl Iterator<Item = Duration> + '_ {
        let mut elapsed = Duration::default();
        let mut stop_next_iteration = false;
        self.take_while(move |d| {
            if stop_next_iteration {
                false
            } else {
                elapsed += *d;
                if elapsed > time {
                    stop_next_iteration = true;
                }
                true
            }
        })
    }
}

impl<Transform> Iterator for Exponential<Transform>
where
    Transform: Fn(usize) -> usize,
{
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        let wait = Duration::from_millis((self.transform)(self.multiplier) as u64);

        self.multiplier += 2 * self.exponent + 1;
        if self.multiplier > self.max_multiplier {
            self.multiplier = self.max_multiplier;
        } else {
            self.exponent += 1;
        }
        Some(wait)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    const EXPECTED_TILL_SECOND: &[usize] = &[
        1usize, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196, 225, 256, 289, 324, 361, 400, 441, 484, 529,
        576, 625, 676, 729, 784, 841, 900, 961, 1000, 1000,
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
                "value too small: {} < {}",
                actual,
                expected
            );
            assert!(
                actual * 1000 <= (expected + 1) * 1250,
                "value too big: {} > {}",
                actual,
                expected
            );
        }
        assert!(
            num_identities < EXPECTED_TILL_SECOND.len(),
            "too many untransformed values: {}",
            num_identities
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
}
