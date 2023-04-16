type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
mod capabilities;
mod dir;
mod stack;
