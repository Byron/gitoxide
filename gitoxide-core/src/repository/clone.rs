use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub bare: bool,
    pub handshake_info: bool,
}

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

pub(crate) mod function {
    use anyhow::bail;
    use git_repository as git;
    use git_repository::remote::fetch::Status;
    use std::ffi::OsStr;

    use super::Options;
    use crate::repository::fetch::function::print_updates;
    use crate::OutputFormat;

    pub fn clone(
        remote: impl AsRef<OsStr>,
        directory: impl AsRef<std::path::Path>,
        mut progress: impl git::Progress,
        mut out: impl std::io::Write,
        err: impl std::io::Write,
        Options {
            format,
            handshake_info,
            bare,
        }: Options,
    ) -> anyhow::Result<()> {
        if format != OutputFormat::Human {
            bail!("JSON output isn't yet supported for fetching.");
        }

        let mut prepare = git::clone::PrepareFetch::new(
            remote.as_ref(),
            directory,
            git::create::Options {
                bare,
                fs_capabilities: None,
            },
            {
                let mut opts = git::open::Options::default();
                opts.permissions.config.git_binary = true;
                opts
            },
        )?;
        let (mut checkout, fetch_outcome) =
            prepare.fetch_then_checkout(progress.add_child("fetch"), &git::interrupt::IS_INTERRUPTED)?;

        if handshake_info {
            writeln!(out, "Handshake Information")?;
            writeln!(out, "\t{:?}", fetch_outcome.ref_map.handshake)?;
        }

        match fetch_outcome.status {
            Status::NoChange => {
                unreachable!("clone always has changes")
            }
            Status::DryRun { .. } => unreachable!("dry-run unsupported"),
            Status::Change {
                update_refs,
                write_pack_bundle,
            } => {
                let repo = checkout.repo();
                let remote = repo
                    .find_default_remote(git::remote::Direction::Fetch)
                    .expect("one origin remote")?;
                let ref_specs = remote.refspecs(git::remote::Direction::Fetch);
                print_updates(
                    checkout.repo(),
                    update_refs,
                    ref_specs,
                    fetch_outcome.ref_map,
                    &mut out,
                    err,
                )?;
                if let Some(data_path) = write_pack_bundle.data_path {
                    writeln!(out, "pack  file: \"{}\"", data_path.display()).ok();
                }
                if let Some(index_path) = write_pack_bundle.index_path {
                    writeln!(out, "index file: \"{}\"", index_path.display()).ok();
                }
            }
        };

        let repo = if bare {
            checkout.persist()
        } else {
            checkout.main_worktree(progress.add_child("clone"), &git::interrupt::IS_INTERRUPTED)?
        };
        writeln!(
            out,
            "clone (bare = {}) successful at \"{}\"",
            bare,
            repo.work_dir().unwrap_or_else(|| repo.git_dir()).display()
        )?;
        Ok(())
    }
}
