mod prepare_and_commit {
    use bstr::BString;
    use git_actor::{Sign, Time};
    use git_hash::ObjectId;
    use git_ref::{file, file::log};

    fn reflog_lines(store: &file::Store, name: &str, buf: &mut Vec<u8>) -> crate::Result<Vec<log::mutable::Line>> {
        store
            .reflog_iter(name, buf)?
            .expect("existing reflog")
            .map(|l| l.map(log::mutable::Line::from))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn empty_store(log_mode: git_ref::file::WriteReflog) -> crate::Result<(tempfile::TempDir, file::Store)> {
        let dir = tempfile::TempDir::new().unwrap();
        let mut store: file::Store = dir.path().to_owned().into();
        store.write_reflog = log_mode;
        Ok((dir, store))
    }

    fn committer() -> git_actor::Signature {
        git_actor::Signature {
            name: "committer".into(),
            email: "committer@example.com".into(),
            time: Time {
                time: 1234,
                offset: 1800,
                sign: Sign::Plus,
            },
        }
    }

    fn log_line(previous: ObjectId, new: ObjectId, message: impl Into<BString>) -> log::mutable::Line {
        log::mutable::Line {
            previous_oid: previous,
            new_oid: new,
            signature: committer(),
            message: message.into(),
        }
    }

    mod create_or_update;

    mod delete;
}
