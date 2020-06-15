#[macro_use]
extern crate clap;
extern crate git_core as git;
use anyhow::{Context, Result};

mod app {
    use clap::{App, AppSettings, SubCommand};

    pub fn new<'a, 'b>() -> App<'a, 'b> {
        let app: App = app_from_crate!();
        app.setting(AppSettings::SubcommandRequired).subcommand(
            SubCommand::with_name("init")
                .alias("initialize")
                .about("Initialize the repository in the current directory."),
        )
    }
}

fn main() -> Result<()> {
    let app = app::new();
    let matches = app.get_matches();
    match matches.subcommand() {
        ("init", Some(_args)) => {
            git::init::repository().with_context(|| "Repository initialization failed")
        }
        _ => unreachable!(),
    }?;
    Ok(())
}
