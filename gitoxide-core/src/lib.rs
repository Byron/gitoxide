#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]
#![cfg_attr(feature = "async-client", allow(unused))]

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

pub mod net;

pub mod commitgraph;
#[cfg(feature = "estimate-hours")]
pub mod hours;
#[cfg(feature = "organize")]
pub mod organize;
pub mod pack;
#[cfg(any(feature = "async-client", feature = "blocking-client"))]
pub mod remote;
pub mod repository;

#[cfg(all(feature = "async-client", feature = "blocking-client"))]
compile_error!("Cannot set both 'blocking-client' and 'async-client' features as they are mutually exclusive");
