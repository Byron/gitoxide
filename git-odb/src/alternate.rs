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
    #[error("Alternates form a cycle: {} -> {}", .0.iter().map(|p| format!("'{}'", p.display())).collect::<Vec<_>>().join(" -> "), .0.first().expect("more than one directories").display())]
    Cycle(Vec<PathBuf>),
}

pub fn resolve(objects_directory: impl Into<PathBuf>) -> Result<Vec<compound::Db>, Error> {
    let mut dirs = vec![(0, objects_directory.into())];
    let mut out = Vec::new();
    let mut seen = Vec::new();
    while let Some((depth, dir)) = dirs.pop() {
        if seen.contains(&dir) {
            return Err(Error::Cycle(seen));
        }
        seen.push(dir.clone());
        match fs::read(dir.join("info").join("alternates")) {
            Ok(content) => {
                dirs.push((
                    depth + 1,
                    content
                        .as_bstr()
                        .to_path()
                        .map(ToOwned::to_owned)
                        .map_err(|_| Error::PathConversion(content))?,
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
    Ok(out)
}
