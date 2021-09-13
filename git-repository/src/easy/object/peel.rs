#![allow(missing_docs)]
use crate::{
    easy,
    easy::{
        ext::ObjectAccessExt,
        object,
        object::{peel, Kind},
        ObjectRef,
    },
};

pub mod to_kind {
    mod error {

        use crate::easy::object;

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            FindExistingObject(#[from] object::find::existing::Error),
            #[error("Last encountered object kind was {} while trying to peel to {}", .actual, .expected)]
            NotFound {
                actual: object::Kind,
                expected: object::Kind,
            },
        }
    }
    pub use error::Error;
}

impl<'repo, A> ObjectRef<'repo, A>
where
    A: easy::Access + Sized,
{
    // TODO: tests
    pub fn peel_to_kind(mut self, kind: Kind) -> Result<Self, peel::to_kind::Error> {
        loop {
            match self.kind {
                any_kind if kind == any_kind => {
                    return Ok(self);
                }
                Kind::Commit => {
                    let tree_id = self.to_commit_iter().expect("commit").tree_id().expect("valid commit");
                    let access = self.access;
                    drop(self);
                    self = access.find_object(tree_id)?;
                }
                Kind::Tag => {
                    let target_id = self.to_tag_iter().expect("tag").target_id().expect("valid tag");
                    let access = self.access;
                    drop(self);
                    self = access.find_object(target_id)?;
                }
                Kind::Tree | Kind::Blob => {
                    return Err(peel::to_kind::Error::NotFound {
                        actual: self.kind,
                        expected: kind,
                    })
                }
            }
        }
    }

    // TODO: tests
    pub fn peel_to_end(mut self) -> Result<Self, object::find::existing::Error> {
        loop {
            match self.kind {
                Kind::Commit | Kind::Tree | Kind::Blob => break Ok(self),
                Kind::Tag => {
                    let target_id = self.to_tag_iter().expect("tag").target_id().expect("valid tag");
                    let access = self.access;
                    drop(self);
                    self = access.find_object(target_id)?;
                }
            }
        }
    }
}
