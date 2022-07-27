use super::Error;
use crate::bstr::BString;
use crate::ext::ObjectIdExt;
use crate::{bstr, Repository};
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
        title: BString,
    },
}

impl std::fmt::Display for CandidateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CandidateInfo::FindError { source } => write!(f, "lookup error: {}", source),
            CandidateInfo::Tag { name } => write!(f, "tag {:?}", name),
            CandidateInfo::Object { kind } => std::fmt::Display::fmt(kind, f),
            CandidateInfo::Commit { date, title } => write!(f, "commit {} {:?}", date.to_bstring(), title),
        }
    }
}

impl Error {
    pub(crate) fn ambiguous(candidates: HashSet<ObjectId>, prefix: git_hash::Prefix, repo: &Repository) -> Self {
        let candidates = {
            let mut c: Vec<_> = candidates.into_iter().collect();
            c.sort();
            c
        };
        Error::AmbiguousPrefix {
            prefix,
            info: candidates
                .into_iter()
                .map(|oid| {
                    let info = match repo.find_object(oid) {
                        Ok(obj) => match obj.kind {
                            git_object::Kind::Tree | git_object::Kind::Blob => CandidateInfo::Object { kind: obj.kind },
                            git_object::Kind::Tag => {
                                let tag = obj.to_tag_ref();
                                CandidateInfo::Tag { name: tag.name.into() }
                            }
                            git_object::Kind::Commit => {
                                use bstr::ByteSlice;
                                let commit = obj.to_commit_ref();
                                CandidateInfo::Commit {
                                    date: commit.committer().time,
                                    title: commit.message().title.trim().into(),
                                }
                            }
                        },
                        Err(err) => CandidateInfo::FindError { source: err },
                    };
                    (
                        oid.attach(repo).shorten().unwrap_or_else(|_| {
                            git_hash::Prefix::new(oid, oid.kind().len_in_hex()).expect("hex-len in range")
                        }),
                        info,
                    )
                })
                .collect(),
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
