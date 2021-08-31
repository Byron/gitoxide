///
pub mod tz {
    mod error {
        use std::fmt;

        /// The error returned by [`offset()`]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Error;

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("The system's UTC offset could not be determined")
            }
        }

        impl std::error::Error for Error {}
    }
    pub use error::Error;

    /// The UTC offset in seconds
    pub type UTCOffsetInSeconds = i32;

    /// Return time offset in seconds from UTC based on the current timezone.
    ///
    /// Note that there may be various legitimate reasons for failure, which should be accounted for.
    pub fn current_utc_offset() -> Result<UTCOffsetInSeconds, Error> {
        // TODO: make this work without cfg(unsound_local_offset), see
        //       https://github.com/time-rs/time/issues/293#issuecomment-909158529
        // TODO: get a function to return the current time as well to avoid double-lookups
        //       (to get the offset, the current time is needed)
        time::UtcOffset::current_local_offset()
            .map(|ofs| ofs.whole_seconds())
            .map_err(|_| Error)
    }
}
