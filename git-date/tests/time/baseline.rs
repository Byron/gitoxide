use std::{collections::HashMap, time::SystemTime};

use git_date::time::{format, Format};
use once_cell::sync::Lazy;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Sample {
    format_name: Option<String>,
    exit_code: usize,
    time_in_seconds_since_unix_epoch: u32,
}

static BASELINE: Lazy<HashMap<String, Sample>> = Lazy::new(|| {
    (|| -> Result<_> {
        let base = git_testtools::scripted_fixture_read_only("generate_git_date_baseline.sh")?;
        let mut map = HashMap::new();
        let file = std::fs::read(base.join("baseline.git"))?;
        let baseline = std::str::from_utf8(&file).expect("valid utf");
        let mut lines = baseline.lines();
        while let Some(date_str) = lines.next() {
            let format_name = lines.next().expect("four lines per baseline").to_string();
            let exit_code = lines.next().expect("four lines per baseline").parse()?;
            let time_in_seconds_since_unix_epoch: u32 = lines
                .next()
                .expect("four lines per baseline")
                .parse()
                .expect("valid epoch value");
            map.insert(
                date_str.into(),
                Sample {
                    format_name: (!format_name.is_empty()).then(|| format_name),
                    exit_code,
                    time_in_seconds_since_unix_epoch,
                },
            );
        }
        Ok(map)
    })()
    .expect("baseline format is well known and can always be parsed")
});

#[test]
fn parse_compare_format() {
    for (
        pattern,
        Sample {
            format_name,
            exit_code,
            time_in_seconds_since_unix_epoch,
        },
    ) in BASELINE.iter()
    {
        let res = git_date::parse(pattern.as_str(), Some(SystemTime::now()));
        assert_eq!(
            res.is_ok(),
            *exit_code == 0,
            "{pattern:?} disagrees with baseline: {res:?}"
        );
        if let Ok(t) = res {
            let actual = t.seconds_since_unix_epoch;
            assert_eq!(
                actual, *time_in_seconds_since_unix_epoch,
                "{pattern:?} disagrees with baseline seconds since epoch: {actual:?}"
            );
            if let Some(format_name) = format_name {
                let reformatted = t.format(match format_name.as_str() {
                    "RFC2822" => Format::Custom(format::RFC2822),
                    "ISO8601" => Format::Custom(format::ISO8601),
                    "ISO8601_STRICT" => Format::Custom(format::ISO8601_STRICT),
                    "DEFAULT" => Format::Custom(format::DEFAULT),
                    "UNIX" => Format::Unix,
                    "RAW" => Format::Raw,
                    unknown => unreachable!("All formats should be well-known and implemented: {unknown:?}"),
                });
                assert_eq!(
                    reformatted, *pattern,
                    "{reformatted:?} disagrees with baseline pattern: {pattern:?}"
                );
            }
        }
    }
}
