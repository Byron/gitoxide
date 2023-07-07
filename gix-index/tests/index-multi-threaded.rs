#[cfg(feature = "internal-testing-gix-features-parallel")]
pub use gix_testtools::Result;

#[cfg(feature = "internal-testing-gix-features-parallel")]
mod index;
#[cfg(feature = "internal-testing-gix-features-parallel")]
pub use index::*;
