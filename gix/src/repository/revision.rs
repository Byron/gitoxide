use crate::{bstr::BStr, revision, Id};
use gix_macros::momo;

/// Methods for resolving revisions by spec or working with the commit graph.
impl crate::Repository {
    /// Parse a revision specification and turn it into the object(s) it describes, similar to `git rev-parse`.
    ///
    /// # Deviation
    ///
    /// - `@` actually stands for `HEAD`, whereas `git` resolves it to the object pointed to by `HEAD` without making the
    ///   `HEAD` ref available for lookups.
    #[doc(alias = "revparse", alias = "git2")]
    #[momo]
    pub fn rev_parse<'a>(&self, spec: impl Into<&'a BStr>) -> Result<revision::Spec<'_>, revision::spec::parse::Error> {
        revision::Spec::from_bstr(
            spec,
            self,
            revision::spec::parse::Options {
                object_kind_hint: self.config.object_kind_hint,
                ..Default::default()
            },
        )
    }

    /// Parse a revision specification and return single object id as represented by this instance.
    #[doc(alias = "revparse_single", alias = "git2")]
    pub fn rev_parse_single<'repo, 'a>(
        &'repo self,
        spec: impl Into<&'a BStr>,
    ) -> Result<Id<'repo>, revision::spec::parse::single::Error> {
        let spec = spec.into();
        self.rev_parse(spec)?
            .single()
            .ok_or(revision::spec::parse::single::Error::RangedRev { spec: spec.into() })
    }

    /// Create the baseline for a revision walk by initializing it with the `tips` to start iterating on.
    ///
    /// It can be configured further before starting the actual walk.
    #[doc(alias = "revwalk", alias = "git2")]
    pub fn rev_walk(
        &self,
        tips: impl IntoIterator<Item = impl Into<gix_hash::ObjectId>>,
    ) -> revision::walk::Platform<'_> {
        revision::walk::Platform::new(tips, self)
    }
}
