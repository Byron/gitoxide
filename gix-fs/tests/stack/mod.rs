#![allow(clippy::join_absolute_paths)]
use std::path::{Path, PathBuf};

use gix_fs::Stack;

#[derive(Debug, Default, Eq, PartialEq)]
struct Record {
    push_dir: usize,
    dirs: Vec<PathBuf>,
    push: usize,
}

impl gix_fs::stack::Delegate for Record {
    fn push_directory(&mut self, stack: &Stack) -> std::io::Result<()> {
        self.push_dir += 1;
        self.dirs.push(stack.current().into());
        Ok(())
    }

    fn push(&mut self, _is_last_component: bool, _stack: &Stack) -> std::io::Result<()> {
        self.push += 1;
        Ok(())
    }

    fn pop_directory(&mut self) {
        self.dirs.pop();
    }
}

fn p(s: &str) -> &Path {
    s.as_ref()
}

/// Just to learn the specialities of `Path::join()`, which boils down to `Path::push(component)`.
#[test]
#[cfg(windows)]
fn path_join_handling() {
    let absolute = p("/absolute");
    assert!(
        absolute.is_relative(),
        "on Windows, absolute Linux paths are considered relative (and relative to the current drive)"
    );
    let bs_absolute = p("\\absolute");
    assert!(
        absolute.is_relative(),
        "on Windows, strange single-backslash paths are relative (and relative to the current drive)"
    );
    assert_eq!(
        p("relative").join(absolute),
        absolute,
        "relative + absolute = absolute - however, they kind of act like they are absolute in conjunction with relative base paths"
    );
    assert_eq!(
        p("relative").join(bs_absolute),
        bs_absolute,
        "relative + absolute = absolute - backslashes aren't special here, and it just acts like it's absolute"
    );

    assert_eq!(
        p("c:").join("relative"),
        p("c:relative"),
        "drive + relative = strange joined result with missing backslash, but it's a valid path that works just like `c:\relative`"
    );
    assert_eq!(
        p("c:\\").join("relative"),
        p("c:\\relative"),
        "absolute + relative = joined result"
    );

    assert_eq!(
        p("\\\\?\\base").join(absolute),
        p("\\\\?\\base\\absolute"),
        "absolute1 + unix-absolute2 = joined result with backslash"
    );
    assert_eq!(
        p("\\\\.\\base").join(absolute),
        p("\\\\.\\base\\absolute"),
        "absolute1 + absolute2 = joined result with backslash (device namespace)"
    );
    assert_eq!(
        p("\\\\?\\base").join(bs_absolute),
        p("\\\\?\\base\\absolute"),
        "absolute1 + absolute2 = joined result"
    );
    assert_eq!(
        p("\\\\.\\base").join(bs_absolute),
        p("\\\\.\\base\\absolute"),
        "absolute1 + absolute2 = joined result (device namespace)"
    );

    assert_eq!(p("/").join("C:"), p("C:"), "unix-absolute + win-drive = win-drive");
    assert_eq!(
        p("d:/").join("C:"),
        p("C:"),
        "d-drive + c-drive = c-drive - interesting, as C: is supposed to be relative"
    );
    assert_eq!(
        p("d:\\").join("C:\\"),
        p("C:\\"),
        "d-drive-with-bs + c-drive-with-bs = c-drive-with-bs - nothing special happens with backslashes"
    );
    assert_eq!(
        p("c:\\").join("\\\\.\\"),
        p("\\\\.\\"),
        "c-drive-with-bs + device-namespace-unc = device-namespace-unc"
    );
    assert_eq!(
        p("/").join("C:/"),
        p("C:\\"),
        "unix-absolute + win-drive = win-drive, strangely enough it changed the trailing slash to backslash, so better not have trailing slashes"
    );
    assert_eq!(p("/").join("C:\\"), p("C:\\"), "unix-absolute + win-drive = win-drive");
    assert_eq!(
        p("\\\\.").join("C:"),
        p("C:"),
        "device-namespace-unc + win-drive-relative = win-drive-relative - c: was supposed to be relative, but it's not acting like it."
    );
    assert_eq!(p("relative").join("C:"), p("C:"), "relative + win-drive = win-drive");

    assert_eq!(
        p("/").join("\\\\localhost"),
        p("\\localhost"),
        "unix-absolute + win-absolute-unc = win-absolute-unc"
    );
    assert_eq!(
        p("relative").join("\\\\localhost"),
        p("\\\\localhost"),
        "relative + win-absolute-unc = win-absolute-unc"
    );
}

/// Just to learn the specialities of `Path::join()`, which boils down to `Path::push(component)`.
#[test]
#[cfg(not(windows))]
fn path_join_handling() {
    assert_eq!(
        p("relative").join("/absolute"),
        p("/absolute"),
        "relative + absolute = absolute"
    );

    assert_eq!(
        p("/").join("relative"),
        p("/relative"),
        "absolute + relative = joined result"
    );

    assert_eq!(
        p("/").join("/absolute"),
        p("/absolute"),
        "absolute1 + absolute2 = absolute2"
    );

    assert_eq!(p("/").join("C:"), p("/C:"), "absolute + win-drive = joined result");
    assert_eq!(p("/").join("C:/"), p("/C:/"), "absolute + win-absolute = joined result");
    assert_eq!(
        p("/").join("C:\\"),
        p("/C:\\"),
        "absolute + win-absolute = joined result"
    );
    assert_eq!(
        p("relative").join("C:"),
        p("relative/C:"),
        "relative + win-drive = joined result"
    );

    assert_eq!(
        p("/").join("\\localhost"),
        p("/\\localhost"),
        "absolute + win-absolute-unc = joined result"
    );
    assert_eq!(
        p("relative").join("\\localhost"),
        p("relative/\\localhost"),
        "relative + win-absolute-unc = joined result"
    );
}

#[test]
fn empty_paths_are_noop_if_no_path_was_pushed_before() {
    let root = PathBuf::from(".");
    let mut s = Stack::new(root.clone());

    let mut r = Record::default();
    s.make_relative_path_current("".as_ref(), &mut r).unwrap();
    assert_eq!(
        s.current_relative().to_string_lossy(),
        "",
        "it's fine to push an empty path to get a value for the stack root, once"
    );
}

#[test]
fn relative_components_are_invalid() {
    let root = PathBuf::from(".");
    let mut s = Stack::new(root.clone());

    let mut r = Record::default();
    let err = s.make_relative_path_current("a/..".as_ref(), &mut r).unwrap_err();
    assert_eq!(
        err.to_string(),
        format!(
            "Input path {input:?} contains relative or absolute components",
            input = "a/.."
        )
    );

    s.make_relative_path_current("a/./b".as_ref(), &mut r)
        .expect("dot is ignored");
    assert_eq!(
        r,
        Record {
            push_dir: 2,
            dirs: vec![".".into(), "./a".into()],
            push: 2,
        },
        "The `a` directory is pushed, and the leaf, for a total of 2 pushes"
    );
    assert_eq!(
        s.current().to_string_lossy(),
        if cfg!(windows) { ".\\a\\b" } else { "./a/b" },
        "dot is silently ignored"
    );
    s.make_relative_path_current("a//b/".as_ref(), &mut r)
        .expect("multiple-slashes are ignored");
    assert_eq!(
        r,
        Record {
            push_dir: 2,
            dirs: vec![".".into(), "./a".into()],
            push: 2,
        },
        "nothing changed"
    );
}

#[test]
fn absolute_paths_are_invalid() -> crate::Result {
    let root = PathBuf::from(".");
    let mut s = Stack::new(root.clone());

    let mut r = Record::default();
    let err = s.make_relative_path_current("/".as_ref(), &mut r).unwrap_err();
    assert_eq!(
        err.to_string(),
        "Input path \"/\" contains relative or absolute components",
        "a leading slash is always considered absolute"
    );
    s.make_relative_path_current("a/".as_ref(), &mut r)?;
    assert_eq!(
        s.current(),
        p("./a/"),
        "trailing slashes aren't a problem at this stage, as they cannot cause a 'breakout'"
    );
    s.make_relative_path_current("b\\".as_ref(), &mut r)?;
    assert_eq!(
        s.current(),
        p("./b\\"),
        "trailing backslashes are fine both on Windows and Unix - on Unix it's part fo the filename"
    );

    #[cfg(windows)]
    {
        let err = s.make_relative_path_current("\\".as_ref(), &mut r).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Input path \"\\\" contains relative or absolute components",
            "on Windows, backslashes are considered absolute and replace the base if it is relative, \
            hence they are forbidden."
        );

        let err = s.make_relative_path_current("c:".as_ref(), &mut r).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Input path \"c:\" contains relative or absolute components",
            "on Windows, drive-letters without trailing backslash or slash are also absolute (even though they ought to be relative)"
        );
        let err = s.make_relative_path_current("c:\\".as_ref(), &mut r).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Input path \"c:\\\" contains relative or absolute components",
            "on Windows, drive-letters are absolute, which is expected"
        );

        s.make_relative_path_current("֍:".as_ref(), &mut r)?;
        assert_eq!(
            s.current().to_string_lossy(),
            ".\\֍:",
            "on Windows, almost any unicode character will do as virtual drive-letter actually with `subst`, \
            but we just turn it into a presumably invalid path which is fine, i.e. we get a joined path"
        );
        let err = s
            .make_relative_path_current(r#"\\localhost\hello"#.as_ref(), &mut r)
            .unwrap_err();
        assert_eq!(
            err.to_string(),
            r#"Input path "\\localhost\hello" contains relative or absolute components"#,
            "there is UNC paths as well"
        );

        let err = s.make_relative_path_current(r#"\\?\C:"#.as_ref(), &mut r).unwrap_err();
        assert_eq!(
            err.to_string(),
            r#"Input path "\\?\C:" contains relative or absolute components"#,
            "there is UNC paths as well, sometimes they look different"
        );
    }
    Ok(())
}

#[test]
fn delegate_calls_are_consistent() -> crate::Result {
    let root = PathBuf::from(".");
    let mut s = Stack::new(root.clone());

    assert_eq!(s.current(), root);
    assert_eq!(s.current_relative(), Path::new(""));

    let mut r = Record::default();
    s.make_relative_path_current("a/b".as_ref(), &mut r)?;
    let mut dirs = vec![root.clone(), root.join("a")];
    assert_eq!(
        r,
        Record {
            push_dir: 2,
            dirs: dirs.clone(),
            push: 2,
        },
        "it pushes the root-directory first, then the intermediate one"
    );

    s.make_relative_path_current("a/b2".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 2,
            dirs: dirs.clone(),
            push: 3,
        },
        "dirs remain the same as b2 is a leaf/file, hence the new `push`"
    );

    s.make_relative_path_current("c/d/e".as_ref(), &mut r)?;
    dirs.pop();
    dirs.extend([root.join("c"), root.join("c").join("d")]);
    assert_eq!(
        r,
        Record {
            push_dir: 4,
            dirs: dirs.clone(),
            push: 6,
        },
        "each directory is pushed individually, after popping 'a' which isn't included anymore"
    );

    dirs.push(root.join("c").join("d").join("x"));
    s.make_relative_path_current("c/d/x/z".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 5,
            dirs: dirs.clone(),
            push: 8,
        },
        "a new path component is added, hence `push_dir + 1`, but two components are added in total"
    );

    dirs.drain(1..).count();
    s.make_relative_path_current("f".as_ref(), &mut r)?;
    assert_eq!(s.current_relative(), Path::new("f"));
    assert_eq!(
        r,
        Record {
            push_dir: 5,
            dirs: dirs.clone(),
            push: 9,
        },
        "Now we only keep the root, as `f` is a leaf, hence `push + 1`"
    );

    dirs.push(root.join("x"));
    s.make_relative_path_current("x/z".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 6,
            dirs: dirs.clone(),
            push: 11,
        },
        "a new directory is pushed, or two new components total, hence `push + 2`"
    );

    dirs.push(root.join("x").join("z"));
    s.make_relative_path_current("x/z/a".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 7,
            dirs: dirs.clone(),
            push: 12,
        },
        "and another sub-directory is added"
    );

    dirs.push(root.join("x").join("z").join("a"));
    dirs.push(root.join("x").join("z").join("a").join("b"));
    s.make_relative_path_current("x/z/a/b/c".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 9,
            dirs: dirs.clone(),
            push: 14,
        },
        "and more subdirectories, two at once this time."
    );

    dirs.drain(1 /*root*/ + 1 /*x*/ + 1 /*x/z*/ ..).count();
    s.make_relative_path_current("x/z".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 9,
            dirs: dirs.clone(),
            push: 14,
        },
        "this only pops components, and as x/z/a/ was previously a directory, x/z is still a directory"
    );
    assert_eq!(
        dirs.last(),
        Some(&PathBuf::from("./x/z")),
        "the stack is state so keeps thinking it's a directory which is consistent. Git does it differently though."
    );

    let err = s.make_relative_path_current("".as_ref(), &mut r).unwrap_err();
    assert_eq!(
        err.to_string(),
        "empty inputs are not allowed",
        "this is to protect us from double-counting the root path next time a component is pushed, \
        and besides that really shouldn't happen"
    );

    s.make_relative_path_current("leaf".as_ref(), &mut r)?;
    dirs.drain(1..).count();
    assert_eq!(
        r,
        Record {
            push_dir: 9,
            dirs: dirs.clone(),
            push: 15,
        },
        "reset as much as possible, with just a leaf-component and the root directory"
    );

    s.make_relative_path_current("a//b".as_ref(), &mut r)?;
    dirs.push(root.join("a"));
    assert_eq!(
        r,
        Record {
            push_dir: 10,
            dirs: dirs.clone(),
            push: 17,
        },
        "double-slashes are automatically cleaned, even though they shouldn't happen, it's not forbidden"
    );

    #[cfg(not(windows))]
    {
        s.make_relative_path_current("\\/b".as_ref(), &mut r)?;
        dirs.pop();
        dirs.push(root.join("\\"));
        assert_eq!(
            r,
            Record {
                push_dir: 11,
                dirs: dirs.clone(),
                push: 19,
            },
            "a backslash is a normal character outside of Windows, so it's fine to have it as component"
        );

        s.make_relative_path_current("\\".as_ref(), &mut r)?;
        assert_eq!(
            r,
            Record {
                push_dir: 11,
                dirs: dirs.clone(),
                push: 19,
            },
        );
        assert_eq!(
            s.current().to_string_lossy(),
            "./\\",
            "a backslash can also be a valid leaf component - here we only popped the 'b', leaving the \\ 'directory'"
        );

        s.make_relative_path_current("\\\\".as_ref(), &mut r)?;
        dirs.pop();
        assert_eq!(
            r,
            Record {
                push_dir: 11,
                dirs: dirs.clone(),
                push: 20,
            },
        );
        assert_eq!(
            s.current().to_string_lossy(),
            "./\\\\",
            "the backslash can also be an ordinary leaf, without the need for it to be a directory"
        );
    }

    #[cfg(windows)]
    {
        s.make_relative_path_current("c\\/d".as_ref(), &mut r)?;
        dirs.pop();
        dirs.push(root.join("c"));
        assert_eq!(
            r,
            Record {
                push_dir: 11,
                dirs: dirs.clone(),
                push: 19,
            },
        );
        assert_eq!(
            s.current().to_string_lossy(),
            ".\\c\\d",
            "the backslash is a path-separator, and so is the `/`, which is turned into backslash"
        );
    }

    Ok(())
}
