#[cfg(unix)]
use std::os::unix::prelude::MetadataExt;
use std::{
    fs,
    io::{ErrorKind, ErrorKind::AlreadyExists},
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use gix_features::progress;
use gix_object::bstr::ByteSlice;
use gix_odb::FindExt;
use gix_testtools::tempfile::TempDir;
use gix_worktree_state::checkout::Collision;
use once_cell::sync::Lazy;

use crate::fixture_path;

static DRIVER: Lazy<PathBuf> = Lazy::new(|| {
    let mut cargo = std::process::Command::new(env!("CARGO"));
    let res = cargo
        .args(["build", "-p=gix-filter", "--example", "arrow"])
        .status()
        .expect("cargo should run fine");
    assert!(res.success(), "cargo invocation should be successful");

    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .ancestors()
        .nth(1)
        .expect("first parent in target dir")
        .join("debug")
        .join("examples")
        .join(if cfg!(windows) { "arrow.exe" } else { "arrow" });
    assert!(path.is_file(), "Expecting driver to be located at {path:?}");
    path
});

fn driver_exe() -> String {
    let mut exe = DRIVER.to_string_lossy().into_owned();
    if cfg!(windows) {
        exe = exe.replace('\\', "/");
    }
    exe
}

#[test]
fn submodules_are_instantiated_as_directories() -> crate::Result {
    let mut opts = opts_from_probe();
    opts.overwrite_existing = false;
    let (_source_tree, destination, _index, _outcome) = checkout_index_in_tmp_dir(opts.clone(), "make_mixed")?;

    for path in ["m1", "modules/m1"] {
        let sm = destination.path().join(path);
        assert!(sm.is_dir());
        assure_is_empty(sm)?;
    }

    Ok(())
}

fn assure_is_empty(dir: impl AsRef<Path>) -> std::io::Result<()> {
    assert_eq!(std::fs::read_dir(dir)?.count(), 0);
    Ok(())
}

#[test]
fn accidental_writes_through_symlinks_are_prevented_if_overwriting_is_forbidden() {
    let mut opts = opts_from_probe();
    // without overwrite mode, everything is safe.
    opts.overwrite_existing = false;
    let (source_tree, destination, _index, outcome) =
        checkout_index_in_tmp_dir(opts.clone(), "make_dangerous_symlink").unwrap();

    let source_files = dir_structure(&source_tree);
    let worktree_files = dir_structure(&destination);

    if opts.fs.ignore_case {
        assert_eq!(
            stripped_prefix(&source_tree, &source_files),
            stripped_prefix(&destination, &worktree_files),
        );
        if multi_threaded() {
            assert_eq!(outcome.collisions.len(), 2);
        } else {
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
        }
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
        checkout_index_in_tmp_dir(opts.clone(), "make_dangerous_symlink").unwrap();

    let source_files = dir_structure(&source_tree);
    let worktree_files = dir_structure(&destination);

    if opts.fs.ignore_case {
        assert_eq!(
            stripped_prefix(&source_tree, &source_files),
            paths(["A-dir/a", "A-file", "fake-dir/b", "fake-file"]),
        );
        assert_eq!(
            stripped_prefix(&destination, &worktree_files),
            paths([
                if cfg!(windows) { "A-dir\\a" } else { "A-dir/a" },
                "A-file",
                "FAKE-DIR",
                if cfg!(windows) { "fake-file" } else { "FAKE-FILE" }
            ]),
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
fn delayed_driver_process() -> crate::Result {
    let mut opts = opts_from_probe();
    opts.overwrite_existing = true;
    opts.filter_process_delay = gix_filter::driver::apply::Delay::Allow;
    opts.destination_is_initially_empty = false;
    setup_filter_pipeline(opts.filters.options_mut());
    let (_source, destination, _index, outcome) =
        checkout_index_in_tmp_dir_opts(opts, "make_mixed_without_submodules_and_symlinks", |_| true, |_| Ok(()))?;
    assert_eq!(outcome.collisions.len(), 0);
    assert_eq!(outcome.errors.len(), 0);
    assert_eq!(outcome.files_updated, 5);

    let dest = destination.path();
    assert_eq!(
        std::fs::read(dest.join("executable"))?.as_bstr(),
        "content",
        "unfiltered"
    );
    assert_eq!(
        std::fs::read(dest.join("dir").join("content"))?.as_bstr(),
        "➡other content\r\n"
    );
    assert_eq!(
        std::fs::read(dest.join("dir").join("sub-dir").join("file"))?.as_bstr(),
        "➡even other content\r\n"
    );
    Ok(())
}

#[test]
#[cfg_attr(
    windows,
    ignore = "on windows, the symlink to a directory doesn't seem to work and we really want to test with symlinks"
)]
fn overwriting_files_and_lone_directories_works() -> crate::Result {
    for delay in [
        gix_filter::driver::apply::Delay::Allow,
        gix_filter::driver::apply::Delay::Forbid,
    ] {
        let mut opts = opts_from_probe();
        opts.overwrite_existing = true;
        opts.filter_process_delay = delay;
        opts.destination_is_initially_empty = false;
        setup_filter_pipeline(opts.filters.options_mut());
        let (source, destination, _index, outcome) = checkout_index_in_tmp_dir_opts(
            opts.clone(),
            "make_mixed",
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
        )?;

        assert!(outcome.collisions.is_empty());

        assert_eq!(
            stripped_prefix(&destination, &dir_structure(&destination)),
            paths(["dir/content", "dir/sub-dir/symlink", "empty", "executable"])
        );
        let meta = std::fs::symlink_metadata(destination.path().join("empty"))?;
        assert!(meta.is_file(), "'empty' is now a file");
        assert_eq!(meta.len(), 0, "'empty' is indeed empty");

        let exe = destination.path().join("executable");
        assert_eq!(std::fs::read(&exe)?, b"content", "'exe' has the correct content");

        let meta = std::fs::symlink_metadata(exe)?;
        assert!(meta.is_file());
        if opts.fs.executable_bit {
            #[cfg(unix)]
            assert_eq!(meta.mode() & 0o700, 0o700, "the executable bit is set where supported");
        }

        assert_eq!(
            std::fs::read(source.join("dir/content"))?.as_bstr(),
            "other content\n",
            "in the worktree, we have LF"
        );
        assert_eq!(
            std::fs::read(destination.path().join("dir/content"))?.as_bstr(),
            "➡other content\r\n",
            "autocrlf is enabled, so we get CRLF when checking out as the pipeline is active, and we have a filter"
        );

        let symlink = destination.path().join("dir/sub-dir/symlink");
        // on windows, git won't create symlinks as its probe won't detect the capability, even though we do.
        assert_eq!(std::fs::symlink_metadata(&symlink)?.is_symlink(), cfg!(unix));
        assert_eq!(
            std::fs::read(symlink)?.as_bstr(),
            "➡other content\r\n",
            "autocrlf is enabled"
        );
    }
    Ok(())
}

#[test]
fn symlinks_become_files_if_disabled() -> crate::Result {
    let mut opts = opts_from_probe();
    opts.fs.symlink = false;
    let (source_tree, destination, _index, outcome) =
        checkout_index_in_tmp_dir(opts.clone(), "make_mixed_without_submodules")?;

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
            checkout_index_in_tmp_dir(opts.clone(), "make_mixed_without_submodules")?;

        assert_equality(&source_tree, &destination, opts.fs.symlink)?;
        assert!(outcome.collisions.is_empty());
    }
    Ok(())
}

#[test]
fn keep_going_collects_results() {
    let mut opts = opts_from_probe();
    opts.keep_going = true;
    let count = AtomicUsize::default();
    let (_source_tree, destination, _index, outcome) = checkout_index_in_tmp_dir_opts(
        opts,
        "make_mixed_without_submodules",
        |_id| {
            count
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
                    (current < 2).then_some(current + 1)
                })
                .is_err()
        },
        |_| Ok(()),
    )
    .unwrap();

    if multi_threaded() {
        assert_eq!(
            outcome.errors.len(),
            2,
            "content changes due to non-deterministic nature of racy threads"
        )
    } else {
        assert_eq!(
            outcome
                .errors
                .iter()
                .map(|r| r.path.to_path_lossy().into_owned())
                .collect::<Vec<_>>(),
            paths(if cfg!(unix) {
                [".gitattributes", "dir/content"]
            } else {
                // not actually a symlink anymore, even though symlinks are supported but git think differently.
                ["dir/content", "dir/sub-dir/symlink"]
            })
        );
    }

    if multi_threaded() {
        let actual = dir_structure(&destination);
        assert!(
            (2..=3).contains(&actual.len()),
            "it's 3 most of the time, but can be 2 of the 'empty' file is missing as the object couldn't be accessed.\
             It's unclear why there isn't more, as it would keep going"
        );
    } else {
        assert_eq!(
            stripped_prefix(&destination, &dir_structure(&destination)),
            paths(if cfg!(unix) {
                Box::new(["dir/sub-dir/symlink", "empty", "executable"].into_iter()) as Box<dyn Iterator<Item = &str>>
            } else {
                Box::new(["empty", "executable"].into_iter())
            }),
            "some files could not be created"
        );
    }

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
        checkout_index_in_tmp_dir(opts.clone(), "make_ignorecase_collisions").unwrap();

    assert!(outcome.collisions.is_empty());
    let num_files = assert_equality(&source_tree, &destination, opts.fs.symlink).unwrap();
    assert_eq!(
        num_files,
        index.entries().len() - 1,
        "it checks out all files (minus 1 to account for .gitattributes which is skipped in the worktree in our tests)"
    );
    assert!(
        destination.path().join(".gitattributes").is_file(),
        "we do have attributes even though, dot files are ignored in `assert_equality`"
    );
}

#[test]
fn collisions_are_detected_on_a_case_insensitive_filesystem_even_with_delayed_filters() {
    let mut opts = opts_from_probe();
    if !opts.fs.ignore_case {
        eprintln!("Skipping case-insensitive testing on what would be a case-sensitive file system");
        return;
    }
    setup_filter_pipeline(opts.filters.options_mut());
    opts.filter_process_delay = gix_filter::driver::apply::Delay::Allow;
    let (source_tree, destination, _index, outcome) =
        checkout_index_in_tmp_dir(opts, "make_ignorecase_collisions").unwrap();

    let source_files = dir_structure(&source_tree);
    assert_eq!(
        stripped_prefix(&source_tree, &source_files),
        paths(["d", "file_x", "link-to-X", "x"]),
        "plenty of collisions prevent a checkout"
    );

    let dest_files = dir_structure(&destination);
    if multi_threaded() {
        assert!(
            (4..=6).contains(&dest_files.len()),
            "due to the clash happening at nearly any time, and keep-going is false, we get a variance of files"
        );
    } else {
        assert_eq!(
            stripped_prefix(&destination, &dest_files),
            paths(["D/B", "D/C", "FILE_X", "X", "link-to-X"]),
            "we checkout files in order and generally handle collision detection differently, hence the difference"
        );
    }

    let error_kind = ErrorKind::AlreadyExists;
    #[cfg(windows)]
    let error_kind_dir = ErrorKind::PermissionDenied;
    #[cfg(not(windows))]
    let error_kind_dir = error_kind;

    if multi_threaded() {
        assert!(
            (5..=6).contains(&outcome.collisions.len()),
            "can only assert on number as it's racily creating files so unclear which one clashes, and due to keep-going = false there is variance"
        );
    } else {
        assert_eq!(
            outcome.collisions,
            vec![
                Collision {
                    path: "d".into(),
                    error_kind: error_kind_dir,
                },
                Collision {
                    path: "FILE_x".into(),
                    error_kind,
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
}

fn multi_threaded() -> bool {
    gix_features::parallel::num_threads(None) > 1
}

fn assert_equality(source_tree: &Path, destination: &TempDir, allow_symlinks: bool) -> crate::Result<usize> {
    let source_files = dir_structure(source_tree);
    let worktree_files = dir_structure(destination);

    assert_eq!(
        stripped_prefix(source_tree, &source_files),
        stripped_prefix(destination, &worktree_files),
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
    opts: gix_worktree_state::checkout::Options,
    name: &str,
) -> crate::Result<(PathBuf, TempDir, gix_index::File, gix_worktree_state::checkout::Outcome)> {
    checkout_index_in_tmp_dir_opts(opts, name, |_d| true, |_| Ok(()))
}

fn checkout_index_in_tmp_dir_opts(
    opts: gix_worktree_state::checkout::Options,
    name: &str,
    mut allow_return_object: impl FnMut(&gix_hash::oid) -> bool + Send + Clone,
    prep_dest: impl Fn(&Path) -> std::io::Result<()>,
) -> crate::Result<(PathBuf, TempDir, gix_index::File, gix_worktree_state::checkout::Outcome)> {
    let source_tree = fixture_path(name);
    let git_dir = source_tree.join(".git");
    let mut index = gix_index::File::at(git_dir.join("index"), gix_hash::Kind::Sha1, false, Default::default())?;
    let odb = gix_odb::at(git_dir.join("objects"))?.into_inner().into_arc()?;
    let destination = gix_testtools::tempfile::tempdir_in(std::env::current_dir()?)?;
    prep_dest(destination.path()).expect("preparation must succeed");

    let outcome = gix_worktree_state::checkout(
        &mut index,
        destination.path(),
        move |oid, buf| {
            if allow_return_object(oid) {
                odb.find_blob(oid, buf)
            } else {
                Err(gix_odb::find::existing_object::Error::NotFound { oid: oid.to_owned() })
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

fn probe_gitoxide_dir() -> crate::Result<gix_fs::Capabilities> {
    Ok(gix_fs::Capabilities::probe(
        gix_discover::upwards(".")?
            .0
            .into_repository_and_work_tree_directories()
            .0,
    ))
}

fn opts_from_probe() -> gix_worktree_state::checkout::Options {
    gix_worktree_state::checkout::Options {
        fs: probe_gitoxide_dir().unwrap(),
        destination_is_initially_empty: true,
        thread_limit: gix_features::parallel::num_threads(None).into(),
        ..Default::default()
    }
}

fn paths<'a>(p: impl IntoIterator<Item = &'a str>) -> Vec<PathBuf> {
    p.into_iter().map(PathBuf::from).collect()
}

fn setup_filter_pipeline(opts: &mut gix_filter::pipeline::Options) {
    opts.eol_config.auto_crlf = gix_filter::eol::AutoCrlf::Enabled;
    opts.drivers = vec![gix_filter::Driver {
        name: "arrow".into(),
        clean: None,
        smudge: None,
        process: Some((driver_exe() + " process").into()),
        required: true,
    }];
}
