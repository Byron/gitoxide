#![allow(dead_code)]

use crate::command::release::Options;
use crate::utils::{will, Program};
use crate::Context;
use cargo_metadata::Package;
use std::process::Command;

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
        Support { gh: Program::new("gh") }
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
    if !Program::new("gh").found {
        log::warn!("To create github releases, please install the 'gh' program and try again");
        return Ok(());
    } else if !dry_run && !cmd.status()?.success() {
        log::warn!("'gh' tool execution failed - this is considered non-critical and we keep trying.");
    }
    Ok(())
}
