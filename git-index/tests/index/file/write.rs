use filetime::FileTime;
use git_features::hash;
use git_index::{decode, write, State, Version};
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

    for (fixture, options) in input {
        let path = crate::fixture_index_path(fixture);
        let expected_index = git_index::File::at(&path, decode::Options::default()).unwrap();
        let expected_bytes = std::fs::read(&path).unwrap();
        let mut generated_bytes = Vec::<u8>::new();
        let mut hasher = hash::Write::new(&mut generated_bytes, options.hash_kind);
        expected_index.state.write_to(&mut hasher, options).unwrap();
        let hash = hasher.hash.digest();
        generated_bytes.write_all(&hash).unwrap();
        let (generated_index, _) =
            State::from_bytes(&generated_bytes, FileTime::now(), decode::Options::default()).unwrap();

        compare_states(&generated_index, &expected_index, options, fixture);
        compare_raw_bytes(&generated_bytes, &expected_bytes, fixture);
    }
}

#[test]
fn v2_index_no_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = crate::fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = write::Options {
            hash_kind: git_hash::Kind::Sha1,
            version: Version::V2,
            tree_cache: false,
            end_of_index_entry: false,
        };
        let mut hasher = hash::Write::new(&mut out, options.hash_kind);
        expected.write_to(&mut hasher, options).unwrap();
        let hash = hasher.hash.digest();
        out.write_all(&hash).unwrap();

        let (generated, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&generated, &expected, options, fixture);
    }
}

#[test]
fn v2_index_tree_extensions() {
    let input = [
        "V2_empty",
        "v2",
        "v2_more_files",
        "v2_split_index",
        "v4_more_files_IEOT",
    ];

    for fixture in input {
        let path = crate::fixture_index_path(fixture);
        let expected = git_index::File::at(&path, decode::Options::default()).unwrap();

        let mut out = Vec::<u8>::new();
        let options = write::Options {
            hash_kind: git_hash::Kind::Sha1,
            version: Version::V2,
            tree_cache: true,
            end_of_index_entry: false,
        };
        let mut hasher = hash::Write::new(&mut out, options.hash_kind);
        expected.write_to(&mut hasher, options).unwrap();
        let hash = hasher.hash.digest();
        out.write_all(&hash).unwrap();

        let (generated, _) = State::from_bytes(&out, FileTime::now(), decode::Options::default()).unwrap();
        compare_states(&generated, &expected, options, fixture);
    }
}

fn compare_states(generated: &State, expected: &State, options: write::Options, fixture: &str) {
    assert_eq!(generated.version(), options.version, "version mismatch in {}", fixture);
    assert_eq!(
        generated.tree(),
        match options.tree_cache {
            true => expected.tree(),
            false => None,
        },
        "tree extension mismatch in {}",
        fixture
    );
    assert_eq!(
        generated.entries().len(),
        expected.entries().len(),
        "entry count mismatch in {}",
        fixture
    );
    assert_eq!(
        generated.entries(),
        expected.entries(),
        "entries mismatch in {}",
        fixture
    );
    assert_eq!(
        generated.path_backing(),
        expected.path_backing(),
        "path_backing mismatch in {}",
        fixture
    );
}

fn compare_raw_bytes<'a, 'b>(generated: &'a [u8], expected: &'b [u8], fixture: &str) {
    assert_eq!(generated.len(), expected.len(), "file length mismatch in {}", fixture);

    let print_range = 10;
    for (index, (a, b)) in generated.iter().zip(expected.iter()).enumerate() {
        if a != b {
            let range_left = max(index - print_range, 0);
            let range_right = min(index + print_range, generated.len());
            let generated = &generated[range_left..range_right];
            let expected = &expected[range_left..range_right];

            panic! {"\n\nRoundtrip failed for index in fixture {:?} at position {:?}\n\
            \t   Input: ... {:?} ...\n\
            \tExpected: ... {:?} ...\n\n\
            ", &fixture, index, generated, expected}
        }
    }
}
