/// The path to the default TTY on linux
pub const TTY_PATH: &str = "/dev/tty";

#[cfg(unix)]
pub(crate) mod imp {
    use std::{
        io::{BufRead, Write},
        os::unix::io::{AsRawFd, RawFd},
    };

    use nix::sys::{termios, termios::Termios};
    use parking_lot::{const_mutex, lock_api::MutexGuard, Mutex, RawMutex};

    use crate::{unix::TTY_PATH, Error, Mode, Options};

    static TERM_STATE: Mutex<Option<Termios>> = const_mutex(None);

    /// Ask the user given a `prompt`, returning the result.
    pub(crate) fn ask(prompt: &str, Options { mode, .. }: &Options<'_>) -> Result<String, Error> {
        match mode {
            Mode::Disable => Err(Error::Disabled),
            Mode::Hidden => {
                let state = TERM_STATE.lock();
                let mut in_out = std::fs::OpenOptions::new().write(true).read(true).open(TTY_PATH)?;
                let restore = save_term_state_and_disable_echo(state, in_out.as_raw_fd())?;
                in_out.write_all(prompt.as_bytes())?;

                let mut buf_read = std::io::BufReader::with_capacity(64, in_out);
                let mut out = String::with_capacity(64);
                buf_read.read_line(&mut out)?;

                out.pop();
                if out.ends_with('\r') {
                    out.pop();
                }
                restore.now()?;
                Ok(out)
            }
            Mode::Visible => {
                let mut in_out = std::fs::OpenOptions::new().write(true).read(true).open(TTY_PATH)?;
                in_out.write_all(prompt.as_bytes())?;

                let mut buf_read = std::io::BufReader::with_capacity(64, in_out);
                let mut out = String::with_capacity(64);
                buf_read.read_line(&mut out)?;
                Ok(out.trim_end().to_owned())
            }
        }
    }

    type TermiosGuard<'a> = MutexGuard<'a, RawMutex, Option<Termios>>;

    struct RestoreTerminalStateOnDrop<'a> {
        state: TermiosGuard<'a>,
        fd: RawFd,
    }

    impl<'a> RestoreTerminalStateOnDrop<'a> {
        fn now(mut self) -> Result<(), Error> {
            let state = self.state.take().expect("BUG: we exist only if something is saved");
            termios::tcsetattr(self.fd, termios::SetArg::TCSAFLUSH, &state)?;
            Ok(())
        }
    }

    impl<'a> Drop for RestoreTerminalStateOnDrop<'a> {
        fn drop(&mut self) {
            if let Some(state) = self.state.take() {
                termios::tcsetattr(self.fd, termios::SetArg::TCSAFLUSH, &state).ok();
            }
        }
    }

    fn save_term_state_and_disable_echo(
        mut state: TermiosGuard<'_>,
        fd: RawFd,
    ) -> Result<RestoreTerminalStateOnDrop<'_>, Error> {
        assert!(
            state.is_none(),
            "BUG: recursive calls are not possible and we restore afterwards"
        );

        let prev = termios::tcgetattr(fd)?;
        let mut new = prev.clone();
        *state = prev.into();

        new.local_flags &= !termios::LocalFlags::ECHO;
        new.local_flags |= termios::LocalFlags::ECHONL;
        termios::tcsetattr(fd, termios::SetArg::TCSAFLUSH, &new)?;

        Ok(RestoreTerminalStateOnDrop { fd, state })
    }
}
