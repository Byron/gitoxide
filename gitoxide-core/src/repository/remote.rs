#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod refs_impl {
    use super::by_name_or_url;
    use crate::OutputFormat;
    use anyhow::bail;
    use git_repository as git;
    use git_repository::{
        protocol::fetch,
        refspec::{match_group::validate::Fix, RefSpec},
    };

    pub mod refs {
        use git_repository::bstr::BString;

        use crate::OutputFormat;

        pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

        pub enum Kind {
            Remote,
            Tracking { ref_specs: Vec<BString> },
        }

        pub struct Options {
            pub format: OutputFormat,
            pub name_or_url: Option<String>,
            pub handshake_info: bool,
        }

        pub(crate) use super::{print, print_refmap};
    }

    #[git::protocol::maybe_async::maybe_async]
    pub async fn refs_fn(
        repo: git::Repository,
        kind: refs::Kind,
        mut progress: impl git::Progress,
        mut out: impl std::io::Write,
        err: impl std::io::Write,
        refs::Options {
            format,
            name_or_url,
            handshake_info,
        }: refs::Options,
    ) -> anyhow::Result<()> {
        use anyhow::Context;
        let mut remote = by_name_or_url(&repo, name_or_url.as_deref())?;
        if let refs::Kind::Tracking { ref_specs, .. } = &kind {
            if format != OutputFormat::Human {
                bail!("JSON output isn't yet supported for listing ref-mappings.");
            }
            if !ref_specs.is_empty() {
                remote.replace_refspecs(ref_specs.iter(), git::remote::Direction::Fetch)?;
            }
        }
        progress.info(format!(
            "Connecting to {:?}",
            remote
                .url(git::remote::Direction::Fetch)
                .context("Remote didn't have a URL to connect to")?
                .to_bstring()
        ));
        let map = remote
            .connect(git::remote::Direction::Fetch, progress)
            .await?
            .ref_map(git::remote::ref_map::Options {
                prefix_from_spec_as_filter_on_remote: !matches!(kind, refs::Kind::Remote),
                ..Default::default()
            })
            .await?;

        if handshake_info {
            writeln!(out, "Handshake Information")?;
            writeln!(out, "\t{:?}", map.handshake)?;
        }
        match kind {
            refs::Kind::Tracking { .. } => {
                print_refmap(&repo, remote.refspecs(git::remote::Direction::Fetch), map, out, err)
            }
            refs::Kind::Remote => {
                match format {
                    OutputFormat::Human => drop(print(out, &map.remote_refs)),
                    #[cfg(feature = "serde1")]
                    OutputFormat::Json => serde_json::to_writer_pretty(
                        out,
                        &map.remote_refs.into_iter().map(JsonRef::from).collect::<Vec<_>>(),
                    )?,
                };
                Ok(())
            }
        }
    }

    pub(crate) fn print_refmap(
        repo: &git::Repository,
        refspecs: &[RefSpec],
        mut map: git::remote::fetch::RefMap<'_>,
        mut out: impl std::io::Write,
        mut err: impl std::io::Write,
    ) -> anyhow::Result<()> {
        let mut last_spec_index = usize::MAX;
        map.mappings.sort_by_key(|m| m.spec_index);
        for mapping in &map.mappings {
            if mapping.spec_index != last_spec_index {
                last_spec_index = mapping.spec_index;
                let spec = &refspecs[mapping.spec_index];
                spec.to_ref().write_to(&mut out)?;
                writeln!(out)?;
            }

            write!(out, "\t")?;
            let target_id = match &mapping.remote {
                git::remote::fetch::Source::ObjectId(id) => {
                    write!(out, "{}", id)?;
                    id
                }
                git::remote::fetch::Source::Ref(r) => print_ref(&mut out, r)?,
            };
            match &mapping.local {
                Some(local) => {
                    write!(out, " -> {local} ")?;
                    match repo.try_find_reference(local)? {
                        Some(tracking) => {
                            let msg = match tracking.try_id() {
                                Some(id) => (id.as_ref() == target_id)
                                    .then(|| "[up-to-date]")
                                    .unwrap_or("[changed]"),
                                None => "[skipped]",
                            };
                            writeln!(out, "{msg}")
                        }
                        None => writeln!(out, "[new]"),
                    }
                }
                None => writeln!(out, " (fetch only)"),
            }?;
        }
        if !map.fixes.is_empty() {
            writeln!(
                err,
                "The following destination refs were removed as they didn't start with 'ref/'"
            )?;
            map.fixes.sort_by_key(|f| match f {
                Fix::MappingWithPartialDestinationRemoved { spec, .. } => *spec,
            });
            let mut prev_spec = None;
            for fix in &map.fixes {
                match fix {
                    Fix::MappingWithPartialDestinationRemoved { name, spec } => {
                        if prev_spec.map_or(true, |prev_spec| prev_spec != spec) {
                            prev_spec = spec.into();
                            spec.write_to(&mut err)?;
                            writeln!(err)?;
                        }
                        writeln!(err, "\t{name}")?;
                    }
                }
            }
        }
        if map.remote_refs.len() - map.mappings.len() != 0 {
            writeln!(
                err,
                "server sent {} tips, {} were filtered due to {} refspec(s).",
                map.remote_refs.len(),
                map.remote_refs.len() - map.mappings.len(),
                refspecs.len()
            )?;
        }
        if refspecs.is_empty() {
            bail!("Without ref-specs there is nothing to show here. Add ref-specs as arguments or configure them in git-config.")
        }
        Ok(())
    }

    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum JsonRef {
        Peeled {
            path: String,
            tag: String,
            object: String,
        },
        Direct {
            path: String,
            object: String,
        },
        Symbolic {
            path: String,
            target: String,
            object: String,
        },
    }

    impl From<fetch::Ref> for JsonRef {
        fn from(value: fetch::Ref) -> Self {
            match value {
                fetch::Ref::Direct {
                    full_ref_name: path,
                    object,
                } => JsonRef::Direct {
                    path: path.to_string(),
                    object: object.to_string(),
                },
                fetch::Ref::Symbolic {
                    full_ref_name: path,
                    target,
                    object,
                } => JsonRef::Symbolic {
                    path: path.to_string(),
                    target: target.to_string(),
                    object: object.to_string(),
                },
                fetch::Ref::Peeled {
                    full_ref_name: path,
                    tag,
                    object,
                } => JsonRef::Peeled {
                    path: path.to_string(),
                    tag: tag.to_string(),
                    object: object.to_string(),
                },
            }
        }
    }

    fn print_ref(mut out: impl std::io::Write, r: &fetch::Ref) -> std::io::Result<&git::hash::oid> {
        match r {
            fetch::Ref::Direct {
                full_ref_name: path,
                object,
            } => write!(&mut out, "{} {}", object, path).map(|_| object.as_ref()),
            fetch::Ref::Peeled {
                full_ref_name: path,
                tag,
                object,
            } => write!(&mut out, "{} {} object:{}", tag, path, object).map(|_| tag.as_ref()),
            fetch::Ref::Symbolic {
                full_ref_name: path,
                target,
                object,
            } => write!(&mut out, "{} {} symref-target:{}", object, path, target).map(|_| object.as_ref()),
        }
    }

    pub(crate) fn print(mut out: impl std::io::Write, refs: &[fetch::Ref]) -> std::io::Result<()> {
        for r in refs {
            print_ref(&mut out, r)?;
            writeln!(out)?;
        }
        Ok(())
    }
}
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub use refs_impl::{refs, refs_fn as refs, JsonRef};

#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub(crate) fn by_name_or_url<'repo>(
    repo: &'repo git_repository::Repository,
    name_or_url: Option<&str>,
) -> anyhow::Result<git_repository::Remote<'repo>> {
    use anyhow::Context;
    Ok(match name_or_url {
        Some(name) => {
            if name.contains('/') || name.contains('.') {
                repo.remote_at(git_repository::url::parse(name.into())?)?
            } else {
                repo.find_remote(&name)?
            }
        }
        None => repo
            .head()?
            .into_remote(git_repository::remote::Direction::Fetch)
            .context("Cannot find a remote for unborn branch")??,
    })
}
