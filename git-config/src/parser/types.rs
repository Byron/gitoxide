macro_rules! generate_case_insensitive {
    ($name:ident, $cow_inner_type:ty, $comment:literal) => {
        #[doc = $comment]
        #[derive(Clone, Eq, Ord, Debug, Default)]
        pub struct $name<'a>(pub std::borrow::Cow<'a, $cow_inner_type>);

        impl $name<'_> {
            /// Coerces into an owned instance. This differs from the standard
            /// [`clone`] implementation as calling clone will _not_ copy the
            /// borrowed variant, while this method will. In other words:
            ///
            /// | Borrow type | `.clone()` | `to_owned()` |
            /// | ----------- | ---------- | ------------ |
            /// | Borrowed    | Borrowed   | Owned        |
            /// | Owned       | Owned      | Owned        |
            ///
            /// This can be most effectively seen by the differing lifetimes
            /// between the two. This method guarantees a `'static` lifetime,
            /// while `clone` does not.
            ///
            /// [`clone`]: Self::clone
            #[must_use]
            pub fn to_owned(&self) -> $name<'static> {
                $name(std::borrow::Cow::Owned(self.0.clone().into_owned()))
            }
        }

        impl PartialEq for $name<'_> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq_ignore_ascii_case(&other.0)
            }
        }

        impl std::fmt::Display for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl PartialOrd for $name<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0
                    .to_ascii_lowercase()
                    .partial_cmp(&other.0.to_ascii_lowercase())
            }
        }

        impl std::hash::Hash for $name<'_> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.to_ascii_lowercase().hash(state)
            }
        }

        impl<'a> From<&'a str> for $name<'a> {
            fn from(s: &'a str) -> Self {
                Self(std::borrow::Cow::Borrowed(s.into()))
            }
        }

        impl<'a> From<std::borrow::Cow<'a, bstr::BStr>> for $name<'a> {
            fn from(s: std::borrow::Cow<'a, bstr::BStr>) -> Self {
                Self(s)
            }
        }

        impl<'a> std::ops::Deref for $name<'a> {
            type Target = $cow_inner_type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
generate_case_insensitive!(
    SectionHeaderName,
    bstr::BStr,
    "Wrapper struct for section header names, since section headers are case-insensitive."
);

generate_case_insensitive!(
    Key,
    bstr::BStr,
    "Wrapper struct for key names, since keys are case-insensitive."
);
