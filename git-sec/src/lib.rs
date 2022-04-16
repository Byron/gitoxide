#![deny(unsafe_code, rust_2018_idioms, missing_docs)]
//! A shared trust model for `gitoxide` crates.

/// which programs we may execute in order to run them.
pub mod execute {
    /// What kind of executables we can run
    pub enum Permission {
        /// The greatest permission level without any restrictions.
        ///
        /// The executables can be found in the PATH or configured from any git config file.
        ///
        /// Note that, however, some executables still won't be picked up from repository-local configuration
        /// for safety reasons.
        All,
        /// Only run executables if these have been configured by git config files that are owned by the user executing
        /// the application.
        IfUnderUserControl {
            /// If an executable is not under user control, instead of failing, fallback to a configuration setting that
            /// is or try to not fail by using suitable defaults if possible.
            allow_fallback: bool,
        },
    }
}

/// Various types to identify entities.
pub mod identity;
