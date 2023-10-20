use std::{
    ops::DerefMut,
    sync::atomic::{AtomicBool, Ordering},
};

use gix_odb::{store::RefreshMode, FindExt};
use gix_protocol::{
    fetch::Arguments,
    transport::{client::Transport, packetline::read::ProgressAction},
};

use crate::{
    config::{
        cache::util::ApplyLeniency,
        tree::{Clone, Fetch, Key},
    },
    remote,
    remote::{
        connection::fetch::config,
        fetch,
        fetch::{
            negotiate, negotiate::Algorithm, outcome, refs, Error, Outcome, Prepare, ProgressId, RefLogMessage,
            Shallow, Status,
        },
    },
    Repository,
};

impl<'remote, 'repo, T> Prepare<'remote, 'repo, T>
where
    T: Transport,
{
    /// Receive the pack and perform the operation as configured by git via `git-config` or overridden by various builder methods.
    /// Return `Ok(None)` if there was nothing to do because all remote refs are at the same state as they are locally, or `Ok(Some(outcome))`
    /// to inform about all the changes that were made.
    ///
    /// ### Negotiation
    ///
    /// "fetch.negotiationAlgorithm" describes algorithms `git` uses currently, with the default being `consecutive` and `skipping` being
    /// experimented with. We currently implement something we could call 'naive' which works for now.
    ///
    /// ### Pack `.keep` files
    ///
    /// That packs that are freshly written to the object database are vulnerable to garbage collection for the brief time that it takes between
    /// them being placed and the respective references to be written to disk which binds their objects to the commit graph, making them reachable.
    ///
    /// To circumvent this issue, a `.keep` file is created before any pack related file (i.e. `.pack` or `.idx`) is written, which indicates the
    /// garbage collector (like `git maintenance`, `git gc`) to leave the corresponding pack file alone.
    ///
    /// If there were any ref updates or the received pack was empty, the `.keep` file will be deleted automatically leaving in its place at
    /// `write_pack_bundle.keep_path` a `None`.
    /// However, if no ref-update happened the path will still be present in `write_pack_bundle.keep_path` and is expected to be handled by the caller.
    /// A known application for this behaviour is in `remote-helper` implementations which should send this path via `lock <path>` to stdout
    /// to inform git about the file that it will remove once it updated the refs accordingly.
    ///
    /// ### Deviation
    ///
    /// When **updating refs**, the `git-fetch` docs state that the following:
    ///
    /// > Unlike when pushing with git-push, any updates outside of refs/{tags,heads}/* will be accepted without + in the refspec (or --force), whether that’s swapping e.g. a tree object for a blob, or a commit for another commit that’s doesn’t have the previous commit as an ancestor etc.
    ///
    /// We explicitly don't special case those refs and expect the user to take control. Note that by its nature,
    /// force only applies to refs pointing to commits and if they don't, they will be updated either way in our
    /// implementation as well.
    ///
    /// ### Async Mode Shortcoming
    ///
    /// Currently the entire process of resolving a pack is blocking the executor. This can be fixed using the `blocking` crate, but it
    /// didn't seem worth the tradeoff of having more complex code.
    ///
    /// ### Configuration
    ///
    /// - `gitoxide.userAgent` is read to obtain the application user agent for git servers and for HTTP servers as well.
    ///
    #[gix_protocol::maybe_async::maybe_async]
    pub async fn receive<P>(self, mut progress: P, should_interrupt: &AtomicBool) -> Result<Outcome, Error>
    where
        P: gix_features::progress::NestedProgress,
        P::SubProgress: 'static,
    {
        self.receive_inner(&mut progress, should_interrupt).await
    }

    #[gix_protocol::maybe_async::maybe_async]
    #[allow(clippy::drop_non_drop)]
    pub(crate) async fn receive_inner(
        mut self,
        progress: &mut dyn crate::DynNestedProgress,
        should_interrupt: &AtomicBool,
    ) -> Result<Outcome, Error> {
        let _span = gix_trace::coarse!("fetch::Prepare::receive()");
        let mut con = self.con.take().expect("receive() can only be called once");

        let handshake = &self.ref_map.handshake;
        let protocol_version = handshake.server_protocol_version;

        let fetch = gix_protocol::Command::Fetch;
        let repo = con.remote.repo;
        let fetch_features = {
            let mut f = fetch.default_features(protocol_version, &handshake.capabilities);
            f.push(repo.config.user_agent_tuple());
            f
        };

        gix_protocol::fetch::Response::check_required_features(protocol_version, &fetch_features)?;
        let sideband_all = fetch_features.iter().any(|(n, _)| *n == "sideband-all");
        let mut arguments = gix_protocol::fetch::Arguments::new(protocol_version, fetch_features, con.trace);
        if matches!(con.remote.fetch_tags, crate::remote::fetch::Tags::Included) {
            if !arguments.can_use_include_tag() {
                return Err(Error::MissingServerFeature {
                    feature: "include-tag",
                    description:
                        // NOTE: if this is an issue, we could probably do what's proposed here.
                        "To make this work we would have to implement another pass to fetch attached tags separately",
                });
            }
            arguments.use_include_tag();
        }
        let (shallow_commits, mut shallow_lock) = add_shallow_args(&mut arguments, &self.shallow, repo)?;

        if self.ref_map.object_hash != repo.object_hash() {
            return Err(Error::IncompatibleObjectHash {
                local: repo.object_hash(),
                remote: self.ref_map.object_hash,
            });
        }

        let negotiate_span = gix_trace::detail!(
            "negotiate",
            protocol_version = self.ref_map.handshake.server_protocol_version as usize
        );
        let mut negotiator = repo
            .config
            .resolved
            .string_by_key(Fetch::NEGOTIATION_ALGORITHM.logical_name().as_str())
            .map(|n| Fetch::NEGOTIATION_ALGORITHM.try_into_negotiation_algorithm(n))
            .transpose()
            .with_leniency(repo.config.lenient_config)?
            .unwrap_or(Algorithm::Consecutive)
            .into_negotiator();
        let graph_repo = {
            let mut r = repo.clone();
            // assure that checking for unknown server refs doesn't trigger ODB refreshes.
            r.objects.refresh = RefreshMode::Never;
            // we cache everything of importance in the graph and thus don't need an object cache.
            r.objects.unset_object_cache();
            r
        };
        let mut graph = graph_repo.revision_graph();
        let action = negotiate::mark_complete_and_common_ref(
            &graph_repo,
            negotiator.deref_mut(),
            &mut graph,
            &self.ref_map,
            &self.shallow,
            negotiate::make_refmapping_ignore_predicate(con.remote.fetch_tags, &self.ref_map),
        )?;
        let mut previous_response = None::<gix_protocol::fetch::Response>;
        let (mut write_pack_bundle, negotiate) = match &action {
            negotiate::Action::NoChange | negotiate::Action::SkipToRefUpdate => {
                gix_protocol::indicate_end_of_interaction(&mut con.transport, con.trace)
                    .await
                    .ok();
                (None, None)
            }
            negotiate::Action::MustNegotiate {
                remote_ref_target_known,
            } => {
                negotiate::add_wants(
                    repo,
                    &mut arguments,
                    &self.ref_map,
                    remote_ref_target_known,
                    &self.shallow,
                    negotiate::make_refmapping_ignore_predicate(con.remote.fetch_tags, &self.ref_map),
                );
                let mut rounds = Vec::new();
                let is_stateless =
                    arguments.is_stateless(!con.transport.connection_persists_across_multiple_requests());
                let mut haves_to_send = gix_negotiate::window_size(is_stateless, None);
                let mut seen_ack = false;
                let mut in_vain = 0;
                let mut common = is_stateless.then(Vec::new);
                let mut reader = 'negotiation: loop {
                    let _round = gix_trace::detail!("negotiate round", round = rounds.len() + 1);
                    progress.step();
                    progress.set_name(format!("negotiate (round {})", rounds.len() + 1));

                    let is_done = match negotiate::one_round(
                        negotiator.deref_mut(),
                        &mut graph,
                        haves_to_send,
                        &mut arguments,
                        previous_response.as_ref(),
                        common.as_mut(),
                    ) {
                        Ok((haves_sent, ack_seen)) => {
                            if ack_seen {
                                in_vain = 0;
                            }
                            seen_ack |= ack_seen;
                            in_vain += haves_sent;
                            rounds.push(outcome::negotiate::Round {
                                haves_sent,
                                in_vain,
                                haves_to_send,
                                previous_response_had_at_least_one_in_common: ack_seen,
                            });
                            let is_done = haves_sent != haves_to_send || (seen_ack && in_vain >= 256);
                            haves_to_send = gix_negotiate::window_size(is_stateless, Some(haves_to_send));
                            is_done
                        }
                        Err(err) => {
                            gix_protocol::indicate_end_of_interaction(&mut con.transport, con.trace)
                                .await
                                .ok();
                            return Err(err.into());
                        }
                    };
                    let mut reader = arguments.send(&mut con.transport, is_done).await?;
                    if sideband_all {
                        setup_remote_progress(progress, &mut reader, should_interrupt);
                    }
                    let response = gix_protocol::fetch::Response::from_line_reader(
                        protocol_version,
                        &mut reader,
                        is_done,
                        !is_done,
                    )
                    .await?;
                    let has_pack = response.has_pack();
                    previous_response = Some(response);
                    if has_pack {
                        progress.step();
                        progress.set_name("receiving pack".into());
                        if !sideband_all {
                            setup_remote_progress(progress, &mut reader, should_interrupt);
                        }
                        break 'negotiation reader;
                    }
                };
                let graph = graph.detach();
                drop(graph_repo);
                drop(negotiate_span);

                let previous_response = previous_response.expect("knowledge of a pack means a response was received");
                if !previous_response.shallow_updates().is_empty() && shallow_lock.is_none() {
                    let reject_shallow_remote = repo
                        .config
                        .resolved
                        .boolean_filter_by_key("clone.rejectShallow", &mut repo.filter_config_section())
                        .map(|val| Clone::REJECT_SHALLOW.enrich_error(val))
                        .transpose()?
                        .unwrap_or(false);
                    if reject_shallow_remote {
                        return Err(Error::RejectShallowRemote);
                    }
                    shallow_lock = acquire_shallow_lock(repo).map(Some)?;
                }

                let options = gix_pack::bundle::write::Options {
                    thread_limit: config::index_threads(repo)?,
                    index_version: config::pack_index_version(repo)?,
                    iteration_mode: gix_pack::data::input::Mode::Verify,
                    object_hash: con.remote.repo.object_hash(),
                };

                let write_pack_bundle = if matches!(self.dry_run, fetch::DryRun::No) {
                    #[cfg(not(feature = "async-network-client"))]
                    let mut rd = reader;
                    #[cfg(feature = "async-network-client")]
                    let mut rd = gix_protocol::futures_lite::io::BlockOn::new(reader);
                    let res = gix_pack::Bundle::write_to_directory(
                        &mut rd,
                        Some(&repo.objects.store_ref().path().join("pack")),
                        progress,
                        should_interrupt,
                        Some(Box::new({
                            let repo = repo.clone();
                            move |oid, buf| repo.objects.find(&oid, buf).ok()
                        })),
                        options,
                    )?;
                    #[cfg(feature = "async-network-client")]
                    {
                        reader = rd.into_inner();
                    }
                    #[cfg(not(feature = "async-network-client"))]
                    {
                        reader = rd;
                    }
                    Some(res)
                } else {
                    None
                };
                drop(reader);

                if matches!(protocol_version, gix_protocol::transport::Protocol::V2) {
                    gix_protocol::indicate_end_of_interaction(&mut con.transport, con.trace)
                        .await
                        .ok();
                }

                if let Some(shallow_lock) = shallow_lock {
                    if !previous_response.shallow_updates().is_empty() {
                        crate::shallow::write(shallow_lock, shallow_commits, previous_response.shallow_updates())?;
                    }
                }
                (write_pack_bundle, Some(outcome::Negotiate { graph, rounds }))
            }
        };

        let update_refs = refs::update(
            repo,
            self.reflog_message
                .take()
                .unwrap_or_else(|| RefLogMessage::Prefixed { action: "fetch".into() }),
            &self.ref_map.mappings,
            con.remote.refspecs(remote::Direction::Fetch),
            &self.ref_map.extra_refspecs,
            con.remote.fetch_tags,
            self.dry_run,
            self.write_packed_refs,
        )?;

        if let Some(bundle) = write_pack_bundle.as_mut() {
            if !update_refs.edits.is_empty() || bundle.index.num_objects == 0 {
                if let Some(path) = bundle.keep_path.take() {
                    std::fs::remove_file(&path).map_err(|err| Error::RemovePackKeepFile { path, source: err })?;
                }
            }
        }

        let out = Outcome {
            ref_map: std::mem::take(&mut self.ref_map),
            status: match write_pack_bundle {
                Some(write_pack_bundle) => Status::Change {
                    write_pack_bundle,
                    update_refs,
                    negotiate: negotiate.expect("if we have a pack, we always negotiated it"),
                },
                None => Status::NoPackReceived {
                    dry_run: matches!(self.dry_run, fetch::DryRun::Yes),
                    negotiate,
                    update_refs,
                },
            },
        };
        Ok(out)
    }
}

fn acquire_shallow_lock(repo: &Repository) -> Result<gix_lock::File, Error> {
    gix_lock::File::acquire_to_update_resource(repo.shallow_file(), gix_lock::acquire::Fail::Immediately, None)
        .map_err(Into::into)
}

fn add_shallow_args(
    args: &mut Arguments,
    shallow: &Shallow,
    repo: &Repository,
) -> Result<(Option<crate::shallow::Commits>, Option<gix_lock::File>), Error> {
    let expect_change = *shallow != Shallow::NoChange;
    let shallow_lock = expect_change.then(|| acquire_shallow_lock(repo)).transpose()?;

    let shallow_commits = repo.shallow_commits()?;
    if (shallow_commits.is_some() || expect_change) && !args.can_use_shallow() {
        // NOTE: if this is an issue, we can always unshallow the repo ourselves.
        return Err(Error::MissingServerFeature {
            feature: "shallow",
            description: "shallow clones need server support to remain shallow, otherwise bigger than expected packs are sent effectively unshallowing the repository",
        });
    }
    if let Some(shallow_commits) = &shallow_commits {
        for commit in shallow_commits.iter() {
            args.shallow(commit);
        }
    }
    match shallow {
        Shallow::NoChange => {}
        Shallow::DepthAtRemote(commits) => args.deepen(commits.get() as usize),
        Shallow::Deepen(commits) => {
            args.deepen(*commits as usize);
            args.deepen_relative();
        }
        Shallow::Since { cutoff } => {
            args.deepen_since(cutoff.seconds);
        }
        Shallow::Exclude {
            remote_refs,
            since_cutoff,
        } => {
            if let Some(cutoff) = since_cutoff {
                args.deepen_since(cutoff.seconds);
            }
            for ref_ in remote_refs {
                args.deepen_not(ref_.as_ref().as_bstr());
            }
        }
    }
    Ok((shallow_commits, shallow_lock))
}

fn setup_remote_progress(
    progress: &mut dyn crate::DynNestedProgress,
    reader: &mut Box<dyn gix_protocol::transport::client::ExtendedBufRead + Unpin + '_>,
    should_interrupt: &AtomicBool,
) {
    use gix_protocol::transport::client::ExtendedBufRead;
    reader.set_progress_handler(Some(Box::new({
        let mut remote_progress = progress.add_child_with_id("remote".to_string(), ProgressId::RemoteProgress.into());
        // SAFETY: Ugh, so, with current Rust I can't declare lifetimes in the involved traits the way they need to
        //         be and I also can't use scoped threads to pump from local scopes to an Arc version that could be
        //         used here due to the this being called from sync AND async code (and the async version doesn't work
        //         with a surrounding `std::thread::scope()`.
        //         Thus there is only claiming this is 'static which we know works for *our* implementations of ExtendedBufRead
        //         and typical implementations, but of course it's possible for user code to come along and actually move this
        //         handler into a context where it can outlive the current function. Is this going to happen? Probably not unless
        //         somebody really wants to break it. So, with standard usage this value is never used past its actual lifetime.
        #[allow(unsafe_code)]
        let should_interrupt: &'static AtomicBool = unsafe { std::mem::transmute(should_interrupt) };
        move |is_err: bool, data: &[u8]| {
            gix_protocol::RemoteProgress::translate_to_progress(is_err, data, &mut remote_progress);
            if should_interrupt.load(Ordering::Relaxed) {
                ProgressAction::Interrupt
            } else {
                ProgressAction::Continue
            }
        }
    }) as gix_protocol::transport::client::HandleProgress));
}
