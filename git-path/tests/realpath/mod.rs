use git_path::{create_symlink, realpath, realpath::Error, CanonicalizedTempDir};
use std::path::Path;
use tempfile::tempdir;

#[test]
fn assorted() {
    let cwd = tempdir().unwrap();
    let cwd = cwd.path();
    let symlinks_disabled = 0;

    assert!(
        matches!(realpath("", cwd, symlinks_disabled), Err(Error::EmptyPath)),
        "Empty path is not allowed"
    );

    assert_eq!(
        realpath("b/.git", cwd, symlinks_disabled).unwrap(),
        cwd.join("b").join(".git"),
        "relative paths are prefixed with current dir"
    );

    assert_eq!(
        realpath("b//.git", cwd, symlinks_disabled).unwrap(),
        cwd.join("b").join(".git"),
        "empty path components are ignored"
    );

    assert_eq!(
        realpath("./tmp/.git", cwd, symlinks_disabled).unwrap(),
        cwd.join("tmp").join(".git"),
        "path starting with dot is relative and is prefixed with current dir"
    );

    assert_eq!(
        realpath("./tmp/a/./.git", cwd, symlinks_disabled).unwrap(),
        cwd.join("tmp").join("a").join(".git"),
        "all ./ path components are ignored unless they the one at the beginning of the path"
    );

    assert_eq!(
        realpath("./b/../tmp/.git", cwd, symlinks_disabled).unwrap(),
        cwd.join("tmp").join(".git"),
        "dot dot goes to parent path component"
    );

    {
        #[cfg(not(target_os = "windows"))]
        let absolute_path = Path::new("/c/d/.git");
        #[cfg(target_os = "windows")]
        let absolute_path = Path::new("C:\\c\\d\\.git");
        assert_eq!(
            realpath(absolute_path, cwd, symlinks_disabled).unwrap(),
            absolute_path,
            "absolute path without symlinks has nothing to resolve and remains unchanged"
        );
    }
}

#[test]
fn link_cycle_is_detected() {
    let tmp_dir = CanonicalizedTempDir::new();
    let link_name = "link";
    let link_destination = tmp_dir.join(link_name);
    let link_path = tmp_dir.join(link_name);
    create_symlink(&link_path, &link_destination);
    let max_symlinks = 8;

    assert!(
        matches!(
            realpath(link_path.join(".git"), "", max_symlinks),
            Err(Error::MaxSymlinksExceeded { max_symlinks: 8 })
        ),
        "link cycle is detected"
    );
}

#[test]
fn symlink_with_absolute_path_gets_expanded() {
    let tmp_dir = CanonicalizedTempDir::new();
    let link_from = tmp_dir.join("a").join("b").join("tmp_p_q_link");
    let link_to = tmp_dir.join("p").join("q");
    create_symlink(&link_from, &link_to);
    let max_symlinks = 8;
    assert_eq!(
        realpath(link_from.join(".git"), tmp_dir, max_symlinks).unwrap(),
        link_to.join(".git"),
        "symlink with absolute path gets expanded"
    );
}

#[test]
fn symlink_to_relative_path_gets_expanded_into_absolute_path() {
    let cwd = CanonicalizedTempDir::new();
    let link_name = "pq_link";
    create_symlink(&cwd.join("r").join(link_name), &Path::new("p").join("q"));
    assert_eq!(
        realpath(Path::new(link_name).join(".git"), cwd.join("r"), 8).unwrap(),
        cwd.join("r").join("p").join("q").join(".git"),
        "symlink to relative path gets expanded into absolute path"
    );
}

#[test]
fn symlink_processing_is_disabled_if_the_value_is_zero() {
    let cwd = CanonicalizedTempDir::new();
    let link_name = "x_link";
    create_symlink(&cwd.join(link_name), Path::new("link destination does not exist"));
    assert!(
        matches!(
            realpath(&Path::new(link_name).join(".git"), &cwd, 0),
            Err(Error::MaxSymlinksExceeded { max_symlinks: 0 })
        ),
        "symlink processing is disabled if the value is zero"
    );
}
