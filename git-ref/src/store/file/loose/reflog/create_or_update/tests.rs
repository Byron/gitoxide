use super::*;
use crate::{file::WriteReflog, FullNameRef};
use git_actor::{Sign, Signature, Time};
use git_lock::acquire::Fail;
use git_object::bstr::ByteSlice;
use git_testtools::hex_to_id;
use std::{convert::TryInto, path::Path};
use tempfile::TempDir;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn empty_store(writemode: WriteReflog) -> Result<(TempDir, file::Store)> {
    let dir = TempDir::new()?;
    let store = file::Store::at(dir.path(), writemode);
    Ok((dir, store))
}

fn reflock(store: &file::Store, full_name: &str) -> Result<git_lock::Marker> {
    let full_name: FullNameRef<'_> = full_name.try_into()?;
    git_lock::Marker::acquire_to_hold_resource(
        store.reference_path(&full_name.to_path()),
        Fail::Immediately,
        Some(store.base.clone()),
    )
    .map_err(Into::into)
}

fn reflog_lines(store: &file::Store, name: &str, buf: &mut Vec<u8>) -> Result<Vec<crate::log::Line>> {
    store
        .reflog_iter(name, buf)?
        .expect("existing reflog")
        .map(|l| l.map(crate::log::Line::from))
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(Into::into)
}

const WRITE_MODES: &[WriteReflog] = &[WriteReflog::Normal, WriteReflog::Disable];

#[test]
fn reflock_resource_to_log_path() -> Result {
    let (_keep, store) = empty_store(WriteReflog::Normal)?;
    for name in &["HEAD", "refs/heads/main"] {
        assert_eq!(
            store.reflock_resource_to_log_path(&reflock(&store, name).unwrap()),
            store.reflog_path_inner(Path::new(name))
        );
    }
    Ok(())
}

#[test]
fn should_autocreate_is_unaffected_by_writemode() -> Result {
    let (_keep, store) = empty_store(WriteReflog::Disable)?;
    for should_create_name in &["HEAD", "refs/heads/main", "refs/remotes/any", "refs/notes/any"] {
        assert!(store.should_autocreate_reflog(Path::new(should_create_name)));
    }
    for should_not_create_name in &["FETCH_HEAD", "SOMETHING", "refs/special/this", "refs/tags/0.1.0"] {
        assert!(!store.should_autocreate_reflog(Path::new(should_not_create_name)));
    }
    Ok(())
}

#[test]
fn missing_reflog_creates_it_even_if_similarly_named_empty_dir_exists_and_append_log_lines() -> Result {
    for mode in WRITE_MODES {
        let (_keep, store) = empty_store(*mode)?;
        let full_name = "refs/heads/main";
        let lock = reflock(&store, full_name)?;
        let new = hex_to_id("28ce6a8b26aa170e1de65536fe8abe1832bd3242");
        let committer = Signature {
            name: "committer".into(),
            email: "commiter@example.com".into(),
            time: Time {
                time: 1234,
                offset: 1800,
                sign: Sign::Plus,
            },
        };
        store.reflog_create_or_append(&lock, None, &new, &committer, b"the message".as_bstr(), false)?;

        let mut buf = Vec::new();
        match mode {
            WriteReflog::Normal => {
                assert_eq!(
                    reflog_lines(&store, full_name, &mut buf)?,
                    vec![crate::log::Line {
                        previous_oid: ObjectId::null_sha1(),
                        new_oid: new,
                        signature: committer.clone(),
                        message: "the message".into()
                    }]
                );
                let previous = hex_to_id("0000000000000000000000111111111111111111");
                store.reflog_create_or_append(
                    &lock,
                    Some(previous),
                    &new,
                    &committer,
                    b"next message".as_bstr(),
                    false,
                )?;

                let lines = reflog_lines(&store, full_name, &mut buf)?;
                assert_eq!(lines.len(), 2, "now there is another line");
                assert_eq!(
                    lines.last().expect("non-empty"),
                    &crate::log::Line {
                        previous_oid: previous,
                        new_oid: new,
                        signature: committer.clone(),
                        message: "next message".into()
                    }
                );
            }
            WriteReflog::Disable => {
                assert!(
                    store.reflog_iter(full_name, &mut buf)?.is_none(),
                    "there is no logs in disabled mode"
                );
            }
        };

        // create onto existing directory
        let full_name = "refs/heads/other";
        let lock = reflock(&store, full_name)?;
        let reflog_path = store.reflog_path_inner(Path::new(full_name));
        let directory_in_place_of_reflog = reflog_path.join("empty-a").join("empty-b");
        std::fs::create_dir_all(&directory_in_place_of_reflog)?;

        store.reflog_create_or_append(
            &lock,
            None,
            &new,
            &committer,
            b"more complicated reflog creation".as_bstr(),
            false,
        )?;

        match mode {
            WriteReflog::Normal => {
                assert_eq!(
                    reflog_lines(&store, full_name, &mut buf)?.len(),
                    1,
                    "reflog was written despite directory"
                );
                assert!(
                    reflog_path.is_file(),
                    "the empty directory was replaced with the reflog file"
                );
            }
            WriteReflog::Disable => {
                assert!(
                    store.reflog_iter(full_name, &mut buf)?.is_none(),
                    "reflog still doesn't exist"
                );
                assert!(
                    store.reflog_iter_rev(full_name, &mut buf)?.is_none(),
                    "reflog still doesn't exist"
                );
                assert!(reflog_path.is_dir(), "reflog directory wasn't touched");
            }
        }
    }
    Ok(())
}
