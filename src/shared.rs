#[cfg(any(feature = "prodash-line-renderer", feature = "prodash-tui-renderer"))]
pub const DEFAULT_FRAME_RATE: f32 = 6.0;

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
pub fn from_env<T: argh::TopLevelCommand>() -> T {
    static VERSION: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));
    let strings: Vec<String> = std::env::args().collect();
    let strs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
    T::from_args(&[strs[0]], &strs[1..]).unwrap_or_else(|early_exit| {
        // This allows us to make subcommands mandatory,
        // and trigger a helpful message unless --version is specified
        if let Some(arg) = std::env::args().skip(1).next() {
            if arg == "--version" {
                println!("{}", VERSION);
                std::process::exit(0);
            }
        }
        println!("{}", early_exit.output);
        std::process::exit(match early_exit.status {
            Ok(()) => 0,
            Err(()) => 1,
        })
    })
}
