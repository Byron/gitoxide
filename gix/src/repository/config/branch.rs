use std::{borrow::Cow, collections::BTreeSet};

use gix_ref::{FullName, FullNameRef};

use crate::bstr::BStr;
use crate::config::cache::util::ApplyLeniencyDefault;
use crate::config::tree::{Branch, Push, Section};
use crate::repository::{branch_remote_ref_name, branch_remote_tracking_ref_name};
use crate::{push, remote};

/// Query configuration related to branches.
impl crate::Repository {
    /// Return a set of unique short branch names for which custom configuration exists in the configuration,
    /// if we deem them [trustworthy][crate::open::Options::filter_config_section()].
    ///
    /// ### Note
    ///
    /// Branch names that have illformed UTF-8 will silently be skipped.
    pub fn branch_names(&self) -> BTreeSet<&str> {
        self.subsection_str_names_of("branch")
    }

    /// Returns the validated reference on the remote associated with the given `name`,
    /// which will be used when *merging*.
    /// The returned value corresponds to the `branch.<short_branch_name>.merge` configuration key.
    ///
    /// Returns `None` if there is no value at the given key, or if no remote or remote ref is configured.
    /// May return an error if the reference name to be returned is invalid.
    ///
    /// ### Note
    ///
    /// This name refers to what Git calls upstream branch (as opposed to upstream *tracking* branch).
    /// The value is also fast to retrieve compared to its tracking branch.
    /// Also note that a [remote::Direction] isn't used here as Git only supports (and requires) configuring
    /// the remote to fetch from, not the one to push to.
    ///
    /// See also [`Reference::remote_ref_name()`](crate::Reference::remote_ref_name()).
    #[doc(alias = "branch_upstream_name", alias = "git2")]
    pub fn branch_remote_ref_name(
        &self,
        name: &FullNameRef,
        direction: remote::Direction,
    ) -> Option<Result<Cow<'_, FullNameRef>, branch_remote_ref_name::Error>> {
        match direction {
            remote::Direction::Fetch => {
                let short_name = name.shorten();
                self.config
                    .resolved
                    .string("branch", Some(short_name), Branch::MERGE.name)
                    .map(|name| crate::config::tree::branch::Merge::try_into_fullrefname(name).map_err(Into::into))
            }
            remote::Direction::Push => {
                let remote = match self.branch_remote(name.shorten(), direction)? {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err.into())),
                };
                if remote.push_specs.is_empty() {
                    let push_default = match self
                        .config
                        .resolved
                        .string(Push.name(), None, Push::DEFAULT.name)
                        .map_or(Ok(Default::default()), |v| {
                            Push::DEFAULT
                                .try_into_default(v)
                                .with_lenient_default(self.config.lenient_config)
                        }) {
                        Ok(v) => v,
                        Err(err) => return Some(Err(err.into())),
                    };
                    match push_default {
                        push::Default::Nothing => None,
                        push::Default::Current | push::Default::Matching => Some(Ok(Cow::Owned(name.to_owned()))),
                        push::Default::Upstream => self.branch_remote_ref_name(name, remote::Direction::Fetch),
                        push::Default::Simple => match self.branch_remote_ref_name(name, remote::Direction::Fetch)? {
                            Ok(fetch_ref) if fetch_ref.as_ref() == name => Some(Ok(fetch_ref)),
                            Err(err) => Some(Err(err)),
                            Ok(_different_fetch_ref) => None,
                        },
                    }
                } else {
                    matching_remote(name, remote.push_specs.iter(), self.object_hash())
                        .map(|res| res.map_err(Into::into))
                }
            }
        }
    }

    /// Return the validated name of the reference that tracks the corresponding reference of `name` on the remote for
    /// `direction`. Note that a branch with that name might not actually exist.
    ///
    /// * with `remote` being [remote::Direction::Fetch], we return the tracking branch that is on the destination
    ///   side of a `src:dest` refspec. For instance, with `name` being `main` and the default refspec
    ///   `refs/heads/*:refs/remotes/origin/*`, `refs/heads/main` would match and produce `refs/remotes/origin/main`.
    /// * with `remote` being [remote::Direction::Push], we return the tracking branch that corresponds to the remote
    ///   branch that we would push to. For instance, with `name` being `main` and no setup at all, we
    ///   would push to `refs/heads/main` on the remote. And that one would be fetched matching the
    ///   `refs/heads/*:refs/remotes/origin/*` fetch refspec, hence `refs/remotes/origin/main` is returned.
    ///   Note that `push` refspecs can be used to map `main` to `other` (using a push refspec `refs/heads/main:refs/heads/other`),
    ///   which would then lead to `refs/remotes/origin/other` to be returned instead.
    ///
    /// Note that if there is an ambiguity, that is if `name` maps to multiple tracking branches, the first matching mapping
    /// is returned, according to the order in which the fetch or push refspecs occur in the configuration file.
    ///
    /// See also [`Reference::remote_tracking_ref_name()`](crate::Reference::remote_tracking_ref_name()).
    #[doc(alias = "branch_upstream_name", alias = "git2")]
    pub fn branch_remote_tracking_ref_name(
        &self,
        name: &FullNameRef,
        direction: remote::Direction,
    ) -> Option<Result<Cow<'_, FullNameRef>, branch_remote_tracking_ref_name::Error>> {
        let remote_ref = match self.branch_remote_ref_name(name, direction)? {
            Ok(r) => r,
            Err(err) => return Some(Err(err.into())),
        };
        let remote = match self.branch_remote(name.shorten(), direction)? {
            Ok(r) => r,
            Err(err) => return Some(Err(err.into())),
        };

        if remote.fetch_specs.is_empty() {
            return None;
        }
        matching_remote(remote_ref.as_ref(), remote.fetch_specs.iter(), self.object_hash())
            .map(|res| res.map_err(Into::into))
    }

    /// Returns the unvalidated name of the remote associated with the given `short_branch_name`,
    /// typically `main` instead of `refs/heads/main`.
    /// In some cases, the returned name will be an URL.
    /// Returns `None` if the remote was not found or if the name contained illformed UTF-8.
    ///
    /// * if `direction` is [remote::Direction::Fetch], we will query the `branch.<short_name>.remote` configuration.
    /// * if `direction` is [remote::Direction::Push], the push remote will be queried by means of `branch.<short_name>.pushRemote`
    ///   or `remote.pushDefault` as fallback.
    ///
    /// See also [`Reference::remote_name()`](crate::Reference::remote_name()) for a more typesafe version
    /// to be used when a `Reference` is available.
    ///
    /// `short_branch_name` can typically be obtained by [shortening a full branch name](FullNameRef::shorten()).
    #[doc(alias = "branch_upstream_remote", alias = "git2")]
    pub fn branch_remote_name<'a>(
        &self,
        short_branch_name: impl Into<&'a BStr>,
        direction: remote::Direction,
    ) -> Option<remote::Name<'_>> {
        let name = short_branch_name.into();
        let config = &self.config.resolved;
        (direction == remote::Direction::Push)
            .then(|| {
                config
                    .string("branch", Some(name), Branch::PUSH_REMOTE.name)
                    .or_else(|| config.string("remote", None, crate::config::tree::Remote::PUSH_DEFAULT.name))
            })
            .flatten()
            .or_else(|| config.string("branch", Some(name), Branch::REMOTE.name))
            .and_then(|name| name.try_into().ok())
    }

    /// Like [`branch_remote_name(…)`](Self::branch_remote_name()), but returns a [Remote](crate::Remote).
    /// `short_branch_name` is the name to use for looking up `branch.<short_branch_name>.*` values in the
    /// configuration.
    ///
    /// See also [`Reference::remote()`](crate::Reference::remote()).
    pub fn branch_remote<'a>(
        &self,
        short_branch_name: impl Into<&'a BStr>,
        direction: remote::Direction,
    ) -> Option<Result<crate::Remote<'_>, remote::find::existing::Error>> {
        let name = self.branch_remote_name(short_branch_name, direction)?;
        self.try_find_remote(name.as_bstr())
            .map(|res| res.map_err(Into::into))
            .or_else(|| match name {
                remote::Name::Url(url) => gix_url::parse(url.as_ref())
                    .map_err(Into::into)
                    .and_then(|url| {
                        self.remote_at(url)
                            .map_err(|err| remote::find::existing::Error::Find(remote::find::Error::Init(err)))
                    })
                    .into(),
                remote::Name::Symbol(_) => None,
            })
    }
}

fn matching_remote<'a>(
    lhs: &FullNameRef,
    specs: impl IntoIterator<Item = &'a gix_refspec::RefSpec>,
    object_hash: gix_hash::Kind,
) -> Option<Result<Cow<'static, FullNameRef>, gix_validate::reference::name::Error>> {
    let search = gix_refspec::MatchGroup {
        specs: specs
            .into_iter()
            .map(gix_refspec::RefSpec::to_ref)
            .filter(|spec| spec.source().is_some() && spec.destination().is_some())
            .collect(),
    };
    let null_id = object_hash.null();
    let out = search.match_remotes(
        Some(gix_refspec::match_group::Item {
            full_ref_name: lhs.as_bstr(),
            target: &null_id,
            object: None,
        })
        .into_iter(),
    );
    out.mappings.into_iter().next().and_then(|m| {
        m.rhs.map(|name| {
            FullName::try_from(name.into_owned())
                .map(Cow::Owned)
                .map_err(Into::into)
        })
    })
}
