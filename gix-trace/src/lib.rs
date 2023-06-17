//! A crate providing macros for creating spans in various detail levels. `coarse!` shoudl be used for top-level operations, whereas
//! `detail!` should be used in plumbing crates unless their operations are likely to cost a lot of time.
//!
//! The application is supposed to explicitly turn on tracing via `gix-features`.
//! Crates that use `gix-features` should use `gix_features::trace`, and those who don't can use `gix_trace` directly.
//! ## Feature Flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(missing_docs, rust_2018_idioms, unsafe_code)]

/// The level at which the tracing item should be created.
///
/// It's used to filter items early.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Level {
    /// A coarse-grained trace level, one that should span entire operations with low frequency.
    Coarse = 1,
    /// Finer grained trace level that further subdivides coarse-level traces.
    ///
    /// Note that these should only be created for areas of the code which have significant cost.
    Detail = 2,
}

/// The maximum allowed level for tracing items, as compiled in.
#[cfg(feature = "tracing-detail")]
pub const MAX_LEVEL: Level = Level::Detail;
/// The maximum allowed level for tracing items, as compiled in.
#[cfg(not(feature = "tracing-detail"))]
pub const MAX_LEVEL: Level = Level::Coarse;

#[cfg(feature = "tracing")]
mod enabled;

#[cfg(feature = "tracing")]
pub use enabled::{field, Span};

impl Span {
    /// Execute `f` in with this span active, consuming it.
    pub fn into_scope<T>(self, f: impl FnOnce() -> T) -> T {
        f()
    }
}

#[cfg(feature = "tracing")]
#[doc(hidden)]
pub use enabled::{metadata, MetaOnlyCallsite, Metadata};

#[cfg(not(feature = "tracing"))]
mod disabled;
#[cfg(not(feature = "tracing"))]
pub use disabled::Span;

/// Create a new [coarse][Level::Coarse] span.
#[macro_export]
macro_rules! coarse {
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::Coarse,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::coarse!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::Coarse,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::coarse!($name,)};
}

/// Create a new [detail][Level::Detail] span.
#[macro_export]
macro_rules! detail {
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::Detail,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::detail!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::Detail,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::coarse!($name,)};
}
