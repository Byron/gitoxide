use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use crosstermion::crossterm::style::Stylize;
use owo_colors::OwoColorize;
use tabled::{Style, TableIteratorExt, Tabled};

#[derive(Clone)]
enum Usage {
    /// It's not reasonable to implement it as the prerequisites don't apply.
    NotApplicable { reason: &'static str },
    /// We have no intention to implement it, but that can change if there is demand.
    NotPlanned { reason: &'static str },
    /// We definitely want to implement this configuration value.
    Planned { note: Option<&'static str> },
    /// The configuration is already used, possibly with a given `deviation`.
    InUse { deviation: Option<&'static str> },
    /// Needs analysis, unclear how it works or what it does.
    Puzzled,
}
use Usage::*;

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Puzzled => f.write_str("❓")?,
            NotApplicable { reason } => write!(f, "not applicable: {reason}")?,
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
            InUse { deviation } => {
                if let Some(deviation) = deviation {
                    write!(f, "{}", format!("❗️{deviation}❗️").bright_white())?
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
            NotApplicable { .. } => "❌",
            Planned { .. } => "🕒",
            NotPlanned { .. } => "🤔",
            InUse { deviation, .. } => {
                if deviation.is_some() {
                    "👌️"
                } else {
                    "✅"
                }
            }
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

    fn fields(&self) -> Vec<Cow<'_, str>> {
        let mut tokens = self.config.split('.');
        let mut buf = vec![{
            let name = tokens.next().expect("present");
            if name == "gitoxide" {
                name.bold().green()
            } else {
                name.bold()
            }
            .to_string()
        }];
        buf.extend(tokens.map(ToOwned::to_owned));

        vec![
            Cow::Borrowed(self.usage.icon()),
            buf.join(".").into(),
            self.usage.to_string().into(),
        ]
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![]
    }
}

static GIT_CONFIG: &[Record] = &[
    Record {
        config: "core.symlinks",
        usage: Planned {note: Some("needed to handle checkouts faithfully")}
    },
    Record {
        config: "core.hideDotFiles",
        usage: Planned {note: Some("Seems useful, but needs demand from windows users")}
    },
    Record {
        config: "core.packedGitWindowSize",
        usage: NotPlanned { reason: "an optimization for handling many large packs more efficiently seems unnecessary" }
    },
    Record {
        config: "core.packedGitLimit",
        usage: NotApplicable { reason: "we target 32bit systems only and don't use a windowing mechanism" }
    },
    Record {
        config: "core.alternateRefsCommand",
        usage: NotPlanned { reason: "there is no need as we can perform the required operation in-binary. This could happen though if there is a use-case and demand." }
    },
    Record {
        config: "core.alternateRefsPrefixes",
        usage: NotPlanned { reason: "seems like a niche feature, but can be implemented if there is demand" }
    },
    Record {
        config: "core.bigFileThreshold",
        usage: Planned { note: Some("unfortunately we can't stream packed files yet, even if not delta-compressed, but respecting the threshold for other operations is definitely a must") }
    },
    Record {
        config: "core.compression",
        usage: Planned { note: Some("Allow to remove similar hardcoded value - passing it through will be some effort") },
    },
    Record {
        config: "core.loosecompression",
        usage: Planned { note: None },
    },
    Record {
        config: "core.protectHFS",
        usage: Planned { note: Some("relevant for checkout on MacOS") },
    },
    Record {
        config: "core.protectNTFS",
        usage: NotPlanned { reason: "lack of demand"},
    },
    Record {
        config: "core.sparseCheckout",
        usage: Planned { note: Some("we want to support huge repos and be the fastest in doing so") },
    },
    Record {
        config: "core.sparseCheckoutCone",
        usage: Planned { note: Some("this is a nice improvement over spareCheckout alone and should one day be available too") },
    },
    Record {
        config: "core.gitProxy",
        usage: NotPlanned { reason: "the transport mechanism works differently enough to not support it for now, but of course it's possible to add support if there is demand" },
    },
    Record {
        config: "checkout.defaultRemote",
        usage: Planned { note: Some("needed for correct checkout behaviour, similar to what git does") },
    },
    Record {
        config: "core.untrackedCache",
        usage: Planned { note: Some("needed for fast worktree operation") },
    },
    Record {
        config: "checkout.guess",
        usage: Planned { note: None },
    },
    Record {
        config: "checkout.thresholdForParallelism",
        usage: NotApplicable {reason: "parallelism is efficient enough to always run with benefit"},
    },
    Record {
        config: "feature.manyFiles",
        usage: Planned {note: Some("big repositories are on the roadmap")},
    },
    Record {
        config: "core.preloadIndex",
        usage: Planned {note: Some("it's enabled by default and allows parallel stat checks - it's using a lot of CPU for just minor performance boosts though")},
    },
    Record {
        config: "commitGraph.generationVersion",
        usage: NotPlanned { reason: "couldn't find a test that would require corrected generation numbers, even `git` has no test for this." },
    },
    Record {
        config: "commitGraph.maxNewFilters",
        usage: NotPlanned { reason: "can be considered when the underlying feature is actually used or needed" },
    },
    Record {
        config: "commitGraph.readChangedPaths",
        usage: NotPlanned { reason: "can be considered when the underlying feature is actually used or needed" },
    },
    Record {
        config: "index.sparse",
        usage: Planned {note: Some("we can read sparse indices and support for it will be added early on")},
    },
    Record {
        config: "merge.renormalize",
        usage: Planned {note: Some("once merging is being implemented, renormalization should be respected")},
    },
    Record {
        config: "sparse.expectFilesOutsideOfPatterns",
        usage: Planned {note: Some("a feature definitely worth having")},
    },
    Record {
        config: "submodule.recurse",
        usage: Planned {note: Some("very relevant for doing the right thing during checkouts. Note that 'clone' isnt' affected by it, even though we could make it so for good measure.")},
    },
    Record {
        config: "submodule.propagateBranches",
        usage: NotPlanned {reason: "it is experimental, let's see how it pans out"}
    },
    Record {
        config: "submodule.alternateLocation",
        usage: NotPlanned {reason: "not currently supported when we clone either"}
    },
    Record {
        config: "submodule.alternateErrorStrategy",
        usage: NotPlanned {reason: "not currently supported when we clone either"}
    },
    Record {
        config: "submodule.fetchJobs",
        usage: Planned {note: Some("relevant for fetching")},
    },
    Record {
        config: "branch.autoSetupRebase",
        usage: Planned {
            note: Some("for when we allow setting up upstream branches")
        },
    },
    Record {
        config: "branch.<name>.rebase",
        usage: Planned {
            note: Some("for when we can merge, rebase should be supported")
        },
    },
    Record {
        config: "branch.<name>.description",
        usage: NotPlanned {
            reason: "no plan to implement format-patch or request-pull summary"
        },
    },
    Record {
        config: "core.fsync",
        usage: Planned {note: Some("more safety for disk write operations is a good thing, definitely on the server")}
    },
    Record {
        config: "core.fsyncMethod",
        usage: Planned {note: Some("needed to support `core.fsync`")}
    },
    Record {
        config: "core.sharedRepository",
        usage: NotPlanned {reason: "on demand"}
    },
    Record {
        config: "core.createObject",
        usage: NotPlanned {reason: "it's valuable not to do writes unless needed on the lowest level, but we hope to avoid issues by not writing duplicate objects in the first place"}
    },
    Record {
    config: "clone.filterSubmodules,",
        usage: Planned {
            note: Some("currently object filtering isn't support, a prerequisite for this, see --filter=blob:none for more"),
        },
    },
    Record {
        config: "clone.rejectShallow",
        usage: Planned {
            note: Some("probably trivial to implement once there is protocol support for shallow clones"),
        },
    },
    Record {
        config: "receive.shallowUpdate",
        usage: NotPlanned {
            reason: "it looks like a server-only setting that allows boundaries to change if refs are pushed that are outside of the boundary.",
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
        usage: NotApplicable {reason: "we don't support advices"},
    },
    Record {
        config: "fetch.output",
        usage: NotPlanned {reason: "'gix' might support it, but there is no intention on copying the 'git' CLI"},
    },
    Record {
        config: "remotes.<group>",
        usage: Planned {
            note: Some("useful for multi-remote fetches as part of the standard API, maybe just `group(name) -> Option<Vec<Remote>>`"),
        },
    },
    Record {
        config: "advice.updateSparsePath",
        usage: NotApplicable { reason: "gitoxide does not yet have an 'advice' system" },
    },
    Record {
        config: "core.sparseCheckout",
        usage: Planned { note: Some("together with 'index.sparse' and 'core.sparseCheckoutCone', configures if the index should be written sparse or not") },
    },
    Record {
        config: "core.sparseCheckoutCone",
        usage: Planned { note: Some("non-cone mode is deprecated but should still fail gracefully if encountered") },
    },
    Record {
        config: "core.splitIndex",
        usage: NotPlanned { reason: "we don't want to be able to create split indices, but we will read them. It's (somewhat) superseded by sparse indices" },
    },
    Record {
        config: "splitIndex.maxPercentageChange",
        usage: NotPlanned { reason: "seems like it's superseded by sparse indices" },
    },
    Record {
        config: "splitIndex.sharedIndexExpire",
        usage: NotPlanned { reason: "seems like it's superseded by sparse indices" },
    },
    Record {
        config: "index.sparse",
        usage: Planned { note: Some("together with 'core.sparseCheckout' and 'core.sparseCheckoutCone', configures if the index should be written sparse or not") },
    },
    Record {
        config: "index.version",
        usage: Planned { note: Some("once V4 indices can be written, we need to be able to set a desired version. For now we write the smallest possible index version only.") },
    },
    Record {
        config: "http.<url>.*",
        usage: Planned { note: Some("definitely needed for correctness, testing against baseline is a must") }
    },
    Record {
        config: "http.proxySSLCert",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.proxySSLKey",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.proxySSLCertPasswordProtected",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.proxySSLCAInfo",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.emptyAuth",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.delegation",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.cookieFile",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.saveCookies",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.curloptResolve",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCipherList",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCipherList",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslVerify",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCert",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslKey",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCertPasswordProtected",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCertPasswordProtected",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCAPath",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslBackend",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.pinnedPubkey",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslTry",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.maxRequests",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.minSessions",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.postBuffer",
        usage: Planned { note: Some("relevant when implementing push, we should understand how memory allocation works when streaming") }
    },
    Record {
        config: "http.noEPSV",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.<url>.*",
        usage: Planned { note: Some("it's a vital part of git configuration. It's unclear how to get a baseline from git for this one.") }
    },
    Record {
        config: "init.templateDir",
        usage: NotPlanned { reason: "git expects this dir to be a valid git dir - I'd find additive template dirs more interesting, or changes done afterwards procedurally. Maybe this needs a 'init_or_open' semantic to be really useful" }
    },
    Record {
        config: "sparse.expectFilesOutsideOfPatterns",
        usage: NotPlanned { reason: "todo" },
    },
    Record {
        config: "remote.<name>.promisor",
        usage: Planned {
            note: Some("required for big monorepos, and typically used in conjunction with sparse indices")
        }
    },
    Record {
        config: "remote.<name>.partialCloneFilter",
        usage: Planned {
            note: Some("required for big monorepos, and typically used in conjunction with sparse indices")
        }
    },
    Record {
        config: "merge.renameLimit",
        usage: Planned { note: Some("The same as diff.renameLimit") }
    },
    Record {
        config: "merge.renames",
        usage: Planned { note: Some("The same as diff.renames") }
    },
    Record {
        config: "status.renameLimit",
        usage: Planned { note: Some("definitely needed to do status properly, even though it doesn't have to be there for day one. The same as diff.renameLimit") }
    },
    Record {
        config: "status.renames",
        usage: Planned { note: Some("the same as diff.renames") }
    },
    Record {
        config: "transfer.credentialsInUrl",
        usage: Planned { note: Some("currently we are likely to expose passwords in errors or in other places, and it's better to by default not do that") }
    },
    Record {
        config: "diff.*.textconv",
        usage: Planned { note: None }
    },
    Record {
        config: "diff.*.cachetextconv",
        usage: Planned { note: None }
    },
    Record {
        config: "diff.*.command",
        usage: Planned { note: None }
    },
];

/// A programmatic way to record and display progress.
pub fn show_progress() -> anyhow::Result<()> {
    let sorted = {
        let mut v: Vec<_> = GIT_CONFIG.into();
        v.extend(gix::config::Tree.sections().iter().flat_map(|section| {
            fn to_record(key: &dyn gix::config::tree::Key) -> Record {
                let config = key.logical_name();
                let note = key.note().map(|note| match note {
                    gix::config::tree::Note::Deviation(n) | gix::config::tree::Note::Informative(n) => n.to_string(),
                });
                let link = key.link().map(|link| match link {
                    gix::config::tree::Link::FallbackKey(key) => format!("fallback is '{}'", key.logical_name()),
                    gix::config::tree::Link::EnvironmentOverride(name) => format!("overridden by '{name}'"),
                });
                let deviation = match (note, link) {
                    (Some(n), Some(l)) => Some(format!("{n}. {l}")),
                    (Some(n), None) | (None, Some(n)) => Some(n),
                    (None, None) => None,
                }
                .map(|d| &*Box::leak(d.into_boxed_str()));
                Record {
                    config: Box::leak(config.into_boxed_str()),
                    usage: InUse { deviation },
                }
            }
            section
                .sub_sections()
                .iter()
                .flat_map(|sub_section| sub_section.keys().iter().map(|key| to_record(*key)))
                .chain(section.keys().iter().map(|key| to_record(*key)))
        }));
        v.sort_by_key(|r| r.config);
        v.dedup_by_key(|r| r.config);
        v
    };

    let mut buf = String::new();
    use std::fmt::Write;
    writeln!(&mut buf,
        "\nTotal records: {} ({perfect_icon} = {perfect}, {deviation_icon} = {deviation}, {planned_icon} = {planned}, {ondemand_icon} = {ondemand}, {not_applicable_icon} = {not_applicable})",
        sorted.len(),
        perfect_icon = InUse {
            deviation: None
        }
        .icon(),
        deviation_icon = InUse {
            deviation: Some("")
        }
        .icon(),
        planned_icon = Planned { note: None }.icon(),
        planned = sorted.iter().filter(|e| matches!(e.usage, Planned { .. })).count(),
        ondemand_icon = NotPlanned { reason: "" }.icon(),
        not_applicable_icon = NotApplicable { reason: "" }.icon(),
        perfect = sorted
            .iter()
            .filter(|e| matches!(e.usage, InUse { deviation, .. } if deviation.is_none()))
            .count(),
        deviation = sorted
            .iter()
            .filter(|e| matches!(e.usage, InUse { deviation, .. } if deviation.is_some()))
            .count(),
        ondemand = sorted
            .iter()
            .filter(|e| matches!(e.usage, NotPlanned { .. }))
            .count(),
        not_applicable = sorted
            .iter()
            .filter(|e| matches!(e.usage, NotApplicable { .. }))
            .count()
    )?;
    println!("{}", sorted.table().with(Style::blank()));
    println!("{buf}");
    Ok(())
}
