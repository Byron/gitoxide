use std::fmt::{Display, Formatter};

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
    /// The configuration is already effective and used (at least) in the given module `name`.
    InModule {
        name: &'static str,
        deviation: Option<&'static str>,
    },
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
            NotApplicable { .. } => "❌",
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

        vec![self.usage.icon().into(), buf.join("."), self.usage.to_string()]
    }

    fn headers() -> Vec<String> {
        vec![]
    }
}

static GIT_CONFIG: &[Record] = &[
    Record {
        config: "core.safeCRLF",
        usage: Planned { note: Some("safety is not optional") },
    },
    Record {
        config: "core.fileMode",
        usage: InModule {name: "config", deviation: None},
    },
    Record {
        config: "core.hideDotFiles",
        usage: Planned {note: Some("Seems useful, but needs demand from windows users")}
    },
    Record {
        config: "core.trustCTime",
        usage: Planned { note: Some("Needed for checkout - read from config but not used yet") },
    },
    Record {
        config: "core.checkStat",
        usage: Planned { note: Some("Needed for checkout - read from config but not used yet further down") },
    },
    Record {
        config: "core.symlinks",
        usage: InModule {name: "config", deviation: None},
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
        config: "core.deltaBaseCacheLimit",
        usage: InModule {
            name: "repository::cache",
            deviation: Some("if unset, we default to a small 64 slot fixed-size cache that holds at most 64 full delta base objects of any size. Overridable by 'GITOXIDE_PACK_CACHE_MEMORY'. Set to 0 to deactivate it entirely.")
        }
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
        config: "core.ignorecase",
        usage: InModule {name: "config", deviation: None}
    },
    Record {
        config: "core.precomposeUnicode",
        usage: InModule {name: "config", deviation: Some("This must be explicitly handled when data is coming into the program to fully work")}
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
        config: "checkout.workers",
        usage: InModule {name: "clone::checkout", deviation: Some("if unset, uses all cores instead of just one")},
    },
    Record {
        config: "checkout.thresholdForParallelism",
        usage: NotApplicable {reason: "parallelism is efficient enough to always run with benefit"},
    },
    Record {
        config: "feature.manyFile",
        usage: Planned {note: Some("big repositories are on the roadmap")},
    },
    Record {
        config: "core.preloadIndex",
        usage: Planned {note: Some("it's enabled by default and allows parallel stat checks - it's using a lot of CPU for just minor performance boosts though")},
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
        usage: Planned {note: Some("very relevant for doing the right thing during checkouts")},
    },
    Record {
        config: "branch.autoSetupRebase",
        usage: Planned {
            note: Some("for when we allow setting up upstream branches")
        },
    },
    Record {
        config: "branch.<name>.remote",
        usage: InModule {
            name: "reference::remote",
            deviation: None
        },
    },
    Record {
        config: "branch.<name>.pushRemote",
        usage: InModule {
            name: "reference::remote",
            deviation: None
        },
    },
    Record {
        config: "branch.<name>.merge",
        usage: InModule {
            name: "repository::config",
            deviation: None
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
        config: "core.attributesFile",
        usage: Planned {note: Some("for checkout - it's already queried but needs building of attributes group, and of course support during checkout")},
    },
    Record {
        config: "core.abbrev",
        usage: InModule {
            name: "config::cache",
            deviation: None,
        },
    },
    Record {
        config: "core.askPass",
        usage: InModule {
            name: "config::snapshot::credential_helpers",
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
        config: "diff.algorithm",
        usage: InModule {name: "config::cache::access", deviation: Some("'patience' diff is not implemented and can default to 'histogram' if lenient config is used, and defaults to histogram if unset for fastest and best results")},
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
        config: "extensions.worktreeconfig",
        usage: Planned {
            note: Some("Seems to be turned on when sparse indices are used")
        },
    },
    Record {
        config: "committer.name",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("overridden by 'GIT_COMMITTER_NAME'"),
        },
    },
    Record {
        config: "committer.email",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("overridden by 'GIT_COMMITTER_EMAIL'"),
        },
    },
    Record {
        config: "author.name",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("overridden by 'GIT_AUTHOR_NAME'"),
        },
    },
    Record {
        config: "author.email",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("overridden by 'GIT_AUTHOR_EMAIL'"),
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
        usage: NotApplicable {reason: "we don't support advices"},
    },
    Record {
        config: "fetch.output",
        usage: NotPlanned {reason: "'gix' might support it, but there is no intention on copying the 'git' CLI"},
    },
    Record {
        config: "fetch.negotiationAlgorithm",
        usage: Planned {
            note: Some("Implements our own 'naive' algorithm, only"),
        },
    },
    Record {
        config: "init.templateDir",
        usage: Planned {
            note: Some("copy non-hidden files from here into the GIT_DIR for support")
        },
    },
    Record {
        config: "init.defaultBranch",
        usage: InModule {
            name: "init",
            deviation: Some("If unset, we default to 'main' instead of 'master'")
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
        config: "http.proxy",
        usage: InModule { name: "repository::config::transport", deviation: Some("ignores strings with illformed UTF-8") }
    },
    Record {
        config: "http.extraHeader",
        usage: InModule { name: "repository::config::transport", deviation: Some("ignores strings with illformed UTF-8") }
    },
    Record {
        config: "http.proxyAuthMethod",
        usage: InModule { name: "repository::config::transport", deviation: Some("implemented like git, but I never tried it so who knows") },
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
        config: "http.version",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.curloptResolve",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.sslVersion",
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
        config: "http.sslCAInfo",
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
        config: "http.schannelCheckRevoke",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.schannelUseSSLCAInfo",
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
        config: "http.lowSpeedLimit",
        usage: InModule { name: "repository::config::transport", deviation: Some("fails on negative values") }
    },
    Record {
        config: "http.lowSpeedTime",
        usage: InModule { name: "repository::config::transport", deviation: Some("fails on negative values") }
    },
    Record {
        config: "http.userAgent",
        usage: InModule { name: "repository::config::transport", deviation: Some("ignores strings with illformed UTF-8") }
    },
    Record {
        config: "http.noEPSV",
        usage: NotPlanned { reason: "on demand" }
    },
    Record {
        config: "http.followRedirects",
        usage: InModule { name: "repository::config::transport", deviation: None }
    },
    Record {
        config: "http.<url>.*",
        usage: Planned { note: Some("it's a vital part of git configuration. It's unclear how to get a baseline from git for this one.") }
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
        config: "remote.<name>.proxy",
        usage: Planned {
            note: None
        }
    },
    Record {
        config: "remote.<name>.proxyAuthMethod",
        usage: Planned {
            note: None
        }
    },
    Record {
        config: "gitoxide.userAgent",
        usage: InModule {
            name: "config::cache",
            deviation: Some("The user agent presented on the git protocol layer, serving as fallback for when no http.userAgent is set.")
        }
    },
    Record {
        config: "gitoxide.https.proxy",
        usage: InModule {
            name: "repository::config::transport",
            deviation: Some("Used only if the url to access is https, created from 'HTTPS_PROXY' and 'https_proxy' env-vars")
        }
    },
    Record {
        config: "gitoxide.http.proxy",
        usage: InModule {
            name: "repository::config::transport",
            deviation: Some("created from 'http_proxy' env-var.")
        }
    },
    Record {
        config: "gitoxide.http.allProxy",
        usage: InModule {
            name: "repository::config::transport",
            deviation: Some("created from 'all_proxy' or 'ALL_PROXY' env-var.")
        }
    },
    Record {
        config: "gitoxide.http.verbose",
        usage: InModule {
            name: "repository::config::transport",
            deviation: Some("created from 'GIT_CURL_VERBOSE' to print debug output to stderr.")
        }
    },
    Record {
        config: "gitoxide.http.noProxy",
        usage: InModule {
            name: "repository::config::transport",
            deviation: Some("created from 'no_proxy' or 'NO_PROXY' env-var.")
        }
    },
    Record {
        config: "gitoxide.http.connectTimeout",
        usage: InModule {
            name: "repository::config::transport",
            deviation: Some("entirely new, and in milliseconds like all other timeout suffixed variables in the git config")
        }
    },
    Record {
        config: "gitoxide.allow.protocolFromUser",
        usage: InModule {
            name: "remote::url::scheme_permission",
            deviation: Some("corresponds to GIT_PROTOCOL_FROM_USER environment variable")
        }
    },
    Record {
        config: "gitoxide.objects.replaceRefBase",
        usage: InModule {
            name: "open",
            deviation: Some("corresponds to the GIT_REPLACE_REF_BASE environment variable")
        }
    },
    Record {
        config: "gitoxide.objects.noReplace",
        usage: InModule {
            name: "open",
            deviation: Some("corresponds to the GIT_NO_REPLACE_OBJECTS environment variable")
        }
    },
    Record {
        config: "gitoxide.commit.authorDate",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("corresponds to the GIT_AUTHOR_DATE environment variable")
        }
    },
    Record {
        config: "gitoxide.commit.committerDate",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("corresponds to the GIT_COMMITTER_DATE environment variable")
        }
    },
    Record {
        config: "gitoxide.author.nameFallback",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("corresponds to the GIT_AUTHOR_NAME environment variable and is a fallback for `author.name`")
        }
    },
    Record {
        config: "gitoxide.author.emailFallback",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("corresponds to the GIT_AUTHOR_EMAIL environment variable and is a fallback for `author.email`")
        }
    },
    Record {
        config: "gitoxide.committer.nameFallback",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("corresponds to the GIT_COMMITTER_NAME environment variable and is a fallback for `committer.name`")
        }
    },
    Record {
        config: "gitoxide.committer.emailFallback",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("corresponds to the GIT_COMMITTER_EMAIL environment variable and is a fallback for `committer.email`")
        }
    },
    Record {
        config: "gitoxide.user.emailFallback",
        usage: InModule {
            name: "repository::identity",
            deviation: Some("corresponds to the EMAIL environment variable and is a fallback for `user.email`")
        }
    },
    Record {
        config: "gitoxide.objects.cacheLimit",
        usage: InModule {
            name: "repository::cache",
            deviation: Some("corresponds to the GITOXIDE_OBJECT_CACHE_MEMORY environment variable. If unset or 0, there is no object cache")
        }
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
    println!(
        "\nTotal records: {} ({perfect_icon} = {perfect}, {deviation_icon} = {deviation}, {planned_icon} = {planned}, {ondemand_icon} = {ondemand}, {not_applicable_icon} = {not_applicable})",
        GIT_CONFIG.len(),
        perfect_icon = InModule {
            name: "",
            deviation: None
        }
        .icon(),
        deviation_icon = InModule {
            name: "",
            deviation: Some("")
        }
        .icon(),
        planned_icon = Planned { note: None }.icon(),
        planned = GIT_CONFIG.iter().filter(|e| matches!(e.usage, Planned { .. })).count(),
        ondemand_icon = NotPlanned { reason: "" }.icon(),
        not_applicable_icon = NotApplicable { reason: "" }.icon(),
        perfect = GIT_CONFIG
            .iter()
            .filter(|e| matches!(e.usage, InModule { deviation, .. } if deviation.is_none()))
            .count(),
        deviation = GIT_CONFIG
            .iter()
            .filter(|e| matches!(e.usage, InModule { deviation, .. } if deviation.is_some()))
            .count(),
        ondemand = GIT_CONFIG
            .iter()
            .filter(|e| matches!(e.usage, NotPlanned { .. }))
            .count(),
        not_applicable = GIT_CONFIG
            .iter()
            .filter(|e| matches!(e.usage, NotApplicable { .. }))
            .count()
    );
    Ok(())
}
