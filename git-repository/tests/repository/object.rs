use git_repository as git;
use git_testtools::tempfile;

mod write_object {
    use crate::repository::object::empty_bare_repo;

    #[test]
    fn empty_tree() -> crate::Result {
        let (_tmp, repo) = empty_bare_repo()?;
        let oid = repo.write_object(&git_repository::objs::TreeRef::empty())?;
        assert_eq!(
            oid,
            git_repository::hash::ObjectId::empty_tree(repo.object_hash()),
            "it produces a well-known empty tree id"
        );
        Ok(())
    }
}

mod write_blob {
    use std::io::{Seek, SeekFrom};

    use git_hash::hex_to_id;

    use crate::repository::object::empty_bare_repo;

    #[test]
    fn from_slice() -> crate::Result {
        let (_tmp, repo) = empty_bare_repo()?;
        let oid = repo.write_blob(b"hello world")?;
        assert_eq!(oid, hex_to_id("95d09f2b10159347eece71399a7e2e907ea3df4f"));
        Ok(())
    }

    #[test]
    fn from_stream() -> crate::Result {
        let (_tmp, repo) = empty_bare_repo()?;
        let mut cursor = std::io::Cursor::new(b"hello world");
        let mut seek_cursor = cursor.clone();
        let oid = repo.write_blob_stream(&mut cursor)?;
        assert_eq!(oid, hex_to_id("95d09f2b10159347eece71399a7e2e907ea3df4f"));

        seek_cursor.seek(SeekFrom::Start(6))?;
        let oid = repo.write_blob_stream(&mut seek_cursor)?;
        assert_eq!(
            oid,
            hex_to_id("04fea06420ca60892f73becee3614f6d023a4b7f"),
            "it computes the object size correctly"
        );

        assert_eq!(
            oid.object()?.data,
            &b"world"[..],
            "the seek position is taken into account, so only part of the input data is written"
        );
        Ok(())
    }
}

mod find {
    use git_pack::Find;
    use git_repository as git;

    use crate::basic_repo;

    #[test]
    fn find_and_try_find_with_and_without_object_cache() -> crate::Result {
        let mut repo = basic_repo()?;

        assert_eq!(
            repo.worktrees()?.len(),
            0,
            "it's OK to query linked worktrees in a repo without worktrees"
        );
        for round in 1..=2 {
            match round {
                1 => repo.object_cache_size(None),
                2 => repo.object_cache_size(128 * 1024),
                _ => unreachable!("BUG"),
            }
            for commit_id in repo.head()?.peeled()?.id().expect("born").ancestors().all()? {
                let commit = commit_id?;
                assert_eq!(commit.object()?.kind, git_object::Kind::Commit);
                if round == 2 {
                    assert_eq!(
                        commit.object()?.kind,
                        git_object::Kind::Commit,
                        "repeated request triggers cache and doesn't fail"
                    );
                }
                assert_eq!(commit.try_object()?.expect("exists").kind, git_object::Kind::Commit,);
            }
        }
        Ok(())
    }

    #[test]
    fn empty_tree_can_always_be_found() -> crate::Result {
        let repo = basic_repo()?;
        let empty_tree = git::hash::ObjectId::empty_tree(repo.object_hash());
        assert_eq!(repo.find_object(empty_tree)?.into_tree().iter().count(), 0);
        assert_eq!(
            repo.try_find_object(empty_tree)?
                .expect("present")
                .into_tree()
                .iter()
                .count(),
            0
        );

        let mut buf = Vec::new();
        assert!(
            repo.objects.try_find(empty_tree, &mut buf)?.is_none(),
            "the lower level has no such special case so one can determine if this object exists or not"
        );
        Ok(())
    }
}

mod tag {
    #[test]
    fn simple() -> crate::Result {
        let (repo, _keep) = crate::repo_rw("make_basic_repo.sh")?;
        let current_head_id = repo.head_id()?;
        let message = "a multi\nline message";
        let tag_ref = repo.tag(
            "v1.0.0",
            current_head_id,
            git_object::Kind::Commit,
            Some(repo.committer().expect("present")),
            message,
            git_ref::transaction::PreviousValue::MustNotExist,
        )?;
        assert_eq!(tag_ref.name().as_bstr(), "refs/tags/v1.0.0");
        assert_ne!(tag_ref.id(), current_head_id, "it points to the tag object");
        let tag = tag_ref.id().object()?;
        let tag = tag.try_to_tag_ref()?;
        assert_eq!(tag.name, "v1.0.0");
        assert_eq!(current_head_id, tag.target(), "the tag points to the commit");
        assert_eq!(tag.target_kind, git_object::Kind::Commit);
        assert_eq!(
            tag.tagger.as_ref().expect("tagger").actor(),
            repo.committer().expect("present").actor()
        );
        assert_eq!(tag.message, message);
        Ok(())
    }
}

mod commit_as {
    use git_repository as git;
    use git_testtools::tempfile;

    #[test]
    fn specify_committer_and_author() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo = git::open_opts(git::init(&tmp)?.path(), crate::restricted())?;
        let empty_tree = repo.empty_tree();
        let committer = git::actor::Signature {
            name: "c".into(),
            email: "c@example.com".into(),
            time: git::actor::Time::new(1, 1800),
        };
        let author = git::actor::Signature {
            name: "a".into(),
            email: "a@example.com".into(),
            time: git::actor::Time::new(3, 3600),
        };

        let commit_id = repo.commit_as(
            &committer,
            &author,
            "HEAD",
            "initial",
            empty_tree.id,
            git::commit::NO_PARENT_IDS,
        )?;
        let commit = commit_id.object()?.into_commit();

        assert_eq!(commit.committer()?, committer.to_ref());
        assert_eq!(commit.author()?, author.to_ref());
        Ok(())
    }
}

mod commit {
    use git_hash::hex_to_id;
    use git_repository as git;
    use git_testtools::tempfile;

    use crate::{freeze_time, restricted_and_git};

    #[test]
    fn parent_in_initial_commit_causes_failure() -> crate::Result {
        let tmp = tempfile::tempdir()?;
        let repo = git::ThreadSafeRepository::init_opts(
            &tmp,
            git::create::Kind::WithWorktree,
            Default::default(),
            crate::restricted(),
        )?
        .to_thread_local();
        let empty_tree_id = repo.write_object(&git::objs::Tree::empty())?.detach();
        let err = repo
            .commit("HEAD", "initial", empty_tree_id, [empty_tree_id])
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            "Reference \"refs/heads/main\" was supposed to exist with value 4b825dc642cb6eb9a060e54bf8d69288fbee4904, but didn't.",
            "cannot provide parent id in initial commit"
        );
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn single_line_initial_commit_empty_tree_ref_nonexisting() -> crate::Result {
        let _env = freeze_time();
        let tmp = tempfile::tempdir()?;
        let repo = git::open_opts(git::init(&tmp)?.path(), restricted_and_git())?;
        let empty_tree_id = repo.write_object(&git::objs::Tree::empty())?;
        let commit_id = repo.commit("HEAD", "initial", empty_tree_id, git::commit::NO_PARENT_IDS)?;
        assert_eq!(
            commit_id,
            hex_to_id("3a774843723a713a8d361b4d4d98ad4092ef05bd"),
            "the commit id is stable"
        );

        let head = repo.head()?.try_into_referent().expect("born");
        assert_eq!(head.name().as_bstr(), "refs/heads/main", "'main' is the default name");
        assert_eq!(
            head.log_iter()
                .rev()?
                .expect("log present")
                .next()
                .expect("one line")?
                .message,
            "commit (initial): initial"
        );
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn multi_line_commit_message_uses_first_line_in_ref_log_ref_nonexisting() -> crate::Result {
        let _env = freeze_time();
        let (repo, _keep) = crate::repo_rw_opts("make_basic_repo.sh", restricted_and_git())?;
        let parent = repo.find_reference("HEAD")?.peel_to_id_in_place()?;
        let empty_tree_id = parent.object()?.to_commit_ref_iter().tree_id().expect("tree to be set");
        assert_eq!(
            parent
                .try_object()?
                .expect("present")
                .to_commit_ref_iter()
                .tree_id()
                .expect("tree to be set"),
            empty_tree_id,
            "try and non-try work the same"
        );
        let first_commit_id = repo.commit("HEAD", "hello there \r\n\nthe body", empty_tree_id, Some(parent))?;
        assert_eq!(
            first_commit_id,
            hex_to_id("e7c7273539cfc1a52802fa9d61aa578f6ccebcb4"),
            "the commit id is stable"
        );

        let head_log_entries: Vec<_> = repo
            .head()?
            .log_iter()
            .rev()?
            .expect("log present")
            .map(Result::unwrap)
            .map(|l| l.message)
            .collect();
        assert_eq!(
            head_log_entries,
            vec!["commit: hello there", "commit: c2", "commit (initial): c1"],
            "we get the actual HEAD log, not the log of some reference"
        );
        let current_commit = repo.head()?.into_fully_peeled_id().expect("born")?;
        assert_eq!(current_commit, first_commit_id, "the commit was set");

        let second_commit_id = repo.commit(
            "refs/heads/new-branch",
            "committing into a new branch creates it",
            empty_tree_id,
            Some(first_commit_id),
        )?;

        assert_eq!(
            second_commit_id,
            hex_to_id("e1412f169e0812eb260601bdab3854ca0f1a7b33"),
            "the second commit id is stable"
        );

        let mut branch = repo.find_reference("new-branch")?;
        let current_commit = branch.peel_to_id_in_place()?;
        assert_eq!(current_commit, second_commit_id, "the commit was set");

        let mut log = branch.log_iter();
        let mut log_iter = log.rev()?.expect("log present");
        assert_eq!(
            log_iter.next().expect("one line")?.message,
            "commit: committing into a new branch creates it"
        );
        assert!(
            log_iter.next().is_none(),
            "there is only one log line in the new branch"
        );
        Ok(())
    }
}

fn empty_bare_repo() -> crate::Result<(tempfile::TempDir, git::Repository)> {
    let tmp = tempfile::tempdir()?;
    let repo = git::ThreadSafeRepository::init_opts(
        tmp.path(),
        git::create::Kind::Bare,
        git::create::Options::default(),
        git::open::Options::isolated(),
    )?
    .into();
    Ok((tmp, repo))
}
