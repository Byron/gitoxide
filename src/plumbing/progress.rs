use std::fmt::{Display, Formatter};

use crosstermion::crossterm::style::Stylize;
use owo_colors::OwoColorize;
use tabled::{Style, TableIteratorExt, Tabled};

#[derive(Clone)]
enum Usage {
    NotApplicable,
    Planned {
        note: Option<&'static str>,
    },
    NotPlanned {
        reason: &'static str,
    },
    InModule {
        name: &'static str,
        deviation: Option<&'static str>,
    },
    /// Needs analysis
    Puzzled,
}
use Usage::*;

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Puzzled => f.write_str("❓")?,
            NotApplicable => f.write_str("not applicable")?,
            NotPlanned { reason } => {
                write!(f, "{}", "not planned".blink())?;
                write!(f, " ℹ {} ℹ", reason.bright_white())?;
            }
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
            Puzzled => "?",
            NotApplicable => "❌",
            Planned { .. } => "🕒",
            NotPlanned { .. } => "🤔",
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
        let mut tokens = self.config.split('.');
        let mut buf = vec![tokens.next().expect("present").bold().to_string()];
        buf.extend(tokens.map(ToOwned::to_owned));

        vec![self.usage.icon().into(), buf.join("."), self.usage.to_string()]
    }

    fn headers() -> Vec<String> {
        vec![]
    }
}

static GIT_CONFIG: &[Record] = &[
    Record {
        config: "core.bare",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.excludesFile",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.abbrev",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.ignoreCase",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.multiPackIndex",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.disambiguate",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.eol",
        usage: Planned {note: Some("needed for filters, but also for doing diffs correctly")}
    },
    Record {
        config: "core.filesRefLockTimeout",
        usage: InModule {name: "config::cache::access", deviation: None},
    },
    Record {
        config: "core.packedRefsTimeout",
        usage: InModule {name: "config::cache::access", deviation: None},
    },
    Record {
        config: "core.logAllRefUpdates",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.repositoryFormatVersion",
        usage: InModule {
            name: "config::cache::incubate",
            deviation: None,
        },
    },
    Record {
        config: "extensions.objectFormat",
        usage: InModule {
            name: "config::cache::incubate",
            deviation: Some(
                "Support for SHA256 is prepared but not fully implemented yet. For now we abort when encountered.",
            ),
        },
    },
    Record {
        config: "committer.name",
        usage: InModule {
            name: "repository::identity",
            deviation: None,
        },
    },
    Record {
        config: "committer.email",
        usage: InModule {
            name: "repository::identity",
            deviation: None,
        },
    },
    Record {
        config: "author.name",
        usage: InModule {
            name: "repository::identity",
            deviation: None,
        },
    },
    Record {
        config: "author.email",
        usage: InModule {
            name: "repository::identity",
            deviation: None,
        },
    },
    Record {
        config: "user.name",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("defaults to 'gitoxide'"),
        },
    },
    Record {
        config: "user.email",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("defaults to 'gitoxide@localhost'"),
        },
    },
    Record {
    config: "clone.filterSubmodules,",
        usage: Planned {
            note: Some("currently object filtering isn't support, a prerequisite for this, see --filter=blob:none for more"),
        },
    },
    Record {
        config: "clone.defaultRemoteName",
        usage: InModule {
            name: "clone::prepare",
            deviation: None
        },
    },
    Record {
        config: "clone.rejectShallow",
        usage: NotPlanned {
            reason: "it's not a use-case we consider important now, but once that changes it can be implemented",
        },
    },
    Record {
        config: "fetch.recurseSubmodules",
        usage: Planned {
            note: Some("Seems useful for cargo as well"),
        },
    },
    Record {
        config: "fetch.fsckObjects",
        usage: Puzzled,
    },
    Record {
        config: "fetch.fsck.<msg-id>",
        usage: Puzzled,
    },
    Record {
        config: "fetch.fsck.skipList",
        usage: Puzzled,
    },
    Record {
        config: "fetch.unpackLimit",
        usage: Planned { note: None },
    },
    Record {
        config: "fetch.prune",
        usage: Planned { note: None },
    },
    Record {
        config: "fetch.pruneTags",
        usage: Planned { note: None },
    },
    Record {
        config: "fetch.writeCommitGraph",
        usage: Planned { note: None },
    },
    Record {
        config: "fetch.parallel",
        usage: Planned { note: None },
    },
    Record {
        config: "fetch.showForcedUpdates",
        usage: NotApplicable,
    },
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
    Record {
        config: "pack.indexVersion",
        usage: InModule {
            name: "remote::connection::fetch",
            deviation: None,
        },
    },
    Record {
        config: "protocol.allow",
        usage: InModule {
            name: "remote::url::scheme_permission",
            deviation: None,
        },
    },
    Record {
        config: "protocol.<name>.allow",
        usage: InModule {
            name: "remote::url::scheme_permission",
            deviation: None,
        },
    },
    Record {
        config: "remotes.<group>",
        usage: Planned {
            note: Some("useful for multi-remote fetches as part of the standard API, maybe just `group(name) -> Option<Vec<Remote>>`"),
        },
    },
    Record {
        config: "url.<base>.insteadOf",
        usage: InModule {
            name: "remote::url::rewrite",
            deviation: None,
        },
    },
    Record {
        config: "url.<base>.pushInsteadOf",
        usage: InModule {
            name: "remote::url::rewrite",
            deviation: None,
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
