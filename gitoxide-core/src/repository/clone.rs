use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub bare: bool,
    pub handshake_info: bool,
}

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=4;

pub(crate) mod function {
    use anyhow::bail;
    use git_repository as git;
    use git_repository::remote::fetch::Status;
    use git_repository::Progress;
    use std::ffi::OsStr;

    use super::Options;
    use crate::repository::fetch::function::print_updates;
    use crate::OutputFormat;

    pub fn clone<P>(
        remote: impl AsRef<OsStr>,
        directory: impl AsRef<std::path::Path>,
        mut progress: P,
        mut out: impl std::io::Write,
        err: impl std::io::Write,
        Options {
            format,
            handshake_info,
            bare,
        }: Options,
    ) -> anyhow::Result<()>
    where
        P: Progress,
        P::SubProgress: 'static,
    {
        if format != OutputFormat::Human {
            bail!("JSON output isn't yet supported for fetching.");
        }

        let mut prepare = git::clone::PrepareFetch::new(
            remote.as_ref(),
            directory,
            bare.then(|| git::create::Kind::Bare)
                .unwrap_or(git::create::Kind::WithWorktree),
            git::create::Options::default(),
            {
                let mut opts = git::open::Options::default();
                opts.permissions.config.git_binary = true;
                opts
            },
        )?;
        let (mut checkout, fetch_outcome) =
            prepare.fetch_then_checkout(&mut progress, &git::interrupt::IS_INTERRUPTED)?;

        let repo = if bare {
            checkout.persist()
        } else {
            checkout.main_worktree(progress, &git::interrupt::IS_INTERRUPTED)?
        };

        if handshake_info {
            writeln!(out, "Handshake Information")?;
            writeln!(out, "\t{:?}", fetch_outcome.ref_map.handshake)?;
        }

        match fetch_outcome.status {
            Status::NoChange => {
                unreachable!("clone always has changes")
            }
            Status::DryRun { .. } => unreachable!("dry-run unsupported"),
            Status::Change { update_refs, .. } => {
                let remote = repo
                    .find_default_remote(git::remote::Direction::Fetch)
                    .expect("one origin remote")?;
                let ref_specs = remote.refspecs(git::remote::Direction::Fetch);
                print_updates(&repo, update_refs, ref_specs, fetch_outcome.ref_map, &mut out, err)?;
            }
        };
        Ok(())
    }
}
