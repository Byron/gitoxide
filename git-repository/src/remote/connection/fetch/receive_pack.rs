use std::sync::atomic::AtomicBool;

use git_odb::FindExt;
use git_protocol::transport::client::Transport;

use crate::{
    remote,
    remote::{
        connection::fetch::config,
        fetch,
        fetch::{negotiate, refs, Error, Outcome, Prepare, RefLogMessage, Status},
    },
    Progress,
};

impl<'remote, 'repo, T, P> Prepare<'remote, 'repo, T, P>
where
    T: Transport,
    P: Progress,
    P::SubProgress: 'static,
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
    #[git_protocol::maybe_async::maybe_async]
    pub async fn receive(mut self, should_interrupt: &AtomicBool) -> Result<Outcome, Error> {
        let mut con = self.con.take().expect("receive() can only be called once");

        let handshake = &self.ref_map.handshake;
        let protocol_version = handshake.server_protocol_version;

        let fetch = git_protocol::Command::Fetch;
        let progress = &mut con.progress;
        let repo = con.remote.repo;
        let fetch_features = {
            let mut f = fetch.default_features(protocol_version, &handshake.capabilities);
            f.push(repo.config.user_agent_tuple());
            f
        };

        git_protocol::fetch::Response::check_required_features(protocol_version, &fetch_features)?;
        let sideband_all = fetch_features.iter().any(|(n, _)| *n == "sideband-all");
        let mut arguments = git_protocol::fetch::Arguments::new(protocol_version, fetch_features);
        if matches!(con.remote.fetch_tags, crate::remote::fetch::Tags::Included) {
            if !arguments.can_use_include_tag() {
                unimplemented!("we expect servers to support 'include-tag', otherwise we have to implement another pass to fetch attached tags separately");
            }
            arguments.use_include_tag();
        }
        let mut previous_response = None::<git_protocol::fetch::Response>;
        let mut round = 1;

        if self.ref_map.object_hash != repo.object_hash() {
            return Err(Error::IncompatibleObjectHash {
                local: repo.object_hash(),
                remote: self.ref_map.object_hash,
            });
        }

        let reader = 'negotiation: loop {
            progress.step();
            progress.set_name(format!("negotiate (round {})", round));

            let is_done = match negotiate::one_round(
                negotiate::Algorithm::Naive,
                round,
                repo,
                &self.ref_map,
                con.remote.fetch_tags,
                &mut arguments,
                previous_response.as_ref(),
            ) {
                Ok(_) if arguments.is_empty() => {
                    git_protocol::indicate_end_of_interaction(&mut con.transport).await.ok();
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
                    return Ok(Outcome {
                        ref_map: std::mem::take(&mut self.ref_map),
                        status: Status::NoPackReceived { update_refs },
                    });
                }
                Ok(is_done) => is_done,
                Err(err) => {
                    git_protocol::indicate_end_of_interaction(&mut con.transport).await.ok();
                    return Err(err.into());
                }
            };
            round += 1;
            let mut reader = arguments.send(&mut con.transport, is_done).await?;
            if sideband_all {
                setup_remote_progress(progress, &mut reader);
            }
            let response = git_protocol::fetch::Response::from_line_reader(protocol_version, &mut reader).await?;
            if response.has_pack() {
                progress.step();
                progress.set_name("receiving pack");
                if !sideband_all {
                    setup_remote_progress(progress, &mut reader);
                }
                break 'negotiation reader;
            } else {
                previous_response = Some(response);
            }
        };

        let options = git_pack::bundle::write::Options {
            thread_limit: config::index_threads(repo)?,
            index_version: config::pack_index_version(repo)?,
            iteration_mode: git_pack::data::input::Mode::Verify,
            object_hash: con.remote.repo.object_hash(),
        };

        let mut write_pack_bundle = if matches!(self.dry_run, fetch::DryRun::No) {
            Some(git_pack::Bundle::write_to_directory(
                #[cfg(feature = "async-network-client")]
                {
                    git_protocol::futures_lite::io::BlockOn::new(reader)
                },
                #[cfg(not(feature = "async-network-client"))]
                {
                    reader
                },
                Some(repo.objects.store_ref().path().join("pack")),
                con.progress,
                should_interrupt,
                Some(Box::new({
                    let repo = repo.clone();
                    move |oid, buf| repo.objects.find(oid, buf).ok()
                })),
                options,
            )?)
        } else {
            drop(reader);
            None
        };

        if matches!(protocol_version, git_protocol::transport::Protocol::V2) {
            git_protocol::indicate_end_of_interaction(&mut con.transport).await.ok();
        }

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

        Ok(Outcome {
            ref_map: std::mem::take(&mut self.ref_map),
            status: match write_pack_bundle {
                Some(write_pack_bundle) => Status::Change {
                    write_pack_bundle,
                    update_refs,
                },
                None => Status::DryRun { update_refs },
            },
        })
    }
}

fn setup_remote_progress<P>(
    progress: &mut P,
    reader: &mut Box<dyn git_protocol::transport::client::ExtendedBufRead + Unpin + '_>,
) where
    P: Progress,
    P::SubProgress: 'static,
{
    use git_protocol::transport::client::ExtendedBufRead;
    reader.set_progress_handler(Some(Box::new({
        let mut remote_progress = progress.add_child_with_id("remote", *b"FERP"); /* FEtch Remote Progress*/
        move |is_err: bool, data: &[u8]| {
            git_protocol::RemoteProgress::translate_to_progress(is_err, data, &mut remote_progress)
        }
    }) as git_protocol::transport::client::HandleProgress));
}
