pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 0..=3;

pub struct Engine<P> {
    progress: P,
    con: rusqlite::Connection,
    gitoxide_version: String,
}

pub struct RunOutcome {
    /// the relative path to the repositories that could not be found on disk
    pub missing_repos_rela_paths: usize,
}

pub(crate) mod db;
pub(crate) mod engine;
