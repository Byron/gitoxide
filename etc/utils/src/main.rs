use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    let cmd = args
        .next()
        .ok_or_else(|| anyhow!("The first argument is the subcommand"))?;

    match cmd.as_str() {
        "crate-path" => {
            let _crate_name = args
                .next()
                .ok_or_else(|| anyhow!("The first argument is the name of the crate whose path "))?;
        }
        cmd => bail!("Unknown subcommand: {}", cmd),
    }
    Ok(())
}
