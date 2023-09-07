use std::borrow::Cow;

use gix_date::SecondsSinceUnixEpoch;
use gix_negotiate::Flags;
use gix_odb::HeaderExt;
use gix_pack::Find;

use crate::remote::{fetch, fetch::Shallow};

type Queue = gix_revwalk::PriorityQueue<SecondsSinceUnixEpoch, gix_hash::ObjectId>;

/// The error returned during negotiation.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("We were unable to figure out what objects the server should send after {rounds} round(s)")]
    NegotiationFailed { rounds: usize },
    #[error(transparent)]
    LookupCommitInGraph(#[from] gix_revwalk::graph::lookup::commit::Error),
    #[error(transparent)]
    InitRefsIterator(#[from] crate::reference::iter::init::Error),
    #[error(transparent)]
    InitRefsIteratorPlatform(#[from] crate::reference::iter::Error),
    #[error(transparent)]
    ObtainRefDuringIteration(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    LoadIndex(#[from] gix_odb::store::load_index::Error),
}

#[must_use]
pub(crate) enum Action {
    /// None of the remote refs moved compared to our last recorded state (via tracking refs), so there is nothing to do at all,
    /// not even a ref update.
    NoChange,
    /// Don't negotiate, don't fetch the pack, skip right to updating the references.
    ///
    /// This happens if we already have all local objects even though the server seems to have changed.
    SkipToRefUpdate,
    /// We can't know for sure if fetching *is not* needed, so we go ahead and negotiate.
    MustNegotiate {
        /// Each `ref_map.mapping` has a slot here which is `true` if we have the object the remote ref points to locally.
        remote_ref_target_known: Vec<bool>,
    },
}

/// This function is modeled after the similarly named one in the git codebase to do the following:
///
/// * figure out all advertised refs on the remote *that we already have* and keep track of the oldest one as cutoff date.
/// * mark all of our own refs as tips for a traversal.
/// * mark all their parents, recursively, up to (and including) the cutoff date up to which we have seen the servers commit that we have.
/// * pass all known-to-be-common-with-remote commits to the negotiator as common commits.
///
/// This is done so that we already find the most recent common commits, even if we are ahead, which is *potentially* better than
/// what we would get if we would rely on tracking refs alone, particularly if one wouldn't trust the tracking refs for some reason.
///
/// Note that git doesn't trust its own tracking refs as the server *might* have changed completely, for instance by force-pushing, so
/// marking our local tracking refs as known is something that's actually not proven to be correct so it's not done.
///
/// Additionally, it does what's done in `transport.c` and we check if a fetch is actually needed as at least one advertised ref changed.
///
/// Finally, we also mark tips in the `negotiator` in one go to avoid traversing all refs twice, since we naturally encounter all tips during
/// our own walk.
///
/// Return whether or not we should negotiate, along with a queue for later use.
pub(crate) fn mark_complete_and_common_ref(
    repo: &crate::Repository,
    negotiator: &mut dyn gix_negotiate::Negotiator,
    graph: &mut gix_negotiate::Graph<'_>,
    ref_map: &fetch::RefMap,
    shallow: &fetch::Shallow,
    mapping_is_ignored: impl Fn(&fetch::Mapping) -> bool,
) -> Result<Action, Error> {
    let _span = gix_trace::detail!("mark_complete_and_common_ref", mappings = ref_map.mappings.len());
    if let fetch::Shallow::Deepen(0) = shallow {
        // Avoid deepening (relative) with zero as it seems to upset the server. Git also doesn't actually
        // perform the negotiation for some reason (couldn't find it in code).
        return Ok(Action::NoChange);
    }
    if let Some(fetch::Mapping {
        remote: fetch::Source::Ref(gix_protocol::handshake::Ref::Unborn { .. }),
        ..
    }) = ref_map.mappings.last().filter(|_| ref_map.mappings.len() == 1)
    {
        // There is only an unborn branch, as the remote has an empty repository. This means there is nothing to do except for
        // possibly reproducing the unborn branch locally.
        return Ok(Action::SkipToRefUpdate);
    }

    // Compute the cut-off date by checking which of the refs advertised (and matched in refspecs) by the remote we have,
    // and keep the oldest one.
    let mut cutoff_date = None::<SecondsSinceUnixEpoch>;
    let mut num_mappings_with_change = 0;
    let mut remote_ref_target_known: Vec<bool> = std::iter::repeat(false).take(ref_map.mappings.len()).collect();
    let mut remote_ref_included: Vec<bool> = std::iter::repeat(false).take(ref_map.mappings.len()).collect();

    for (mapping_idx, mapping) in ref_map.mappings.iter().enumerate() {
        let want_id = mapping.remote.as_id();
        let have_id = mapping.local.as_ref().and_then(|name| {
            // this is the only time git uses the peer-id.
            let r = repo.find_reference(name).ok()?;
            r.target().try_id().map(ToOwned::to_owned)
        });

        // Even for ignored mappings we want to know if the `want` is already present locally, so skip nothing else.
        if !mapping_is_ignored(mapping) {
            remote_ref_included[mapping_idx] = true;
            // Like git, we don't let known unchanged mappings participate in the tree traversal
            if want_id.zip(have_id).map_or(true, |(want, have)| want != have) {
                num_mappings_with_change += 1;
            }
        }

        if let Some(commit) = want_id
            .and_then(|id| graph.try_lookup_or_insert_commit(id.into(), |_| {}).transpose())
            .transpose()?
        {
            remote_ref_target_known[mapping_idx] = true;
            cutoff_date = cutoff_date.unwrap_or_default().max(commit.commit_time).into();
        } else if want_id.map_or(false, |maybe_annotated_tag| repo.objects.contains(maybe_annotated_tag)) {
            remote_ref_target_known[mapping_idx] = true;
        }
    }

    if matches!(shallow, Shallow::NoChange) {
        if num_mappings_with_change == 0 {
            return Ok(Action::NoChange);
        } else if remote_ref_target_known
            .iter()
            .zip(remote_ref_included)
            .filter_map(|(known, included)| included.then_some(known))
            .all(|known| *known)
        {
            return Ok(Action::SkipToRefUpdate);
        }
    }

    // color our commits as complete as identified by references, unconditionally
    // (`git` is conditional here based on `deepen`, but it doesn't make sense and it's hard to extract from history when that happened).
    let mut queue = Queue::new();
    mark_all_refs_in_repo(repo, graph, &mut queue, Flags::COMPLETE)?;
    mark_alternate_complete(repo, graph, &mut queue)?;
    // Keep track of the tips, which happen to be on our queue right, before we traverse the graph with cutoff.
    let tips = if let Some(cutoff) = cutoff_date {
        let tips = Cow::Owned(queue.clone());
        // color all their parents up to the cutoff date, the oldest commit we know the server has.
        mark_recent_complete_commits(&mut queue, graph, cutoff)?;
        tips
    } else {
        Cow::Borrowed(&queue)
    };

    gix_trace::detail!("mark known_common").into_scope(|| -> Result<_, Error> {
        // mark all complete advertised refs as common refs.
        for mapping in ref_map
            .mappings
            .iter()
            .zip(remote_ref_target_known.iter().copied())
            // We need this filter as the graph wouldn't contain annotated tags.
            .filter_map(|(mapping, known)| (!known).then_some(mapping))
        {
            let want_id = mapping.remote.as_id();
            if let Some(common_id) = want_id
                .and_then(|id| graph.get(id).map(|c| (c, id)))
                .filter(|(c, _)| c.data.flags.contains(Flags::COMPLETE))
                .map(|(_, id)| id)
            {
                negotiator.known_common(common_id.into(), graph)?;
            }
        }
        Ok(())
    })?;

    // As negotiators currently may rely on getting `known_common` calls first and tips after, we adhere to that which is the only
    // reason we cached the set of tips.
    gix_trace::detail!("mark tips", num_tips = tips.len()).into_scope(|| -> Result<_, Error> {
        for tip in tips.iter_unordered() {
            negotiator.add_tip(*tip, graph)?;
        }
        Ok(())
    })?;

    Ok(Action::MustNegotiate {
        remote_ref_target_known,
    })
}

/// Create a predicate that checks if a refspec mapping should be ignored.
///
/// We want to ignore mappings during negotiation if they would be handled implicitly by the server, which is the case
/// when tags would be sent implicitly due to `Tags::Included`.
pub(crate) fn make_refmapping_ignore_predicate(
    fetch_tags: fetch::Tags,
    ref_map: &fetch::RefMap,
) -> impl Fn(&fetch::Mapping) -> bool + '_ {
    // With included tags, we have to keep mappings of tags to handle them later when updating refs, but we don't want to
    // explicitly `want` them as the server will determine by itself which tags are pointing to a commit it wants to send.
    // If we would not exclude implicit tag mappings like this, we would get too much of the graph.
    let tag_refspec_to_ignore = matches!(fetch_tags, crate::remote::fetch::Tags::Included)
        .then(|| fetch_tags.to_refspec())
        .flatten();
    move |mapping| {
        tag_refspec_to_ignore.map_or(false, |tag_spec| {
            mapping
                .spec_index
                .implicit_index()
                .and_then(|idx| ref_map.extra_refspecs.get(idx))
                .map_or(false, |spec| spec.to_ref() == tag_spec)
        })
    }
}

/// Add all `wants` to `arguments`, which is the unpeeled direct target that the advertised remote ref points to.
pub(crate) fn add_wants(
    repo: &crate::Repository,
    arguments: &mut gix_protocol::fetch::Arguments,
    ref_map: &fetch::RefMap,
    mapping_known: &[bool],
    shallow: &fetch::Shallow,
    mapping_is_ignored: impl Fn(&fetch::Mapping) -> bool,
) {
    // When using shallow, we can't exclude `wants` as the remote won't send anything then. Thus we have to resend everything
    // we have as want instead to get exactly the same graph, but possibly deepened.
    let is_shallow = !matches!(shallow, fetch::Shallow::NoChange);
    let wants = ref_map
        .mappings
        .iter()
        .zip(mapping_known)
        .filter_map(|(m, known)| (is_shallow || !*known).then_some(m))
        .filter(|m| !mapping_is_ignored(m));
    for want in wants {
        let id_on_remote = want.remote.as_id();
        if !arguments.can_use_ref_in_want() || matches!(want.remote, fetch::Source::ObjectId(_)) {
            if let Some(id) = id_on_remote {
                arguments.want(id);
            }
        } else {
            arguments.want_ref(
                want.remote
                    .as_name()
                    .expect("name available if this isn't an object id"),
            )
        }
        let id_is_annotated_tag_we_have = id_on_remote
            .and_then(|id| repo.objects.header(id).ok().map(|h| (id, h)))
            .filter(|(_, h)| h.kind() == gix_object::Kind::Tag)
            .map(|(id, _)| id);
        if let Some(tag_on_remote) = id_is_annotated_tag_we_have {
            // Annotated tags are not handled at all by negotiators in the commit-graph - they only see commits and thus won't
            // ever add `have`s for tags. To correct for that, we add these haves here to avoid getting them sent again.
            arguments.have(tag_on_remote)
        }
    }
}

/// Remove all commits that are more recent than the cut-off, which is the commit time of the oldest common commit we have with the server.
fn mark_recent_complete_commits(
    queue: &mut Queue,
    graph: &mut gix_negotiate::Graph<'_>,
    cutoff: SecondsSinceUnixEpoch,
) -> Result<(), Error> {
    let _span = gix_trace::detail!("mark_recent_complete", queue_len = queue.len());
    while let Some(id) = queue
        .peek()
        .and_then(|(commit_time, id)| (commit_time >= &cutoff).then_some(*id))
    {
        queue.pop_value();
        let commit = graph.get(&id).expect("definitely set when adding tips or parents");
        for parent_id in commit.parents.clone() {
            let mut was_complete = false;
            if let Some(parent) = graph
                .try_lookup_or_insert_commit(parent_id, |md| {
                    was_complete = md.flags.contains(Flags::COMPLETE);
                    md.flags |= Flags::COMPLETE
                })?
                .filter(|_| !was_complete)
            {
                queue.insert(parent.commit_time, parent_id)
            }
        }
    }
    Ok(())
}

fn mark_all_refs_in_repo(
    repo: &crate::Repository,
    graph: &mut gix_negotiate::Graph<'_>,
    queue: &mut Queue,
    mark: Flags,
) -> Result<(), Error> {
    let _span = gix_trace::detail!("mark_all_refs");
    for local_ref in repo.references()?.all()?.peeled() {
        let local_ref = local_ref?;
        let id = local_ref.id().detach();
        let mut is_complete = false;
        if let Some(commit) = graph
            .try_lookup_or_insert_commit(id, |md| {
                is_complete = md.flags.contains(Flags::COMPLETE);
                md.flags |= mark
            })?
            .filter(|_| !is_complete)
        {
            queue.insert(commit.commit_time, id);
        };
    }
    Ok(())
}

fn mark_alternate_complete(
    repo: &crate::Repository,
    graph: &mut gix_negotiate::Graph<'_>,
    queue: &mut Queue,
) -> Result<(), Error> {
    let alternates = repo.objects.store_ref().alternate_db_paths()?;
    let _span = gix_trace::detail!("mark_alternate_refs", num_odb = alternates.len());

    for alternate_repo in alternates.into_iter().filter_map(|path| {
        path.ancestors()
            .nth(1)
            .and_then(|git_dir| crate::open_opts(git_dir, repo.options.clone()).ok())
    }) {
        mark_all_refs_in_repo(&alternate_repo, graph, queue, Flags::ALTERNATE | Flags::COMPLETE)?;
    }
    Ok(())
}

/// Negotiate the nth `round` with `negotiator` sending `haves_to_send` after possibly making the known common commits
/// as sent by the remote known to `negotiator` using `previous_response` if this isn't the first round.
/// All `haves` are added to `arguments` accordingly.
/// Returns the amount of haves actually sent.
pub(crate) fn one_round(
    negotiator: &mut dyn gix_negotiate::Negotiator,
    graph: &mut gix_negotiate::Graph<'_>,
    haves_to_send: usize,
    arguments: &mut gix_protocol::fetch::Arguments,
    previous_response: Option<&gix_protocol::fetch::Response>,
    mut common: Option<&mut Vec<gix_hash::ObjectId>>,
) -> Result<(usize, bool), Error> {
    let mut seen_ack = false;
    if let Some(response) = previous_response {
        use gix_protocol::fetch::response::Acknowledgement;
        for ack in response.acknowledgements() {
            match ack {
                Acknowledgement::Common(id) => {
                    seen_ack = true;
                    negotiator.in_common_with_remote(*id, graph)?;
                    if let Some(ref mut common) = common {
                        common.push(*id);
                    }
                }
                Acknowledgement::Ready => {
                    // NOTE: In git, there is some logic dealing with whether to expect a DELIM or FLUSH package,
                    //       but we handle this with peeking.
                }
                Acknowledgement::Nak => {}
            }
        }
    }

    // `common` is set only if this is a stateless transport, and we repeat previously confirmed common commits as HAVE, because
    // we are not going to repeat them otherwise.
    if let Some(common) = common {
        for have_id in common {
            arguments.have(have_id);
        }
    }

    let mut haves_sent = 0;
    for have_id in (0..haves_to_send).map_while(|_| negotiator.next_have(graph)) {
        arguments.have(have_id?);
        haves_sent += 1;
    }
    // Note that we are differing from the git implementation, which does an extra-round of with no new haves sent at all.
    // For us it seems better to just say we are done when we know we are done, as potentially additional acks won't affect the
    // queue of any of our implementation at all (so the negotiator won't come up with more haves next time either).
    Ok((haves_sent, seen_ack))
}
