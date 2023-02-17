pub use gix_testtools::Result;

#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
mod index;
#[cfg(not(feature = "internal-testing-gix-features-parallel"))]
pub use index::*;
