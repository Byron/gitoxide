#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum OutputFormat {
    Human,
    #[cfg(feature = "serde1")]
    Json,
}

impl OutputFormat {
    pub fn variants() -> &'static [&'static str] {
        &[
            "human",
            #[cfg(feature = "serde1")]
            "json",
        ]
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lc = s.to_ascii_lowercase();
        Ok(match s_lc.as_str() {
            "human" => OutputFormat::Human,
            #[cfg(feature = "serde1")]
            "json" => OutputFormat::Json,
            _ => return Err(format!("Invalid output format: '{}'", s)),
        })
    }
}

mod protocol;
pub use protocol::Protocol;

pub mod commitgraph;
pub mod pack;
pub mod remote;
pub mod repository;
