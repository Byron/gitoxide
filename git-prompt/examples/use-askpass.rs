use git_prompt::{Mode, Options};
use std::borrow::Cow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pass = git_prompt::ask(
        "Password: ",
        Options {
            askpass: Some(Cow::Owned(
                std::env::var_os("ASKPASS_PROGRAM")
                    .ok_or("Please set the ASKPASS_PROGRAM environment variable")?
                    .into(),
            )),
            mode: Mode::Disable,
        },
    )?;
    eprintln!("{pass:?}");
    Ok(())
}
