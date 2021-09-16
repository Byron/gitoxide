use crate::command::changelog::Options;

mod commit;
mod git;

pub fn changelog(options: Options, crates: Vec<String>) -> anyhow::Result<()> {
    let ctx = crate::Context::new(crates)?;
    let crate_names = if options.dependencies {
        crate::traverse::dependencies(&ctx, false, true)?
    } else {
        ctx.crate_names.clone()
    };
    assure_working_tree_is_unchanged(options)?;
    let history = match git::commit_history(&ctx.repo)? {
        None => return Ok(()),
        Some(history) => history,
    };
    for crate_name in &crate_names {
        let _segments = git::ref_segments(crate_name, &ctx, &history)?;
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
