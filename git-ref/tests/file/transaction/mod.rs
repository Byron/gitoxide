mod prepare_and_commit {
    use git_actor::{Sign, Time};
    use git_hash::ObjectId;
    use git_object::bstr::BString;
    use git_ref::file;

    fn reflog_lines(store: &file::Store, name: &str) -> crate::Result<Vec<git_ref::log::Line>> {
        let mut buf = Vec::new();
        let res = store
            .reflog_iter(name, &mut buf)?
            .expect("existing reflog")
            .map(|l| l.map(git_ref::log::Line::from))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(res)
    }

    fn empty_store() -> crate::Result<(tempfile::TempDir, file::Store)> {
        let dir = tempfile::TempDir::new().unwrap();
        let store: file::Store = dir.path().to_owned().into();
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

    fn log_line(previous: ObjectId, new: ObjectId, message: impl Into<BString>) -> git_ref::log::Line {
        git_ref::log::Line {
            previous_oid: previous,
            new_oid: new,
            signature: committer(),
            message: message.into(),
        }
    }

    mod create_or_update;

    mod delete;
}
