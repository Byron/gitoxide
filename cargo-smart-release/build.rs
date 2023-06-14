use std::process::Command;

fn main() {
    let version = Command::new(if cfg!(windows) { "git.exe" } else { "git" })
        .args(["describe", "--match=cargo-smart-release-*"])
        .output()
        .ok()
        .and_then(|out| parse_describe(&out.stdout))
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").into());

    println!("cargo:rustc-env=CARGO_SMART_RELEASE_VERSION={version}");
}

fn parse_describe(input: &[u8]) -> Option<String> {
    let input = std::str::from_utf8(input).ok()?;
    input.trim().to_owned().into()
}
