pub(crate) mod prepare_and_commit {
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
        let store = file::Store::at(dir.path(), git_ref::store::WriteReflog::Normal, git_hash::Kind::Sha1);
        Ok((dir, store))
    }

    pub(crate) fn committer() -> git_actor::Signature {
        git_actor::Signature {
            name: "committer".into(),
            email: "committer@example.com".into(),
            time: Time {
                seconds_since_unix_epoch: 1234,
                offset_in_seconds: 1800,
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
