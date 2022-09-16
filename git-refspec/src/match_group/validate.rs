use crate::match_group::{Outcome, Source};
use bstr::BString;
use std::collections::BTreeMap;

/// All possible issues found while validating matched mappings.
#[derive(Debug, PartialEq, Eq)]
pub enum Issue {
    /// Multiple sources try to write the same destination.
    ///
    /// Note that this issue doesn't take into consideration that these sources might contain the same object behind a reference.
    Conflict {
        /// The unenforced full name of the reference to be written.
        destination_full_ref_name: BString,
        /// The list of sources that map to this destination.
        sources: Vec<Source>,
        /// The list of specs that caused the mapping conflict, each matching the respective one in `sources` to allow both
        /// `sources` and `specs` to be zipped together.
        specs: Vec<BString>,
    },
}

impl std::fmt::Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Issue::Conflict {
                destination_full_ref_name,
                sources,
                specs,
            } => {
                write!(
                    f,
                    "Conflicting destination {destination_full_ref_name:?} would be written by {}",
                    sources
                        .iter()
                        .zip(specs.iter())
                        .map(|(src, spec)| format!("{src} ({spec:?})"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

/// All possible fixes corrected while validating matched mappings.
#[derive(Debug, PartialEq, Eq)]
pub enum Fix {}

/// The error returned [outcome validation][Outcome::validated()].
#[derive(Debug)]
pub struct Error {
    /// All issues discovered during validation.
    pub issues: Vec<Issue>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Found {} {} the refspec mapping to be used: \n\t{}",
            self.issues.len(),
            (self.issues.len() == 1)
                .then(|| "issue that prevents")
                .unwrap_or("issues that prevent"),
            self.issues
                .iter()
                .map(|issue| issue.to_string())
                .collect::<Vec<_>>()
                .join("\n\t")
        )
    }
}

impl std::error::Error for Error {}

impl<'spec, 'item> Outcome<'spec, 'item> {
    /// Validate all mappings or dissolve them into an error stating the discovered issues.
    /// Return `(modified self, issues)` providing a fixed-up set of mappings in `self` with the fixed `issues`
    /// provided as part of it.
    /// Terminal issues are communicated using the [`Error`] type accordingly.
    pub fn validated(self) -> Result<(Self, Vec<Fix>), Error> {
        let fixed = Vec::new();
        let mut sources_by_destinations = BTreeMap::new();
        for (dst, (spec_index, src)) in self
            .mappings
            .iter()
            .filter_map(|m| m.rhs.as_ref().map(|dst| (dst.as_ref(), (m.spec_index, &m.lhs))))
        {
            let sources = sources_by_destinations.entry(dst).or_insert_with(Vec::new);
            if !sources.iter().any(|(_, lhs)| lhs == &src) {
                sources.push((spec_index, src))
            }
        }
        let mut issues = Vec::new();
        for (dst, conflicting_sources) in sources_by_destinations.into_iter().filter(|(_, v)| v.len() > 1) {
            issues.push(Issue::Conflict {
                destination_full_ref_name: dst.to_owned(),
                specs: conflicting_sources
                    .iter()
                    .map(|(spec_idx, _)| self.group.specs[*spec_idx].to_bstring())
                    .collect(),
                sources: conflicting_sources.into_iter().map(|(_, src)| src.to_owned()).collect(),
            })
        }
        if !issues.is_empty() {
            Err(Error { issues })
        } else {
            Ok((self, fixed))
        }
    }
}
