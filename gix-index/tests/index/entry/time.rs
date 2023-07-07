use std::time::SystemTime;

use gix_index::entry;

#[test]
fn conversion_roundtrip() {
    for sample in [entry::stat::Time::default(), entry::stat::Time { secs: 42, nsecs: 100 }] {
        let other: SystemTime = sample.into();
        let new_sample: entry::stat::Time = other.try_into().unwrap();
        assert_eq!(
            new_sample, sample,
            "sample is still the same after conversion to system-time and back"
        );
    }
}
