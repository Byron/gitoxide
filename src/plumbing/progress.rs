use crosstermion::crossterm::style::Stylize;
use std::fmt::{Display, Formatter};

enum Usage {
    InModule(&'static str),
}
use Usage::*;

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InModule(m) => write!(f, "mod {m}"),
        }
    }
}

struct Record {
    config: &'static str,
    usage: Usage,
    deviation: Option<&'static str>,
}

static GIT_CONFIG: &[Record] = &[Record {
    config: "pack.threads",
    usage: InModule("remote::connection::fetch"),
    deviation: Some("if unset, it uses all threads as opposed to just 1"),
}];

/// A programmatic way to record and display progress.
pub fn show_progress() -> anyhow::Result<()> {
    for Record {
        config,
        usage,
        deviation,
    } in GIT_CONFIG
    {
        println!(
            "{}: {usage}{}",
            config.bold(),
            deviation.map(|d| format!(" ({d})")).unwrap_or_default().dark_grey()
        );
    }
    Ok(())
}
