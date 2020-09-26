use crate::compound;
use git_object::bstr::ByteSlice;
use std::{fs, io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Could not obtain an object path for the alternate directory '{}'", String::from_utf8_lossy(&.0))]
    PathConversion(Vec<u8>),
    #[error(transparent)]
    Init(#[from] compound::init::Error),
    #[error("Alternates form a cycle: {}", .0.iter().map(|p| format!("'{}'", p.display())).collect::<Vec<_>>().join(" -> "))]
    Cycle(Vec<PathBuf>),
}

pub fn resolve(objects_directory: impl Into<PathBuf>) -> Result<Vec<compound::Db>, Error> {
    let relative_base = objects_directory.into();
    let mut dirs = vec![(0, relative_base.clone())];
    let mut out = Vec::new();
    let mut seen = Vec::new();
    while let Some((depth, dir)) = dirs.pop() {
        match fs::read(dir.join("info").join("alternates")) {
            Ok(content) => {
                if seen.contains(&dir) {
                    continue;
                }
                seen.push(dir.clone());
                dirs.push((
                    depth + 1,
                    relative_base
                        .join(
                            content
                                .as_bstr()
                                .to_path()
                                .map(ToOwned::to_owned)
                                .map_err(|_| Error::PathConversion(content))?,
                        )
                        .canonicalize()?,
                ));
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                // Only resolve for repositories with at least one link, otherwise the line below isn't safe to call
                if depth != 0 {
                    // The tail of a chain doesn't have alternates, and thus is the real deal
                    out.push(compound::Db::at(dir)?);
                }
            }
            Err(err) => return Err(err.into()),
        };
    }

    if out.is_empty() && !seen.is_empty() {
        return Err(Error::Cycle(seen));
    }
    Ok(out)
}
