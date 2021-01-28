/// A color and its representations within a git config file
#[allow(missing_docs)]
pub enum Color {
    Red,
    BrightRed,
    /// Any color as indicated by 8 bit red, green and blue channels.
    Ansi {
        r: u8,
        g: u8,
        c: u8,
    },
}

///
pub mod resolve {
    use bstr::BStr;
    use quick_error::quick_error;
    use std::path::PathBuf;

    quick_error! {
        /// An error returned by [`path()`].
        #[allow(missing_docs)]
        #[derive(Debug)]
        pub enum Error {
            Tbd {
                display("TBD")
            }
        }
    }
    /// Git-config paths can contain `~` and more, see [the git documentation](https://github.com/git/git/blob/e67fbf927dfdf13d0b21dc6ea15dc3c7ef448ea0/Documentation/config.txt#L295:L295)
    /// on what needs to be supported.
    pub fn path(_value: &BStr) -> Result<PathBuf, Error> {
        unimplemented!("resolve::path")
    }
}
