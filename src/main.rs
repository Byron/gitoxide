extern crate failure;
extern crate failure_tools;
#[macro_use]
extern crate clap;
extern crate git_core as git;

use failure_tools::ok_or_exit;
use failure::{Error, ResultExt};

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

fn run() -> Result<(), Error> {
    let app = app::new();
    let matches = app.get_matches();
    match matches.subcommand() {
        ("init", Some(_args)) => git::init().with_context(|_| "Repository initialization failed"),
        _ => unreachable!(),
    }.map_err(Into::into)
}

fn main() {
    ok_or_exit(run())
}
