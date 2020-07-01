#[cfg(feature = "pretty-cli")]
pub mod pretty;

#[cfg(all(feature = "lean-cli", not(feature = "pretty-cli")))]
pub mod lean;

/// If verbose is true, the env logger will be forcibly set to 'info' logging level. Otherwise env logging facilities
/// will just be initialized.
#[cfg(feature = "env_logger")]
fn init_env_logger(verbose: bool) {
    if verbose {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    } else {
        env_logger::init();
    }
}
