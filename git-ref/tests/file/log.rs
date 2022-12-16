mod line {
    mod write_to {
        use git_object::bstr::ByteVec;
        use git_ref::file::log;

        #[test]
        fn newlines_in_message_of_the_input_fails_and_we_trust_signature_writing_validation() -> crate::Result {
            let line = "0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1";
            let mut line = log::LineRef::from_bytes(line.as_bytes())?.to_owned();
            line.message.push_str("and here come\nthe newline");
            let err = line
                .write_to(&mut Vec::new())
                .expect_err("newlines in messages are caught");
            assert!(err.to_string().contains("newline"));
            Ok(())
        }

        #[test]
        fn round_trips() -> crate::Result {
            let lines = &["0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1\n", 
                         "0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	\n"];
            for line in lines {
                let line = log::LineRef::from_bytes(line.as_bytes())?;
                let mut buf = Vec::new();
                line.to_owned().write_to(&mut buf)?;
                let same_line = log::LineRef::from_bytes(&buf)?;
                assert_eq!(line, same_line);
            }
            Ok(())
        }
    }
}

mod iter {
    use std::path::PathBuf;

    fn reflog_dir() -> crate::Result<PathBuf> {
        Ok(git_testtools::scripted_fixture_read_only("make_repo_for_reflog.sh")?
            .join(".git")
            .join("logs"))
    }
    fn reflog(name: &str) -> crate::Result<Vec<u8>> {
        Ok(std::fs::read(reflog_dir()?.join(name))?)
    }

    mod backward {
        mod with_zero_sized_buffer {

            #[test]
            fn any_line() {
                let mut buf = [0u8; 0];
                assert!(
                    git_ref::file::log::iter::reverse(std::io::Cursor::new(b"won't matter".as_ref()), &mut buf)
                        .is_err(),
                    "zero sized buffers aren't allowed"
                );
            }
        }

        mod with_buffer_too_small_for_single_line {
            use std::error::Error;

            #[test]
            fn single_line() -> crate::Result {
                let mut buf = [0u8; 128];
                let two_lines: Vec<u8> = b"0000000000000000000000000000000000000000 134385f6d781b7e97062102c6a483440bfda2a03 committer <committer@example.com> 946771200 +0000	commit (initial): c1".to_vec();
                let two_lines_trailing_nl = {
                    let mut l = two_lines.clone();
                    l.push(b'\n');
                    l
                };
                for line in &[two_lines, two_lines_trailing_nl] {
                    let read = std::io::Cursor::new(line);
                    let mut iter = git_ref::file::log::iter::reverse(read, &mut buf)?;
                    assert_eq!(
                        iter.next()
                            .expect("an error")
                            .expect_err("buffer too small")
                            .source()
                            .expect("source")
                            .to_string(),
                        "buffer too small for line size"
                    );
                    assert!(iter.next().is_none(), "iterator depleted");
                }
                Ok(())
            }
        }

        mod with_buffer_big_enough_for_largest_line {
            use git_ref::log::Line;
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
                    } = iter.next().expect("a single line")?;
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
                for buf_size in &[1024usize, 256] {
                    let mut buf = vec![0; *buf_size];
                    for line in &lines {
                        let read = std::io::Cursor::new(line);
                        let mut iter = git_ref::file::log::iter::reverse(read, &mut buf)?;
                        let Line {
                            previous_oid,
                            new_oid,
                            signature: _,
                            message,
                        } = iter.next().expect("a single line")?;
                        assert_eq!(previous_oid, hex_to_id("0000000000000000000000000000000000000000"));
                        assert_eq!(new_oid, hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03"));
                        assert_eq!(message, "commit (initial): c1");
                        let Line {
                            previous_oid,
                            new_oid,
                            signature: _,
                            message,
                        } = iter.next().expect("a single line")?;
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
        use git_object::bstr::B;

        use crate::file::log::iter::reflog;

        #[test]
        fn all_success() -> crate::Result {
            let log = reflog("HEAD")?;
            let iter = git_ref::file::log::iter::forward(&log);
            assert_eq!(iter.count(), 5, "the log as a known amount of entries");

            let mut iter = git_ref::file::log::iter::forward(&log);
            let line = iter.next().unwrap()?;
            assert_eq!(line.previous_oid(), git_hash::Kind::Sha1.null());
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
