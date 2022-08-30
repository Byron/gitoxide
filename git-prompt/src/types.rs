use std::path::Path;

/// The error returned by [ask()].
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Terminal prompts are disabled")]
    Disabled,
    #[error("The current platform has no implementation for prompting in the terminal")]
    UnsupportedPlatform,
    #[error(
        "Failed to open terminal at {:?} for writing prompt, or to write it",
        crate::unix::TTY_PATH
    )]
    TtyIo(#[from] std::io::Error),
    #[cfg(unix)]
    #[error("Failed to obtain or set terminal configuration")]
    TerminalConfiguration(#[from] nix::errno::Errno),
}

/// The way the user is prompted.
#[derive(Copy, Clone)]
pub enum Mode {
    /// Visibly show user input.
    Visible,
    /// Do not show user input, suitable for sensitive data.
    Hidden,
    /// Do not prompt the user at all but rather abort with an error. This is useful in conjunction with [Option::askpass].
    Disable,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Hidden
    }
}

/// The options used in `[ask()]`.
#[derive(Default, Copy, Clone)]
pub struct Options<'a> {
    /// The path or name (for lookup in `PATH`) to the askpass program to call before prompting the user.
    ///
    /// It's called like this `askpass <prompt>`, but note that it won't know if the input should be hidden or not.
    pub askpass: Option<&'a Path>,
    /// The way the user is prompted.
    pub mode: Mode,
}
