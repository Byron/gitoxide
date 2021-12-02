pub type PackCache = dyn git_pack::cache::DecodeEntry + Send + 'static;
pub type NewPackCacheFn = dyn Fn() -> Box<PackCache> + 'static;

pub type ObjectCache = dyn git_pack::cache::Object + Send + 'static;
pub type NewObjectCacheFn = dyn Fn() -> Box<ObjectCache> + 'static;
