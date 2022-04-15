#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, missing_docs)]
//! A shared trust model for `gitoxide` crates.

/// Various types to identify entities.
pub mod identity {
    /// A unix user id as obtained from the file system.
    #[cfg(not(windows))]
    pub type UserId = u32;

    /// A windows [security identifier](https://docs.microsoft.com/en-us/windows/security/identity-protection/access-control/security-identifiers)
    /// in its stringified form.
    #[cfg(windows)]
    pub type UserId = String;

    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    /// An account based identity
    pub struct Account {
        /// The user's name
        pub username: String,
        /// The user's password
        pub password: String,
    }
}
