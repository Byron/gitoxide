use git_repository as git;
use std::ffi::OsString;

pub fn explain(
    _repo: git::Repository,
    _spec: OsString,
    mut _out: impl std::io::Write,
    mut _err: impl std::io::Write,
) -> anyhow::Result<()> {
    Ok(())
}
