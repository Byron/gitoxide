use std::path::Path;

use crate::{cache::state::IgnoreMatchGroup, PathIdMapping};
use bstr::{BStr, BString, ByteSlice};
use gix_glob::pattern::Case;

/// State related to the exclusion of files.
#[derive(Default, Clone)]
#[allow(unused)]
pub struct Ignore {
    /// Ignore patterns passed as overrides to everything else, typically passed on the command-line and the first patterns to
    /// be consulted.
    overrides: IgnoreMatchGroup,
    /// Ignore patterns that match the currently set director (in the stack), which is pushed and popped as needed.
    stack: IgnoreMatchGroup,
    /// Ignore patterns which aren't tied to the repository root, hence are global. They are consulted last.
    globals: IgnoreMatchGroup,
    /// A matching stack of pattern indices which is empty if we have just been initialized to indicate that the
    /// currently set directory had a pattern matched. Note that this one could be negated.
    /// (index into match groups, index into list of pattern lists, index into pattern list)
    matched_directory_patterns_stack: Vec<Option<(usize, usize, usize)>>,
    ///  The name of the file to look for in directories.
    pub(crate) exclude_file_name_for_directories: BString,
    /// The case to use when matching directories as they are pushed onto the stack. We run them against the exclude engine
    /// to know if an entire path can be ignored as a parent directory is ignored.
    pub(crate) case: Case,
}

impl Ignore {
    /// The `exclude_file_name_for_directories` is an optional override for the filename to use when checking per-directory
    /// ignore files within the repository, defaults to`.gitignore`.
    ///
    // This is what it should be able represent: https://github.com/git/git/blob/140b9478dad5d19543c1cb4fd293ccec228f1240/dir.c#L3354
    // TODO: more docs
    pub fn new(
        overrides: IgnoreMatchGroup,
        globals: IgnoreMatchGroup,
        exclude_file_name_for_directories: Option<&BStr>,
        case: Case,
    ) -> Self {
        Ignore {
            case,
            overrides,
            globals,
            stack: Default::default(),
            matched_directory_patterns_stack: Vec::with_capacity(6),
            exclude_file_name_for_directories: exclude_file_name_for_directories
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| ".gitignore".into()),
        }
    }
}

impl Ignore {
    pub(crate) fn pop_directory(&mut self) {
        self.matched_directory_patterns_stack.pop().expect("something to pop");
        self.stack.patterns.pop().expect("something to pop");
    }
    /// The match groups from lowest priority to highest.
    pub(crate) fn match_groups(&self) -> [&IgnoreMatchGroup; 3] {
        [&self.globals, &self.stack, &self.overrides]
    }

    pub(crate) fn matching_exclude_pattern(
        &self,
        relative_path: &BStr,
        is_dir: Option<bool>,
        case: Case,
    ) -> Option<gix_ignore::search::Match<'_, ()>> {
        let groups = self.match_groups();
        let mut dir_match = None;
        if let Some((source, mapping)) = self
            .matched_directory_patterns_stack
            .iter()
            .rev()
            .filter_map(|v| *v)
            .map(|(gidx, plidx, pidx)| {
                let list = &groups[gidx].patterns[plidx];
                (list.source.as_deref(), &list.patterns[pidx])
            })
            .next()
        {
            let match_ = gix_ignore::search::Match {
                pattern: &mapping.pattern,
                value: &mapping.value,
                sequence_number: mapping.sequence_number,
                source,
            };
            if mapping.pattern.is_negative() {
                dir_match = Some(match_);
            } else {
                // Note that returning here is wrong if this pattern _was_ preceded by a negative pattern that
                // didn't match the directory, but would match now.
                // Git does it similarly so we do too even though it's incorrect.
                // To fix this, one would probably keep track of whether there was a preceding negative pattern, and
                // if so we check the path in full and only use the dir match if there was no match, similar to the negative
                // case above whose fix fortunately won't change the overall result.
                return match_.into();
            }
        }
        groups
            .iter()
            .rev()
            .find_map(|group| group.pattern_matching_relative_path(relative_path, is_dir, case))
            .or(dir_match)
    }

    /// Like `matching_exclude_pattern()` but without checking if the current directory is excluded.
    /// It returns a triple-index into our data structure from which a match can be reconstructed.
    pub(crate) fn matching_exclude_pattern_no_dir(
        &self,
        relative_path: &BStr,
        is_dir: Option<bool>,
        case: Case,
    ) -> Option<(usize, usize, usize)> {
        let groups = self.match_groups();
        groups.iter().enumerate().rev().find_map(|(gidx, group)| {
            let basename_pos = relative_path.rfind(b"/").map(|p| p + 1);
            group
                .patterns
                .iter()
                .enumerate()
                .rev()
                .find_map(|(plidx, pl)| {
                    gix_ignore::search::pattern_idx_matching_relative_path(
                        pl,
                        relative_path,
                        basename_pos,
                        is_dir,
                        case,
                    )
                    .map(|idx| (plidx, idx))
                })
                .map(|(plidx, pidx)| (gidx, plidx, pidx))
        })
    }

    pub(crate) fn push_directory<Find, E>(
        &mut self,
        root: &Path,
        dir: &Path,
        buf: &mut Vec<u8>,
        id_mappings: &[PathIdMapping],
        mut find: Find,
    ) -> std::io::Result<()>
    where
        Find: for<'b> FnMut(&gix_hash::oid, &'b mut Vec<u8>) -> Result<gix_object::BlobRef<'b>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        let dir_bstr = gix_path::into_bstr(dir);
        let mut rela_dir = gix_glob::search::pattern::strip_base_handle_recompute_basename_pos(
            gix_path::into_bstr(root).as_ref(),
            dir_bstr.as_ref(),
            None,
            self.case,
        )
        .expect("dir in root")
        .0;
        if rela_dir.starts_with(b"/") {
            rela_dir = &rela_dir[1..];
        }
        self.matched_directory_patterns_stack
            .push(self.matching_exclude_pattern_no_dir(rela_dir, Some(true), self.case));

        let ignore_path_relative =
            gix_path::to_unix_separators_on_windows(gix_path::join_bstr_unix_pathsep(rela_dir, ".gitignore"));
        let ignore_file_in_index = id_mappings.binary_search_by(|t| t.0.as_bstr().cmp(ignore_path_relative.as_ref()));
        let follow_symlinks = ignore_file_in_index.is_err();
        if !gix_glob::search::add_patterns_file(
            &mut self.stack.patterns,
            dir.join(".gitignore"),
            follow_symlinks,
            Some(root),
            buf,
        )? {
            match ignore_file_in_index {
                Ok(idx) => {
                    let ignore_blob = find(&id_mappings[idx].1, buf)
                        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
                    let ignore_path = gix_path::from_bstring(ignore_path_relative.into_owned());
                    self.stack
                        .add_patterns_buffer(ignore_blob.data, ignore_path, Some(Path::new("")));
                }
                Err(_) => {
                    // Need one stack level per component so push and pop matches.
                    self.stack.patterns.push(Default::default())
                }
            }
        }
        Ok(())
    }
}
