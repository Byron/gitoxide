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
}

pub fn resolve(objects_directory: impl Into<PathBuf>) -> Result<Option<compound::Db>, Error> {
    let mut dir = objects_directory.into();
    let mut count = 0;
    loop {
        let content = match fs::read(dir.join("info").join("alternates")) {
            Ok(d) => d,
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                return if count == 0 {
                    Ok(None)
                } else {
                    Ok(Some(compound::Db::at(dir)?))
                }
            }
            Err(err) => return Err(err.into()),
        };
        dir = content
            .as_bstr()
            .to_path()
            .map(ToOwned::to_owned)
            .map_err(|_| Error::PathConversion(content.into()))?;
        count += 1;
    }
}
