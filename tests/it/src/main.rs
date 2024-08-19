use clap::Parser;

mod commands;

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    match args.cmd {
        Subcommands::CopyRoyal {
            dry_run,
            worktree_dir: worktree_root,
            destination_dir,
            patterns,
        } => commands::copy_royal::doit(dry_run, &worktree_root, destination_dir, patterns),
    }
}

mod args;
use args::{Args, Subcommands};
