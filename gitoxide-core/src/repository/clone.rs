use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub bare: bool,
    pub handshake_info: bool,
    pub no_tags: bool,
}

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

pub(crate) mod function {
    use std::ffi::OsStr;

    use anyhow::bail;
    use git_repository as git;
    use git_repository::{bstr::BString, remote::fetch::Status, Progress};

    use super::Options;
    use crate::{repository::fetch::function::print_updates, OutputFormat};

    pub fn clone<P>(
        remote: impl AsRef<OsStr>,
        directory: impl AsRef<std::path::Path>,
        overrides: Vec<BString>,
        mut progress: P,
        mut out: impl std::io::Write,
        mut err: impl std::io::Write,
        Options {
            format,
            handshake_info,
            bare,
            no_tags,
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
                let mut opts = git::open::Options::default().config_overrides(overrides);
                opts.permissions.config.git_binary = true;
                opts
            },
        )?;
        if no_tags {
            prepare = prepare.configure_remote(|r| Ok(r.with_fetch_tags(git::remote::fetch::Tags::None)));
        }
        let (mut checkout, fetch_outcome) =
            prepare.fetch_then_checkout(&mut progress, &git::interrupt::IS_INTERRUPTED)?;

        let (repo, outcome) = if bare {
            (checkout.persist(), None)
        } else {
            let (repo, outcome) = checkout.main_worktree(progress, &git::interrupt::IS_INTERRUPTED)?;
            (repo, Some(outcome))
        };

        if handshake_info {
            writeln!(out, "Handshake Information")?;
            writeln!(out, "\t{:?}", fetch_outcome.ref_map.handshake)?;
        }

        match fetch_outcome.status {
            Status::NoPackReceived { .. } => {
                unreachable!("clone always has changes")
            }
            Status::DryRun { .. } => unreachable!("dry-run unsupported"),
            Status::Change { update_refs, .. } => {
                let remote = repo
                    .find_default_remote(git::remote::Direction::Fetch)
                    .expect("one origin remote")?;
                let ref_specs = remote.refspecs(git::remote::Direction::Fetch);
                print_updates(&repo, update_refs, ref_specs, fetch_outcome.ref_map, &mut out, &mut err)?;
            }
        };

        if let Some(git::worktree::index::checkout::Outcome { collisions, errors, .. }) = outcome {
            if !(collisions.is_empty() && errors.is_empty()) {
                let mut messages = Vec::new();
                if !errors.is_empty() {
                    messages.push(format!("kept going through {} errors(s)", errors.len()));
                    for record in errors {
                        writeln!(err, "{}: {}", record.path, record.error).ok();
                    }
                }
                if !collisions.is_empty() {
                    messages.push(format!("encountered {} collision(s)", collisions.len()));
                    for col in collisions {
                        writeln!(err, "{}: collision ({:?})", col.path, col.error_kind).ok();
                    }
                }
                bail!(
                    "One or more errors occurred - checkout is incomplete: {}",
                    messages.join(", ")
                );
            }
        }
        Ok(())
    }
}
