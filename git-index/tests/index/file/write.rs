use filetime::FileTime;
use git_index::{decode, write, Version};
use std::{
    cmp::{max, min},
    io::Write,
};

#[test]
fn roundtrips() {
    let input = [
        ("V2_empty", write::Options::default()),
        ("v2", write::Options::default()),
        (
            "v2_more_files",
            write::Options {
                end_of_index_entry: false,
                ..write::Options::default()
            },
        ),
    ];

    for (fixture_name, options) in input {
        let path = crate::fixture_index_path(fixture_name);
        let actual_index = git_index::File::at(&path, decode::Options::default()).unwrap();
        let actual_bytes = std::fs::read(&path).unwrap();

        let mut output = Vec::<u8>::new();
        let mut write_hasher = git_features::hash::Write::new(&mut output, options.hash_kind);
        actual_index.state.write_to(&mut write_hasher, options).unwrap();
        let hash = write_hasher.hash.digest();
        output.write_all(&hash).unwrap();

        let (output_index, _) =
            git_index::State::from_bytes(&output, FileTime::now(), decode::Options::default()).unwrap();

        assert_eq!(output_index.version(), Version::V2);
        assert_eq!(output_index.entries().len(), actual_index.entries().len());

        if let Some((index, input, expected)) = find_first_nonmatching_byte(&output, &actual_bytes, fixture_name) {
            panic! {"\n\nRoundtrip failed for index in fixture {:?} at position {:?}\n\
            \t   Input: ...{:?}...\n\
            \tExpected: ...{:?}...\n\n\
            ", &fixture_name, index, &input, &expected}
        }
    }
}

fn find_first_nonmatching_byte<'a, 'b>(
    output: &'a [u8],
    actual: &'b [u8],
    fixture_name: &str,
) -> Option<(usize, &'a [u8], &'b [u8])> {
    assert_eq!(output.len(), actual.len(), "lengths not equal in {:?}", fixture_name);

    let return_range_on_error = 10;
    for (i, (a, b)) in output.iter().zip(actual.iter()).enumerate() {
        if a != b {
            return Some((
                i,
                &output[max(i - return_range_on_error, 0)..min(i + return_range_on_error, output.len())],
                &actual[max(i - return_range_on_error, 0)..min(i + return_range_on_error, actual.len())],
            ));
        }
    }

    None
}
