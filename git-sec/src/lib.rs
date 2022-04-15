#![deny(unsafe_code, rust_2018_idioms, missing_docs)]
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

    ///
    pub mod user_id {
        use crate::identity::UserId;
        use std::borrow::Cow;
        use std::path::Path;

        /// Obtain the owner of the given `path`.
        pub fn from_path(path: Cow<'_, Path>) -> std::io::Result<UserId> {
            impl_::from_path(path)
        }

        /// Obtain the of the currently running process.
        pub fn from_process() -> Result<UserId, from_process::Error> {
            impl_::from_process()
        }

        ///
        pub mod from_process {
            use crate::identity::user_id::impl_;

            /// The error returned by [from_process()][super::from_process()].
            pub type Error = impl_::FromProcessError;
        }

        #[cfg(not(windows))]
        mod impl_ {
            use crate::identity::UserId;
            use std::borrow::Cow;
            use std::path::Path;

            pub fn from_path(path: Cow<'_, Path>) -> std::io::Result<UserId> {
                use std::os::unix::fs::MetadataExt;
                let meta = std::fs::symlink_metadata(path)?;
                Ok(meta.uid())
            }

            pub type FromProcessError = std::convert::Infallible;
            pub fn from_process() -> Result<UserId, FromProcessError> {
                // SAFETY: there is no documented possibility for failure
                #[allow(unsafe_code)]
                let uid = unsafe { libc::geteuid() };
                Ok(uid)
            }
        }

        #[cfg(windows)]
        mod impl_ {
            use crate::identity::UserId;
            use std::borrow::Cow;
            use std::path::Path;

            pub fn from_path(path: Cow<'_, Path>) -> std::io::Result<UserId> {
                todo!("unix")
            }
            pub fn from_process() -> std::io::Result<UserId> {
                todo!("process")
            }
        }
    }
}
