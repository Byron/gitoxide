use bitflags::bitflags;

// TODO: we essentially treat this as an enum withj the only exception being
// that `FILE_EXECUTABLE.contains(FILE)` works might want to turn this into an
// enum proper
bitflags! {
    /// The kind of file of an entry.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Mode: u32 {
        /// directory (only used for sparse checkouts), equivalent to a tree, which is _excluded_ from the index via
        /// cone-mode.
        const DIR = 0o040000;
        /// regular file
        const FILE = 0o100644;
        /// regular file, executable
        const FILE_EXECUTABLE = 0o100755;
        /// Symbolic link
        const SYMLINK = 0o120000;
        /// A git commit for submodules
        const COMMIT = 0o160000;
    }
}

#[cfg(unix)]
/// Returns whether a a file has the executable permission set
pub fn is_executable(metadata: &std::fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    (metadata.mode() & 0o100) != 0
}

#[cfg(not(unix))]
/// Returns whether a a file has the executable permission set
pub fn is_executable(_metadata: &std::fs::Metadata) -> bool {
    false
}

impl Mode {
    /// Return true if this is a sparse entry, as it points to a directory which usually isn't what an unsparse index tracks.
    pub fn is_sparse(&self) -> bool {
        *self == Self::DIR
    }

    /// Compares this mode to the file system version ([`std::fs::symlink_metadata`])
    /// and returns the change needed to update this mode to match the file if
    pub fn change_to_match_fs(
        self,
        stat: &std::fs::Metadata,
        has_symlinks: bool,
        executable_bit: bool,
    ) -> Option<Change> {
        match self {
            Mode::FILE if !stat.is_file() => (),
            Mode::SYMLINK if has_symlinks && !stat.is_symlink() => (),
            Mode::SYMLINK if !has_symlinks && !stat.is_file() => (),
            Mode::COMMIT | Mode::DIR if !stat.is_dir() => (),
            Mode::FILE if executable_bit && is_executable(stat) => return Some(Change::ExecutableBit),
            Mode::FILE_EXECUTABLE if executable_bit && !is_executable(stat) => return Some(Change::ExecutableBit),
            _ => return None,
        };
        let new_mode = if stat.is_dir() {
            Mode::DIR
        } else if executable_bit && is_executable(stat) {
            Mode::FILE_EXECUTABLE
        } else {
            Mode::FILE
        };
        Some(Change::Type { new_mode })
    }
}

/// A change of a [`Mode`]
pub enum Change {
    /// The type of mode changed (like symlink => file)
    Type {
        /// The mode representing the new index type
        new_mode: Mode,
    },
    /// The executable permission of this file has changed
    ExecutableBit,
}

impl Change {
    /// Applies this change to a `Mode`
    pub fn apply(self, mode: &mut Mode) {
        *mode = match self {
            Change::Type { new_mode } => new_mode,
            Change::ExecutableBit => match *mode {
                Mode::FILE => Mode::FILE_EXECUTABLE,
                Mode::FILE_EXECUTABLE => Mode::FILE,
                _ => unreachable!("invalid mode change: can't flip executable bit of {mode:?}"),
            },
        }
    }
}
