use crate::{command::changelog::Options, git, utils::package_by_name, ChangeLog};

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
    for crate_name in &crate_names {
        let package = package_by_name(&ctx.meta, crate_name)?;
        let _log = ChangeLog::from_history_segments(
            package,
            &git::history::crate_ref_segments(package, &ctx, &history)?,
            &ctx.repo,
        );
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
