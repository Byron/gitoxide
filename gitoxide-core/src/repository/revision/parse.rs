use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
}

pub(crate) mod function {
    use super::Options;
    use crate::OutputFormat;
    use git_repository as git;
    use std::ffi::OsString;

    pub fn parse(
        mut repo: git::Repository,
        specs: Vec<OsString>,
        mut out: impl std::io::Write,
        Options { format }: Options,
    ) -> anyhow::Result<()> {
        repo.object_cache_size_if_unset(1024 * 1024);

        match format {
            OutputFormat::Human => {
                for spec in specs {
                    let spec = git::path::os_str_into_bstr(&spec)?;
                    let spec = repo.rev_parse(spec)?.detach();
                    writeln!(out, "{spec}")?;
                }
            }
            #[cfg(feature = "serde1")]
            OutputFormat::Json => {
                serde_json::to_writer_pretty(
                    &mut out,
                    &specs
                        .into_iter()
                        .map(|spec| {
                            git::path::os_str_into_bstr(&spec)
                                .map_err(anyhow::Error::from)
                                .and_then(|spec| repo.rev_parse(spec).map_err(Into::into))
                                .map(|spec| spec.detach())
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )?;
            }
        }
        Ok(())
    }
}
