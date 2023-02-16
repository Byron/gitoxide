#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod file;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod fullname;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod namespace;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod packed;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod reference;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod store;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod transaction;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod util;
