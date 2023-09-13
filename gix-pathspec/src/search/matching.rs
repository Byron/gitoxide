use bstr::{BStr, BString, ByteSlice};
use gix_glob::pattern::Case;

use crate::{
    search::{Match, Spec},
    MagicSignature, Pattern, Search, SearchMode,
};

impl Search {
    /// Return the first [`Match`] of `relative_path`, or `None`.
    /// `is_dir` is `true` if `relative_path` is a directory.
    /// `attributes` is called as `attributes(relative_path, case, is_dir, outcome) -> has_match` to obtain for attributes for `relative_path`, if
    /// the underlying pathspec defined an attribute filter, to be stored in `outcome`, returning true if there was a match.
    /// All attributes of the pathspec have to be present in the defined value for the pathspec to match.
    ///
    /// Note that `relative_path` is expected to be starting at the same root as is assumed for this pattern, see [`Pattern::normalize()`].
    /// Further, empty searches match everything, as if `:` was provided.
    ///
    /// ### Deviation
    ///
    /// The case-sensivity of the attribute match is controlled by the sensitivity of the pathspec, instead of being based on the
    /// case folding settings of the repository. That way we assure that the matching is consistent.
    /// Higher-level crates should control this default case folding of pathspecs when instantiating them, which is when they can
    /// set it to match the repository setting for more natural behaviour when, for instance, adding files to a repository:
    /// as it stands, on a case-insensitive file system, `touch File && git add file` will not add the file, but also not error.
    pub fn pattern_matching_relative_path(
        &mut self,
        relative_path: &BStr,
        is_dir: Option<bool>,
        attributes: &mut dyn FnMut(&BStr, Case, bool, &mut gix_attributes::search::Outcome) -> bool,
    ) -> Option<Match<'_>> {
        let basename_not_important = None;
        if relative_path
            .get(..self.common_prefix_len)
            .map_or(true, |rela_path_prefix| rela_path_prefix != self.common_prefix())
        {
            return None;
        }

        let is_dir = is_dir.unwrap_or(false);
        let patterns_len = self.patterns.len();
        let res = self.patterns.iter_mut().find_map(|mapping| {
            let ignore_case = mapping.value.pattern.signature.contains(MagicSignature::ICASE);
            let prefix = mapping.value.pattern.prefix_directory();
            if ignore_case && !prefix.is_empty() {
                let pattern_requirement_is_met = relative_path.get(prefix.len()).map_or_else(|| is_dir, |b| *b == b'/');
                if !pattern_requirement_is_met
                    || relative_path.get(..prefix.len()).map(ByteSlice::as_bstr) != Some(prefix)
                {
                    return None;
                }
            }

            let case = if ignore_case { Case::Fold } else { Case::Sensitive };
            let mut is_match = mapping.value.pattern.is_nil() || mapping.value.pattern.path.is_empty();
            if !is_match {
                is_match = if mapping.pattern.first_wildcard_pos.is_none() {
                    match_verbatim(mapping, relative_path, is_dir, case)
                } else {
                    let wildmatch_mode = match mapping.value.pattern.search_mode {
                        SearchMode::ShellGlob => Some(gix_glob::wildmatch::Mode::empty()),
                        SearchMode::Literal => None,
                        SearchMode::PathAwareGlob => Some(gix_glob::wildmatch::Mode::NO_MATCH_SLASH_LITERAL),
                    };
                    match wildmatch_mode {
                        Some(wildmatch_mode) => {
                            let is_match = mapping.pattern.matches_repo_relative_path(
                                relative_path,
                                basename_not_important,
                                Some(is_dir),
                                case,
                                wildmatch_mode,
                            );
                            if !is_match {
                                match_verbatim(mapping, relative_path, is_dir, case)
                            } else {
                                true
                            }
                        }
                        None => match_verbatim(mapping, relative_path, is_dir, case),
                    }
                }
            }

            if let Some(attrs) = mapping.value.attrs_match.as_mut() {
                if !attributes(relative_path, Case::Sensitive, is_dir, attrs) {
                    // we have attrs, but it didn't match any
                    return None;
                }
                for (actual, expected) in attrs.iter_selected().zip(mapping.value.pattern.attributes.iter()) {
                    if actual.assignment != expected.as_ref() {
                        return None;
                    }
                }
            }

            is_match.then_some(Match {
                pattern: &mapping.value.pattern,
                sequence_number: mapping.sequence_number,
            })
        });

        if res.is_none() && self.all_patterns_are_excluded {
            static MATCH_ALL_STAND_IN: Pattern = Pattern {
                path: BString::new(Vec::new()),
                signature: MagicSignature::empty(),
                search_mode: SearchMode::ShellGlob,
                attributes: Vec::new(),
                prefix_len: 0,
                nil: true,
            };
            Some(Match {
                pattern: &MATCH_ALL_STAND_IN,
                sequence_number: patterns_len,
            })
        } else {
            res
        }
    }
}

fn match_verbatim(
    mapping: &gix_glob::search::pattern::Mapping<Spec>,
    relative_path: &BStr,
    is_dir: bool,
    case: Case,
) -> bool {
    let pattern_len = mapping.value.pattern.path.len();
    let mut relative_path_ends_with_slash_at_pattern_len = false;
    let match_is_allowed = relative_path.get(pattern_len).map_or_else(
        || relative_path.len() == pattern_len,
        |b| {
            relative_path_ends_with_slash_at_pattern_len = *b == b'/';
            relative_path_ends_with_slash_at_pattern_len
        },
    );
    let pattern_requirement_is_met = !mapping.pattern.mode.contains(gix_glob::pattern::Mode::MUST_BE_DIR)
        || (relative_path_ends_with_slash_at_pattern_len || is_dir);

    if match_is_allowed && pattern_requirement_is_met {
        let dir_or_file = &relative_path[..mapping.value.pattern.path.len()];
        match case {
            Case::Sensitive => mapping.value.pattern.path == dir_or_file,
            Case::Fold => mapping.value.pattern.path.eq_ignore_ascii_case(dir_or_file),
        }
    } else {
        false
    }
}
