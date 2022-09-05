pub use git_config_value::Error;

mod normalize;
pub use normalize::{normalize, normalize_bstr, normalize_bstring};
