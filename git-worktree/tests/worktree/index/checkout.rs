mod cache {
    use git_index::entry::Mode;
    use git_worktree::index::checkout::PathCache;
    use std::path::Path;
    use tempfile::{tempdir, TempDir};

    #[test]
    fn root_is_assumed_to_exist_and_files_in_root_do_not_create_directory() {
        let dir = tempdir().unwrap();
        let mut cache = PathCache::new(dir.path().join("non-existing-root"));
        assert_eq!(cache.test_mkdir_calls, 0);

        let path = cache
            .append_relative_path_assure_leading_dir("hello", Mode::FILE)
            .unwrap();
        assert!(!path.parent().unwrap().exists(), "prefix itself is never created");
        assert_eq!(cache.test_mkdir_calls, 0);
    }

    #[test]
    fn directory_paths_are_created_in_full() {
        let (mut cache, _tmp) = new_cache();

        for (name, mode) in &[
            ("dir", Mode::DIR),
            ("submodule", Mode::COMMIT),
            ("file", Mode::FILE),
            ("exe", Mode::FILE_EXECUTABLE),
            ("link", Mode::SYMLINK),
        ] {
            let path = cache
                .append_relative_path_assure_leading_dir(Path::new("dir").join(name), *mode)
                .unwrap();
            assert!(path.parent().unwrap().is_dir(), "dir exists");
        }

        assert_eq!(cache.test_mkdir_calls, 3);
    }

    #[test]
    fn existing_directories_are_fine() {
        let (mut cache, tmp) = new_cache();
        std::fs::create_dir(tmp.path().join("dir")).unwrap();

        let path = cache
            .append_relative_path_assure_leading_dir("dir/file", Mode::FILE)
            .unwrap();
        assert!(path.parent().unwrap().is_dir(), "directory is still present");
        assert!(!path.exists(), "it won't create the file");
        assert_eq!(cache.test_mkdir_calls, 1);
    }

    #[test]
    fn symlinks_or_files_in_path_are_forbidden_or_unlinked_when_forced() {
        let (mut cache, tmp) = new_cache();
        let forbidden = tmp.path().join("forbidden");
        std::fs::create_dir(&forbidden).unwrap();
        symlink::symlink_dir(&forbidden, tmp.path().join("link-to-dir")).unwrap();
        std::fs::write(tmp.path().join("file-in-dir"), &[]).unwrap();

        for dirname in &["file-in-dir", "link-to-dir"] {
            cache.unlink_on_collision = false;
            let relative_path = format!("{}/file", dirname);
            assert_eq!(
                cache
                    .append_relative_path_assure_leading_dir(&relative_path, Mode::FILE)
                    .unwrap_err()
                    .kind(),
                std::io::ErrorKind::AlreadyExists
            );
        }
        assert_eq!(
            cache.test_mkdir_calls, 2,
            "it tries to create each directory once, but it's a file"
        );
        cache.test_mkdir_calls = 0;
        for dirname in &["link-to-dir", "file-in-dir"] {
            cache.unlink_on_collision = true;
            let relative_path = format!("{}/file", dirname);
            let path = cache
                .append_relative_path_assure_leading_dir(&relative_path, Mode::FILE)
                .unwrap();
            assert!(path.parent().unwrap().is_dir(), "directory was forcefully created");
            assert!(!path.exists());
        }
        assert_eq!(
            cache.test_mkdir_calls, 4,
            "like before, but it unlinks what's there and tries again"
        );
    }

    fn new_cache() -> (PathCache, TempDir) {
        let dir = tempdir().unwrap();
        let cache = PathCache::new(dir.path());
        (cache, dir)
    }
}

#[cfg(unix)]
use std::os::unix::prelude::MetadataExt;
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use git_features::progress;
use git_object::bstr::ByteSlice;
use git_odb::FindExt;
use git_worktree::{fs::Capabilities, index, index::checkout::Collision};
use std::io::ErrorKind::AlreadyExists;
use std::sync::atomic::AtomicBool;
use tempfile::TempDir;

use crate::fixture_path;

#[test]
fn accidental_writes_through_symlinks_are_prevented_if_overwriting_is_forbidden() {
    let mut opts = opts_from_probe();
    // without overwrite mode, everything is safe.
    opts.overwrite_existing = false;
    let (source_tree, destination, _index, outcome) =
        checkout_index_in_tmp_dir(opts, "make_dangerous_symlink").unwrap();

    let source_files = dir_structure(&source_tree);
    let worktree_files = dir_structure(&destination);

    if opts.fs.ignore_case {
        assert_eq!(
            stripped_prefix(&source_tree, &source_files),
            stripped_prefix(&destination, &worktree_files),
        );
        assert_eq!(
            outcome.collisions,
            vec![
                Collision {
                    path: "FAKE-DIR".into(),
                    error_kind: AlreadyExists
                },
                Collision {
                    path: "FAKE-FILE".into(),
                    error_kind: AlreadyExists
                }
            ]
        );
    } else {
        let expected = ["A-dir/a", "A-file", "FAKE-DIR", "FAKE-FILE", "fake-dir/b", "fake-file"];
        assert_eq!(stripped_prefix(&source_tree, &source_files), paths(expected));
        assert_eq!(stripped_prefix(&destination, &worktree_files), paths(expected));
        assert!(outcome.collisions.is_empty());
    };
}

#[test]
fn writes_through_symlinks_are_prevented_even_if_overwriting_is_allowed() {
    let mut opts = opts_from_probe();
    // with overwrite mode
    opts.overwrite_existing = true;
    let (source_tree, destination, _index, outcome) =
        checkout_index_in_tmp_dir(opts, "make_dangerous_symlink").unwrap();

    let source_files = dir_structure(&source_tree);
    let worktree_files = dir_structure(&destination);

    if opts.fs.ignore_case {
        assert_eq!(
            stripped_prefix(&source_tree, &source_files),
            paths(["A-dir/a", "A-file", "fake-dir/b", "fake-file"]),
        );
        assert_eq!(
            stripped_prefix(&destination, &worktree_files),
            paths(["A-dir/a", "A-file", "FAKE-DIR", "FAKE-FILE"]),
        );
        assert!(outcome.collisions.is_empty());
    } else {
        let expected = ["A-dir/a", "A-file", "FAKE-DIR", "FAKE-FILE", "fake-dir/b", "fake-file"];
        assert_eq!(stripped_prefix(&source_tree, &source_files), paths(expected));
        assert_eq!(stripped_prefix(&destination, &worktree_files), paths(expected));
        assert!(outcome.collisions.is_empty());
    };
}

#[test]
fn overwriting_files_and_lone_directories_works() {
    let mut opts = opts_from_probe();
    opts.overwrite_existing = true;
    opts.destination_is_initially_empty = false;
    let (_source_tree, destination, _index, outcome) = checkout_index_in_tmp_dir_opts(
        opts,
        "make_mixed_without_submodules",
        |_| true,
        |d| {
            let empty = d.join("empty");
            symlink::symlink_dir(d.join(".."), &empty)?; // empty is symlink to the directory above
            std::fs::write(d.join("executable"), b"foo")?; // executable is regular file and has different content
            let dir = d.join("dir");
            std::fs::create_dir(&dir)?;
            std::fs::create_dir(dir.join("content"))?; // 'content' is a directory now

            let dir = dir.join("sub-dir");
            std::fs::create_dir(&dir)?;

            symlink::symlink_dir(empty, dir.join("symlink"))?; // 'symlink' is a symlink to another file
            Ok(())
        },
    )
    .unwrap();

    assert!(outcome.collisions.is_empty());

    assert_eq!(
        stripped_prefix(&destination, &dir_structure(&destination)),
        paths(["dir/content", "dir/sub-dir/symlink", "empty", "executable"])
    );
    let meta = std::fs::symlink_metadata(destination.path().join("empty")).unwrap();
    assert!(meta.is_file(), "'empty' is now a file");
    assert_eq!(meta.len(), 0, "'empty' is indeed empty");

    let exe = destination.path().join("executable");
    assert_eq!(
        std::fs::read(&exe).unwrap(),
        b"content",
        "'exe' has the correct content"
    );

    let meta = std::fs::symlink_metadata(exe).unwrap();
    assert!(meta.is_file());
    if opts.fs.executable_bit {
        #[cfg(unix)]
        assert_eq!(meta.mode() & 0o700, 0o700, "the executable bit is set where supported");
    }

    assert_eq!(
        std::fs::read(destination.path().join("dir/content")).unwrap(),
        b"other content"
    );

    let symlink = destination.path().join("dir/sub-dir/symlink");
    // on windows, git won't create symlinks as its probe won't detect the capability, even though we do.
    assert_eq!(std::fs::symlink_metadata(&symlink).unwrap().is_symlink(), cfg!(unix));
    assert_eq!(std::fs::read(symlink).unwrap(), b"other content");
}

#[test]
fn symlinks_become_files_if_disabled() -> crate::Result {
    let mut opts = opts_from_probe();
    opts.fs.symlink = false;
    let (source_tree, destination, _index, outcome) = checkout_index_in_tmp_dir(opts, "make_mixed_without_submodules")?;

    assert_equality(&source_tree, &destination, opts.fs.symlink)?;
    assert!(outcome.collisions.is_empty());
    Ok(())
}

#[test]
fn allow_or_disallow_symlinks() -> crate::Result {
    let mut opts = opts_from_probe();
    for allowed in &[false, true] {
        opts.fs.symlink = *allowed;
        let (source_tree, destination, _index, outcome) =
            checkout_index_in_tmp_dir(opts, "make_mixed_without_submodules")?;

        assert_equality(&source_tree, &destination, opts.fs.symlink)?;
        assert!(outcome.collisions.is_empty());
    }
    Ok(())
}

#[test]
fn keep_going_collects_results() {
    let mut opts = opts_from_probe();
    opts.keep_going = true;
    let mut count = 0;
    let (_source_tree, destination, _index, outcome) = checkout_index_in_tmp_dir_opts(
        opts,
        "make_mixed_without_submodules",
        |_id| {
            if count < 2 {
                count += 1;
                false
            } else {
                true
            }
        },
        |_| Ok(()),
    )
    .unwrap();

    assert_eq!(
        outcome
            .errors
            .iter()
            .map(|r| r.path.to_path_lossy().into_owned())
            .collect::<Vec<_>>(),
        paths(if cfg!(unix) {
            ["dir/content", "empty"]
        } else {
            // not actually a symlink anymore, even though symlinks are supported but git think differently.
            ["dir/content", "dir/sub-dir/symlink"]
        })
    );

    assert_eq!(
        stripped_prefix(&destination, &dir_structure(&destination)),
        paths(if cfg!(unix) {
            ["dir/sub-dir/symlink", "executable"]
        } else {
            ["empty", "executable"]
        }),
        "some files could not be created"
    );

    assert!(outcome.collisions.is_empty());
}

#[test]
fn no_case_related_collisions_on_case_sensitive_filesystem() {
    let opts = opts_from_probe();
    if opts.fs.ignore_case {
        eprintln!("Skipping case-sensitive testing on what would be a case-insensitive file system");
        return;
    }
    let (source_tree, destination, index, outcome) =
        checkout_index_in_tmp_dir(opts, "make_ignorecase_collisions").unwrap();

    assert!(outcome.collisions.is_empty());
    let num_files = assert_equality(&source_tree, &destination, opts.fs.symlink).unwrap();
    assert_eq!(num_files, index.entries().len(), "it checks out all files");
}

#[test]
fn collisions_are_detected_on_a_case_sensitive_filesystem() {
    let opts = opts_from_probe();
    if !opts.fs.ignore_case {
        eprintln!("Skipping case-insensitive testing on what would be a case-senstive file system");
        return;
    }
    let (source_tree, destination, _index, outcome) =
        checkout_index_in_tmp_dir(opts, "make_ignorecase_collisions").unwrap();

    let source_files = dir_structure(&source_tree);
    assert_eq!(
        stripped_prefix(&source_tree, &source_files),
        paths(["d", "file_x", "link-to-X", "x"]),
        "plenty of collisions prevent a checkout"
    );

    let dest_files = dir_structure(&destination);
    assert_eq!(
        stripped_prefix(&destination, &dest_files),
        paths(["D/B", "D/C", "FILE_X", "X", "link-to-X"]),
        "we checkout files in order and generally handle collision detection differently, hence the difference"
    );

    let error_kind = ErrorKind::AlreadyExists;
    #[cfg(windows)]
    let error_kind_dir = ErrorKind::PermissionDenied;
    #[cfg(not(windows))]
    let error_kind_dir = error_kind;

    assert_eq!(
        outcome.collisions,
        vec![
            Collision {
                path: "FILE_x".into(),
                error_kind,
            },
            Collision {
                path: "d".into(),
                error_kind: error_kind_dir,
            },
            Collision {
                path: "file_X".into(),
                error_kind,
            },
            Collision {
                path: "file_x".into(),
                error_kind,
            },
            Collision {
                path: "x".into(),
                error_kind,
            },
        ],
        "these files couldn't be checked out"
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
        .filter_map(|e| (!e.path().symlink_metadata().map_or(true, |m| m.is_dir())).then(|| e.path().to_path_buf()))
        .collect();
    files.sort();
    files
}

fn checkout_index_in_tmp_dir(
    opts: index::checkout::Options,
    name: &str,
) -> crate::Result<(
    PathBuf,
    TempDir,
    git_index::File,
    git_worktree::index::checkout::Outcome,
)> {
    checkout_index_in_tmp_dir_opts(opts, name, |_d| true, |_| Ok(()))
}

fn checkout_index_in_tmp_dir_opts(
    opts: index::checkout::Options,
    name: &str,
    mut allow_return_object: impl FnMut(&git_hash::oid) -> bool,
    prep_dest: impl Fn(&Path) -> std::io::Result<()>,
) -> crate::Result<(
    PathBuf,
    TempDir,
    git_index::File,
    git_worktree::index::checkout::Outcome,
)> {
    let source_tree = fixture_path(name);
    let git_dir = source_tree.join(".git");
    let mut index = git_index::File::at(git_dir.join("index"), Default::default())?;
    let odb = git_odb::at(git_dir.join("objects"))?;
    let destination = tempfile::tempdir_in(std::env::current_dir()?)?;
    prep_dest(destination.path())?;

    let outcome = index::checkout(
        &mut index,
        destination.path(),
        move |oid, buf| {
            if allow_return_object(oid) {
                odb.find_blob(oid, buf)
            } else {
                Err(git_odb::find::existing_object::Error::NotFound { oid: oid.to_owned() })
            }
        },
        &mut progress::Discard,
        &mut progress::Discard,
        &AtomicBool::default(),
        opts,
    )?;
    Ok((source_tree, destination, index, outcome))
}

fn stripped_prefix(prefix: impl AsRef<Path>, source_files: &[PathBuf]) -> Vec<&Path> {
    source_files.iter().flat_map(|p| p.strip_prefix(&prefix)).collect()
}

fn probe_gitoxide_dir() -> crate::Result<Capabilities> {
    Ok(git_worktree::fs::Capabilities::probe(
        std::env::current_dir()?.join("..").join(".git"),
    ))
}

fn opts_from_probe() -> index::checkout::Options {
    index::checkout::Options {
        fs: probe_gitoxide_dir().unwrap(),
        destination_is_initially_empty: true,
        thread_limit: git_features::parallel::num_threads(None).into(),
        ..Default::default()
    }
}

fn paths<'a>(p: impl IntoIterator<Item = &'a str>) -> Vec<PathBuf> {
    p.into_iter().map(PathBuf::from).collect()
}
