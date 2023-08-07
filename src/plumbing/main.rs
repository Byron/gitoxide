use std::{
    io::{stdin, BufReader},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use anyhow::{Context, Result};
use clap::Parser;
use gitoxide_core as core;
use gitoxide_core::pack::verify;
use gix::bstr::io::BufReadExt;

use crate::{
    plumbing::{
        options::{
            attributes, commit, commitgraph, config, credential, exclude, free, index, mailmap, odb, revision, tree,
            Args, Subcommands,
        },
        show_progress,
    },
    shared::pretty::prepare_and_run,
};

#[cfg(feature = "gitoxide-core-async-client")]
pub mod async_util {
    use crate::shared::ProgressRange;

    #[cfg(not(feature = "prodash-render-line"))]
    compile_error!("BUG: Need at least a line renderer in async mode");

    pub fn prepare(
        verbose: bool,
        name: &str,
        range: impl Into<Option<ProgressRange>>,
    ) -> (
        Option<prodash::render::line::JoinHandle>,
        gix_features::progress::DoOrDiscard<prodash::tree::Item>,
    ) {
        use crate::shared::{self, STANDARD_RANGE};
        shared::init_env_logger();

        if verbose {
            let progress = shared::progress_tree();
            let sub_progress = progress.add_child(name);
            let ui_handle = shared::setup_line_renderer_range(&progress, range.into().unwrap_or(STANDARD_RANGE));
            (Some(ui_handle), Some(sub_progress).into())
        } else {
            (None, None.into())
        }
    }
}

pub fn main() -> Result<()> {
    let args: Args = Args::parse_from(gix::env::args_os());
    #[allow(unsafe_code)]
    unsafe {
        // SAFETY: we don't manipulate the environment from any thread
        time::util::local_offset::set_soundness(time::util::local_offset::Soundness::Unsound);
    }
    let thread_limit = args.threads;
    let verbose = args.verbose;
    let format = args.format;
    let cmd = args.cmd;
    #[cfg_attr(not(feature = "tracing"), allow(unused_mut))]
    #[cfg_attr(feature = "tracing", allow(unused_assignments))]
    let mut trace = false;
    #[cfg(feature = "tracing")]
    {
        trace = args.trace
    }
    let object_hash = args.object_hash;
    let config = args.config;
    let repository = args.repository;
    enum Mode {
        Strict,
        StrictWithGitInstallConfig,
        Lenient,
        LenientWithGitInstallConfig,
    }

    let repository = {
        let config = config.clone();
        move |mode: Mode| -> Result<gix::Repository> {
            let mut mapping: gix::sec::trust::Mapping<gix::open::Options> = Default::default();
            let strict_toggle = matches!(mode, Mode::Strict | Mode::StrictWithGitInstallConfig) || args.strict;
            mapping.full = mapping.full.strict_config(strict_toggle);
            mapping.reduced = mapping.reduced.strict_config(strict_toggle);
            let git_installation = matches!(
                mode,
                Mode::StrictWithGitInstallConfig | Mode::LenientWithGitInstallConfig
            );
            let to_match_settings = |mut opts: gix::open::Options| {
                opts.permissions.config.git_binary = git_installation;
                opts.permissions.attributes.git_binary = git_installation;
                if config.is_empty() {
                    opts
                } else {
                    opts.cli_overrides(config.clone())
                }
            };
            mapping.full.modify(to_match_settings);
            mapping.reduced.modify(to_match_settings);
            let mut repo = gix::ThreadSafeRepository::discover_opts(repository, Default::default(), mapping)
                .map(gix::Repository::from)?;
            if !config.is_empty() {
                repo.config_snapshot_mut()
                    .append_config(config.iter(), gix::config::Source::Cli)
                    .context("Unable to parse command-line configuration")?;
            }
            Ok(repo)
        }
    };

    let progress;
    let progress_keep_open;
    #[cfg(feature = "prodash-render-tui")]
    {
        progress = args.progress;
        progress_keep_open = args.progress_keep_open;
    }
    #[cfg(not(feature = "prodash-render-tui"))]
    {
        progress = false;
        progress_keep_open = false;
    }
    let auto_verbose = !progress && !args.no_verbose;

    let should_interrupt = Arc::new(AtomicBool::new(false));
    gix::interrupt::init_handler({
        let should_interrupt = Arc::clone(&should_interrupt);
        move || should_interrupt.store(true, Ordering::SeqCst)
    })?;

    match cmd {
        #[cfg(feature = "gitoxide-core-tools-archive")]
        Subcommands::Archive(crate::plumbing::options::archive::Platform {
            format,
            prefix,
            compression_level,
            add_path,
            add_virtual_file,
            output_file,
            treeish,
        }) => prepare_and_run(
            "archive",
            trace,
            auto_verbose,
            progress,
            progress_keep_open,
            None,
            move |progress, _out, _err| {
                if add_virtual_file.len() % 2 != 0 {
                    anyhow::bail!(
                        "Virtual files must be specified in pairs of two: slash/separated/path content, got {}",
                        add_virtual_file.join(", ")
                    )
                }
                core::repository::archive::stream(
                    repository(Mode::Lenient)?,
                    &output_file,
                    treeish.as_deref(),
                    progress,
                    core::repository::archive::Options {
                        add_paths: add_path,
                        prefix,
                        files: add_virtual_file
                            .chunks(2)
                            .map(|c| (c[0].to_owned(), c[1].clone()))
                            .collect(),
                        format: format.map(|f| match f {
                            crate::plumbing::options::archive::Format::Internal => {
                                gix::worktree::archive::Format::InternalTransientNonPersistable
                            }
                            crate::plumbing::options::archive::Format::Tar => gix::worktree::archive::Format::Tar,
                            crate::plumbing::options::archive::Format::TarGz => {
                                gix::worktree::archive::Format::TarGz { compression_level }
                            }
                            crate::plumbing::options::archive::Format::Zip => {
                                gix::worktree::archive::Format::Zip { compression_level }
                            }
                        }),
                    },
                )
            },
        ),
        #[cfg(feature = "gitoxide-core-tools-corpus")]
        Subcommands::Corpus(crate::plumbing::options::corpus::Platform { db, path, cmd }) => {
            let reverse_trace_lines = progress;
            prepare_and_run(
                "corpus",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                core::corpus::PROGRESS_RANGE,
                move |progress, _out, _err| {
                    let mut engine = core::corpus::Engine::open_or_create(
                        db,
                        core::corpus::engine::State {
                            gitoxide_version: option_env!("GITOXIDE_VERSION")
                                .ok_or_else(|| anyhow::anyhow!("GITOXIDE_VERSION must be set in build-script"))?
                                .into(),
                            progress,
                            trace_to_progress: trace,
                            reverse_trace_lines,
                        },
                    )?;
                    match cmd {
                        crate::plumbing::options::corpus::SubCommands::Run {
                            dry_run,
                            repo_sql_suffix,
                            include_task,
                        } => engine.run(path, thread_limit, dry_run, repo_sql_suffix, include_task),
                        crate::plumbing::options::corpus::SubCommands::Refresh => engine.refresh(path),
                    }
                },
            )
        }
        Subcommands::CommitGraph(cmd) => match cmd {
            commitgraph::Subcommands::List { spec } => prepare_and_run(
                "commitgraph-list",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    core::repository::commitgraph::list(repository(Mode::Lenient)?, spec, out, format)
                },
            )
            .map(|_| ()),
            commitgraph::Subcommands::Verify { statistics } => prepare_and_run(
                "commitgraph-verify",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    let output_statistics = if statistics { Some(format) } else { None };
                    core::repository::commitgraph::verify(
                        repository(Mode::Lenient)?,
                        core::repository::commitgraph::verify::Context {
                            err,
                            out,
                            output_statistics,
                        },
                    )
                },
            )
            .map(|_| ()),
        },
        #[cfg(feature = "gitoxide-core-blocking-client")]
        Subcommands::Clone(crate::plumbing::options::clone::Platform {
            handshake_info,
            bare,
            no_tags,
            remote,
            shallow,
            directory,
        }) => {
            let opts = core::repository::clone::Options {
                format,
                bare,
                handshake_info,
                no_tags,
                shallow: shallow.into(),
            };
            prepare_and_run(
                "clone",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                core::repository::clone::PROGRESS_RANGE,
                move |progress, out, err| core::repository::clone(remote, directory, config, progress, out, err, opts),
            )
        }
        #[cfg(feature = "gitoxide-core-blocking-client")]
        Subcommands::Fetch(crate::plumbing::options::fetch::Platform {
            dry_run,
            handshake_info,
            negotiation_info,
            open_negotiation_graph,
            remote,
            shallow,
            ref_spec,
        }) => {
            let opts = core::repository::fetch::Options {
                format,
                dry_run,
                remote,
                handshake_info,
                negotiation_info,
                open_negotiation_graph,
                shallow: shallow.into(),
                ref_specs: ref_spec,
            };
            prepare_and_run(
                "fetch",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                core::repository::fetch::PROGRESS_RANGE,
                move |progress, out, err| {
                    core::repository::fetch(repository(Mode::LenientWithGitInstallConfig)?, progress, out, err, opts)
                },
            )
        }
        Subcommands::ConfigTree => show_progress(),
        Subcommands::Credential(cmd) => core::repository::credential(
            repository(Mode::StrictWithGitInstallConfig)?,
            match cmd {
                credential::Subcommands::Fill => gix::credentials::program::main::Action::Get,
                credential::Subcommands::Approve => gix::credentials::program::main::Action::Store,
                credential::Subcommands::Reject => gix::credentials::program::main::Action::Erase,
            },
        ),
        #[cfg(any(feature = "gitoxide-core-async-client", feature = "gitoxide-core-blocking-client"))]
        Subcommands::Remote(crate::plumbing::options::remote::Platform {
            name,
            cmd,
            handshake_info,
        }) => {
            use crate::plumbing::options::remote;
            match cmd {
                remote::Subcommands::Refs | remote::Subcommands::RefMap { .. } => {
                    let kind = match cmd {
                        remote::Subcommands::Refs => core::repository::remote::refs::Kind::Remote,
                        remote::Subcommands::RefMap {
                            ref_spec,
                            show_unmapped_remote_refs,
                        } => core::repository::remote::refs::Kind::Tracking {
                            ref_specs: ref_spec,
                            show_unmapped_remote_refs,
                        },
                    };
                    let context = core::repository::remote::refs::Options {
                        name_or_url: name,
                        format,
                        handshake_info,
                    };
                    #[cfg(feature = "gitoxide-core-blocking-client")]
                    {
                        prepare_and_run(
                            "remote-refs",
                            trace,
                            auto_verbose,
                            progress,
                            progress_keep_open,
                            core::repository::remote::refs::PROGRESS_RANGE,
                            move |progress, out, err| {
                                core::repository::remote::refs(
                                    repository(Mode::LenientWithGitInstallConfig)?,
                                    kind,
                                    progress,
                                    out,
                                    err,
                                    context,
                                )
                            },
                        )
                    }
                    #[cfg(feature = "gitoxide-core-async-client")]
                    {
                        let (_handle, progress) = async_util::prepare(
                            auto_verbose,
                            "remote-refs",
                            Some(core::repository::remote::refs::PROGRESS_RANGE),
                        );
                        futures_lite::future::block_on(core::repository::remote::refs(
                            repository(Mode::LenientWithGitInstallConfig)?,
                            kind,
                            progress,
                            std::io::stdout(),
                            std::io::stderr(),
                            context,
                        ))
                    }
                }
            }
        }
        Subcommands::Config(config::Platform { filter }) => prepare_and_run(
            "config-list",
            trace,
            verbose,
            progress,
            progress_keep_open,
            None,
            move |_progress, out, _err| {
                core::repository::config::list(
                    repository(Mode::LenientWithGitInstallConfig)?,
                    filter,
                    config,
                    format,
                    out,
                )
            },
        )
        .map(|_| ()),
        Subcommands::Free(subcommands) => match subcommands {
            free::Subcommands::CommitGraph(cmd) => match cmd {
                free::commitgraph::Subcommands::Verify { path, statistics } => prepare_and_run(
                    "commitgraph-verify",
                    trace,
                    auto_verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |_progress, out, err| {
                        let output_statistics = if statistics { Some(format) } else { None };
                        core::commitgraph::verify(
                            path,
                            core::commitgraph::verify::Context {
                                err,
                                out,
                                output_statistics,
                            },
                        )
                    },
                )
                .map(|_| ()),
            },
            free::Subcommands::Index(free::index::Platform {
                object_hash,
                index_path,
                cmd,
            }) => match cmd {
                free::index::Subcommands::FromList {
                    force,
                    index_output_path,
                    file,
                } => prepare_and_run(
                    "index-from-list",
                    trace,
                    verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |_progress, _out, _err| core::repository::index::from_list(file, index_output_path, force),
                ),
                free::index::Subcommands::CheckoutExclusive {
                    directory,
                    empty_files,
                    repository,
                    keep_going,
                } => prepare_and_run(
                    "index-checkout",
                    trace,
                    auto_verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |progress, _out, err| {
                        core::index::checkout_exclusive(
                            index_path,
                            directory,
                            repository,
                            err,
                            progress,
                            &should_interrupt,
                            core::index::checkout_exclusive::Options {
                                index: core::index::Options { object_hash, format },
                                empty_files,
                                keep_going,
                                thread_limit,
                            },
                        )
                    },
                ),
                free::index::Subcommands::Info { no_details } => prepare_and_run(
                    "index-entries",
                    trace,
                    verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |_progress, out, err| {
                        core::index::information(
                            index_path,
                            out,
                            err,
                            core::index::information::Options {
                                index: core::index::Options { object_hash, format },
                                extension_details: !no_details,
                            },
                        )
                    },
                ),
                free::index::Subcommands::Verify => prepare_and_run(
                    "index-verify",
                    trace,
                    auto_verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |_progress, out, _err| {
                        core::index::verify(index_path, out, core::index::Options { object_hash, format })
                    },
                ),
            },
            free::Subcommands::Mailmap {
                cmd: free::mailmap::Platform { path, cmd },
            } => match cmd {
                free::mailmap::Subcommands::Verify => prepare_and_run(
                    "mailmap-verify",
                    trace,
                    auto_verbose,
                    progress,
                    progress_keep_open,
                    core::mailmap::PROGRESS_RANGE,
                    move |_progress, out, _err| core::mailmap::verify(path, format, out),
                ),
            },
            free::Subcommands::Pack(subcommands) => match subcommands {
                free::pack::Subcommands::Create {
                    repository,
                    expansion,
                    thin,
                    statistics,
                    nondeterministic_count,
                    tips,
                    pack_cache_size_mb,
                    counting_threads,
                    object_cache_size_mb,
                    output_directory,
                } => {
                    let has_tips = !tips.is_empty();
                    prepare_and_run(
                        "pack-create",
                        trace,
                        verbose,
                        progress,
                        progress_keep_open,
                        core::pack::create::PROGRESS_RANGE,
                        move |progress, out, _err| {
                            let input = if has_tips { None } else { stdin_or_bail()?.into() };
                            let repository = repository.unwrap_or_else(|| PathBuf::from("."));
                            let context = core::pack::create::Context {
                                thread_limit,
                                thin,
                                nondeterministic_thread_count: nondeterministic_count.then_some(counting_threads),
                                pack_cache_size_in_bytes: pack_cache_size_mb.unwrap_or(0) * 1_000_000,
                                object_cache_size_in_bytes: object_cache_size_mb.unwrap_or(0) * 1_000_000,
                                statistics: if statistics { Some(format) } else { None },
                                out,
                                expansion: expansion.unwrap_or(if has_tips {
                                    core::pack::create::ObjectExpansion::TreeTraversal
                                } else {
                                    core::pack::create::ObjectExpansion::None
                                }),
                            };
                            core::pack::create(repository, tips, input, output_directory, progress, context)
                        },
                    )
                }
                #[cfg(feature = "gitoxide-core-async-client")]
                free::pack::Subcommands::Receive {
                    protocol,
                    url,
                    directory,
                    refs,
                    refs_directory,
                } => {
                    let (_handle, progress) =
                        async_util::prepare(verbose, "pack-receive", core::pack::receive::PROGRESS_RANGE);
                    let fut = core::pack::receive(
                        protocol,
                        &url,
                        directory,
                        refs_directory,
                        refs.into_iter().map(|s| s.into()).collect(),
                        progress,
                        core::pack::receive::Context {
                            thread_limit,
                            format,
                            out: std::io::stdout(),
                            should_interrupt,
                            object_hash,
                        },
                    );
                    return futures_lite::future::block_on(fut);
                }
                #[cfg(feature = "gitoxide-core-blocking-client")]
                free::pack::Subcommands::Receive {
                    protocol,
                    url,
                    directory,
                    refs,
                    refs_directory,
                } => prepare_and_run(
                    "pack-receive",
                    trace,
                    verbose,
                    progress,
                    progress_keep_open,
                    core::pack::receive::PROGRESS_RANGE,
                    move |progress, out, _err| {
                        core::pack::receive(
                            protocol,
                            &url,
                            directory,
                            refs_directory,
                            refs.into_iter().map(Into::into).collect(),
                            progress,
                            core::pack::receive::Context {
                                thread_limit,
                                format,
                                should_interrupt,
                                out,
                                object_hash,
                            },
                        )
                    },
                ),
                free::pack::Subcommands::Explode {
                    check,
                    sink_compress,
                    delete_pack,
                    pack_path,
                    object_path,
                    verify,
                } => prepare_and_run(
                    "pack-explode",
                    trace,
                    auto_verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |progress, _out, _err| {
                        core::pack::explode::pack_or_pack_index(
                            pack_path,
                            object_path,
                            check,
                            progress,
                            core::pack::explode::Context {
                                thread_limit,
                                delete_pack,
                                sink_compress,
                                verify,
                                should_interrupt,
                                object_hash,
                            },
                        )
                    },
                ),
                free::pack::Subcommands::Verify {
                    args:
                        free::pack::VerifyOptions {
                            algorithm,
                            decode,
                            re_encode,
                            statistics,
                        },
                    path,
                } => prepare_and_run(
                    "pack-verify",
                    trace,
                    auto_verbose,
                    progress,
                    progress_keep_open,
                    verify::PROGRESS_RANGE,
                    move |progress, out, err| {
                        let mode = verify_mode(decode, re_encode);
                        let output_statistics = if statistics { Some(format) } else { None };
                        verify::pack_or_pack_index(
                            path,
                            progress,
                            verify::Context {
                                output_statistics,
                                out,
                                err,
                                thread_limit,
                                mode,
                                algorithm,
                                should_interrupt: &should_interrupt,
                                object_hash,
                            },
                        )
                    },
                )
                .map(|_| ()),
                free::pack::Subcommands::MultiIndex(free::pack::multi_index::Platform { multi_index_path, cmd }) => {
                    match cmd {
                        free::pack::multi_index::Subcommands::Entries => prepare_and_run(
                            "pack-multi-index-entries",
                            trace,
                            verbose,
                            progress,
                            progress_keep_open,
                            core::pack::multi_index::PROGRESS_RANGE,
                            move |_progress, out, _err| core::pack::multi_index::entries(multi_index_path, format, out),
                        ),
                        free::pack::multi_index::Subcommands::Info => prepare_and_run(
                            "pack-multi-index-info",
                            trace,
                            verbose,
                            progress,
                            progress_keep_open,
                            core::pack::multi_index::PROGRESS_RANGE,
                            move |_progress, out, err| {
                                core::pack::multi_index::info(multi_index_path, format, out, err)
                            },
                        ),
                        free::pack::multi_index::Subcommands::Verify => prepare_and_run(
                            "pack-multi-index-verify",
                            trace,
                            auto_verbose,
                            progress,
                            progress_keep_open,
                            core::pack::multi_index::PROGRESS_RANGE,
                            move |progress, _out, _err| {
                                core::pack::multi_index::verify(multi_index_path, progress, &should_interrupt)
                            },
                        ),
                        free::pack::multi_index::Subcommands::Create { index_paths } => prepare_and_run(
                            "pack-multi-index-create",
                            trace,
                            verbose,
                            progress,
                            progress_keep_open,
                            core::pack::multi_index::PROGRESS_RANGE,
                            move |progress, _out, _err| {
                                core::pack::multi_index::create(
                                    index_paths,
                                    multi_index_path,
                                    progress,
                                    &should_interrupt,
                                    object_hash,
                                )
                            },
                        ),
                    }
                }
                free::pack::Subcommands::Index(subcommands) => match subcommands {
                    free::pack::index::Subcommands::Create {
                        iteration_mode,
                        pack_path,
                        directory,
                    } => prepare_and_run(
                        "pack-index-create",
                        trace,
                        verbose,
                        progress,
                        progress_keep_open,
                        core::pack::index::PROGRESS_RANGE,
                        move |progress, out, _err| {
                            use gitoxide_core::pack::index::PathOrRead;
                            let input = if let Some(path) = pack_path {
                                PathOrRead::Path(path)
                            } else {
                                use is_terminal::IsTerminal;
                                if std::io::stdin().is_terminal() {
                                    anyhow::bail!(
                                        "Refusing to read from standard input as no path is given, but it's a terminal."
                                    )
                                }
                                PathOrRead::Read(Box::new(stdin()))
                            };
                            core::pack::index::from_pack(
                                input,
                                directory,
                                progress,
                                core::pack::index::Context {
                                    thread_limit,
                                    iteration_mode,
                                    format,
                                    out,
                                    object_hash,
                                    should_interrupt: &gix::interrupt::IS_INTERRUPTED,
                                },
                            )
                        },
                    ),
                },
            },
        },
        Subcommands::Verify {
            args:
                free::pack::VerifyOptions {
                    statistics,
                    algorithm,
                    decode,
                    re_encode,
                },
        } => prepare_and_run(
            "verify",
            trace,
            auto_verbose,
            progress,
            progress_keep_open,
            core::repository::verify::PROGRESS_RANGE,
            move |progress, out, _err| {
                core::repository::verify::integrity(
                    repository(Mode::Strict)?,
                    out,
                    progress,
                    &should_interrupt,
                    core::repository::verify::Context {
                        output_statistics: statistics.then_some(format),
                        algorithm,
                        verify_mode: verify_mode(decode, re_encode),
                        thread_limit,
                    },
                )
            },
        ),
        Subcommands::Revision(cmd) => match cmd {
            revision::Subcommands::List { spec, svg, limit } => prepare_and_run(
                "revision-list",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                core::repository::revision::list::PROGRESS_RANGE,
                move |progress, out, _err| {
                    core::repository::revision::list(
                        repository(Mode::Lenient)?,
                        progress,
                        out,
                        core::repository::revision::list::Context {
                            limit,
                            spec,
                            format,
                            text: svg.map_or(core::repository::revision::list::Format::Text, |path| {
                                core::repository::revision::list::Format::Svg { path }
                            }),
                        },
                    )
                },
            ),
            revision::Subcommands::PreviousBranches => prepare_and_run(
                "revision-previousbranches",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    core::repository::revision::previous_branches(repository(Mode::Lenient)?, out, format)
                },
            ),
            revision::Subcommands::Explain { spec } => prepare_and_run(
                "revision-explain",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| core::repository::revision::explain(spec, out),
            ),
            revision::Subcommands::Resolve {
                specs,
                explain,
                cat_file,
            } => prepare_and_run(
                "revision-parse",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    core::repository::revision::resolve(
                        repository(Mode::Strict)?,
                        specs,
                        out,
                        core::repository::revision::resolve::Options {
                            format,
                            explain,
                            cat_file,
                        },
                    )
                },
            ),
        },
        Subcommands::Commit(cmd) => match cmd {
            commit::Subcommands::Describe {
                annotated_tags,
                all_refs,
                first_parent,
                always,
                long,
                statistics,
                max_candidates,
                rev_spec,
            } => prepare_and_run(
                "commit-describe",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    core::repository::commit::describe(
                        repository(Mode::Strict)?,
                        rev_spec.as_deref(),
                        out,
                        err,
                        core::repository::commit::describe::Options {
                            all_tags: !annotated_tags,
                            all_refs,
                            long_format: long,
                            first_parent,
                            statistics,
                            max_candidates,
                            always,
                        },
                    )
                },
            ),
        },
        Subcommands::Tree(cmd) => match cmd {
            tree::Subcommands::Entries {
                treeish,
                recursive,
                extended,
            } => prepare_and_run(
                "tree-entries",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    core::repository::tree::entries(
                        repository(Mode::Strict)?,
                        treeish.as_deref(),
                        recursive,
                        extended,
                        format,
                        out,
                    )
                },
            ),
            tree::Subcommands::Info { treeish, extended } => prepare_and_run(
                "tree-info",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    core::repository::tree::info(
                        repository(Mode::Strict)?,
                        treeish.as_deref(),
                        extended,
                        format,
                        out,
                        err,
                    )
                },
            ),
        },
        Subcommands::Odb(cmd) => match cmd {
            odb::Subcommands::Stats => prepare_and_run(
                "odb-stats",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                core::repository::odb::statistics::PROGRESS_RANGE,
                move |progress, out, err| {
                    core::repository::odb::statistics(
                        repository(Mode::Strict)?,
                        progress,
                        out,
                        err,
                        core::repository::odb::statistics::Options { format, thread_limit },
                    )
                },
            ),
            odb::Subcommands::Entries => prepare_and_run(
                "odb-entries",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| core::repository::odb::entries(repository(Mode::Strict)?, format, out),
            ),
            odb::Subcommands::Info => prepare_and_run(
                "odb-info",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| core::repository::odb::info(repository(Mode::Strict)?, format, out, err),
            ),
        },
        Subcommands::Mailmap(cmd) => match cmd {
            mailmap::Subcommands::Entries => prepare_and_run(
                "mailmap-entries",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    core::repository::mailmap::entries(repository(Mode::Lenient)?, format, out, err)
                },
            ),
        },
        Subcommands::Attributes(cmd) => match cmd {
            attributes::Subcommands::Query { statistics, pathspecs } => prepare_and_run(
                "attributes-query",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    use gix::bstr::ByteSlice;
                    core::repository::attributes::query(
                        repository(Mode::Strict)?,
                        if pathspecs.is_empty() {
                            Box::new(
                                stdin_or_bail()?
                                    .byte_lines()
                                    .filter_map(Result::ok)
                                    .filter_map(|line| gix::path::Spec::from_bytes(line.as_bstr())),
                            ) as Box<dyn Iterator<Item = gix::path::Spec>>
                        } else {
                            Box::new(pathspecs.into_iter())
                        },
                        out,
                        err,
                        core::repository::attributes::query::Options { format, statistics },
                    )
                },
            ),
            attributes::Subcommands::ValidateBaseline { statistics, no_ignore } => prepare_and_run(
                "attributes-validate-baseline",
                trace,
                auto_verbose,
                progress,
                progress_keep_open,
                None,
                move |progress, out, err| {
                    use gix::bstr::ByteSlice;
                    core::repository::attributes::validate_baseline(
                        repository(Mode::StrictWithGitInstallConfig)?,
                        stdin_or_bail().ok().map(|stdin| {
                            stdin
                                .byte_lines()
                                .filter_map(Result::ok)
                                .filter_map(|line| gix::path::Spec::from_bytes(line.as_bstr()))
                        }),
                        progress,
                        out,
                        err,
                        core::repository::attributes::validate_baseline::Options {
                            format,
                            statistics,
                            ignore: !no_ignore,
                        },
                    )
                },
            ),
        },
        Subcommands::Exclude(cmd) => match cmd {
            exclude::Subcommands::Query {
                statistics,
                patterns,
                pathspecs,
                show_ignore_patterns,
            } => prepare_and_run(
                "exclude-query",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    use gix::bstr::ByteSlice;
                    core::repository::exclude::query(
                        repository(Mode::Strict)?,
                        if pathspecs.is_empty() {
                            Box::new(
                                stdin_or_bail()?
                                    .byte_lines()
                                    .filter_map(Result::ok)
                                    .filter_map(|line| gix::path::Spec::from_bytes(line.as_bstr())),
                            ) as Box<dyn Iterator<Item = gix::path::Spec>>
                        } else {
                            Box::new(pathspecs.into_iter())
                        },
                        out,
                        err,
                        core::repository::exclude::query::Options {
                            format,
                            show_ignore_patterns,
                            overrides: patterns,
                            statistics,
                        },
                    )
                },
            ),
        },
        Subcommands::Index(cmd) => match cmd {
            index::Subcommands::Entries {
                no_attributes,
                attributes_from_index,
                statistics,
            } => prepare_and_run(
                "index-entries",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    core::repository::index::entries(
                        repository(Mode::LenientWithGitInstallConfig)?,
                        out,
                        err,
                        core::repository::index::entries::Options {
                            format,
                            attributes: if no_attributes {
                                None
                            } else {
                                Some(if attributes_from_index {
                                    core::repository::index::entries::Attributes::Index
                                } else {
                                    core::repository::index::entries::Attributes::WorktreeAndIndex
                                })
                            },
                            statistics,
                        },
                    )
                },
            ),
            index::Subcommands::FromTree {
                force,
                index_output_path,
                spec,
            } => prepare_and_run(
                "index-from-tree",
                trace,
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, _out, _err| {
                    core::repository::index::from_tree(spec, index_output_path, force, repository(Mode::Strict)?)
                },
            ),
        },
    }?;
    Ok(())
}

fn stdin_or_bail() -> Result<std::io::BufReader<std::io::Stdin>> {
    use is_terminal::IsTerminal;
    if std::io::stdin().is_terminal() {
        anyhow::bail!("Refusing to read from standard input while a terminal is connected")
    }
    Ok(BufReader::new(stdin()))
}

fn verify_mode(decode: bool, re_encode: bool) -> verify::Mode {
    match (decode, re_encode) {
        (true, false) => verify::Mode::HashCrc32Decode,
        (_, true) => verify::Mode::HashCrc32DecodeEncode,
        (false, false) => verify::Mode::HashCrc32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clap() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
