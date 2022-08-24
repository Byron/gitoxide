use crate::Time;

/// Access
impl Time {
    /// Return true if this time has been initialized to anything non-default, i.e. 0.
    pub fn is_set(&self) -> bool {
        *self != Self::default()
    }

    /// Return the passed seconds since epoch since this signature was made.
    pub fn seconds(&self) -> u32 {
        self.seconds_since_unix_epoch
    }
}

/// Indicates if a number is positive or negative for use in [`Time`].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Sign {
    Plus,
    Minus,
}

/// Various ways to describe a time format.
#[derive(Debug, Clone, Copy)]
pub enum Format<'a> {
    /// A custom format typically defined with the [`format_description`][time::format_description] macro.
    Custom(&'a [time::format_description::FormatItem<'a>]),
    /// The seconds since 1970, also known as unix epoch, like `1660874655`.
    Unix,
    /// The seconds since 1970, followed by the offset, like `1660874655 +0800`
    Raw,
}

///
pub mod format;
mod init;
mod write;

mod sign {
    use crate::time::Sign;

    impl From<i32> for Sign {
        fn from(v: i32) -> Self {
            if v < 0 {
                Sign::Minus
            } else {
                Sign::Plus
            }
        }
    }
}

mod impls {
    use crate::time::Sign;
    use crate::Time;

    impl Default for Time {
        fn default() -> Self {
            Time {
                seconds_since_unix_epoch: 0,
                offset_in_seconds: 0,
                sign: Sign::Plus,
            }
        }
    }
}
