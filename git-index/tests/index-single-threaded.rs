pub use git_testtools::Result;

#[cfg(not(feature = "internal-testing-git-features-parallel"))]
mod index;
#[cfg(not(feature = "internal-testing-git-features-parallel"))]
pub use index::*;
