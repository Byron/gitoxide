use std::{collections::HashMap, time::SystemTime};

use git_date::time::{format, Format};
use once_cell::sync::Lazy;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

static BASELINE: Lazy<HashMap<String, (String, usize, u32)>> = Lazy::new(|| {
    let base = git_testtools::scripted_fixture_repo_read_only("generate_git_date_baseline.sh").unwrap();

    (|| -> Result<_> {
        let mut map = HashMap::new();
        let file = std::fs::read(base.join("baseline.git"))?;
        let baseline = std::str::from_utf8(&file).expect("valid utf");
        let mut lines = baseline.lines();
        while let Some(date_str) = lines.next() {
            let name = lines.next().expect("four lines per baseline").to_string();
            let exit_code = lines.next().expect("four lines per baseline").parse()?;
            let output: u32 = lines
                .next()
                .expect("four lines per baseline")
                .parse()
                .expect("valid epoch value");
            map.insert(date_str.into(), (name, exit_code, output));
        }
        Ok(map)
    })()
    .unwrap()
});

#[test]
fn baseline() {
    for (pattern, (name, exit_code, output)) in BASELINE.iter() {
        let res = git_date::parse(pattern.as_str(), Some(SystemTime::now()));
        assert_eq!(
            res.is_ok(),
            *exit_code == 0,
            "{pattern:?} disagrees with baseline: {res:?}"
        );
        if *exit_code == 0 {
            let t = res.unwrap();
            let actual = t.seconds_since_unix_epoch;
            assert_eq!(actual, *output, "{pattern:?} disagrees with baseline: {actual:?}");
            if name == "" {
                // This test is not appropriate for round-trip, as the input is malformed.
                continue;
            }
            let reformatted = t.format(match name.as_str() {
                "RFC2822" => Format::Custom(format::RFC2822),
                "ISO8601" => Format::Custom(format::ISO8601),
                "ISO8601_STRICT" => Format::Custom(format::ISO8601_STRICT),
                "DEFAULT" => Format::Custom(format::DEFAULT),
                "UNIX" => Format::Unix,
                "RAW" => Format::Raw,
                &_ => Format::Raw,
            });
            assert_eq!(
                reformatted, *pattern,
                "{reformatted:?} disagrees with baseline: {pattern:?}"
            );
        }
    }
}
