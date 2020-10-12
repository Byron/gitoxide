use crate::loose::Db;
use git_features::fs::WalkDir;
use git_object::owned;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    WalkDir(#[from] git_features::fs::walkdir::Error),
}

/// Iteration and traversal
impl Db {
    /// Return an iterator over all objects contained in the database.
    ///
    /// The [`Id`][owned::Id]s returned by the iterator can typically be used in the [`locate(…)`][Db::locate()] method.
    /// _Note_ that the result is not sorted or stable, thus ordering can change between runs.
    pub fn iter(&self) -> impl Iterator<Item = Result<owned::Id, Error>> {
        use std::path::Component::Normal;
        WalkDir::new(&self.path)
            .min_depth(2)
            .max_depth(3)
            .follow_links(false)
            .into_iter()
            .filter_map(|res| {
                let mut is_valid_path = false;
                let e = res.map_err(Error::WalkDir).map(|e| {
                    let p = e.path();
                    let (c1, c2) = p.components().fold((None, None), |(_c1, c2), cn| (c2, Some(cn)));
                    if let (Some(Normal(c1)), Some(Normal(c2))) = (c1, c2) {
                        if c1.len() == 2 && c2.len() == 38 {
                            if let (Some(c1), Some(c2)) = (c1.to_str(), c2.to_str()) {
                                let mut buf = [0u8; 40];
                                {
                                    let (first_byte, rest) = buf.split_at_mut(2);
                                    first_byte.copy_from_slice(c1.as_bytes());
                                    rest.copy_from_slice(c2.as_bytes());
                                }
                                if let Ok(b) = owned::Id::from_40_bytes_in_hex(&buf[..]) {
                                    is_valid_path = true;
                                    return b;
                                }
                            }
                        }
                    }
                    owned::Id::null_sha1()
                });
                if is_valid_path {
                    Some(e)
                } else {
                    None
                }
            })
    }
}
