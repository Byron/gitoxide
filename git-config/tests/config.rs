type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod git_config;
mod parser;
mod value;
mod values;
