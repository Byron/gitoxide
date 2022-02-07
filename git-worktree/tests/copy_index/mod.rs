use std::fs;
#[cfg(unix)]
use std::os::unix::prelude::MetadataExt;

use git_object::bstr::ByteSlice;
use git_odb::FindExt;
use git_worktree::index;

use crate::{dir_structure, fixture_path};

#[test]
fn test_copy_index() -> crate::Result<()> {
    let path = fixture_path("make_repo");
    let path_git = path.join(".git");
    let mut file = git_index::File::at(path_git.join("index"), git_index::decode::Options::default())?;
    let output_dir = tempfile::tempdir()?;
    let output = output_dir.path();
    let odb_handle = git_odb::at(path_git.join("objects"))?;

    index::checkout(
        &mut file,
        &output,
        move |oid, buf| odb_handle.find_blob(oid, buf).ok(),
        index::checkout::Options::default(),
    )?;

    let repo_files = dir_structure(&path);
    let copy_files = dir_structure(output);

    assert_eq!(
        repo_files
            .iter()
            .flat_map(|p| p.strip_prefix(&path))
            .collect::<Vec<_>>(),
        copy_files
            .iter()
            .flat_map(|p| p.strip_prefix(output))
            .collect::<Vec<_>>()
    );

    for (file1, file2) in repo_files.iter().zip(copy_files.iter()) {
        assert_eq!(fs::read(file1)?, fs::read(file2)?);
        #[cfg(unix)]
        assert_eq!(
            fs::symlink_metadata(file1)?.mode() & 0o700,
            fs::symlink_metadata(file2)?.mode() & 0o700,
            "Testing if the permissions (normal/executable) of {} and {} are the same",
            file1.display(),
            file2.display(),
        );
    }

    Ok(())
}

#[test]
fn test_copy_index_without_symlinks() -> crate::Result<()> {
    let path = fixture_path("make_repo");
    let path_git = path.join(".git");
    let mut file = git_index::File::at(path_git.join("index"), git_index::decode::Options::default())?;
    let output_dir = tempfile::tempdir()?;
    let output = output_dir.path();
    let odb_handle = git_odb::at(path_git.join("objects"))?;

    index::checkout(
        &mut file,
        &output,
        move |oid, buf| odb_handle.find_blob(oid, buf).ok(),
        index::checkout::Options {
            symlinks: false,
            ..Default::default()
        },
    )?;

    let repo_files = dir_structure(&path);
    let copy_files = dir_structure(output);

    assert_eq!(
        repo_files
            .iter()
            .flat_map(|p| p.strip_prefix(&path))
            .collect::<Vec<_>>(),
        copy_files
            .iter()
            .flat_map(|p| p.strip_prefix(output))
            .collect::<Vec<_>>()
    );

    for (file1, file2) in repo_files.iter().zip(copy_files.iter()) {
        if file1.is_symlink() {
            assert!(!file2.is_symlink());
            assert_eq!(fs::read(file2)?.to_path()?, fs::read_link(file1)?);
        } else {
            assert_eq!(fs::read(file1)?, fs::read(file2)?);
            #[cfg(unix)]
            assert_eq!(
                fs::symlink_metadata(file1)?.mode() & 0o700,
                fs::symlink_metadata(file2)?.mode() & 0o700,
                "Testing if the permissions (normal/executable) of {} and {} are the same",
                file1.display(),
                file2.display(),
            );
        }
    }

    Ok(())
}
