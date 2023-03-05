pub use gix_diff::*;

///
pub mod rename {
    /// Determine how to do rename tracking.
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Tracking {
        /// Do not track renames at all, the fastest option.
        Disabled,
        /// Track renames.
        Renames,
        /// Track renames and copies.
        ///
        /// This is the most expensive option.
        RenamesAndCopies,
    }
}
