use tracing_core::{dispatcher::get_default as with_dispatcher, span, span::Id, Dispatch};
// these are used later in macros.
pub use tracing_core::{field, metadata, Event, Metadata};

/// An entered span which will exit on drop.
#[derive(Clone)]
pub struct Span {
    id: Option<(Id, Dispatch, &'static Metadata<'static>)>,
}

impl Span {
    /// Create a new span.
    ///
    /// This constructor is typically invoked by the [`crate::span!`] macro.
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
                    id: Some((id, dispatch.clone(), meta)),
                }
            })
        }
    }

    /// Record a single `field` to take `value`.
    ///
    /// ### Panics
    ///
    /// If the field name wasn't mentioned when the span was created.
    pub fn record<V>(&self, field: &str, value: V) -> &Self
    where
        V: field::Value,
    {
        if let Some((_, _, meta)) = &self.id {
            let fields = meta.fields();
            let field = fields
                .field(field)
                .unwrap_or_else(|| panic!("Field name '{field}' must be registered at creation time."));
            self.record_all(&fields.value_set(&[(&field, Some(&value as &dyn field::Value))]));
        }
        self
    }

    fn record_all(&self, values: &field::ValueSet<'_>) -> &Self {
        if let Some((id, dispatch, _)) = &self.id {
            let record = span::Record::new(values);
            dispatch.record(id, &record);
        }
        self
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        if let Some((id, dispatch, _meta)) = self.id.take() {
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

/// A macro to create a span.
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

/// Create an event with the given level.
#[macro_export]
macro_rules! event {
    (target: $target:expr, $lvl:expr, { $($fields:tt)* } )=> (
        {
            static META: $crate::Metadata<'static> = {
                $crate::metadata! {
                    name: concat!(
                        "event ",
                        file!(),
                        ":",
                        line!()
                    ),
                    target: $target,
                    level: $lvl,
                    fields: $crate::fieldset!( $($fields)* ),
                    callsite: &$crate::MetaOnlyCallsite(&META),
                    kind: $crate::metadata::Kind::EVENT,
                }
            };
            $crate::Event::dispatch(
                &META,
                &$crate::valueset!(META.fields(), $($fields)*)
            );
        }
    );
    (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: $target,
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    );
    (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(target: $target, $lvl, { $($k).+ = $($fields)* })
    );
    (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $lvl, { $($arg)+ })
    );
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    );
    ($lvl:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+ = $($field)*}
        )
    );
    ($lvl:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { ?$($k).+, $($field)*}
        )
    );
    ($lvl:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { %$($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+) => (
        $crate::event!($lvl, ?$($k).+,)
    );
    ($lvl:expr, %$($k:ident).+) => (
        $crate::event!($lvl, %$($k).+,)
    );
    ($lvl:expr, $($k:ident).+) => (
        $crate::event!($lvl, $($k).+,)
    );
    ( $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: module_path!(), $lvl, { $($arg)+ })
    );
}

// Copied from`tracing`, would be nice to have it in `tracing-core`.
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
