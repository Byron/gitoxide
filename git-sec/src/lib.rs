#![deny(unsafe_code, rust_2018_idioms, missing_docs)]
//! A shared trust model for `gitoxide` crates.

///
pub mod path {
    /// Permissions related to _locations_ to executables, resources or destinations for operations.
    pub enum Permission {
        /// The greatest permission level without any restrictions, all _locations_ are permitted.
        ///
        /// For _locations_ to executables, it can be found in the `PATH` or configured from any git config file.
        ///
        /// Note that, however, some executables still won't be picked up from repository-local configuration
        /// for safety reasons.
        All,
        /// For _locations_ to executables, only run these if these have been configured by git config files
        /// that are owned by the user executing the application, or if these are in the `PATH`.
        /// Resources or write destinations adhere to the same rules.
        IfUnderUserControl {
            /// If true, if a _location_ is not under user control, instead of failing, fallback to a configuration setting that
            /// is or try to not fail by using suitable defaults. For executables this may mean to search for them in the `PATH`
            /// or fall back to another configuration value from configuration files under user control.
            allow_fallback: bool,
        },
        /// Do not use any _location_ unless it's required for git to function by using defaults.
        None {
            /// If true, and the _location_ is an executable, do not use any configured location but allow using default
            /// executables from the `PATH`.
            /// Otherwise any operation requiring an additional executable isn't allowed to proceed.
            allow_fallback: bool,
        },
    }
}

/// Various types to identify entities.
pub mod identity;
