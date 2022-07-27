use super::Error;
use crate::bstr::BString;
use crate::Repository;
use git_hash::ObjectId;
use std::collections::HashSet;

/// Additional information about candidates that caused ambiguity.
#[derive(Debug)]
pub enum CandidateInfo {
    /// An error occurred when looking up the object in the database.
    FindError {
        /// The reported error.
        source: crate::object::find::existing::OdbError,
    },
    /// The candidate is an object of the given `kind`.
    Object {
        /// The kind of the object.
        kind: git_object::Kind,
    },
    /// The candidate is a tag.
    Tag {
        /// The name of the tag.
        name: BString,
    },
    /// The candidate is a commit.
    Commit {
        /// The date of the commit.
        date: git_date::Time,
        /// The subject line.
        subject: BString,
    },
}

impl std::fmt::Display for CandidateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error {
    pub(crate) fn ambiguous(candidates: HashSet<ObjectId>, prefix: git_hash::Prefix, repo: &Repository) -> Self {
        Error::AmbiguousPrefix {
            prefix,
            info: Vec::new(),
        }
    }

    pub(crate) fn from_errors(errors: Vec<Self>) -> Self {
        assert!(!errors.is_empty());
        match errors.len() {
            0 => unreachable!(
                "BUG: cannot create something from nothing, must have recorded some errors to call from_errors()"
            ),
            1 => errors.into_iter().next().expect("one"),
            _ => {
                let mut it = errors.into_iter().rev();
                let mut recent = Error::Multi {
                    current: Box::new(it.next().expect("at least one error")),
                    next: None,
                };
                for err in it {
                    recent = Error::Multi {
                        current: Box::new(err),
                        next: Some(Box::new(recent)),
                    }
                }
                recent
            }
        }
    }
}
