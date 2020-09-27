use crate::compound;
use std::{fs, io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Parse(#[from] parse::Error),
    #[error(transparent)]
    Init(#[from] compound::init::Error),
    #[error("Alternates form a cycle: {} -> {}", .0.iter().map(|p| format!("'{}'", p.display())).collect::<Vec<_>>().join(" -> "), .0.first().expect("more than one directories").display())]
    Cycle(Vec<PathBuf>),
}

pub mod parse {
    use git_object::bstr::{BStr, ByteSlice};
    use std::{borrow::Cow, path::PathBuf};

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Could not obtain an object path for the alternate directory '{}'", String::from_utf8_lossy(&.0))]
        PathConversion(Vec<u8>),
    }

    fn unquote_ansi_c(line: &BStr) -> Cow<'_, BStr> {
        line.into()
    }

    pub(crate) fn content(input: &[u8]) -> Result<Vec<PathBuf>, Error> {
        let mut out = Vec::new();
        for line in input.split(|b| *b == b'\n') {
            let line = line.as_bstr();
            if line.is_empty() || line.starts_with(b"#") {
                continue;
            }
            out.push(
                if line.starts_with(b"\"") {
                    unquote_ansi_c(line)
                } else {
                    Cow::Borrowed(line)
                }
                .to_path()
                .map(ToOwned::to_owned)
                .map_err(|_| Error::PathConversion(line.to_vec()))?,
            )
        }
        Ok(out)
    }
}

pub fn resolve(objects_directory: impl Into<PathBuf>) -> Result<Vec<compound::Db>, Error> {
    let relative_base = objects_directory.into();
    let mut dirs = vec![(0, relative_base.clone())];
    let mut out = Vec::new();
    let mut seen = vec![relative_base.canonicalize()?];
    while let Some((depth, dir)) = dirs.pop() {
        match fs::read(dir.join("info").join("alternates")) {
            Ok(input) => {
                for path in parse::content(&input)?.into_iter() {
                    let path = relative_base.join(path);
                    let path_canonicalized = path.canonicalize()?;
                    if seen.contains(&path_canonicalized) {
                        continue;
                    }
                    seen.push(path_canonicalized);
                    dirs.push((depth + 1, path));
                }
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // Only resolve for repositories with at least one link, otherwise the line below causes infinite recursion
                if depth != 0 {
                    // The tail of a chain doesn't have alternates, and thus is the real deal
                    out.push(compound::Db::at(dir)?);
                }
            }
            Err(err) => return Err(err.into()),
        };
    }

    if out.is_empty() && seen.len() > 1 {
        return Err(Error::Cycle(seen));
    }
    Ok(out)
}
