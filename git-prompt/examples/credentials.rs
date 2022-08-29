fn main() -> Result<(), git_prompt::Error> {
    let user = git_prompt::openly("Username: ")?;
    let pass = git_prompt::securely("Password: ")?;
    eprintln!("{user}:{pass}");
    Ok(())
}
