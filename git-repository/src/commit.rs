//!

/// An empty array of a type usable with the `git::easy` API to help declaring no parents should be used
pub const NO_PARENT_IDS: [git_hash::ObjectId; 0] = [];

/// The error returned by [`commit(…)`][crate::Repository::commit()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    ReferenceNameValidation(#[from] git_ref::name::Error),
    #[error(transparent)]
    WriteObject(#[from] crate::object::write::Error),
    #[error(transparent)]
    ReferenceEdit(#[from] crate::reference::edit::Error),
}

///
pub mod describe {
    use crate::bstr::BStr;
    use crate::ext::ObjectIdExt;
    use crate::Repository;
    use git_hash::ObjectId;
    use git_odb::Find;
    use std::borrow::Cow;

    /// The result of [try_resolve()][Platform::try_resolve()].
    pub struct Resolution<'repo> {
        /// The outcome of the describe operation.
        pub outcome: git_revision::describe::Outcome<'static>,
        /// The id to describe.
        pub id: crate::Id<'repo>,
    }

    impl<'repo> Resolution<'repo> {
        /// Turn this instance into something displayable
        pub fn format(self) -> Result<git_revision::describe::Format<'static>, Error> {
            let prefix = self.id.shorten()?;
            Ok(self.outcome.into_format(prefix.hex_len()))
        }
    }

    /// The error returned by [try_format()][Platform::try_format()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        Describe(#[from] git_revision::describe::Error<git_odb::store::find::Error>),
        #[error("Could not produce an unambiguous shortened id for formatting.")]
        ShortId(#[from] crate::id::shorten::Error),
        #[error(transparent)]
        RefIter(#[from] crate::reference::iter::Error),
        #[error(transparent)]
        RefIterInit(#[from] crate::reference::iter::init::Error),
    }

    /// A selector to choose what kind of references should contribute to names.
    pub enum SelectRef {
        /// Only use annotated tags for names.
        AnnotatedTags,
        /// Use all tags for names, annotated or plain reference.
        AllTags,
        /// Use all references, including local branch names.
        AllRefs,
    }

    impl SelectRef {
        fn names(
            &self,
            repo: &Repository,
        ) -> Result<git_revision::hash_hasher::HashedMap<ObjectId, Cow<'static, BStr>>, Error> {
            let platform = repo.references()?;
            let into_tuple =
                |r: crate::Reference<'_>| (r.inner.target.into_id(), Cow::from(r.inner.name.shorten().to_owned()));

            Ok(match self {
                SelectRef::AllTags => platform
                    .tags()?
                    .peeled()
                    .filter_map(Result::ok)
                    .map(into_tuple)
                    .collect(),
                SelectRef::AllRefs => platform
                    .all()?
                    .peeled()
                    .filter_map(Result::ok)
                    .map(into_tuple)
                    .collect(),
                SelectRef::AnnotatedTags => platform
                    .tags()?
                    .filter_map(Result::ok)
                    .filter_map(|r: crate::Reference<'_>| {
                        // TODO: we assume direct refs for tags, which is the common case, but it doesn't have to be
                        //       so rather follow symrefs till the first object and then peel tags after the first object was found.
                        let tag = r.try_id()?.object().ok()?.try_into_tag().ok()?;
                        let commit_id = tag.target_id().ok()?.object().ok()?.try_into_commit().ok()?.id;
                        Some((commit_id, r.name().shorten().to_owned().into()))
                    })
                    .collect(),
            })
        }
    }

    impl Default for SelectRef {
        fn default() -> Self {
            SelectRef::AnnotatedTags
        }
    }

    /// A support type to allow configuring a `git describe` operation
    pub struct Platform<'repo> {
        pub(crate) id: git_hash::ObjectId,
        pub(crate) repo: &'repo crate::Repository,
        pub(crate) select: SelectRef,
        pub(crate) first_parent: bool,
        pub(crate) id_as_fallback: bool,
        pub(crate) max_candidates: usize,
    }

    impl<'repo> Platform<'repo> {
        /// Configure which names to `select` from which describe can chose.
        pub fn names(mut self, select: SelectRef) -> Self {
            self.select = select;
            self
        }

        /// If true, shorten the graph traversal time by just traversing the first parent of merge commits.
        pub fn traverse_first_parent(mut self, first_parent: bool) -> Self {
            self.first_parent = first_parent;
            self
        }

        /// Only consider the given amount of candidates, instead of the default of 10.
        pub fn max_candidates(mut self, candidates: usize) -> Self {
            self.max_candidates = candidates;
            self
        }

        /// If true, even if no candidate is available a format will always be produced.
        pub fn id_as_fallback(mut self, use_fallback: bool) -> Self {
            self.id_as_fallback = use_fallback;
            self
        }

        /// Try to find a name for the configured commit id using all prior configuration, returning `Some(describe::Format)`
        /// if one was found.
        ///
        /// Note that there will always be `Some(format)`
        pub fn try_format(self) -> Result<Option<git_revision::describe::Format<'static>>, Error> {
            self.try_resolve()?.map(|r| r.format()).transpose()
        }

        /// Try to find a name for the configured commit id using all prior configuration, returning `Some(Outcome)`
        /// if one was found.
        ///
        /// The outcome provides additional information, but leaves the caller with the burden
        pub fn try_resolve(self) -> Result<Option<crate::commit::describe::Resolution<'repo>>, Error> {
            // TODO: dirty suffix with respective dirty-detection
            let outcome = git_revision::describe(
                &self.id,
                |id, buf| {
                    Ok(self
                        .repo
                        .objects
                        .try_find(id, buf)?
                        .and_then(|d| d.try_into_commit_iter()))
                },
                git_revision::describe::Options {
                    name_by_oid: self.select.names(self.repo)?,
                    fallback_to_oid: self.id_as_fallback,
                    first_parent: self.first_parent,
                    max_candidates: self.max_candidates,
                },
            )?;

            Ok(outcome.map(|outcome| crate::commit::describe::Resolution {
                outcome,
                id: self.id.attach(self.repo),
            }))
        }

        /// Like [`try_format()`][Platform::try_format()], but turns `id_as_fallback()` on to always produce a format.
        pub fn format(mut self) -> Result<git_revision::describe::Format<'static>, Error> {
            self.id_as_fallback = true;
            Ok(self.try_format()?.expect("BUG: fallback must always produce a format"))
        }
    }
}
