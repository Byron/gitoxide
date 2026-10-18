//! Observed replacement behavior, including limitations, rather than a branch-rename contract.
//! Only store construction selects a backend; setup and observations use logical refs and reflogs.

use std::collections::BTreeMap;

use gix_lock::acquire::Fail;
use gix_ref::{
    Target,
    file::Store,
    log::Line,
    transaction::{PreviousValue, RefEdit},
};
use gix_testtools::{Creation, TestResult, scripted_fixture_read_only_with_args, scripted_fixture_writable_with_args};

#[test]
fn replacement_baseline() -> TestResult {
    let script = "make_ref_transaction_baseline.sh";
    // Success columns: gix transaction, Git update-ref transaction, Git branch -m.
    // Git's branch rename handles both directions; ordinary transactions reject both.
    for (source_branch, destination_branch, blocker, successes) in [
        ("topic", "renamed", None, [true, true, true]),
        ("topic/child", "topic", None, [false, false, true]),
        ("topic/nested/child", "topic", None, [false, false, true]),
        ("topic", "topic/child", None, [false, false, true]),
        ("topic/child", "topic", Some("topic/sibling"), [false; 3]),
        ("topic", "renamed", Some("renamed"), [false; 3]),
    ] {
        let setup_args = [
            source_branch,
            destination_branch,
            blocker.unwrap_or("-"),
            "setup",
            "delete-first",
        ];
        let setup = scripted_fixture_read_only_with_args(script, setup_args)?;
        let initial_store = Store::at(setup.join(".git"), crate::fixture_hash_kind());
        let source = format!("refs/heads/{source_branch}");
        let destination = format!("refs/heads/{destination_branch}");
        let commit_id = initial_store.find(source.as_str())?.target.id().to_owned();
        let initial = observe(&initial_store, &source, &destination)?;
        let committer = gix_actor::SignatureRef::from_bytes(b"committer <committer@example.com> 946771200 +0000")?;

        for (operation, succeeds) in ["gix", "update-ref", "branch"].into_iter().zip(successes) {
            for update_first in [false, true] {
                if operation == "branch" && update_first {
                    continue;
                }
                let context = format!(
                    "{source_branch} -> {destination_branch}, {blocker:?}, {operation}, update first: {update_first}"
                );
                let (directory, _keep) = if operation == "gix" {
                    let fixture = scripted_fixture_writable_with_args(script, setup_args, Creation::CopyFromReadOnly)?;
                    (fixture.path().to_owned(), Some(fixture))
                } else {
                    let mut args = setup_args;
                    args[3] = operation;
                    args[4] = if update_first { "update-first" } else { "delete-first" };
                    (scripted_fixture_read_only_with_args(script, args)?, None)
                };
                let store = Store::at(directory.join(".git"), crate::fixture_hash_kind());
                let mut expected = initial.clone();

                let (success, diagnostic) = if operation == "gix" {
                    let mut edits = [
                        RefEdit::delete(
                            source.as_str().try_into()?,
                            PreviousValue::MustExistAndMatch(commit_id.into()),
                        ),
                        RefEdit::update(
                            destination.as_str().try_into()?,
                            commit_id,
                            PreviousValue::MustNotExist,
                            "rename",
                        ),
                    ];
                    if update_first {
                        edits.reverse();
                    }
                    let result = store
                        .transaction()
                        .prepare(edits, Fail::Immediately, Fail::Immediately)
                        .and_then(|transaction| transaction.commit(Some(committer)));
                    (result.is_ok(), format!("{result:?}"))
                } else {
                    let status: i32 = std::fs::read_to_string(directory.join("status"))?.trim().parse()?;
                    (
                        status == 0,
                        String::from_utf8_lossy(&std::fs::read(directory.join("stderr"))?).into_owned(),
                    )
                };
                assert_eq!(success, succeeds, "recorded outcome: {context}\n{diagnostic}");

                if succeeds {
                    expected.refs.remove(&source).expect("source exists before replacement");
                    expected.refs.insert(destination.clone(), Target::Object(commit_id));
                    let source_log = expected.logs.remove(&source).expect("source log observed");
                    assert_eq!(source_log.len(), 2, "the source has history to preserve: {context}");
                    let mut entry = Line {
                        previous_oid: crate::fixture_hash_kind().null(),
                        new_oid: commit_id,
                        signature: committer.to_owned()?,
                        message: "rename".into(),
                    };
                    let destination_log = if operation == "branch" {
                        entry.previous_oid = commit_id;
                        entry.message = format!("Branch: renamed {source} to {destination}").into();
                        expected
                            .refs
                            .insert("HEAD".into(), Target::Symbolic(destination.as_str().try_into()?));
                        let head_log = expected.logs.get_mut("HEAD").expect("HEAD log observed");
                        let mut deletion = entry.clone();
                        deletion.new_oid = crate::fixture_hash_kind().null();
                        head_log.push(deletion);
                        let mut creation = entry.clone();
                        creation.previous_oid = crate::fixture_hash_kind().null();
                        head_log.push(creation);
                        let mut history = source_log;
                        history.push(entry);
                        history
                    } else {
                        // Delete/create loses the source history and leaves HEAD pointing at the deleted branch.
                        if operation == "update-ref" {
                            // Git records the checked-out branch's deletion in HEAD's log; gix does not.
                            let mut deletion = entry.clone();
                            deletion.previous_oid = commit_id;
                            deletion.new_oid = crate::fixture_hash_kind().null();
                            expected.logs.get_mut("HEAD").expect("HEAD log observed").push(deletion);
                        }
                        vec![entry]
                    };
                    expected.logs.insert(destination.clone(), destination_log);
                }
                assert_eq!(
                    observe(&store, &source, &destination)?,
                    expected,
                    "record refs, HEAD and full reflogs, including unchanged state on rejection: {context}"
                );
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    refs: BTreeMap<String, Target>,
    logs: BTreeMap<String, Vec<Line>>,
}

fn observe(store: &Store, source: &str, destination: &str) -> TestResult<State> {
    let mut refs = store
        .iter()?
        .all()?
        .map(|reference| reference.map(|reference| (reference.name.to_string(), reference.target)))
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    refs.insert("HEAD".into(), store.find("HEAD")?.target);
    let mut logs = BTreeMap::new();
    for name in refs.keys().map(String::as_str).chain([source, destination]) {
        let mut buffer = Vec::new();
        if store.reflog_exists(name)? {
            let lines = store
                .reflog_iter(name, &mut buffer)?
                .expect("the observed reflog exists")
                .map(|line| line.map(Line::from))
                .collect::<Result<Vec<_>, _>>()?;
            logs.insert(name.to_owned(), lines);
        }
    }
    Ok(State { refs, logs })
}
