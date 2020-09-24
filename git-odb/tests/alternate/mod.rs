use git_odb::alternate;
use std::{fs, io, path::PathBuf};

fn alternate(objects_at: impl Into<PathBuf>, objects_to: impl Into<PathBuf>) -> Result<(PathBuf, PathBuf), io::Error> {
    let at = objects_at.into();
    let to = objects_to.into();
    let at_info = at.join("info");
    fs::create_dir_all(&at_info)?;
    fs::create_dir(&to)?;
    fs::write(at_info.join("alternates"), to.to_string_lossy().as_bytes())?;
    Ok((at, to))
}

#[test]
#[ignore]
fn circular_alternates_are_detected() -> crate::Result {
    let tmp = tempdir::TempDir::new("alternates")?;
    alternate(tmp.path().join("a"), tmp.path().join("b"))?;
    alternate(tmp.path().join("b"), tmp.path().join("a"))?;

    Ok(())
}

#[test]
fn single_link() -> crate::Result {
    let tmp = tempdir::TempDir::new("alternates")?;
    let non_alternate = tmp.path().join("actual");

    let (from, to) = alternate(tmp.path().join("a"), non_alternate)?;
    assert_eq!(alternate::resolve(from).unwrap().unwrap().loose.path, to);
    Ok(())
}

#[test]
fn no_alternate_in_first_objects_dir() -> crate::Result {
    let tmp = tempdir::TempDir::new("alternates")?;
    assert!(alternate::resolve(tmp.path()).unwrap().is_none());
    Ok(())
}
