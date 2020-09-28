#[cfg(feature = "parallel")]
pub mod walkdir {
    pub use jwalk::{Error, WalkDir};
}

#[cfg(not(feature = "parallel"))]
pub mod walkdir {
    pub use walkdir::{Error, WalkDir};
}

pub use self::walkdir::WalkDir;
