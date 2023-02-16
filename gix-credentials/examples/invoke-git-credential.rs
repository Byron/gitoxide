use std::convert::TryInto;

/// Invokes `git credential` with the passed url as argument and prints obtained credentials.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = git_credentials::builtin(git_credentials::helper::Action::get_for_url(
        std::env::args()
            .nth(1)
            .ok_or("First argument must be the URL to obtain credentials for")?,
    ))?
    .ok_or("Did not obtain credentials")?;
    let ctx: git_credentials::protocol::Context = (&out.next).try_into()?;
    ctx.write_to(std::io::stdout())?;
    Ok(())
}
