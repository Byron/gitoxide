use std::io::Write;

use crate::{
    bat,
    changelog::write::{Components, Linkables},
    command::changelog::Options,
    git,
    traverse::dependency,
    utils::will,
    version::BumpSpec,
    ChangeLog,
};

pub fn changelog(opts: Options, crates: Vec<String>) -> anyhow::Result<()> {
    let Options {
        generator_segments,
        dependencies,
        dry_run,
        preview,
        no_links,
        capitalize_commit,
        ..
    } = opts;
    let bump_spec = if dependencies { BumpSpec::Auto } else { BumpSpec::Keep };
    let force_history_segmentation = false;
    let ctx = crate::Context::new(crates.clone(), force_history_segmentation, bump_spec, bump_spec)?;
    let crates: Vec<_> = {
        crate::traverse::dependencies(
            &ctx,
            crate::traverse::Options {
                allow_auto_publish_of_stable_crates: true,
                bump_when_needed: true,
                isolate_dependencies_from_breaking_changes: true,
                traverse_graph: dependencies,
            },
        )?
        .into_iter()
        .filter_map(|d| match d.mode {
            dependency::Mode::ToBePublished { .. } => Some(d.package),
            dependency::Mode::NotForPublishing { .. } => {
                if crates.contains(&d.package.name) {
                    log::info!(
                        "Skipping '{}' as it won't be published.{}",
                        d.package.name,
                        if !dependencies {
                            " Try not to specify --no-dependencies/--only."
                        } else {
                            ""
                        }
                    );
                }
                None
            }
        })
        .collect()
    };
    assure_working_tree_is_unchanged(opts)?;
    let history = match git::history::collect(&ctx.repo)? {
        None => return Ok(()),
        Some(history) => history,
    };

    let bat = (dry_run && preview).then(bat::Support::new);

    let mut pending_changes = Vec::new();
    let linkables = if dry_run || no_links {
        Linkables::AsText
    } else {
        crate::git::remote_url(&ctx.repo)?
            .map(|url| Linkables::AsLinks {
                repository_url: url.into(),
            })
            .unwrap_or(Linkables::AsText)
    };
    let mut num_crates = 0;
    for (idx, package) in crates.iter().enumerate() {
        num_crates += 1;
        let crate::changelog::init::Outcome {
            log, mut lock, state, ..
        } = ChangeLog::for_package_with_write_lock(package, &history, &ctx, generator_segments)?;
        log::info!(
            "{} write {} sections to {} ({})",
            will(dry_run),
            log.sections.len(),
            lock.resource_path()
                .strip_prefix(&ctx.root)
                .expect("contained in workspace")
                .display(),
            state.as_str(),
        );
        lock.with_mut(|file| {
            let mut buf = String::new();
            log.write_to(
                &mut buf,
                &linkables,
                if dry_run {
                    Components::SECTION_TITLE
                } else {
                    Components::all()
                },
                capitalize_commit,
            )
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
            file.write_all(buf.as_bytes())
        })?;
        if let Some(bat) = bat.as_ref() {
            bat.display_to_tty(
                lock.lock_path(),
                lock.resource_path().strip_prefix(&ctx.root.to_path_buf())?,
                format!("PREVIEW {} / {}, press Ctrl+C to cancel", idx + 1, crates.len()),
            )?;
        }
        if !dry_run {
            pending_changes.push(lock);
        }
    }

    if num_crates == 0 {
        anyhow::bail!(
            "The given crate{} {} didn't change and no changelog could be generated.",
            if ctx.crate_names.len() != 1 { "s" } else { "" },
            ctx.crate_names
                .iter()
                .map(|c| format!("'{c}'"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    let num_changes = pending_changes.len();
    for change in pending_changes {
        change.commit()?;
    }
    if num_changes != 0 {
        log::info!("Wrote {} changelogs", num_changes);
    }

    Ok(())
}

fn assure_working_tree_is_unchanged(options: Options) -> anyhow::Result<()> {
    if options.allow_dirty {
        Ok(())
    } else {
        crate::git::assure_clean_working_tree().or_else(|err|
            if options.dry_run {
                log::warn!("The working tree has changes which will prevent changelog updates with --write unless --allow-dirty is also specified. The latter isn't recommended.");
                Ok(())
            } else {
                Err(err)
            })
    }
}
