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
        mod with_buffer_big_enough_for_largest_line {
            use git_ref::file::log::mutable::Line;
            use git_testtools::hex_to_id;

            #[test]
            fn single_line() -> crate::Result {
                let mut buf = [0u8; 1024];
                let two_lines: Vec<u8> = b"0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1".to_vec();
                let two_lines_trailing_nl = {
                    let mut l = two_lines.clone();
                    l.push(b'\n');
                    l
                };
                for line in &[two_lines, two_lines_trailing_nl] {
                    let read = std::io::Cursor::new(line);
                    let mut iter = git_ref::file::log::iter::reverse(read, &mut buf)?;
                    let Line {
                        previous_oid,
                        new_oid,
                        signature: _,
                        message,
                    } = iter.next().expect("a single line")??;
                    assert_eq!(previous_oid, hex_to_id("0000000000000000000000000000000000000000"));
                    assert_eq!(new_oid, hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03"));
                    assert_eq!(message, "commit (initial): c1");
                    assert!(iter.next().is_none(), "iterator depleted");
                }
                Ok(())
            }

            #[test]
            fn two_lines() -> crate::Result {
                let two_lines: Vec<u8> = b"1000000000000000000000000000000000000000 234385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c2\n0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1".to_vec();
                let two_lines_trailing_nl = {
                    let mut l = two_lines.clone();
                    l.push(b'\n');
                    l
                };
                let lines = [two_lines, two_lines_trailing_nl];
                for buf_size in &[1024usize, 512] {
                    let mut buf = vec![0; *buf_size];
                    for line in &lines {
                        let read = std::io::Cursor::new(line);
                        let mut iter = git_ref::file::log::iter::reverse(read, &mut buf)?;
                        let Line {
                            previous_oid,
                            new_oid,
                            signature: _,
                            message,
                        } = iter.next().expect("a single line")??;
                        assert_eq!(previous_oid, hex_to_id("0000000000000000000000000000000000000000"));
                        assert_eq!(new_oid, hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03"));
                        assert_eq!(message, "commit (initial): c1");
                        let Line {
                            previous_oid,
                            new_oid,
                            signature: _,
                            message,
                        } = iter.next().expect("a single line")??;
                        assert_eq!(message, "commit (initial): c2");
                        assert_eq!(previous_oid, hex_to_id("1000000000000000000000000000000000000000"));
                        assert_eq!(new_oid, hex_to_id("234385f6d781b7e97062102c6a483440bfda2a03"));
                        assert!(iter.next().is_none(), "iterator depleted");
                    }
                }
                Ok(())
            }
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
