use git_features::progress::Progress;
use std::path::PathBuf;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Execute,
    Simulate,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Simulate
    }
}

pub fn run(_mode: Mode, _source_dir: PathBuf, _destination: PathBuf, _progress: impl Progress) -> anyhow::Result<()> {
    unimplemented!("organize")
}
