use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub explain: bool,
    pub cat_file: bool,
    pub tree_mode: TreeMode,
    pub blob_format: BlobFormat,
    pub show_reference: bool,
}

pub enum TreeMode {
    Raw,
    Pretty,
}

#[derive(Copy, Clone)]
pub enum BlobFormat {
    Git,
    Worktree,
    Diff,
    DiffOrGit,
}

pub(crate) mod function {
    use std::ffi::OsString;

    use anyhow::{anyhow, Context};
    use gix::diff::blob::ResourceKind;
    use gix::filter::plumbing::driver::apply::Delay;
    use gix::revision::Spec;

    use super::Options;
    use crate::repository::revision::resolve::BlobFormat;
    use crate::{
        repository::{revision, revision::resolve::TreeMode},
        OutputFormat,
    };

    pub fn resolve(
        mut repo: gix::Repository,
        specs: Vec<OsString>,
        mut out: impl std::io::Write,
        Options {
            format,
            explain,
            cat_file,
            tree_mode,
            blob_format,
            show_reference,
        }: Options,
    ) -> anyhow::Result<()> {
        repo.object_cache_size_if_unset(1024 * 1024);
        let mut cache = (!matches!(blob_format, BlobFormat::Git))
            .then(|| {
                repo.diff_resource_cache(
                    match blob_format {
                        BlobFormat::Git => {
                            unreachable!("checked before")
                        }
                        BlobFormat::Worktree | BlobFormat::Diff => {
                            gix::diff::blob::pipeline::Mode::ToWorktreeAndBinaryToText
                        }
                        BlobFormat::DiffOrGit => gix::diff::blob::pipeline::Mode::ToGitUnlessBinaryToTextIsPresent,
                    },
                    Default::default(),
                )
            })
            .transpose()?;

        match format {
            OutputFormat::Human => {
                for spec in specs {
                    if explain {
                        return revision::explain(spec, out);
                    }
                    let spec = gix::path::os_str_into_bstr(&spec)?;
                    let spec = repo.rev_parse(spec)?;
                    if cat_file {
                        return display_object(&repo, spec, tree_mode, cache.as_mut().map(|c| (blob_format, c)), out);
                    }
                    if let Some(r) = spec.first_reference().filter(|_| show_reference) {
                        writeln!(out, "{}", r.name)?;
                    }
                    if let Some(r) = spec.second_reference().filter(|_| show_reference) {
                        writeln!(out, "{}", r.name)?;
                    }
                    writeln!(out, "{spec}", spec = spec.detach())?;
                }
            }
            #[cfg(feature = "serde")]
            OutputFormat::Json => {
                if explain {
                    anyhow::bail!("Explanations are only for human consumption")
                }
                serde_json::to_writer_pretty(
                    &mut out,
                    &specs
                        .into_iter()
                        .map(|spec| {
                            gix::path::os_str_into_bstr(&spec)
                                .map_err(anyhow::Error::from)
                                .and_then(|spec| repo.rev_parse(spec).map_err(Into::into))
                                .map(Spec::detach)
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )?;
            }
        }
        Ok(())
    }

    fn display_object(
        repo: &gix::Repository,
        spec: Spec<'_>,
        tree_mode: TreeMode,
        cache: Option<(BlobFormat, &mut gix::diff::blob::Platform)>,
        mut out: impl std::io::Write,
    ) -> anyhow::Result<()> {
        let id = spec.single().context("rev-spec must resolve to a single object")?;
        let header = id.header()?;
        match header.kind() {
            gix::object::Kind::Tree if matches!(tree_mode, TreeMode::Pretty) => {
                for entry in id.object()?.into_tree().iter() {
                    writeln!(out, "{}", entry?)?;
                }
            }
            gix::object::Kind::Blob if cache.is_some() && spec.path_and_mode().is_some() => {
                let (path, mode) = spec.path_and_mode().expect("is present");
                match cache.expect("is some") {
                    (BlobFormat::Git, _) => unreachable!("no need for a cache when querying object db"),
                    (BlobFormat::Worktree, cache) => {
                        let platform = cache.attr_stack.at_entry(path, Some(mode.into()), &repo.objects)?;
                        let object = id.object()?;
                        let mut converted = cache.filter.worktree_filter.convert_to_worktree(
                            &object.data,
                            path,
                            &mut |_path, attrs| {
                                let _ = platform.matching_attributes(attrs);
                            },
                            Delay::Forbid,
                        )?;
                        std::io::copy(&mut converted, &mut out)?;
                    }
                    (BlobFormat::Diff | BlobFormat::DiffOrGit, cache) => {
                        cache.set_resource(id.detach(), mode.kind(), path, ResourceKind::OldOrSource, &repo.objects)?;
                        let resource = cache.resource(ResourceKind::OldOrSource).expect("just set");
                        let data = resource
                            .data
                            .as_slice()
                            .ok_or_else(|| anyhow!("Binary data at {} cannot be diffed", path))?;
                        out.write_all(data)?;
                    }
                }
            }
            _ => out.write_all(&id.object()?.data)?,
        }
        Ok(())
    }
}
