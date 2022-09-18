#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod refs_impl {
    use anyhow::bail;
    use git_repository as git;
    use git_repository::protocol::fetch;
    use git_repository::refspec::match_group::validate::Fix;
    use git_repository::refspec::RefSpec;

    use crate::OutputFormat;

    pub mod refs {
        use crate::OutputFormat;
        use git_repository::bstr::BString;

        pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

        pub enum Kind {
            Remote,
            Tracking { ref_specs: Vec<BString> },
        }

        pub struct Context {
            pub format: OutputFormat,
            pub name: Option<String>,
            pub url: Option<git_repository::Url>,
        }

        pub(crate) use super::print;
    }

    #[git::protocol::maybe_async::maybe_async]
    pub async fn refs_fn(
        repo: git::Repository,
        kind: refs::Kind,
        mut progress: impl git::Progress,
        out: impl std::io::Write,
        err: impl std::io::Write,
        refs::Context { format, name, url }: refs::Context,
    ) -> anyhow::Result<()> {
        use anyhow::Context;
        let mut remote = match (name, url) {
            (Some(name), None) => repo.find_remote(&name)?,
            (None, None) => repo
                .head()?
                .into_remote(git::remote::Direction::Fetch)
                .context("Cannot find a remote for unborn branch")??,
            (None, Some(url)) => repo.remote_at(url)?,
            (Some(_), Some(_)) => bail!("Must not set both the remote name and the url - they are mutually exclusive"),
        };
        if let refs::Kind::Tracking { ref_specs } = &kind {
            if format != OutputFormat::Human {
                bail!("JSON output isn't yet supported for listing ref-mappings.");
            }
            remote.replace_refspecs(ref_specs.iter(), git::remote::Direction::Fetch)?;
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
            .ref_map()
            .await?;

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

    fn print_refmap(
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
            } => write!(&mut out, "{} {}", object.to_hex(), path).map(|_| object.as_ref()),
            fetch::Ref::Peeled {
                full_ref_name: path,
                tag,
                object,
            } => write!(&mut out, "{} {} tag:{}", object.to_hex(), path, tag).map(|_| tag.as_ref()),
            fetch::Ref::Symbolic {
                full_ref_name: path,
                target,
                object,
            } => write!(&mut out, "{} {} symref-target:{}", object.to_hex(), path, target).map(|_| object.as_ref()),
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
