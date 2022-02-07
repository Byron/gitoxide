mod checkout {
    #[cfg(unix)]
    use std::os::unix::prelude::MetadataExt;
    use std::{
        fs,
        path::{Path, PathBuf},
    };
    use tempfile::TempDir;

    use git_object::bstr::ByteSlice;
    use git_odb::FindExt;
    use git_worktree::index;

    use crate::fixture_path;

    #[test]
    fn allow_symlinks() -> crate::Result {
        let opts = Default::default();
        let (source_tree, destination) = setup_fixture_with_options(opts)?;

        assert_equality(&source_tree, &destination, opts.symlinks)?;
        Ok(())
    }

    #[test]
    fn symlinks_become_files_if_disabled() -> crate::Result {
        let opts = index::checkout::Options { symlinks: false };
        let (source_tree, destination) = setup_fixture_with_options(opts)?;

        assert_equality(&source_tree, &destination, opts.symlinks)?;

        Ok(())
    }

    fn assert_equality(source_tree: &Path, destination: &TempDir, allow_symlinks: bool) -> crate::Result {
        let source_files = dir_structure(source_tree);
        let worktree_files = dir_structure(&destination);

        assert_eq!(
            stripped_prefix(source_tree, &source_files),
            stripped_prefix(&destination, &worktree_files),
        );

        for (source_file, worktree_file) in source_files.iter().zip(worktree_files.iter()) {
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
        Ok(())
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

    fn setup_fixture_with_options(opts: git_worktree::index::checkout::Options) -> crate::Result<(PathBuf, TempDir)> {
        let source_tree = fixture_path("make_repo");
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
        Ok((source_tree, destination))
    }

    fn stripped_prefix(prefix: impl AsRef<Path>, source_files: &[PathBuf]) -> Vec<&Path> {
        source_files.iter().flat_map(|p| p.strip_prefix(&prefix)).collect()
    }
}
