fn main() -> Result<(), gix_prompt::Error> {
    let user = gix_prompt::openly("Username: ")?;
    eprintln!("{user:?}");
    let pass = gix_prompt::securely("Password: ")?;
    eprintln!("{pass:?}");
    Ok(())
}
