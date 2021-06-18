/// The amount of retries to do during various aspects of the directory removal.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Retries {
    /// How many directories can be deleted in total. 1 means only the target directory itself can be created and
    /// not a single parent directory.
    /// Note that this also counts towards retries needed to combat racy behaviour from other
    /// processes trying to delete empty directories.
    pub on_create_directory: usize,
    /// How often to retry if an interrupt happens.
    pub on_interrupt: usize,
}

impl Default for Retries {
    fn default() -> Self {
        Retries {
            on_interrupt: 10,
            on_create_directory: 100,
        }
    }
}
