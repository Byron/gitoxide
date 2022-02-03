use crate::{dir_structure, fixture_path};
use git_object::bstr::ByteSlice;
use git_odb::FindExt;
use git_worktree::{copy_index, Options};
use std::fs;

#[cfg(unix)]
use std::os::unix::prelude::MetadataExt;

#[test]
fn test_copy_index() -> crate::Result<()> {
    let path = fixture_path("make_repo");
    let path_git = path.join(".git");
    let mut file = git_index::File::at(path_git.join("index"), git_index::decode::Options::default())?;
    let output_dir = tempfile::tempdir()?;
    let output = output_dir.path();
    let odb_handle = git_odb::at(path_git.join("objects"))?;

    copy_index(
        &mut file,
        &output,
        move |oid, buf| odb_handle.find_blob(oid, buf).ok(),
        Options::default(),
    )?;

    let repo_files = dir_structure(&path);
    let copy_files = dir_structure(output);

    let srepo_files: Vec<_> = repo_files.iter().flat_map(|p| p.strip_prefix(&path)).collect();
    let scopy_files: Vec<_> = copy_files.iter().flat_map(|p| p.strip_prefix(output)).collect();
    assert_eq!(
        srepo_files,
        scopy_files,
        "Testing if {} and {} have the same structure",
        path.display(),
        output.display()
    );

    for (file1, file2) in repo_files.iter().zip(copy_files.iter()) {
        assert_eq!(
            fs::read(file1)?,
            fs::read(file2)?,
            "Testing if the contents of {} and {} are the same",
            file1.display(),
            file2.display(),
        );
        #[cfg(unix)]
        assert_eq!(
            fs::symlink_metadata(file1)?.mode() & 0b111 << 6,
            fs::symlink_metadata(file2)?.mode() & 0b111 << 6,
            "Testing if the permissions of {} and {} are the same",
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

    copy_index(
        &mut file,
        &output,
        move |oid, buf| odb_handle.find_blob(oid, buf).ok(),
        Options { symlinks: false },
    )?;

    let repo_files = dir_structure(&path);
    let copy_files = dir_structure(output);

    let srepo_files: Vec<_> = repo_files.iter().flat_map(|p| p.strip_prefix(&path)).collect();
    let scopy_files: Vec<_> = copy_files.iter().flat_map(|p| p.strip_prefix(output)).collect();
    assert_eq!(
        srepo_files,
        scopy_files,
        "Testing if {} and {} have the same structure",
        path.display(),
        output.display()
    );

    for (file1, file2) in repo_files.iter().zip(copy_files.iter()) {
        if file1.is_symlink() {
            assert!(!file2.is_symlink(), "Testing if the new file is not a symlink");
            assert_eq!(
                fs::read(file2)?.to_path()?,
                fs::read_link(file1)?,
                "Testing if the {} contains the path the symlink {} is pointing to",
                file2.display(),
                file1.display(),
            );
        } else {
            assert_eq!(
                fs::read(file1)?,
                fs::read(file2)?,
                "Testing if the contents of {} and {} are the same",
                file1.display(),
                file2.display(),
            );
            #[cfg(unix)]
            assert_eq!(
                fs::symlink_metadata(file1)?.mode() & 0b111 << 6,
                fs::symlink_metadata(file2)?.mode() & 0b111 << 6,
                "Testing if the permissions of {} and {} are the same",
                file1.display(),
                file2.display(),
            );
        }
    }

    Ok(())
}
