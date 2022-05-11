use git_path::{realpath, realpath::Error};
use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, tempdir_in};

struct CanonicalizedTempDir {
    pub dir: tempfile::TempDir,
}

impl CanonicalizedTempDir {
    fn new() -> Self {
        #[cfg(windows)]
        let canonicalized_tempdir = std::env::temp_dir();
        #[cfg(not(windows))]
        let canonicalized_tempdir = std::env::temp_dir().canonicalize().unwrap();
        let dir = tempdir_in(canonicalized_tempdir).unwrap();
        Self { dir }
    }
}

impl Deref for CanonicalizedTempDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.dir.path()
    }
}

fn create_symlink(link: &Path, link_dest: &Path) {
    #[cfg(not(target_os = "windows"))]
    std::os::unix::fs::symlink(link_dest, &link).unwrap();
    #[cfg(target_os = "windows")]
    std::os::windows::fs::symlink_file(link_dest, &link).unwrap();
}

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

    {
        // TODO: turn into its own testcase
        let cwd = tempdir().unwrap();
        let tmp_dir = CanonicalizedTempDir::new();
        let link_destination = tmp_dir.join("p").join("q");
        let link_name = "tmp_p_q_link";
        let root_dir = cwd.path().join("a").join("b");
        create_dir_all(&root_dir).unwrap();
        let link_path = root_dir.join(link_name);
        create_symlink(&link_path, &link_destination);
        let max_symlinks = 8;
        assert_eq!(
            realpath(link_path.join(".git"), cwd, max_symlinks).unwrap(),
            link_destination.join(".git"),
            "symlink with absolute path gets expanded"
        );
    }

    {
        // TODO: turn into its own testcase
        // TODO: can be changed to assure relative symlinks are indeed relative to the symlink location,
        //       and not to the cwd?
        let cwd = tempdir().unwrap();
        let link_destination = Path::new("p").join("q");
        let link_name = "pq_link";
        create_symlink(&cwd.path().join(link_name), &link_destination);
        let relative_path_with_symlink = Path::new(link_name).join(".git");
        let max_symlinks = 8;
        assert_eq!(
            realpath(relative_path_with_symlink, cwd.path(), max_symlinks).unwrap(),
            cwd.path().join("p").join("q").join(".git"),
            "symlink to relative path gets expanded into absolute path"
        );
    }

    {
        // TODO: turn into its own testcase
        let cwd = tempdir().unwrap();
        let link_name = "x_link";
        create_symlink(
            &cwd.path().join(link_name),
            Path::new("link destination does not exist"),
        );
        let relative_path_with_symlink = Path::new(link_name).join(".git");
        assert!(
            matches!(
                realpath(&relative_path_with_symlink, &cwd, 0),
                Err(Error::MaxSymlinksExceeded { max_symlinks: 0 })
            ),
            "symlink processing is disabled if the value is zero"
        );
    }

    // TODO: a test with a symlink cycle to assure cycle checking works.
}

#[test]
#[ignore]
fn prefix_component() {
    // todo!()

    // enum Component.Prefix

    let mut pb = PathBuf::from("/tmp");
    pb.push(std::path::MAIN_SEPARATOR.to_string());

    for c in PathBuf::from("/a/b/c").components() {
        dbg!(c);
    }

    // pass iterator input_path.components() instead of input_path
    // have real_path mut and not return it
    // change error to this_error
}
