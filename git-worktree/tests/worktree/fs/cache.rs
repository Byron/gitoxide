mod create_directory {
    use std::path::Path;

    use git_worktree::fs;
    use tempfile::{tempdir, TempDir};

    #[test]
    fn root_is_assumed_to_exist_and_files_in_root_do_not_create_directory() {
        let dir = tempdir().unwrap();
        let mut cache = fs::Cache::new(
            dir.path().join("non-existing-root"),
            fs::cache::State::for_checkout(false, Default::default()),
            Vec::new(),
        );
        assert_eq!(cache.num_mkdir_calls(), 0);

        let path = cache.at_entry("hello", Some(false)).unwrap().path();
        assert!(!path.parent().unwrap().exists(), "prefix itself is never created");
        assert_eq!(cache.num_mkdir_calls(), 0);
    }

    #[test]
    fn directory_paths_are_created_in_full() {
        let (mut cache, _tmp) = new_cache();

        for (name, is_dir) in &[
            ("dir", Some(true)),
            ("submodule", Some(true)),
            ("file", Some(false)),
            ("exe", Some(false)),
            ("link", None),
        ] {
            let path = cache.at_entry(Path::new("dir").join(name), *is_dir).unwrap().path();
            assert!(path.parent().unwrap().is_dir(), "dir exists");
        }

        assert_eq!(cache.num_mkdir_calls(), 3);
    }

    #[test]
    fn existing_directories_are_fine() {
        let (mut cache, tmp) = new_cache();
        std::fs::create_dir(tmp.path().join("dir")).unwrap();

        let path = cache.at_entry("dir/file", Some(false)).unwrap().path();
        assert!(path.parent().unwrap().is_dir(), "directory is still present");
        assert!(!path.exists(), "it won't create the file");
        assert_eq!(cache.num_mkdir_calls(), 1);
    }

    #[test]
    fn symlinks_or_files_in_path_are_forbidden_or_unlinked_when_forced() {
        let (mut cache, tmp) = new_cache();
        let forbidden = tmp.path().join("forbidden");
        std::fs::create_dir(&forbidden).unwrap();
        symlink::symlink_dir(&forbidden, tmp.path().join("link-to-dir")).unwrap();
        std::fs::write(tmp.path().join("file-in-dir"), &[]).unwrap();

        for dirname in &["file-in-dir", "link-to-dir"] {
            cache.unlink_on_collision(false);
            let relative_path = format!("{}/file", dirname);
            assert_eq!(
                cache.at_entry(&relative_path, Some(false)).unwrap_err().kind(),
                std::io::ErrorKind::AlreadyExists
            );
        }
        assert_eq!(
            cache.num_mkdir_calls(),
            2,
            "it tries to create each directory once, but it's a file"
        );
        cache.reset_mkdir_calls();
        for dirname in &["link-to-dir", "file-in-dir"] {
            cache.unlink_on_collision(true);
            let relative_path = format!("{}/file", dirname);
            let path = cache.at_entry(&relative_path, Some(false)).unwrap().path();
            assert!(path.parent().unwrap().is_dir(), "directory was forcefully created");
            assert!(!path.exists());
        }
        assert_eq!(
            cache.num_mkdir_calls(),
            4,
            "like before, but it unlinks what's there and tries again"
        );
    }

    fn new_cache() -> (fs::Cache, TempDir) {
        let dir = tempdir().unwrap();
        let cache = fs::Cache::new(
            dir.path(),
            fs::cache::State::for_checkout(false, Default::default()),
            Vec::new(),
        );
        (cache, dir)
    }
}

#[allow(unused)]
mod ignore_and_attributes {
    use bstr::{BStr, ByteSlice};
    use std::path::Path;

    use git_index::entry::Mode;
    use git_worktree::fs;
    use tempfile::{tempdir, TempDir};

    struct IgnoreExpectations<'a> {
        lines: bstr::Lines<'a>,
    }

    impl<'a> Iterator for IgnoreExpectations<'a> {
        type Item = (&'a BStr, Option<(&'a BStr, usize, &'a BStr)>);

        fn next(&mut self) -> Option<Self::Item> {
            let line = self.lines.next()?;
            let (left, value) = line.split_at(line.find_byte(b'\t').unwrap());
            let value = value[1..].as_bstr();

            let source_and_line = if left == b"::" {
                None
            } else {
                let mut tokens = left.split(|b| *b == b':');
                let source = tokens.next().unwrap().as_bstr();
                let line_number: usize = tokens.next().unwrap().to_str_lossy().parse().ok().unwrap();
                let pattern = tokens.next().unwrap().as_bstr();
                Some((source, line_number, pattern))
            };
            Some((value, source_and_line))
        }
    }

    #[test]
    #[ignore]
    fn check_against_baseline() {
        let dir = git_testtools::scripted_fixture_repo_read_only("make_ignore_and_attributes_setup.sh").unwrap();
        let worktree_dir = dir.join("repo");
        let git_dir = worktree_dir.join(".git");
        let mut buf = Vec::new();
        let baseline = std::fs::read(git_dir.parent().unwrap().join("git-check-ignore.baseline")).unwrap();
        let user_exclude_path = dir.join("user.exclude");
        assert!(user_exclude_path.is_file());

        let mut cache = fs::Cache::new(
            &worktree_dir,
            git_worktree::fs::cache::State::for_add(
                Default::default(),
                git_worktree::fs::cache::state::Ignore::new(
                    git_attributes::MatchGroup::from_overrides(vec!["!force-include"]),
                    git_attributes::MatchGroup::from_git_dir(&git_dir, Some(user_exclude_path), &mut buf).unwrap(),
                ),
            ),
            buf,
        );

        let case_sensitive = git_glob::pattern::Case::Sensitive;
        for (relative_path, source_and_line) in (IgnoreExpectations {
            lines: baseline.lines(),
        }) {
            let relative_path = git_features::path::from_byte_slice_or_panic_on_windows(relative_path);
            let is_dir = worktree_dir.join(&relative_path).metadata().ok().map(|m| m.is_dir());
            let platform = cache.at_entry(relative_path, is_dir).unwrap();
            let match_ = platform.matching_exclude_pattern(case_sensitive);
            let is_excluded = platform.is_excluded(case_sensitive);
            match (match_, source_and_line) {
                (None, None) => {
                    assert!(!is_excluded);
                }
                (Some(m), Some((source_file, line, pattern))) => {
                    assert_eq!(m.sequence_number, line);
                    assert_eq!(m.pattern.text, pattern);
                    assert_eq!(
                        m.source.map(|p| p.canonicalize().unwrap()),
                        Some(
                            worktree_dir
                                .join(source_file.to_str_lossy().as_ref())
                                .canonicalize()
                                .unwrap()
                        )
                    );
                    todo!()
                }
                (actual, expected) => {
                    panic!("actual {:?} didn't match {:?}", actual, expected);
                }
            }
        }

        // TODO: at least one case-insensitive test
    }
}
