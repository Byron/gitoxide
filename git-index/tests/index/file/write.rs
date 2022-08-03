use git_index::{decode, write, Version};
use std::cmp::{max, min};

#[test]
fn roundtrips() {
    for version in [Version::V2] {
        for fixture_name in ["V2_empty", "v2", "v2_more_files"] {
            let fixture_path = crate::fixture_index_path(fixture_name);
            let actual = git_index::File::at(&fixture_path, decode::Options::default()).unwrap();

            let mut buf = Vec::<u8>::new();
            let options = write::Options {
                hash_kind: git_hash::Kind::Sha1,
                version,
            };
            actual.state.write_to(&mut buf, options).unwrap();

            match version {
                Version::V2 => {
                    let actual_bytes = std::fs::read(&fixture_path).unwrap();
                    // TODO: Compare all bytes, not just the ones written
                    let outcome = compare_bytes(&buf, &actual_bytes[..buf.len()]);
                    if let Some((index, input, expected)) = outcome {
                        panic! {"\n\nRoundtrip failed for index in fixture {:?} at position {:?}\n\
                        \tInput: ...{:?}...\n\
                        \tExpected: ...{:?}...\n\n\
                        ", &fixture_name, index, &input, &expected}
                    }
                }
                _ => {
                    todo!("read back the written index and compare state in memory");
                    // git_index::State::from_bytes(&buf, FileTime::now(), decode::Options::default()).unwrap();
                }
            }
        }
    }
}

fn compare_bytes<'a, 'b>(input: &'a [u8], expected: &'b [u8]) -> Option<(usize, &'a [u8], &'b [u8])> {
    let return_range_on_error = 10;
    for (i, (a, b)) in input.iter().zip(expected.iter()).enumerate() {
        if a != b {
            return Some((
                i,
                &input[max(i - return_range_on_error, 0)..min(i + return_range_on_error, input.len())],
                &expected[max(i - return_range_on_error, 0)..min(i + return_range_on_error, expected.len())],
            ));
        }
    }
    None
}
