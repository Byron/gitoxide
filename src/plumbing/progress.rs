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

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Puzzled => f.write_str("❓")?,
            Self::NotApplicable { reason } => write!(f, "not applicable: {reason}")?,
            Self::NotPlanned { reason } => {
                write!(f, "{}", "not planned".blink())?;
                write!(f, " ℹ {} ℹ", reason.bright_white())?;
            }
            Self::Planned { note } => {
                write!(f, "{}", "planned".blink())?;
                if let Some(note) = note {
                    write!(f, " ℹ {} ℹ", note.bright_white())?;
                }
            }
            Self::InUse { deviation } => {
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
            Self::Puzzled => "?",
            Self::NotApplicable { .. } => "❌",
            Self::Planned { .. } => "🕒",
            Self::NotPlanned { .. } => "🤔",
            Self::InUse { deviation, .. } => {
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
        config: "core.safeCRLF",
        usage: Usage::Planned { note: Some("safety is not optional") },
    },
    Record {
        config: "core.hideDotFiles",
        usage: Usage::Planned {note: Some("Seems useful, but needs demand from windows users")}
    },
    Record {
        config: "core.packedGitWindowSize",
        usage: Usage::NotPlanned { reason: "an optimization for handling many large packs more efficiently seems unnecessary" }
    },
    Record {
        config: "core.packedGitLimit",
        usage: Usage::NotApplicable { reason: "we target 32bit systems only and don't use a windowing mechanism" }
    },
    Record {
        config: "core.alternateRefsCommand",
        usage: Usage::NotPlanned { reason: "there is no need as we can perform the required operation in-binary. This could happen though if there is a use-case and demand." }
    },
    Record {
        config: "core.alternateRefsPrefixes",
        usage: Usage::NotPlanned { reason: "seems like a niche feature, but can be implemented if there is demand" }
    },
    Record {
        config: "core.checkRoundtripEncoding",
        usage: Usage::Planned { note: Some("needed once working-tree-encoding attributes are supported") }
    },
    Record {
        config: "core.bigFileThreshold",
        usage: Usage::Planned { note: Some("unfortunately we can't stream packed files yet, even if not delta-compressed, but respecting the threshold for other operations is definitely a must") }
    },
    Record {
        config: "core.compression",
        usage: Usage::Planned { note: Some("Allow to remove similar hardcoded value - passing it through will be some effort") },
    },
    Record {
        config: "core.loosecompression",
        usage: Usage::Planned { note: None },
    },
    Record {
        config: "core.protectHFS",
        usage: Usage::Planned { note: Some("relevant for checkout on MacOS") },
    },
    Record {
        config: "core.protectNTFS",
        usage: Usage::NotPlanned { reason: "lack of demand"},
    },
    Record {
        config: "core.sparseCheckout",
        usage: Usage::Planned { note: Some("we want to support huge repos and be the fastest in doing so") },
    },
    Record {
        config: "core.sparseCheckoutCone",
        usage: Usage::Planned { note: Some("this is a nice improvement over spareCheckout alone and should one day be available too") },
    },
    Record {
        config: "checkout.defaultRemote",
        usage: Usage::Planned { note: Some("needed for correct checkout behaviour, similar to what git does") },
    },
    Record {
        config: "core.untrackedCache",
        usage: Usage::Planned { note: Some("needed for fast worktree operation") },
    },
    Record {
        config: "checkout.guess",
        usage: Usage::Planned { note: None },
    },
    Record {
        config: "checkout.thresholdForParallelism",
        usage: Usage::NotApplicable {reason: "parallelism is efficient enough to always run with benefit"},
    },
    Record {
        config: "feature.manyFile",
        usage: Usage::Planned {note: Some("big repositories are on the roadmap")},
    },
    Record {
        config: "core.preloadIndex",
        usage: Usage::Planned {note: Some("it's enabled by default and allows parallel stat checks - it's using a lot of CPU for just minor performance boosts though")},
    },
    Record {
        config: "commitGraph.generationVersion",
        usage: Usage::NotPlanned { reason: "couldn't find a test that would require corrected generation numbers, even `git` has no test for this." },
    },
    Record {
        config: "commitGraph.maxNewFilters",
        usage: Usage::NotPlanned { reason: "can be considered when the underlying feature is actually used or needed" },
    },
    Record {
        config: "commitGraph.readChangedPaths",
        usage: Usage::NotPlanned { reason: "can be considered when the underlying feature is actually used or needed" },
    },
    Record {
        config: "index.sparse",
        usage: Usage::Planned {note: Some("we can read sparse indices and support for it will be added early on")},
    },
    Record {
        config: "index.skipHash",
        usage: Usage::Planned {note: Some("important to not unnecessarily reject indices just because they are missing a hash (or it is null)")},
    },
    Record {
        config: "merge.renormalize",
        usage: Usage::Planned {note: Some("once merging is being implemented, renormalization should be respected")},
    },
    Record {
        config: "sparse.expectFilesOutsideOfPatterns",
        usage: Usage::Planned {note: Some("a feature definitely worth having")},
    },
    Record {
        config: "submodule.recurse",
        usage: Usage::Planned {note: Some("very relevant for doing the right thing during checkouts")},
    },
    Record {
        config: "branch.autoSetupRebase",
        usage: Usage::Planned {
            note: Some("for when we allow setting up upstream branches")
        },
    },
    Record {
        config: "branch.<name>.rebase",
        usage: Usage::Planned {
            note: Some("for when we can merge, rebase should be supported")
        },
    },
    Record {
        config: "branch.<name>.description",
        usage: Usage::NotPlanned {
            reason: "no plan to implement format-patch or request-pull summary"
        },
    },
    Record {
        config: "core.eol",
        usage: Usage::Planned {note: Some("needed for filters, but also for doing diffs correctly")}
    },
    Record {
        config: "core.fsync",
        usage: Usage::Planned {note: Some("more safety for disk write operations is a good thing, definitely on the server")}
    },
    Record {
        config: "core.fsyncMethod",
        usage: Usage::Planned {note: Some("needed to support `core.fsync`")}
    },
    Record {
        config: "core.sharedRepository",
        usage: Usage::NotPlanned {reason: "on demand"}
    },
    Record {
        config: "core.createObject",
        usage: Usage::NotPlanned {reason: "it's valuable not to do writes unless needed on the lowest level, but we hope to avoid issues by not writing duplicate objects in the first place"}
    },
    Record {
    config: "clone.filterSubmodules,",
        usage: Usage::Planned {
            note: Some("currently object filtering isn't support, a prerequisite for this, see --filter=blob:none for more"),
        },
    },
    Record {
        config: "clone.rejectShallow",
        usage: Usage::Planned {
            note: Some("probably trivial to implement once there is protocol support for shallow clones"),
        },
    },
    Record {
        config: "receive.shallowUpdate",
        usage: Usage::NotPlanned {
            reason: "it looks like a server-only setting that allows boundaries to change if refs are pushed that are outside of the boundary.",
        },
    },
    Record {
        config: "fetch.recurseSubmodules",
        usage: Usage::Planned {
            note: Some("Seems useful for cargo as well"),
        },
    },
    Record {
        config: "fetch.fsckObjects",
        usage: Usage::Puzzled,
    },
    Record {
        config: "fetch.fsck.<msg-id>",
        usage: Usage::Puzzled,
    },
    Record {
        config: "fetch.fsck.skipList",
        usage: Usage::Puzzled,
    },
    Record {
        config: "fetch.unpackLimit",
        usage: Usage::Planned { note: None },
    },
    Record {
        config: "fetch.prune",
        usage: Usage::Planned { note: None },
    },
    Record {
        config: "fetch.pruneTags",
        usage: Usage::Planned { note: None },
    },
    Record {
        config: "fetch.writeCommitGraph",
        usage: Usage::Planned { note: None },
    },
    Record {
        config: "fetch.parallel",
        usage: Usage::Planned { note: None },
    },
    Record {
        config: "fetch.showForcedUpdates",
        usage: Usage::NotApplicable {reason: "we don't support advices"},
    },
    Record {
        config: "fetch.output",
        usage: Usage::NotPlanned {reason: "'gix' might support it, but there is no intention on copying the 'git' CLI"},
    },
    Record {
        config: "remotes.<group>",
        usage: Usage::Planned {
            note: Some("useful for multi-remote fetches as part of the standard API, maybe just `group(name) -> Option<Vec<Remote>>`"),
        },
    },
    Record {
        config: "advice.updateSparsePath",
        usage: Usage::NotApplicable { reason: "gitoxide does not yet have an 'advice' system" },
    },
    Record {
        config: "core.sparseCheckout",
        usage: Usage::Planned { note: Some("together with 'index.sparse' and 'core.sparseCheckoutCone', configures if the index should be written sparse or not") },
    },
    Record {
        config: "core.sparseCheckoutCone",
        usage: Usage::Planned { note: Some("non-cone mode is deprecated but should still fail gracefully if encountered") },
    },
    Record {
        config: "core.splitIndex",
        usage: Usage::NotPlanned { reason: "we don't want to be able to create split indices, but we will read them. It's (somewhat) superseded by sparse indices" },
    },
    Record {
        config: "splitIndex.maxPercentageChange",
        usage: Usage::NotPlanned { reason: "seems like it's superseded by sparse indices" },
    },
    Record {
        config: "splitIndex.sharedIndexExpire",
        usage: Usage::NotPlanned { reason: "seems like it's superseded by sparse indices" },
    },
    Record {
        config: "index.sparse",
        usage: Usage::Planned { note: Some("together with 'core.sparseCheckout' and 'core.sparseCheckoutCone', configures if the index should be written sparse or not") },
    },
    Record {
        config: "index.version",
        usage: Usage::Planned { note: Some("once V4 indices can be written, we need to be able to set a desired version. For now we write the smallest possible index version only.") },
    },
    Record {
        config: "http.<url>.*",
        usage: Usage::Planned { note: Some("definitely needed for correctness, testing against baseline is a must") }
    },
    Record {
        config: "http.proxySSLCert",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.proxySSLKey",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.proxySSLCertPasswordProtected",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.proxySSLCAInfo",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.emptyAuth",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.delegation",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.cookieFile",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.saveCookies",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.curloptResolve",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCipherList",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCipherList",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslVerify",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCert",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslKey",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCertPasswordProtected",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCertPasswordProtected",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslCAPath",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslBackend",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.pinnedPubkey",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslTry",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.maxRequests",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.minSessions",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.postBuffer",
        usage: Usage::Planned { note: Some("relevant when implementing push, we should understand how memory allocation works when streaming") }
    },
    Record {
        config: "http.noEPSV",
        usage: Usage::NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.<url>.*",
        usage: Usage::Planned { note: Some("it's a vital part of git configuration. It's unclear how to get a baseline from git for this one.") }
    },
    Record {
        config: "init.templateDir",
        usage: Usage::NotPlanned { reason: "git expects this dir to be a valid git dir - I'd find additive template dirs more interesting, or changes done afterwards procedurally. Maybe this needs a 'init_or_open' semantic to be really useful" }
    },
    Record {
        config: "sparse.expectFilesOutsideOfPatterns",
        usage: Usage::NotPlanned { reason: "todo" },
    },
    Record {
        config: "remote.<name>.promisor",
        usage: Usage::Planned {
            note: Some("required for big monorepos, and typically used in conjunction with sparse indices")
        }
    },
    Record {
        config: "remote.<name>.partialCloneFilter",
        usage: Usage::Planned {
            note: Some("required for big monorepos, and typically used in conjunction with sparse indices")
        }
    },
    Record {
        config: "merge.renameLimit",
        usage: Usage::Planned { note: Some("The same as diff.renameLimit") }
    },
    Record {
        config: "merge.renames",
        usage: Usage::Planned { note: Some("The same as diff.renames") }
    },
    Record {
        config: "status.renameLimit",
        usage: Usage::Planned { note: Some("definitely needed to do status properly, even though it doesn't have to be there for day one. The same as diff.renameLimit") }
    },
    Record {
        config: "status.renames",
        usage: Usage::Planned { note: Some("the same as diff.renames") }
    },
    Record {
        config: "diff.*.textconv",
        usage: Usage::Planned { note: None }
    },
    Record {
        config: "diff.*.cachetextconv",
        usage: Usage::Planned { note: None }
    },
    Record {
        config: "diff.*.command",
        usage: Usage::Planned { note: None }
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
                    usage: Usage::InUse { deviation },
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
        perfect_icon = Usage::InUse {
            deviation: None
        }
        .icon(),
        deviation_icon = Usage::InUse {
            deviation: Some("")
        }
        .icon(),
        planned_icon = Usage::Planned { note: None }.icon(),
        planned = sorted.iter().filter(|e| matches!(e.usage, Usage::Planned { .. })).count(),
        ondemand_icon = Usage::NotPlanned { reason: "" }.icon(),
        not_applicable_icon = Usage::NotApplicable { reason: "" }.icon(),
        perfect = sorted
            .iter()
            .filter(|e| matches!(e.usage, Usage::InUse { deviation, .. } if deviation.is_none()))
            .count(),
        deviation = sorted
            .iter()
            .filter(|e| matches!(e.usage, Usage::InUse { deviation, .. } if deviation.is_some()))
            .count(),
        ondemand = sorted
            .iter()
            .filter(|e| matches!(e.usage, Usage::NotPlanned { .. }))
            .count(),
        not_applicable = sorted
            .iter()
            .filter(|e| matches!(e.usage, Usage::NotApplicable { .. }))
            .count()
    )?;
    println!("{}", sorted.table().with(Style::blank()));
    println!("{buf}");
    Ok(())
}
