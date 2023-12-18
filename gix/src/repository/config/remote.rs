use crate::bstr::BStr;
use std::{borrow::Cow, collections::BTreeSet};

use crate::config::tree::{Remote, Section};
use crate::remote;

/// Query configuration related to remotes.
impl crate::Repository {
    /// Returns a sorted list unique of symbolic names of remotes that
    /// we deem [trustworthy][crate::open::Options::filter_config_section()].
    pub fn remote_names(&self) -> BTreeSet<Cow<'_, BStr>> {
        self.config
            .resolved
            .sections_by_name(Remote.name())
            .map(|it| {
                let filter = self.filter_config_section();
                it.filter(move |s| filter(s.meta()))
                    .filter_map(|section| section.header().subsection_name().map(Cow::Borrowed))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Obtain the branch-independent name for a remote for use in the given `direction`, or `None` if it could not be determined.
    ///
    /// For _fetching_, use the only configured remote, or default to `origin` if it exists.
    /// For _pushing_, use the `remote.pushDefault` trusted configuration key, or fall back to the rules for _fetching_.
    ///
    /// # Notes
    ///
    /// It's up to the caller to determine what to do if the current `head` is unborn or detached.
    pub fn remote_default_name(&self, direction: remote::Direction) -> Option<Cow<'_, BStr>> {
        let name = (direction == remote::Direction::Push)
            .then(|| {
                self.config.resolved.string_filter(
                    Remote.name(),
                    None,
                    Remote::PUSH_DEFAULT.name,
                    &mut self.filter_config_section(),
                )
            })
            .flatten();
        name.or_else(|| {
            let names = self.remote_names();
            match names.len() {
                0 => None,
                1 => names.into_iter().next(),
                _more_than_one => {
                    let origin = Cow::Borrowed("origin".into());
                    names.contains(&origin).then_some(origin)
                }
            }
        })
    }
}
