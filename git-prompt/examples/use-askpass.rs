use git_prompt::{Mode, Options};
use std::borrow::Cow;
use std::path::Path;

fn main() -> Result<(), git_prompt::Error> {
    let pass = git_prompt::ask(
        "Password: ",
        Options {
            askpass: Some(Cow::Owned(Path::new(file!()).parent().unwrap().join("askpass.sh"))),
            mode: Mode::Disable,
        },
    )?;
    eprintln!("{pass:?}");
    Ok(())
}
