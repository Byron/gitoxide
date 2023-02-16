mod time {
    use std::time::SystemTime;

    use gix_index::entry;

    #[test]
    fn conversion_roundtrip() {
        for sample in [entry::Time::default(), entry::Time { secs: 42, nsecs: 150 }] {
            let other: SystemTime = sample.into();
            let new_sample: entry::Time = other.into();
            assert_eq!(
                new_sample, sample,
                "sample is still the same after conversion to system-time and back"
            );
        }
    }
}
