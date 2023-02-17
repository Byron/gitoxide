pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod convert;
mod realpath;
mod util;
