use crate::utils::will;
use crate::{command::changelog::Options, git, ChangeLog};

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
        let (log, package) = ChangeLog::for_package(crate_name, &history, &ctx)?;
        log::info!(
            "{} write {} sections to {}",
            will(opts.dry_run),
            log.sections.len(),
            ChangeLog::path_from_manifest(&package.manifest_path)
                .strip_prefix(&ctx.root)
                .expect("contained in workspace")
        )
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
