use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let filepath = PathBuf::new().join("tempfile.ext");
    let _tempfile = git_tempfile::at_path(&filepath)?;
    eprintln!(
        "Observe the tempfile at {} and hit Ctrl+C to see it vanish. I will go to sleep now…",
        filepath.display()
    );
    std::thread::sleep(std::time::Duration::from_secs(8 * 60 * 60));
    Ok(())
}
