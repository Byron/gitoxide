use std::{io, path::Path, process::Command};

use crate::utils::Program;

pub struct Support {
    bat: Program,
}

impl Default for Support {
    fn default() -> Self {
        Self::new()
    }
}

impl Support {
    pub fn new() -> Self {
        Support {
            bat: Program::named("bat"),
        }
    }

    pub fn display_to_tty(&self, path: &Path, path_for_title: &Path) -> io::Result<()> {
        if !self.bat.found {
            log::info!(
                "Would want to use 'bat' for colored preview of '{}', but it wasn't available in the PATH.",
                path.display()
            );
            return Ok(());
        }
        if Command::new("bat")
            .args(&["--paging=always", "-l=md", "--file-name"])
            .arg(path_for_title)
            .arg(path)
            .status()?
            .success()
        {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "bat exited with an error"))
        }
    }
}
