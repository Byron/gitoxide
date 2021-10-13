use crate::{bat, changelog::write::Linkables, command::changelog::Options, git, utils::will, ChangeLog};

pub fn changelog(opts: Options, crates: Vec<String>) -> anyhow::Result<()> {
    let ctx = crate::Context::new(crates)?;
    let crate_names = if opts.dependencies {
        crate::traverse::dependencies(&ctx, false, true)?
    } else {
        ctx.crate_names.clone()
    };
    assure_working_tree_is_unchanged(opts)?;
    let history = match git::history::collect(&ctx.repo)? {
        None => return Ok(()),
        Some(history) => history,
    };

    let bat = (opts.dry_run && opts.preview).then(bat::Support::new);

    let mut pending_changes = Vec::new();
    let linkables = if opts.dry_run {
        Linkables::AsText
    } else {
        Linkables::AsLinks {
            repository_url: crate::git::remote_url()?,
        }
    };
    for crate_name in &crate_names {
        let (crate::changelog::init::Outcome { log, mut lock, state }, _package) =
            ChangeLog::for_crate_by_name_with_write_lock(crate_name, &history, &ctx, opts.generator_segments)?;
        log::info!(
            "{} write {} sections to {} ({})",
            will(opts.dry_run),
            log.sections.len(),
            lock.resource_path()
                .strip_prefix(&ctx.root)
                .expect("contained in workspace")
                .display(),
            state.as_str(),
        );
        lock.with_mut(|file| log.write_to(file, &linkables))?;
        if let Some(bat) = bat.as_ref() {
            bat.display_to_tty(lock.lock_path())?;
        }
        if !opts.dry_run {
            pending_changes.push(lock);
        }
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
