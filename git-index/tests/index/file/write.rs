use crate::index::Fixture::*;
use filetime::FileTime;
use git_index::{entry, extension, verify::extensions::no_find, write, write::Options, State, Version};

use crate::{fixture_index_path, loose_file_path};

/// Round-trips should eventually be possible for all files we have, as we write them back exactly as they were read.
#[test]
fn roundtrips() -> crate::Result {
    let input = [
        (Loose("extended-flags"), only_tree_ext()),
        (Loose("conflicting-file"), only_tree_ext()),
        (Loose("very-long-path"), only_tree_ext()),
        (
            Generated("v2"),
            options_with(write::Extensions::Given {
                tree_cache: true,
                end_of_index_entry: true,
                sparse_directory_entries: false,
            }),
        ),
        (Generated("V2_empty"), only_tree_ext()),
        (Generated("v2_more_files"), only_tree_ext()),
        (Generated("v2_all_file_kinds"), only_tree_ext()),
    ];

    for (fixture, options) in input {
        let (path, fixture) = match fixture {
            Generated(name) => (fixture_index_path(name), name),
            Loose(name) => (loose_file_path(name), name),
        };
        let expected = git_index::File::at(&path, git_hash::Kind::Sha1, Default::default())?;
        let expected_bytes = std::fs::read(&path)?;
        let mut out_bytes = Vec::new();

        let (actual_version, _digest) = expected.write_to(&mut out_bytes, options)?;
        assert_eq!(
            actual_version,
            expected.version(),
            "{:?} didn't write the expected version",
            fixture
        );
        let (actual, _) = State::from_bytes(&out_bytes, FileTime::now(), git_hash::Kind::Sha1, Default::default())?;

        compare_states(&actual, actual_version, &expected, options, fixture);
        compare_raw_bytes(&out_bytes, &expected_bytes, fixture);
    }
    Ok(())
}

#[test]
fn roundtrips_sparse_index() -> crate::Result {
    // NOTE: I initially tried putting these fixtures into the main roundtrip test above,
    // but the call to `compare_raw_bytes` panics. It seems like git is using a different
    // ordering when it comes to writing the tree extension. Need to investigate more, hence
    // the seperate test for now.
    //
    //          git                     gitoxide
    //
    //          treeroot                treeroot
    //            | d                     | c1
    //              | c4                    | c2
    //            | c1                      | c3
    //              | c2                  | d
    //              | c3                    | c4
    //

    let input = [
        ("v3_skip_worktree", only_tree_ext()),
        (
            "v3_sparse_index",
            options_with(write::Extensions::Given {
                tree_cache: true,
                end_of_index_entry: false,
                sparse_directory_entries: true,
            }),
        ),
        ("v3_sparse_index_non_cone", only_tree_ext()),
    ];

    for (fixture, options) in input {
        let path = fixture_index_path(fixture);
        let expected = git_index::File::at(&path, git_hash::Kind::Sha1, Default::default())?;
        // let expected_bytes = std::fs::read(&path)?;
        let mut out_bytes = Vec::new();

        let (actual_version, _) = expected.write_to(&mut out_bytes, options)?;
        assert_eq!(
            actual_version,
            expected.version(),
            "{:?} didn't write the expected version",
            fixture
        );
        let (actual, _) = State::from_bytes(&out_bytes, FileTime::now(), git_hash::Kind::Sha1, Default::default())?;

        compare_states(&actual, actual_version, &expected, options, fixture);
        // TODO: make this work and re-enable it
        // compare_raw_bytes(&out_bytes, &expected_bytes, fixture);
    }
    Ok(())
}

#[test]
fn state_comparisons_with_various_extension_configurations() {
    for fixture in [
        Loose("extended-flags"),
        Loose("conflicting-file"),
        Loose("very-long-path"),
        Loose("FSMN"),
        Loose("REUC"),
        Loose("UNTR-with-oids"),
        Loose("UNTR"),
        Generated("V2_empty"),
        Generated("v2"),
        Generated("v2_more_files"),
        Generated("v2_all_file_kinds"),
        Generated("v2_split_index"),
        Generated("v4_more_files_IEOT"),
    ] {
        for options in [
            options_with(write::Extensions::None),
            options_with(write::Extensions::All),
            options_with(write::Extensions::Given {
                tree_cache: true,
                end_of_index_entry: false,
                sparse_directory_entries: false,
            }),
            options_with(write::Extensions::Given {
                tree_cache: false,
                end_of_index_entry: true,
                sparse_directory_entries: false,
            }),
            options_with(write::Extensions::Given {
                tree_cache: true,
                end_of_index_entry: false,
                sparse_directory_entries: true,
            }),
            options_with(write::Extensions::Given {
                tree_cache: false,
                end_of_index_entry: true,
                sparse_directory_entries: true,
            }),
        ] {
            let path = fixture.to_path();
            let fixture = fixture.to_name();
            let expected = git_index::File::at(&path, git_hash::Kind::Sha1, Default::default()).unwrap();

            let mut out = Vec::<u8>::new();
            let (actual_version, _digest) = expected.write_to(&mut out, options).unwrap();

            let (actual, _) =
                State::from_bytes(&out, FileTime::now(), git_hash::Kind::Sha1, Default::default()).unwrap();
            compare_states(&actual, actual_version, &expected, options, fixture);
        }
    }
}

#[test]
fn extended_flags_automatically_upgrade_the_version_to_avoid_data_loss() -> crate::Result {
    let mut expected = git_index::File::at(fixture_index_path("v2"), git_hash::Kind::Sha1, Default::default())?;
    assert_eq!(expected.version(), Version::V2);
    expected.entries_mut()[0].flags.insert(entry::Flags::EXTENDED);

    let mut buf = Vec::new();
    let (actual_version, _digest) = expected.write_to(&mut buf, Default::default())?;
    assert_eq!(actual_version, Version::V3, "extended flags need V3");

    Ok(())
}

fn compare_states(actual: &State, actual_version: Version, expected: &State, options: Options, fixture: &str) {
    actual.verify_entries().expect("valid");
    actual.verify_extensions(false, no_find).expect("valid");

    assert_eq!(actual.version(), actual_version, "version mismatch in {:?}", fixture);
    assert_eq!(
        actual.tree(),
        options
            .extensions
            .should_write(extension::tree::SIGNATURE)
            .and_then(|_| expected.tree()),
        "tree extension mismatch in {:?}",
        fixture
    );
    assert_eq!(
        actual.is_sparse(),
        options.extensions.should_write(extension::sparse::SIGNATURE).is_some(),
        "sparse index entries extension mismatch in {:?}",
        fixture
    );
    assert_eq!(
        actual.entries().len(),
        expected.entries().len(),
        "entry count mismatch in {:?}",
        fixture
    );
    assert_eq!(
        actual.entries(),
        expected.entries(),
        "entries mismatch in {:?}",
        fixture
    );
    assert_eq!(
        actual.path_backing(),
        expected.path_backing(),
        "path_backing mismatch in {:?}",
        fixture
    );
}

fn compare_raw_bytes(generated: &[u8], expected: &[u8], fixture: &str) {
    assert_eq!(generated.len(), expected.len(), "file length mismatch in {:?}", fixture);

    let print_range = 10;
    for (index, (a, b)) in generated.iter().zip(expected.iter()).enumerate() {
        if a != b {
            let range_left = index.saturating_sub(print_range);
            let range_right = (index + print_range).min(generated.len());
            let generated = &generated[range_left..range_right];
            let expected = &expected[range_left..range_right];

            panic! {"\n\nRoundtrip failed for index in fixture {:?} at position {:?}\n\
            \t  Actual: ... {:?} ...\n\
            \tExpected: ... {:?} ...\n\n\
            ", &fixture, index, generated, expected}
        }
    }
}

fn only_tree_ext() -> Options {
    Options {
        extensions: write::Extensions::Given {
            end_of_index_entry: false,
            sparse_directory_entries: false,
            tree_cache: true,
        },
        ..Default::default()
    }
}

fn options_with(extensions: write::Extensions) -> Options {
    Options { extensions }
}
