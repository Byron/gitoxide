use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub explain: bool,
    pub cat_file: bool,
}

pub(crate) mod function {
    use anyhow::Context;
    use std::ffi::OsString;

    use git_repository as git;

    use super::Options;
    use crate::repository::revision;
    use crate::OutputFormat;

    pub fn resolve(
        mut repo: git::Repository,
        specs: Vec<OsString>,
        mut out: impl std::io::Write,
        Options {
            format,
            explain,
            cat_file,
        }: Options,
    ) -> anyhow::Result<()> {
        repo.object_cache_size_if_unset(1024 * 1024);

        match format {
            OutputFormat::Human => {
                for spec in specs {
                    if explain {
                        return revision::explain(spec, out);
                    }
                    let spec = git::path::os_str_into_bstr(&spec)?;
                    let spec = repo.rev_parse(spec)?;
                    if cat_file {
                        return display_object(spec, out);
                    }
                    writeln!(out, "{spec}", spec = spec.detach())?;
                }
            }
            #[cfg(feature = "serde1")]
            OutputFormat::Json => {
                if explain {
                    anyhow::bail!("Explanations are only for human consumption")
                }
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

    fn display_object(spec: git::revision::Spec<'_>, mut out: impl std::io::Write) -> anyhow::Result<()> {
        let id = spec.single().context("rev-spec must resolve to a single object")?;
        let object = id.object()?;
        match object.kind {
            git::object::Kind::Tree => {
                for entry in object.into_tree().iter() {
                    writeln!(out, "{}", entry?)?;
                }
            }
            _ => out.write_all(&object.data)?,
        }
        Ok(())
    }
}
