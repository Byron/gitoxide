use std::path::Path;

use bstr::ByteSlice;

use crate::stack::Platform;

/// Access
impl<'a> Platform<'a> {
    /// The full path to `relative` will be returned for use on the file system.
    pub fn path(&self) -> &'a Path {
        self.parent.stack.current()
    }

    /// See if the currently set entry is excluded as per exclude and git-ignore files.
    ///
    /// # Panics
    ///
    /// If the cache was configured without exclude patterns.
    pub fn is_excluded(&self) -> bool {
        self.matching_exclude_pattern()
            .map_or(false, |m| !m.pattern.is_negative())
    }

    /// Check all exclude patterns to see if the currently set path matches any of them.
    ///
    /// Note that this pattern might be negated, and means this path in included.
    ///
    /// # Panics
    ///
    /// If the cache was configured without exclude patterns.
    pub fn matching_exclude_pattern(&self) -> Option<gix_ignore::search::Match<'_>> {
        let ignore = self.parent.state.ignore_or_panic();
        let relative_path =
            gix_path::to_unix_separators_on_windows(gix_path::into_bstr(self.parent.stack.current_relative()));
        ignore.matching_exclude_pattern(relative_path.as_bstr(), self.is_dir, self.parent.case)
    }

    /// Match all attributes at the current path and store the result in `out`, returning `true` if at least one attribute was found.
    ///
    /// # Panics
    ///
    /// If the cache was configured without attributes.
    #[cfg(feature = "attributes")]
    pub fn matching_attributes(&self, out: &mut gix_attributes::search::Outcome) -> bool {
        let attrs = self.parent.state.attributes_or_panic();
        let relative_path =
            gix_path::to_unix_separators_on_windows(gix_path::into_bstr(self.parent.stack.current_relative()));
        attrs.matching_attributes(relative_path.as_bstr(), self.parent.case, self.is_dir, out)
    }
}

impl<'a> std::fmt::Debug for Platform<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.path(), f)
    }
}
