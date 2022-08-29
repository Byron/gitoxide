/// The path to the default TTY on linux
pub const TTY_PATH: &str = "/dev/tty";

#[cfg(unix)]
pub(crate) mod imp {
    use crate::unix::TTY_PATH;
    use crate::{Error, Options};
    use std::io::Write;

    /// Ask the user given a `prompt`, returning the result.
    pub fn ask(prompt: &str, Options { secret }: Options) -> Result<String, Error> {
        if secret {
            todo!("hide input")
        } else {
            let mut in_out = std::fs::OpenOptions::new().write(true).read(true).open(TTY_PATH)?;
            in_out.write_all(prompt.as_bytes())?;

            use std::io::BufRead;
            let mut buf_read = std::io::BufReader::with_capacity(64, in_out);
            let mut out = String::with_capacity(64);
            buf_read.read_line(&mut out)?;
            Ok(out.trim_end().to_owned())
        }
    }
}
