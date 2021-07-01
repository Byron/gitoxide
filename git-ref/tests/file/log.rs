mod iter {
    use std::path::PathBuf;

    fn reflog_dir() -> crate::Result<PathBuf> {
        Ok(
            git_testtools::scripted_fixture_repo_read_only("make_repo_for_reflog.sh")?
                .join(".git")
                .join("logs"),
        )
    }
    fn reflog(name: &str) -> crate::Result<Vec<u8>> {
        Ok(std::fs::read(reflog_dir()?.join(name))?)
    }

    mod backward {
        use bstr::B;
        use git_ref::file::log::mutable::Line;
        use git_testtools::hex_to_id;

        #[test]
        fn a_single_line_and_suitably_big_buffer() -> crate::Result {
            let line: Vec<u8> = b"0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1".to_vec();
            let line_with_nl = {
                let mut l = line.clone();
                l.push(b'\n');
                l
            };
            for line in &[line, line_with_nl] {
                let read = std::io::Cursor::new(line);
                let mut iter = git_ref::file::log::iter::reverse::<_, 1024>(read)?;
                let Line {
                    previous_oid,
                    new_oid,
                    signature: _,
                    message,
                } = iter.next().expect("a single line")??;
                assert_eq!(message, B("commit (initial): c1"));
                assert_eq!(previous_oid, hex_to_id("0000000000000000000000000000000000000000"));
                assert_eq!(new_oid, hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03"));
            }
            Ok(())
        }
    }
    mod forward {
        use crate::file::log::iter::reflog;
        use bstr::B;
        use git_hash::ObjectId;

        #[test]
        fn all_success() -> crate::Result {
            let log = reflog("HEAD")?;
            let iter = git_ref::file::log::iter::forward(&log);
            assert_eq!(iter.count(), 5, "the log as a known amount of entries");

            let mut iter = git_ref::file::log::iter::forward(&log);
            let line = iter.next().unwrap()?;
            assert_eq!(line.previous_oid(), ObjectId::null_sha1());
            assert_eq!(line.new_oid, B("134385f6d781b7e97062102c6a483440bfda2a03"));
            assert_eq!(line.message, B("commit (initial): c1"));
            assert!(iter.all(|l| l.is_ok()), "all lines parse fine");
            Ok(())
        }

        #[test]
        fn a_single_failure_does_not_abort_iteration() {
            let log_first_broken = "0000000000000000000000000000000000000000 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit
0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1\n";

            let mut iter = git_ref::file::log::iter::forward(log_first_broken.as_bytes());
            let err = iter.next().expect("error is not none").expect_err("the line is broken");
            assert_eq!(err.to_string(), "In line 1: \"0000000000000000000000000000000000000000 134385fbroken7062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000\\tcommit\" did not match '<old-hexsha> <new-hexsha> <name> <<email>> <timestamp> <tz>\\t<message>'");
            assert!(iter.next().expect("a second line").is_ok(), "line parses ok");
            assert!(iter.next().is_none(), "iterator exhausted");
        }
    }
}
