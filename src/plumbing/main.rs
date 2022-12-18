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
use git_repository::bstr::io::BufReadExt;
use gitoxide_core as core;
use gitoxide_core::pack::verify;

use crate::{
    plumbing::{
        options::{commit, config, credential, exclude, free, index, mailmap, odb, revision, tree, Args, Subcommands},
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
        git_features::progress::DoOrDiscard<prodash::tree::Item>,
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
    let args: Args = Args::parse_from(git_repository::env::args_os());
    let thread_limit = args.threads;
    let verbose = args.verbose;
    let format = args.format;
    let cmd = args.cmd;
    let object_hash = args.object_hash;
    let config = args.config;
    use git_repository as git;
    let repository = args.repository;
    enum Mode {
        Strict,
        StrictWithGitInstallConfig,
        Lenient,
        LenientWithGitInstallConfig,
    }

    let repository = {
        let config = config.clone();
        move |mode: Mode| -> Result<git::Repository> {
            let mut mapping: git::sec::trust::Mapping<git::open::Options> = Default::default();
            let strict_toggle = matches!(mode, Mode::Strict | Mode::StrictWithGitInstallConfig) || args.strict;
            mapping.full = mapping.full.strict_config(strict_toggle);
            mapping.reduced = mapping.reduced.strict_config(strict_toggle);
            let git_installation = matches!(
                mode,
                Mode::StrictWithGitInstallConfig | Mode::LenientWithGitInstallConfig
            );
            let to_match_settings = |mut opts: git::open::Options| {
                opts.permissions.config.git_binary = git_installation;
                if config.is_empty() {
                    opts
                } else {
                    opts.cli_overrides(config.clone())
                }
            };
            mapping.full.modify(to_match_settings);
            mapping.reduced.modify(to_match_settings);
            let mut repo = git::ThreadSafeRepository::discover_opts(repository, Default::default(), mapping)
                .map(git::Repository::from)?;
            if !config.is_empty() {
                repo.config_snapshot_mut()
                    .append_config(config.iter(), git::config::Source::Cli)
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
    git_repository::interrupt::init_handler({
        let should_interrupt = Arc::clone(&should_interrupt);
        move || should_interrupt.store(true, Ordering::SeqCst)
    })?;

    match cmd {
        #[cfg(feature = "gitoxide-core-blocking-client")]
        Subcommands::Clone(crate::plumbing::options::clone::Platform {
            handshake_info,
            bare,
            no_tags,
            remote,
            directory,
        }) => {
            let opts = core::repository::clone::Options {
                format,
                bare,
                handshake_info,
                no_tags,
            };
            prepare_and_run(
                "clone",
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
            remote,
            ref_spec,
        }) => {
            let opts = core::repository::fetch::Options {
                format,
                dry_run,
                remote,
                handshake_info,
                ref_specs: ref_spec,
            };
            prepare_and_run(
                "fetch",
                auto_verbose,
                progress,
                progress_keep_open,
                core::repository::fetch::PROGRESS_RANGE,
                move |progress, out, err| {
                    core::repository::fetch(repository(Mode::LenientWithGitInstallConfig)?, progress, out, err, opts)
                },
            )
        }
        Subcommands::Progress => show_progress(),
        Subcommands::Credential(cmd) => core::repository::credential(
            repository(Mode::StrictWithGitInstallConfig)?,
            match cmd {
                credential::Subcommands::Fill => git::credentials::program::main::Action::Get,
                credential::Subcommands::Approve => git::credentials::program::main::Action::Store,
                credential::Subcommands::Reject => git::credentials::program::main::Action::Erase,
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
            free::Subcommands::CommitGraph(subcommands) => match subcommands {
                free::commitgraph::Subcommands::Verify { path, statistics } => prepare_and_run(
                    "commitgraph-verify",
                    auto_verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |_progress, out, err| {
                        let output_statistics = if statistics { Some(format) } else { None };
                        core::commitgraph::verify::graph_or_file(
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
                free::index::Subcommands::CheckoutExclusive {
                    directory,
                    empty_files,
                    repository,
                    keep_going,
                } => prepare_and_run(
                    "index-checkout",
                    verbose,
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
                free::index::Subcommands::Entries => prepare_and_run(
                    "index-entries",
                    verbose,
                    progress,
                    progress_keep_open,
                    None,
                    move |_progress, out, _err| {
                        core::index::entries(index_path, out, core::index::Options { object_hash, format })
                    },
                ),
                free::index::Subcommands::Verify => prepare_and_run(
                    "index-verify",
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
                                nondeterministic_thread_count: nondeterministic_count.then(|| counting_threads),
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
                            refs.into_iter().map(|r| r.into()).collect(),
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
                            verbose,
                            progress,
                            progress_keep_open,
                            core::pack::multi_index::PROGRESS_RANGE,
                            move |_progress, out, _err| core::pack::multi_index::entries(multi_index_path, format, out),
                        ),
                        free::pack::multi_index::Subcommands::Info => prepare_and_run(
                            "pack-multi-index-info",
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
                                    should_interrupt: &git_repository::interrupt::IS_INTERRUPTED,
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
                        output_statistics: statistics.then(|| format),
                        algorithm,
                        verify_mode: verify_mode(decode, re_encode),
                        thread_limit,
                    },
                )
            },
        ),
        Subcommands::Revision(cmd) => match cmd {
            revision::Subcommands::List { spec } => prepare_and_run(
                "revision-list",
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    core::repository::revision::list(repository(Mode::Lenient)?, spec, out, format)
                },
            ),
            revision::Subcommands::PreviousBranches => prepare_and_run(
                "revision-previousbranches",
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
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| core::repository::odb::entries(repository(Mode::Strict)?, format, out),
            ),
            odb::Subcommands::Info => prepare_and_run(
                "odb-info",
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
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, err| {
                    core::repository::mailmap::entries(repository(Mode::Lenient)?, format, out, err)
                },
            ),
        },
        Subcommands::Exclude(cmd) => match cmd {
            exclude::Subcommands::Query {
                patterns,
                pathspecs,
                show_ignore_patterns,
            } => prepare_and_run(
                "exclude-query",
                verbose,
                progress,
                progress_keep_open,
                None,
                move |_progress, out, _err| {
                    use git::bstr::ByteSlice;
                    core::repository::exclude::query(
                        repository(Mode::Strict)?,
                        if pathspecs.is_empty() {
                            Box::new(
                                stdin_or_bail()?
                                    .byte_lines()
                                    .filter_map(Result::ok)
                                    .filter_map(|line| git::path::Spec::from_bytes(line.as_bstr())),
                            ) as Box<dyn Iterator<Item = git::path::Spec>>
                        } else {
                            Box::new(pathspecs.into_iter())
                        },
                        out,
                        core::repository::exclude::query::Options {
                            format,
                            show_ignore_patterns,
                            overrides: patterns,
                        },
                    )
                },
            ),
        },
        Subcommands::Index(cmd) => match cmd {
            index::Subcommands::FromTree {
                force,
                index_output_path,
                spec,
            } => prepare_and_run(
                "index-from-tree",
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
        (true, true) | (false, true) => verify::Mode::HashCrc32DecodeEncode,
        (false, false) => verify::Mode::HashCrc32,
    }
}
