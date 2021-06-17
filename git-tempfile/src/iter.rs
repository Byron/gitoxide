#![allow(missing_docs)]
use std::path::Path;

pub struct CreateDir<'a> {
    target: &'a Path,
    cursor: Option<&'a Path>,
}

impl<'a> CreateDir<'a> {
    pub fn new(target: &'a Path) -> Self {
        CreateDir {
            target,
            cursor: Some(target),
        }
    }
}

impl<'a> Iterator for CreateDir<'a> {
    type Item = std::io::Result<()>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::io::ErrorKind::*;
        match self.cursor.take() {
            Some(cursor) => match std::fs::create_dir(cursor) {
                Ok(()) => Some(Ok(())),
                Err(err) => match err.kind() {
                    AlreadyExists => Some(Ok(())),
                    _ => todo!("other errors"),
                },
            },
            None => None,
        }
    }
}
