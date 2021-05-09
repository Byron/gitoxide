use anyhow::{anyhow, bail, Context};
use std::{io, io::Write};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let cmd = args
        .next()
        .ok_or_else(|| anyhow!("The first argument is the subcommand"))?;

    match cmd.as_str() {
        "crate-path" => {
            let crate_name = args
                .next()
                .ok_or_else(|| anyhow!("The first argument is the name of the crate whose path "))?;
            let crate_dir = std::env::current_dir()?.join(&crate_name);
            let cargo_toml_path = crate_dir.join("Cargo.toml");
            let manifest = cargo_toml::Manifest::from_path(&cargo_toml_path)
                .with_context(|| format!("Couldn't read Cargo manifest at '{}'", cargo_toml_path.display()))?;
            let version = manifest
                .package
                .ok_or_else(|| anyhow!("Need package information"))
                .map(|p| p.version)?;
            writeln!(io::stdout(), "{}/{}", crate_name, version)?;
        }
        cmd => bail!("Unknown subcommand: {}", cmd),
    }
    Ok(())
}
