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
mod enabled {
    use tracing_core::{dispatcher::get_default as with_dispatcher, span::Id, Dispatch};

    // these are used later in macros.
    pub use tracing_core::{field, metadata, Metadata};

    /// An entered span which will exit on drop.
    pub struct Span {
        id: Option<(Id, Dispatch)>,
    }

    impl Span {
        #[allow(missing_docs)]
        pub fn new(
            level: crate::Level,
            meta: &'static Metadata<'static>,
            values: &tracing_core::field::ValueSet<'_>,
        ) -> Self {
            if level > crate::MAX_LEVEL {
                Self { id: None }
            } else {
                with_dispatcher(|dispatch| {
                    let id = dispatch.new_span(&tracing_core::span::Attributes::new(meta, values));
                    dispatch.enter(&id);
                    Self {
                        id: Some((id, dispatch.clone())),
                    }
                })
            }
        }
    }

    impl Drop for Span {
        fn drop(&mut self) {
            if let Some((id, dispatch)) = self.id.take() {
                dispatch.exit(&id);
                dispatch.try_close(id);
            }
        }
    }

    #[doc(hidden)]
    pub struct MetaOnlyCallsite(pub &'static Metadata<'static>);

    impl tracing_core::callsite::Callsite for MetaOnlyCallsite {
        fn set_interest(&self, _: tracing_core::subscriber::Interest) {}

        fn metadata(&self) -> &Metadata<'_> {
            self.0
        }
    }

    #[doc(hidden)]
    impl crate::Level {
        pub const fn into_tracing_level(self) -> tracing_core::Level {
            match self {
                crate::Level::Coarse => tracing_core::Level::INFO,
                crate::Level::Detail => tracing_core::Level::DEBUG,
            }
        }
    }
}
#[cfg(feature = "tracing")]
pub use enabled::{MetaOnlyCallsite, Span};

#[cfg(feature = "tracing")]
#[doc(hidden)]
pub use enabled::{field, metadata, Metadata};

/// A macro to create a span.
#[cfg(feature = "tracing")]
#[macro_export]
macro_rules! span {
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        {
            static META: $crate::Metadata<'static> = {
                $crate::metadata! {
                    name: $name,
                    target: $target,
                    level: $lvl.into_tracing_level(),
                    fields: $crate::fieldset!( $($fields)* ),
                    callsite: &$crate::MetaOnlyCallsite(&META),
                    kind: $crate::metadata::Kind::SPAN,
                }
            };

            $crate::Span::new(
                $lvl,
                &META,
                &$crate::valueset!(META.fields(), $($fields)*),
            )
        }
    };
    (target: $target:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, $lvl, $name,)
    };
    ($lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
            $($fields)*
        )
    };
    ($lvl:expr, $name:expr) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
        )
    };
}

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

// Copied from`tracing`, would be nice to have it in `tracing-core`.
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! fieldset {
    // == base case ==
    (@ { $(,)* $($out:expr),* $(,)* } $(,)*) => {
        &[ $($out),* ]
    };

    // == recursive cases (more tts) ==
    (@ { $(,)* $($out:expr),* } $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, stringify!($($k).+) } $($rest)*)
    };
    // TODO(#1138): determine a new syntax for uninitialized span fields, and
    // re-enable this.
    // (@ { $($out:expr),* } $($k:ident).+ = _, $($rest:tt)*) => {
    //     $crate::fieldset!(@ { $($out),*, stringify!($($k).+) } $($rest)*)
    // };
    (@ { $(,)* $($out:expr),* } ?$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } %$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, stringify!($($k).+) } $($rest)*)
    };

    // Handle literal names
    (@ { $(,)* $($out:expr),* } $k:literal = ?$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $k } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $k:literal = %$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $k } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $k:literal = $val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $k } $($rest)*)
    };

    // Remainder is unparseable, but exists --- must be format args!
    (@ { $(,)* $($out:expr),* } $($rest:tt)+) => {
        $crate::fieldset!(@ { "message", $($out),*, })
    };

    // == entry ==
    ($($args:tt)*) => {
        $crate::fieldset!(@ { } $($args)*,)
    };
}

// Copied from`tracing`, would be nice to have it in `tracing-core`.
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! valueset {

    // === base case ===
    (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
        &[ $($val),* ]
    };

    // === recursive case (more tts) ===

    // TODO(#1138): determine a new syntax for uninitialized span fields, and
    // re-enable this.
    // (@{ $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = _, $($rest:tt)*) => {
    //     $crate::valueset!(@ { $($out),*, (&$next, None) }, $next, $($rest)*)
    // };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$val) as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$val) as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$($k).+ as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, ?$($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$($k).+) as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, %$($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$($k).+) as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = ?$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$val) as &dyn Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = %$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$val) as &dyn Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = $val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$($k).+ as &dyn Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, ?$($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$($k).+) as &dyn Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, %$($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$($k).+) as &dyn Value)) },
            $next,
        )
    };

    // Handle literal names
    (@ { $(,)* $($out:expr),* }, $next:expr, $k:literal = ?$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$val) as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $k:literal = %$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$val) as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $k:literal = $val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $k:literal = ?$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$val) as &dyn Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $k:literal = %$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$val) as &dyn Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $k:literal = $val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$val as &dyn Value)) },
            $next,
        )
    };

    // Remainder is unparsable, but exists --- must be format args!
    (@ { $(,)* $($out:expr),* }, $next:expr, $($rest:tt)+) => {
        $crate::valueset!(@ { (&$next, Some(&format_args!($($rest)+) as &dyn Value)), $($out),* }, $next, )
    };

    // === entry ===
    ($fields:expr, $($kvs:tt)+) => {
        {
            #[allow(unused_imports)]
            use $crate::field::{debug, display, Value};
            let mut iter = $fields.iter();
            $fields.value_set($crate::valueset!(
                @ { },
                iter.next().expect("FieldSet corrupted (this is a bug)"),
                $($kvs)+
            ))
        }
    };
    ($fields:expr,) => {
        {
            $fields.value_set(&[])
        }
    };
}

/// A macro to create a span.
#[cfg(not(feature = "tracing"))]
#[macro_export]
macro_rules! span {
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        ()
    };
    (target: $target:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, $lvl, $name,)
    };
    ($lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
            $($fields)*
        )
    };
    ($lvl:expr, $name:expr) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
        )
    };
}
