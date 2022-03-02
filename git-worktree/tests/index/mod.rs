mod checkout {
    #[cfg(unix)]
    use std::os::unix::prelude::MetadataExt;
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    use git_object::bstr::ByteSlice;
    use git_odb::FindExt;
    use git_worktree::fs::Capabilities;
    use git_worktree::index;
    use tempfile::TempDir;

    use crate::fixture_path;

    fn probe_gitoxide_dir() -> crate::Result<Capabilities> {
        Ok(git_worktree::fs::Capabilities::probe(
            std::env::current_dir()?.join("..").join(".git"),
        ))
    }

    fn opts_with_symlink(symlink: bool) -> index::checkout::Options {
        index::checkout::Options {
            fs: git_worktree::fs::Capabilities {
                symlink,
                ..Default::default()
            },
            destination_is_initially_empty: true,
            ..Default::default()
        }
    }

    #[test]
    fn symlinks_become_files_if_disabled() -> crate::Result {
        let opts = opts_with_symlink(false);
        let (source_tree, destination, _index) = checkout_index_in_tmp_dir(opts, "make_mixed_without_submodules")?;

        assert_equality(&source_tree, &destination, opts.fs.symlink)?;

        Ok(())
    }

    #[test]
    fn allow_symlinks() -> crate::Result {
        let opts = opts_with_symlink(true);
        if !probe_gitoxide_dir()?.symlink {
            eprintln!("IGNORING symlink test on file system without symlink support");
            // skip if symlinks aren't supported anyway.
            return Ok(());
        };
        let (source_tree, destination, _index) = checkout_index_in_tmp_dir(opts, "make_mixed_without_submodules")?;

        assert_equality(&source_tree, &destination, opts.fs.symlink)?;
        Ok(())
    }

    #[test]
    fn no_case_related_collisions_on_case_sensitive_filesystem() {
        if probe_gitoxide_dir().unwrap().ignore_case {
            eprintln!("Skipping case-sensitive testing on what would be a case-insenstive file system");
            return;
        }
        let opts = opts_with_symlink(true);
        let (source_tree, destination, index) = checkout_index_in_tmp_dir(opts, "make_ignorecase_collisions").unwrap();
        assert_eq!(index.entries().len(), 2, "there is just one colliding item");

        let num_files = assert_equality(&source_tree, &destination, opts.fs.symlink).unwrap();
        assert_eq!(num_files, index.entries().len(), "it checks out all files");
    }

    #[test]
    fn collisions_are_detected_on_a_case_sensitive_filesystem() {
        if !probe_gitoxide_dir().unwrap().ignore_case {
            eprintln!("Skipping case-insensitive testing on what would be a case-senstive file system");
            return;
        }
        let opts = opts_with_symlink(true);
        let (source_tree, destination, index) = checkout_index_in_tmp_dir(opts, "make_ignorecase_collisions").unwrap();
        assert_eq!(index.entries().len(), 2, "there is just one colliding item");

        let source_files = dir_structure(&source_tree);
        assert_eq!(
            stripped_prefix(&source_tree, &source_files),
            vec![PathBuf::from("a")],
            "the source also only contains the first created file"
        );

        let dest_files = dir_structure(&destination);
        assert_eq!(
            stripped_prefix(&destination, &dest_files),
            vec![PathBuf::from("A")],
            "it only creates the first file of a collision"
        );
    }

    fn assert_equality(source_tree: &Path, destination: &TempDir, allow_symlinks: bool) -> crate::Result<usize> {
        let source_files = dir_structure(source_tree);
        let worktree_files = dir_structure(&destination);

        assert_eq!(
            stripped_prefix(source_tree, &source_files),
            stripped_prefix(&destination, &worktree_files),
        );

        let mut count = 0;
        for (source_file, worktree_file) in source_files.iter().zip(worktree_files.iter()) {
            count += 1;
            if !allow_symlinks && source_file.is_symlink() {
                assert!(!worktree_file.is_symlink());
                assert_eq!(fs::read(worktree_file)?.to_path()?, fs::read_link(source_file)?);
            } else {
                assert_eq!(fs::read(source_file)?, fs::read(worktree_file)?);
                #[cfg(unix)]
                assert_eq!(
                    fs::symlink_metadata(source_file)?.mode() & 0o700,
                    fs::symlink_metadata(worktree_file)?.mode() & 0o700,
                    "permissions of source and checked out file are comparable"
                );
            }
        }
        Ok(count)
    }

    pub fn dir_structure<P: AsRef<std::path::Path>>(path: P) -> Vec<std::path::PathBuf> {
        let path = path.as_ref();
        let mut files: Vec<_> = walkdir::WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| e.path() == path || !e.file_name().to_string_lossy().starts_with('.'))
            .flatten()
            .filter_map(|e| (!e.path().is_dir()).then(|| e.path().to_path_buf()))
            .collect();
        files.sort();
        files
    }

    fn checkout_index_in_tmp_dir(
        opts: index::checkout::Options,
        name: &str,
    ) -> crate::Result<(PathBuf, TempDir, git_index::File)> {
        let source_tree = fixture_path(name);
        let git_dir = source_tree.join(".git");
        let mut index = git_index::File::at(git_dir.join("index"), Default::default())?;
        let odb = git_odb::at(git_dir.join("objects"))?;
        let destination = tempfile::tempdir()?;

        index::checkout(
            &mut index,
            &destination,
            move |oid, buf| odb.find_blob(oid, buf).ok(),
            opts,
        )?;
        Ok((source_tree, destination, index))
    }

    fn stripped_prefix(prefix: impl AsRef<Path>, source_files: &[PathBuf]) -> Vec<&Path> {
        source_files.iter().flat_map(|p| p.strip_prefix(&prefix)).collect()
    }
}
