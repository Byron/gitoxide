mod describe;
mod spec;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;
