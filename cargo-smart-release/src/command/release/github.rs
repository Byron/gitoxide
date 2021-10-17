#![allow(dead_code)]

use std::{borrow::Cow, process::Command};

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
    new_version: &semver::Version,
    notes: &str,
    Options { dry_run, .. }: Options,
    ctx: &Context,
) -> anyhow::Result<()> {
    let tag_name = crate::utils::tag_name(publishee, new_version, &ctx.repo);
    let mut cmd = Command::new("gh");
    cmd.args(["release", "create"])
        .arg(&tag_name)
        .arg("--title")
        .arg(format!(
            "{}v{}",
            match crate::utils::tag_prefix(publishee, &ctx.repo) {
                Some(prefix) => Cow::Owned(format!("{} ", prefix)),
                None => "".into(),
            },
            new_version
        ))
        .arg("--notes");
    log::trace!(
        "{} run {:?} \"{}…\" [note truncated]",
        will(dry_run),
        cmd,
        notes.chars().take(15).collect::<String>()
    );

    cmd.arg(notes);
    if !Program::named("gh").found {
        log::warn!("To create github releases, please install the 'gh' program and try again");
        return Ok(());
    } else if !dry_run && !cmd.status()?.success() {
        log::warn!(
            "'gh' tool execution failed - we will keep trying, and you may try to create the release with: {:?}",
            cmd
        );
    }
    Ok(())
}
