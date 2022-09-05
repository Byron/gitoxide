use std::borrow::Cow;

use git_prompt::{Mode, Options};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pass = git_prompt::ask(
        "Password: ",
        &Options {
            askpass: Some(Cow::Owned(std::env::current_exe()?.parent().unwrap().join("askpass"))),
            mode: Mode::Disable,
        },
    )?;
    eprintln!("{pass:?}");
    Ok(())
}
