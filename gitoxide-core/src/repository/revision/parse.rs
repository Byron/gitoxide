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
        repo: git::Repository,
        spec: OsString,
        mut out: impl std::io::Write,
        Options { format }: Options,
    ) -> anyhow::Result<()> {
        let spec = git::path::os_str_into_bstr(&spec)?;
        let spec = repo.rev_parse(spec)?.detach();

        match format {
            OutputFormat::Human => {
                writeln!(out, "{spec:?}")?;
            }
            #[cfg(feature = "serde1")]
            OutputFormat::Json => {
                serde_json::to_writer_pretty(&mut out, &spec)?;
            }
        }
        Ok(())
    }
}
