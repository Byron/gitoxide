use crate::command::changelog::Options;

pub fn changelog(options: Options, crates: Vec<String>) -> anyhow::Result<()> {
    let _ctx = crate::Context::new(crates)?;
    if options.dependencies {}
    Ok(())
}
