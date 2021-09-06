type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod file;
mod fullname;
mod namespace;
mod packed;
mod transaction;
