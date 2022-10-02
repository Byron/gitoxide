use crate::OutputFormat;
use git::bstr::BString;
use git_repository as git;

pub struct Options {
    pub format: OutputFormat,
    pub dry_run: bool,
    pub remote: Option<String>,
    /// If non-empty, override all ref-specs otherwise configured in the remote
    pub ref_specs: Vec<BString>,
}

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

pub(crate) mod function {
    #![allow(unused_variables, unused_mut)]

    use super::Options;
    use crate::OutputFormat;
    use anyhow::bail;
    use git_repository as git;
    use git_repository::remote::fetch::Status;

    pub fn fetch(
        repo: git::Repository,
        mut progress: impl git::Progress,
        mut out: impl std::io::Write,
        err: impl std::io::Write,
        Options {
            format,
            dry_run,
            remote,
            ref_specs,
        }: Options,
    ) -> anyhow::Result<()> {
        if format != OutputFormat::Human {
            bail!("JSON output isn't yet supported for fetching.");
        }
        let mut remote = crate::repository::remote::by_name_or_url(&repo, remote.as_deref())?;

        let res: git::remote::fetch::Outcome<'_> = remote
            .connect(git::remote::Direction::Fetch, progress)?
            .prepare_fetch(Default::default())?
            .with_dry_run(dry_run)
            .receive(&git::interrupt::IS_INTERRUPTED)?;

        match res.status {
            Status::NoChange => crate::repository::remote::refs::print_refmap(
                &repo,
                remote.refspecs(git::remote::Direction::Fetch),
                res.ref_map,
                out,
                err,
            ),
            Status::Change { update_refs, .. } | Status::DryRun { update_refs } => todo!("change printing or dry-run"),
        }
    }
}
