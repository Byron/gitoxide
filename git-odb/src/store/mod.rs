pub mod compound;
pub mod general;
pub mod linked;
pub mod loose;

/// Define how packs will be refreshed when all indices are loaded, which is useful if a lot of objects are missing.
#[derive(Clone, Copy)]
pub enum RefreshMode {
    /// Check for new or changed pack indices (and pack data files) when the last known index is loaded.
    /// During runtime we will keep pack indices stable by never reusing them, however, there is the option for
    /// clearing internal caches which is likely to change pack ids and it will trigger unloading of packs as they are missing on disk.
    AfterAllIndicesLoaded,
    /// Use this if you expect a lot of missing objects that shouldn't trigger refreshes even after all packs are loaded.
    /// This comes at the risk of not learning that the packs have changed in the mean time.
    Never,
}
