use bitflags::bitflags;
bitflags! {
    /// The kind of file of an entry.
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

impl Mode {
    /// Return true if this is a sparse entry, as it points to a directory which usually isn't what an unsparse index tracks.
    pub fn is_sparse(&self) -> bool {
        *self == Self::DIR
    }
}
