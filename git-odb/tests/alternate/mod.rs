use git_odb::alternate;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn alternate(
    objects_at: impl Into<PathBuf>,
    objects_to: impl Into<PathBuf>,
) -> Result<(PathBuf, PathBuf), io::Error> {
    alternate_with(objects_at, objects_to, None)
}

fn alternate_with(
    objects_at: impl Into<PathBuf>,
    objects_to: impl Into<PathBuf>,
    content_before_to: Option<&str>,
) -> Result<(PathBuf, PathBuf), io::Error> {
    let at = objects_at.into();
    let to = objects_to.into();
    let at_info = at.join("info");
    fs::create_dir_all(&at_info)?;
    fs::create_dir_all(&to)?;
    let contents = if let Some(content) = content_before_to {
        let mut c = vec![b'\n'];
        c.extend(content.as_bytes());
        c.extend(to.to_string_lossy().as_bytes());
        c
    } else {
        to.to_string_lossy().as_bytes().to_owned()
    };
    fs::write(at_info.join("alternates"), contents)?;
    Ok((at, to))
}

#[test]
fn circular_alternates_are_detected_with_relative_paths() -> crate::Result {
    let tmp = test_tools::tempdir::TempDir::new("alternates")?;
    let (from, _) = alternate(tmp.path().join("a"), tmp.path().join("b"))?;
    alternate(tmp.path().join("b"), Path::new("..").join("a"))?;

    match alternate::resolve(&from) {
        Err(alternate::Error::Cycle(chain)) => {
            assert_eq!(
                chain
                    .into_iter()
                    .map(|p| p.file_name().expect("non-root").to_str().expect("utf8").to_owned())
                    .collect::<Vec<_>>(),
                vec!["a", "b"]
            );
        }
        res => unreachable!("should be a specific kind of error: {:?}", res),
    }
    Ok(())
}

#[test]
fn single_link_with_comment_before_path_and_ansi_c_escape() -> crate::Result {
    let tmp = test_tools::tempdir::TempDir::new("alternates")?;
    let non_alternate = tmp.path().join("actual");

    // let (from, to) = alternate_with(tmp.path().join("a"), non_alternate, Some("# comment\n\"../a\"\n"))?;
    let (from, to) = alternate_with(tmp.path().join("a"), non_alternate, Some("# comment\n"))?;
    let alternates = alternate::resolve(from)?;
    assert_eq!(alternates.len(), 1);
    assert_eq!(alternates[0], to);
    Ok(())
}

#[test]
fn no_alternate_in_first_objects_dir() -> crate::Result {
    let tmp = test_tools::tempdir::TempDir::new("alternates")?;
    assert!(alternate::resolve(tmp.path())?.is_empty());
    Ok(())
}
