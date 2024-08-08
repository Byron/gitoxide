use std::{ops::Range, path::PathBuf, str::FromStr};

use gix_diff::blob::intern::Token;
use gix_hash::ObjectId;
use gix_odb::pack::FindExt;
use gix_ref::{file::ReferenceExt, store::WriteReflog};

struct Blame {
    _resource_cache: gix_diff::blob::Platform,
}

impl Blame {
    fn new(worktree_root: impl Into<PathBuf>) -> Self {
        let worktree_root: PathBuf = worktree_root.into();
        let git_dir = worktree_root.join(".git");
        let index =
            gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default()).unwrap();

        let capabilities = gix_fs::Capabilities::probe(&git_dir);
        let stack = gix_worktree::Stack::from_state_and_ignore_case(
            &worktree_root,
            false,
            gix_worktree::stack::State::AttributesAndIgnoreStack {
                attributes: Default::default(),
                ignore: Default::default(),
            },
            &index,
            index.path_backing(),
        );

        let resource_cache = gix_diff::blob::Platform::new(
            Default::default(),
            gix_diff::blob::Pipeline::new(
                gix_diff::blob::pipeline::WorktreeRoots {
                    old_root: None,
                    new_root: None,
                },
                gix_filter::Pipeline::new(Default::default(), Default::default()),
                vec![],
                gix_diff::blob::pipeline::Options {
                    large_file_threshold_bytes: 0,
                    fs: capabilities,
                },
            ),
            gix_diff::blob::pipeline::Mode::ToGit,
            stack,
        );

        Blame {
            _resource_cache: resource_cache,
        }
    }
}

#[test]
fn blame_works() {
    let _blame = Blame::new(fixture_path());
}

#[test]
fn it_works() {
    // TODO
    // At a high level, what we want to do is the following:
    //
    // - get the commit that belongs to a commit id
    // - walk through parents
    //   - for each parent, do a diff and mark lines that don’t have a suspect (this is the term
    //     used in `libgit2`) yet, but that have been changed in this commit
    //
    // The algorithm in `libgit2` works by going through parents and keeping a linked list of blame
    // suspects. It can be visualized as follows:
    //
    // <---------------------------------------->
    // <---------------><----------------------->
    // <---><----------><----------------------->
    // <---><----------><-------><-----><------->
    // <---><---><-----><-------><-----><------->
    // <---><---><-----><-------><-----><-><-><->

    let worktree = fixture_path();

    let store = gix_ref::file::Store::at(
        worktree.join(".git"),
        gix_ref::store::init::Options {
            write_reflog: WriteReflog::Disable,
            ..Default::default()
        },
    );
    let odb = odb_at("");

    let mut reference = gix_ref::file::Store::find(&store, "HEAD").unwrap();

    let mut buffer = Vec::new();

    let head_id = reference.peel_to_id_in_place(&store, &odb).unwrap();
    let (head, _) = odb.find_commit(&head_id, &mut buffer).unwrap();

    let mut buffer = Vec::new();
    let head_tree_iter = odb
        .find(&head.tree(), &mut buffer)
        .unwrap()
        .0
        .try_into_tree_iter()
        .unwrap();

    let mut traverse = gix_traverse::commit::Simple::new(Some(head_id), &odb);

    traverse.next();

    let iter = traverse.commit_iter();
    let parent_ids = iter.parent_ids().collect::<Vec<_>>();

    let last_parent_id = parent_ids.last().unwrap();

    let mut buffer = Vec::new();

    let (last_parent, _) = odb.find_commit(&last_parent_id, &mut buffer).unwrap();

    let mut buffer = Vec::new();
    let last_parent_tree_iter = odb
        .find(&last_parent.tree(), &mut buffer)
        .unwrap()
        .0
        .try_into_tree_iter()
        .unwrap();

    let mut recorder = gix_diff::tree::Recorder::default();
    let _result = gix_diff::tree::Changes::from(last_parent_tree_iter)
        .needed_to_obtain(head_tree_iter, gix_diff::tree::State::default(), &odb, &mut recorder)
        .unwrap();

    assert!(matches!(
        recorder.records[..],
        [gix_diff::tree::recorder::Change::Modification { .. }]
    ));

    let [ref modification]: [gix_diff::tree::recorder::Change] = recorder.records[..] else {
        todo!()
    };
    let gix_diff::tree::recorder::Change::Modification { previous_oid, oid, .. } = modification else {
        todo!()
    };

    // The following lines are trying to get a line-diff between two commits.
    let git_dir = fixture_path().join(".git");
    let index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default()).unwrap();
    let stack = gix_worktree::Stack::from_state_and_ignore_case(
        worktree.clone(),
        false,
        gix_worktree::stack::State::AttributesAndIgnoreStack {
            attributes: Default::default(),
            ignore: Default::default(),
        },
        &index,
        index.path_backing(),
    );
    let capabilities = gix_fs::Capabilities::probe(&git_dir);
    let mut resource_cache = gix_diff::blob::Platform::new(
        Default::default(),
        gix_diff::blob::Pipeline::new(
            gix_diff::blob::pipeline::WorktreeRoots {
                old_root: None,
                new_root: None,
            },
            gix_filter::Pipeline::new(Default::default(), Default::default()),
            vec![],
            gix_diff::blob::pipeline::Options {
                large_file_threshold_bytes: 0,
                fs: capabilities,
            },
        ),
        gix_diff::blob::pipeline::Mode::ToGit,
        stack,
    );

    resource_cache
        .set_resource(
            *previous_oid,
            gix_object::tree::EntryKind::Blob,
            "file.txt".into(),
            gix_diff::blob::ResourceKind::OldOrSource,
            &odb,
        )
        .unwrap();
    resource_cache
        .set_resource(
            *oid,
            gix_object::tree::EntryKind::Blob,
            "file.txt".into(),
            gix_diff::blob::ResourceKind::NewOrDestination,
            &odb,
        )
        .unwrap();

    let outcome = resource_cache.prepare_diff().unwrap();
    let input = outcome.interned_input();

    assert_eq!(input.before, [Token(0), Token(1), Token(2),]);
    assert_eq!(input.after, [Token(0), Token(1), Token(2), Token(3)]);

    // Assumption: this works because “imara-diff will compute a line diff by default”, so each
    // token represents a line.
    let number_of_lines: u32 = input.after.len().try_into().unwrap();

    assert_eq!(number_of_lines, 4);

    let lines_to_blame: Vec<Range<u32>> = vec![0..number_of_lines];

    assert_eq!(lines_to_blame, vec![0..4]);

    #[derive(Debug, PartialEq)]
    struct BlameEntry {
        range: Range<u32>,
        oid: ObjectId,
    }

    let mut lines_blamed: Vec<BlameEntry> = vec![];

    let mut lines = Vec::new();

    use gix_ref::bstr::ByteSlice;

    // The following lines were inspired by `gix::object::blob::diff::Platform::lines`.
    gix_diff::blob::diff(
        gix_diff::blob::Algorithm::Histogram,
        &input,
        |before: Range<u32>, after: Range<u32>| {
            lines.clear();
            lines.extend(
                input.before[before.start as usize..before.end as usize]
                    .iter()
                    .map(|&line| input.interner[line].as_bstr()),
            );
            let end_of_before = lines.len();
            lines.extend(
                input.after[after.start as usize..after.end as usize]
                    .iter()
                    .map(|&line| input.interner[line].as_bstr()),
            );
            let hunk_before = &lines[..end_of_before];
            let hunk_after = &lines[end_of_before..];
            if hunk_after.is_empty() {
                // Intentionally empty.
            } else if hunk_before.is_empty() {
                assert_eq!(hunk_after, ["line 4\n"]);
            } else {
            }

            let mut new_lines_to_blame: Vec<Range<u32>> = Vec::new();

            for range in &lines_to_blame {
                if range.contains(&after.start) {
                    if range.contains(&after.end) {
                        // <---------->
                        //     <--->
                        // <-->     <->
                        new_lines_to_blame.push(range.start..after.start);
                        new_lines_to_blame.push((after.end + 1)..range.end);

                        lines_blamed.push(BlameEntry {
                            range: after.clone(),
                            oid: oid.clone(),
                        });
                    } else {
                        // <-------->
                        //     <------->
                        // <-->
                        new_lines_to_blame.push(range.start..after.start);

                        lines_blamed.push(BlameEntry {
                            range: after.start..range.end,
                            oid: oid.clone(),
                        });
                    }
                } else {
                    //    <------->
                    // <------>
                    //         <-->
                    new_lines_to_blame.push((after.end + 1)..range.end);

                    lines_blamed.push(BlameEntry {
                        range: range.start..after.end,
                        oid: oid.clone(),
                    });
                }
            }

            assert_eq!(new_lines_to_blame, vec![0..3]);
            assert_eq!(
                lines_blamed,
                vec![BlameEntry {
                    range: 3..4,
                    oid: ObjectId::from_str("9c2a7090627d0fffa9ed001bf7be98f86c2c8068").unwrap()
                }]
            );
            assert_eq!(lines_blamed, vec![BlameEntry { range: 3..4, oid: *oid }]);
        },
    );

    assert_eq!(lines, ["line 4\n"]);
}

fn odb_at(name: &str) -> gix_odb::Handle {
    gix_odb::at(fixture_path().join(name).join(".git/objects")).unwrap()
}

fn fixture_path() -> PathBuf {
    gix_testtools::scripted_fixture_read_only("make_blame_repo.sh").unwrap()
}
