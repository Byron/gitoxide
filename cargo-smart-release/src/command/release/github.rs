#![allow(dead_code)]

use crate::command::release::Options;
use crate::utils::{will, Program};
use git_repository::bstr::{BStr, ByteSlice};
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

pub fn create_release(tag_name: &BStr, notes: &str, Options { verbose, dry_run, .. }: Options) -> anyhow::Result<()> {
    let mut cmd = Command::new("gh");
    let tag_name = tag_name.to_str().expect("only valid UTF-8 in tags");
    cmd.arg(tag_name).arg("--notes").arg(notes).arg("--title").arg(tag_name);
    if dry_run && verbose {
        log::info!("{} run {:?}", will(dry_run), cmd);
    }
    if !Program::new("gh").found {
        log::warn!("To create github releases, please install the 'gh' program and try again");
        return Ok(());
    }
    if !dry_run {
        if !cmd.status()?.success() {
            log::warn!("'gh' tool execution failed - this is considered non-critical and we keep trying.");
        }
    }
    Ok(())
}
