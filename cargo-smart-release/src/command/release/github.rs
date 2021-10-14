#![allow(dead_code)]

use std::process::Command;

use cargo_metadata::Package;

use crate::{
    command::release::Options,
    utils::{will, Program},
    Context,
};

struct Support {
    gh: Program,
}

impl Default for Support {
    fn default() -> Self {
        Self::new()
    }
}

impl Support {
    fn new() -> Self {
        Support {
            gh: Program::named("gh"),
        }
    }
}

pub fn create_release(
    publishee: &Package,
    new_version: &str,
    notes: &str,
    Options { verbose, dry_run, .. }: Options,
    ctx: &Context,
) -> anyhow::Result<()> {
    let tag_name = crate::utils::tag_name(publishee, new_version, &ctx.repo);
    let mut cmd = Command::new("gh");
    cmd.arg(&tag_name)
        .arg("--notes")
        .arg(notes)
        .arg("--title")
        .arg(format!("{} v{}", publishee.name, new_version));
    if dry_run && verbose {
        log::info!("{} run {:?}", will(dry_run), cmd);
    }
    if !Program::named("gh").found {
        log::warn!("To create github releases, please install the 'gh' program and try again");
        return Ok(());
    } else if !dry_run && !cmd.status()?.success() {
        log::warn!("'gh' tool execution failed - we will keep trying, and you may try to create the release manually using the command invocation above.");
    }
    Ok(())
}
