use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter};
use tabled::{Style, TableIteratorExt, Tabled};

#[derive(Clone)]
enum Usage {
    NotApplicable,
    Planned {
        note: Option<&'static str>,
    },
    InModule {
        name: &'static str,
        deviation: Option<&'static str>,
    },
}
use Usage::*;

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotApplicable => f.write_str("not applicable")?,
            Planned { note } => {
                write!(f, "{}", "planned".blink())?;
                if let Some(note) = note {
                    write!(f, " ℹ {} ℹ", note.bright_white())?;
                }
            }
            InModule { name, deviation } => {
                write!(f, "mod {name}")?;
                if let Some(deviation) = deviation {
                    write!(f, "{}", format!(" ❗️{deviation}❗️").bright_white())?
                }
            }
        }
        Ok(())
    }
}

impl Usage {
    pub fn icon(&self) -> &'static str {
        match self {
            NotApplicable => "❌",
            Planned { .. } => "🕒",
            InModule { deviation, .. } => deviation.is_some().then(|| "👌️").unwrap_or("✅"),
        }
    }
}

#[derive(Clone)]
struct Record {
    config: &'static str,
    usage: Usage,
}

impl Tabled for Record {
    const LENGTH: usize = 3;

    fn fields(&self) -> Vec<String> {
        vec![self.usage.icon().into(), self.config.into(), self.usage.to_string()]
    }

    fn headers() -> Vec<String> {
        vec![]
    }
}

static GIT_CONFIG: &[Record] = &[
    Record {
        config: "fetch.output",
        usage: NotApplicable,
    },
    Record {
        config: "fetch.negotiationAlgorithm",
        usage: Planned {
            note: Some("Implements our own 'naive' algorithm, only"),
        },
    },
    Record {
        config: "pack.threads",
        usage: InModule {
            name: "remote::connection::fetch",
            deviation: Some("if unset, it uses all threads as opposed to just 1"),
        },
    },
];

/// A programmatic way to record and display progress.
pub fn show_progress() -> anyhow::Result<()> {
    let sorted = {
        let mut v: Vec<_> = GIT_CONFIG.into();
        v.sort_by_key(|r| r.config);
        v
    };

    println!("{}", sorted.table().with(Style::blank()));
    println!("\nTotal records: {}", GIT_CONFIG.len());
    Ok(())
}
