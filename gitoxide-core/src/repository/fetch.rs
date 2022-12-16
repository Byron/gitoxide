use git::bstr::BString;
use git_repository as git;

use crate::OutputFormat;

pub struct Options {
    pub format: OutputFormat,
    pub dry_run: bool,
    pub remote: Option<String>,
    /// If non-empty, override all ref-specs otherwise configured in the remote
    pub ref_specs: Vec<BString>,
    pub handshake_info: bool,
}

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=3;

pub(crate) mod function {
    use anyhow::bail;
    use git_repository as git;
    use git_repository::{prelude::ObjectIdExt, refspec::match_group::validate::Fix, remote::fetch::Status};

    use super::Options;
    use crate::OutputFormat;

    pub fn fetch<P>(
        repo: git::Repository,
        progress: P,
        mut out: impl std::io::Write,
        err: impl std::io::Write,
        Options {
            format,
            dry_run,
            remote,
            handshake_info,
            ref_specs,
        }: Options,
    ) -> anyhow::Result<()>
    where
        P: git::Progress,
        P::SubProgress: 'static,
    {
        if format != OutputFormat::Human {
            bail!("JSON output isn't yet supported for fetching.");
        }

        let mut remote = crate::repository::remote::by_name_or_url(&repo, remote.as_deref())?;
        if !ref_specs.is_empty() {
            remote.replace_refspecs(ref_specs.iter(), git::remote::Direction::Fetch)?;
            remote = remote.with_fetch_tags(git::remote::fetch::Tags::None);
        }
        let res: git::remote::fetch::Outcome = remote
            .connect(git::remote::Direction::Fetch, progress)?
            .prepare_fetch(Default::default())?
            .with_dry_run(dry_run)
            .receive(&git::interrupt::IS_INTERRUPTED)?;

        if handshake_info {
            writeln!(out, "Handshake Information")?;
            writeln!(out, "\t{:?}", res.ref_map.handshake)?;
        }

        let ref_specs = remote.refspecs(git::remote::Direction::Fetch);
        match res.status {
            Status::NoPackReceived { update_refs } => {
                print_updates(&repo, update_refs, ref_specs, res.ref_map, &mut out, err)
            }
            Status::DryRun { update_refs } => print_updates(&repo, update_refs, ref_specs, res.ref_map, &mut out, err),
            Status::Change {
                update_refs,
                write_pack_bundle,
            } => {
                print_updates(&repo, update_refs, ref_specs, res.ref_map, &mut out, err)?;
                if let Some(data_path) = write_pack_bundle.data_path {
                    writeln!(out, "pack  file: \"{}\"", data_path.display()).ok();
                }
                if let Some(index_path) = write_pack_bundle.index_path {
                    writeln!(out, "index file: \"{}\"", index_path.display()).ok();
                }
                Ok(())
            }
        }?;
        if dry_run {
            writeln!(out, "DRY-RUN: No ref was updated and no pack was received.").ok();
        }
        Ok(())
    }

    pub(crate) fn print_updates(
        repo: &git::Repository,
        update_refs: git::remote::fetch::refs::update::Outcome,
        refspecs: &[git::refspec::RefSpec],
        mut map: git::remote::fetch::RefMap,
        mut out: impl std::io::Write,
        mut err: impl std::io::Write,
    ) -> anyhow::Result<()> {
        let mut last_spec_index = git::remote::fetch::SpecIndex::ExplicitInRemote(usize::MAX);
        let mut updates = update_refs
            .iter_mapping_updates(&map.mappings, refspecs, &map.extra_refspecs)
            .filter_map(|(update, mapping, spec, edit)| spec.map(|spec| (update, mapping, spec, edit)))
            .collect::<Vec<_>>();
        updates.sort_by_key(|t| t.2);
        let mut skipped_due_to_implicit_tag = None;
        fn consume_skipped_tags(skipped: &mut Option<usize>, out: &mut impl std::io::Write) -> std::io::Result<()> {
            if let Some(skipped) = skipped.take() {
                if skipped != 0 {
                    writeln!(
                        out,
                        "\tskipped {skipped} tags known to the remote without bearing on this commit-graph. Use `gix remote ref-map` to list them."
                    )?;
                }
            }
            Ok(())
        }
        for (update, mapping, spec, edit) in updates {
            if mapping.spec_index != last_spec_index {
                last_spec_index = mapping.spec_index;
                consume_skipped_tags(&mut skipped_due_to_implicit_tag, &mut out)?;
                spec.to_ref().write_to(&mut out)?;
                let is_implicit = mapping.spec_index.implicit_index().is_some();
                if is_implicit {
                    write!(&mut out, " (implicit")?;
                    if spec.to_ref()
                        == git::remote::fetch::Tags::Included
                            .to_refspec()
                            .expect("always yields refspec")
                    {
                        skipped_due_to_implicit_tag = Some(0);
                        write!(&mut out, ", due to auto-tag")?;
                    }
                    write!(&mut out, ")")?;
                }
                writeln!(out)?;
            }

            if let Some(num_skipped) = skipped_due_to_implicit_tag.as_mut() {
                if matches!(update.mode, git::remote::fetch::refs::update::Mode::NoChangeNeeded) {
                    *num_skipped += 1;
                    continue;
                }
            }

            write!(out, "\t")?;
            match &mapping.remote {
                git::remote::fetch::Source::ObjectId(id) => {
                    write!(out, "{}", id.attach(repo).shorten_or_id())?;
                }
                git::remote::fetch::Source::Ref(r) => {
                    crate::repository::remote::refs::print_ref(&mut out, r)?;
                }
            };
            match edit {
                Some(edit) => {
                    writeln!(out, " -> {} [{}]", edit.name, update.mode)
                }
                None => writeln!(out, " [{}]", update.mode),
            }?;
        }
        consume_skipped_tags(&mut skipped_due_to_implicit_tag, &mut out)?;
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
        }
        Ok(())
    }
}
