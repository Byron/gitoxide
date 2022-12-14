#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod refs_impl {
    use anyhow::bail;
    use git_repository as git;
    use git_repository::{
        protocol::handshake,
        refspec::{match_group::validate::Fix, RefSpec},
        remote::fetch::Source,
    };

    use super::by_name_or_url;
    use crate::OutputFormat;

    pub mod refs {
        use git_repository::bstr::BString;

        use crate::OutputFormat;

        pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

        pub enum Kind {
            Remote,
            Tracking {
                ref_specs: Vec<BString>,
                show_unmapped_remote_refs: bool,
            },
        }

        pub struct Options {
            pub format: OutputFormat,
            pub name_or_url: Option<String>,
            pub handshake_info: bool,
        }

        pub(crate) use super::{print, print_ref};
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
        let show_unmapped = if let refs::Kind::Tracking {
            ref_specs,
            show_unmapped_remote_refs,
        } = &kind
        {
            if format != OutputFormat::Human {
                bail!("JSON output isn't yet supported for listing ref-mappings.");
            }
            if !ref_specs.is_empty() {
                remote.replace_refspecs(ref_specs.iter(), git::remote::Direction::Fetch)?;
                remote = remote.with_fetch_tags(git::remote::fetch::Tags::None);
            }
            *show_unmapped_remote_refs
        } else {
            false
        };
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
            refs::Kind::Tracking { .. } => print_refmap(
                &repo,
                remote.refspecs(git::remote::Direction::Fetch),
                map,
                show_unmapped,
                out,
                err,
            ),
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
        mut map: git::remote::fetch::RefMap,
        show_unmapped_remotes: bool,
        mut out: impl std::io::Write,
        mut err: impl std::io::Write,
    ) -> anyhow::Result<()> {
        let mut last_spec_index = git::remote::fetch::SpecIndex::ExplicitInRemote(usize::MAX);
        map.mappings.sort_by_key(|m| m.spec_index);
        for mapping in &map.mappings {
            if mapping.spec_index != last_spec_index {
                last_spec_index = mapping.spec_index;
                let spec = mapping
                    .spec_index
                    .get(refspecs, &map.extra_refspecs)
                    .expect("refspecs here are the ones used for mapping");
                spec.to_ref().write_to(&mut out)?;
                let is_implicit = mapping.spec_index.implicit_index().is_some();
                if is_implicit {
                    write!(&mut out, " (implicit")?;
                    if spec.to_ref()
                        == git::remote::fetch::Tags::Included
                            .to_refspec()
                            .expect("always yields refspec")
                    {
                        write!(&mut out, ", due to auto-tag")?;
                    }
                    write!(&mut out, ")")?;
                }
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
            map.fixes.sort_by(|l, r| match (l, r) {
                (
                    Fix::MappingWithPartialDestinationRemoved { spec: l, .. },
                    Fix::MappingWithPartialDestinationRemoved { spec: r, .. },
                ) => l.cmp(r),
            });
            let mut prev_spec = None;
            for fix in &map.fixes {
                match fix {
                    Fix::MappingWithPartialDestinationRemoved { name, spec } => {
                        if prev_spec.map_or(true, |prev_spec| prev_spec != spec) {
                            prev_spec = spec.into();
                            spec.to_ref().write_to(&mut err)?;
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
            if show_unmapped_remotes {
                writeln!(&mut out, "\nFiltered: ")?;
                for remote_ref in map.remote_refs.iter().filter(|r| {
                    !map.mappings.iter().any(|m| match &m.remote {
                        Source::Ref(other) => other == *r,
                        Source::ObjectId(_) => false,
                    })
                }) {
                    print_ref(&mut out, remote_ref)?;
                    writeln!(&mut out)?;
                }
            }
        }
        if refspecs.is_empty() {
            bail!("Without refspecs there is nothing to show here. Add refspecs as arguments or configure them in git-config.")
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
        Unborn {
            path: String,
            target: String,
        },
        Symbolic {
            path: String,
            target: String,
            object: String,
        },
    }

    impl From<handshake::Ref> for JsonRef {
        fn from(value: handshake::Ref) -> Self {
            match value {
                handshake::Ref::Unborn { full_ref_name, target } => JsonRef::Unborn {
                    path: full_ref_name.to_string(),
                    target: target.to_string(),
                },
                handshake::Ref::Direct {
                    full_ref_name: path,
                    object,
                } => JsonRef::Direct {
                    path: path.to_string(),
                    object: object.to_string(),
                },
                handshake::Ref::Symbolic {
                    full_ref_name: path,
                    target,
                    object,
                } => JsonRef::Symbolic {
                    path: path.to_string(),
                    target: target.to_string(),
                    object: object.to_string(),
                },
                handshake::Ref::Peeled {
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

    pub(crate) fn print_ref(mut out: impl std::io::Write, r: &handshake::Ref) -> std::io::Result<&git::hash::oid> {
        match r {
            handshake::Ref::Direct {
                full_ref_name: path,
                object,
            } => write!(&mut out, "{} {}", object, path).map(|_| object.as_ref()),
            handshake::Ref::Peeled {
                full_ref_name: path,
                tag,
                object,
            } => write!(&mut out, "{} {} object:{}", tag, path, object).map(|_| tag.as_ref()),
            handshake::Ref::Symbolic {
                full_ref_name: path,
                target,
                object,
            } => write!(&mut out, "{} {} symref-target:{}", object, path, target).map(|_| object.as_ref()),
            handshake::Ref::Unborn { full_ref_name, target } => {
                static NULL: git::hash::ObjectId = git::hash::ObjectId::null(git::hash::Kind::Sha1);
                write!(&mut out, "unborn {} symref-target:{}", full_ref_name, target).map(|_| NULL.as_ref())
            }
        }
    }

    pub(crate) fn print(mut out: impl std::io::Write, refs: &[handshake::Ref]) -> std::io::Result<()> {
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
                repo.find_remote(name)?
            }
        }
        None => repo
            .head()?
            .into_remote(git_repository::remote::Direction::Fetch)
            .context("Cannot find a remote for unborn branch")??,
    })
}
