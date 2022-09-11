use std::{collections::HashMap, str::FromStr, time::SystemTime};

use bstr::{BString, ByteSlice};
use git_date::{time::Sign, Time};
use once_cell::sync::Lazy;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

static BASELINE: Lazy<HashMap<BString, (usize, u32)>> = Lazy::new(|| {
    let base = git_testtools::scripted_fixture_repo_read_only("generate_git_date_baseline.sh").unwrap();

    (|| -> Result<_> {
        let mut map = HashMap::new();
        let baseline = std::fs::read(base.join("baseline.git"))?;
        let mut lines = baseline.lines();
        while let Some(date_str) = lines.next() {
            let exit_code = lines.next().expect("three lines per baseline").to_str()?.parse()?;
            let output = u32::from_str(
                lines
                    .next()
                    .expect("three lines per baseline")
                    .to_str()
                    .expect("valid utf"),
            )
            .expect("valid epoch value");
            map.insert(date_str.into(), (exit_code, output));
        }
        Ok(map)
    })()
    .unwrap()
});

#[test]
fn baseline() {
    for (pattern, (exit_code, output)) in BASELINE.iter() {
        let res = git_date::parse(pattern.to_str().expect("valid pattern"), Some(SystemTime::now()));
        assert_eq!(
            res.is_ok(),
            *exit_code == 0,
            "{pattern:?} disagrees with baseline: {res:?}"
        );
        if *exit_code == 0 {
            let actual = res.unwrap().seconds_since_unix_epoch;
            assert_eq!(actual, *output, "{pattern:?} disagrees with baseline: {actual:?}")
        }
    }
}

#[test]
fn special_time_is_ok_for_now() {
    assert_eq!(
        git_date::parse("1979-02-26 18:30:00", Some(SystemTime::now())).unwrap(),
        Time {
            seconds_since_unix_epoch: 42,
            offset_in_seconds: 1800,
            sign: Sign::Plus,
        }
    );
}

#[test]
fn short() {
    assert_eq!(
        git_date::parse("1979-02-26", Some(SystemTime::now())).expect("parsed date"),
        Time {
            seconds_since_unix_epoch: 288835200,
            offset_in_seconds: 0,
            sign: Sign::Plus,
        },
        "could not parse with SHORT format"
    );
}

#[test]
fn rfc2822() {
    assert_eq!(
        git_date::parse("Thu, 18 Aug 2022 12:45:06 +0800", None).expect("parsed rfc2822 string"),
        Time {
            seconds_since_unix_epoch: 1660797906,
            offset_in_seconds: 28800,
            sign: Sign::Plus,
        },
        "could not parse with RFC2822 format"
    );
}

#[test]
fn invalid_dates_can_be_produced_without_current_time() {
    assert!(matches!(
        git_date::parse("foobar", None).unwrap_err(),
        git_date::parse::Error::InvalidDateString
    ));
}

mod relative {
    use std::time::SystemTime;

    use git_date::{parse::Error, time::Sign};
    use time::{Duration, OffsetDateTime};

    #[test]
    fn large_offsets() {
        git_date::parse("999999999999999 weeks ago", Some(std::time::UNIX_EPOCH)).ok();
    }

    #[test]
    #[should_panic] // TODO: fix
    fn large_offsets_can_panic_elsewhere() {
        git_date::parse("9999999999 weeks ago", Some(std::time::UNIX_EPOCH)).ok();
    }

    #[test]
    fn offset_leading_to_before_unix_epoch_cannot_be_represented() {
        let err = git_date::parse("1 second ago", Some(std::time::UNIX_EPOCH)).unwrap_err();
        assert!(matches!(err, Error::TooEarly{timestamp} if timestamp == -1));
    }

    #[test]
    fn various() {
        let now = Some(SystemTime::now());
        let two_weeks_ago = git_date::parse("2 weeks ago", now).expect("valid time");
        assert_eq!(Sign::Plus, two_weeks_ago.sign);
        assert_eq!(0, two_weeks_ago.offset_in_seconds);
        let expected = OffsetDateTime::from(now.unwrap()).saturating_sub(Duration::weeks(2));
        // account for the loss of precision when creating `Time` with seconds
        let expected = expected.replace_nanosecond(0).unwrap();
        assert_eq!(
            OffsetDateTime::from_unix_timestamp(two_weeks_ago.seconds_since_unix_epoch as i64).expect("valid datetime"),
            expected,
            "relative times differ"
        );
    }
}
