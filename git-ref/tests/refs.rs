#[cfg(not(feature = "internal-testing-git-features-parallel"))]
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod file;
#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod fullname;
#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod namespace;
#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod packed;
#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod reference;
#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod store;
#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod transaction;
