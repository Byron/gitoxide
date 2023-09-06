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
        }
    );

    s.make_relative_path_current("a/b2".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 2,
            dirs: dirs.clone(),
            push: 3,
        }
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
        }
    );

    dirs.push(root.join("c").join("d").join("x"));
    s.make_relative_path_current("c/d/x/z".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 5,
            dirs: dirs.clone(),
            push: 8,
        }
    );

    dirs.drain(dirs.len() - 3..).count();
    s.make_relative_path_current("f".as_ref(), &mut r)?;
    assert_eq!(s.current_relative(), Path::new("f"));
    assert_eq!(
        r,
        Record {
            push_dir: 5,
            dirs: dirs.clone(),
            push: 9,
        }
    );

    dirs.push(root.join("x"));
    s.make_relative_path_current("x/z".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 6,
            dirs: dirs.clone(),
            push: 11,
        }
    );

    dirs.push(root.join("x").join("z"));
    s.make_relative_path_current("x/z/a".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 7,
            dirs: dirs.clone(),
            push: 12,
        }
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
        }
    );

    dirs.drain(dirs.len() - 2..).count();
    s.make_relative_path_current("x/z".as_ref(), &mut r)?;
    assert_eq!(
        r,
        Record {
            push_dir: 9,
            dirs: dirs.clone(),
            push: 14,
        }
    );
    assert_eq!(
        dirs.last(),
        Some(&PathBuf::from("./x/z")),
        "the stack is state so keeps thinking it's a directory which is consistent. Git does it differently though."
    );

    Ok(())
}
