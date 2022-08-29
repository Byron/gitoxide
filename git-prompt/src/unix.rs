/// The path to the default TTY on linux
pub const TTY_PATH: &str = "/dev/tty";

#[cfg(unix)]
pub(crate) mod imp {
    use crate::unix::TTY_PATH;
    use crate::{Error, Options};
    use bstr::{BStr, BString};
    use std::io::Write;

    /// Ask the user given a `prompt`, returning the result.
    pub fn ask(prompt: &BStr, Options { secret }: Options) -> Result<BString, Error> {
        if secret {
            todo!("hide input")
        } else {
            let mut out = std::fs::OpenOptions::new()
                .write(true)
                .open(TTY_PATH)
                .map_err(|err| Error::TtyWrite { source: err })?;
            out.write_all(prompt).map_err(|err| Error::TtyWrite { source: err })?;
        }
        todo!("read line back")
    }
}
