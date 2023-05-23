use crate::{Error, Negotiator};
use gix_hash::ObjectId;
use gix_revision::graph::CommitterTimestamp;
use smallvec::SmallVec;
bitflags::bitflags! {
    /// Whether something can be read or written.
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Flags: u8 {
        /// The revision is known to be in common with the remote.
        const COMMON = 1 << 0;
        /// The revision is common and was set by merit of a remote tracking ref (e.g. `refs/heads/origin/main`).
        const COMMON_REF = 1 << 1;
        /// The revision has entered the priority queue.
        const SEEN = 1 << 2;
        /// The revision was popped off our primary priority queue, used to avoid double-counting of `non_common_revs`
        const POPPED = 1 << 3;
    }
}

pub(crate) struct Algorithm<'find> {
    graph: gix_revision::Graph<'find, Flags>,
    revs: gix_revision::PriorityQueue<CommitterTimestamp, ObjectId>,
    non_common_revs: usize,
}

impl<'a> Algorithm<'a> {
    pub fn new(graph: gix_revision::Graph<'a, Flags>) -> Self {
        Self {
            graph,
            revs: gix_revision::PriorityQueue::new(),
            non_common_revs: 0,
        }
    }

    /// Add `id` to our priority queue and *add* `flags` to it.
    fn add_to_queue(&mut self, id: ObjectId, mark: Flags) -> Result<(), Error> {
        let mut is_common = false;
        if self.graph.get(&id).map_or(false, |flags| flags.intersects(mark)) {
            return Ok(());
        }
        let commit = self.graph.try_lookup_and_insert(id, |current| {
            *current |= mark;
            is_common = current.contains(Flags::COMMON);
        })?;
        if let Some(timestamp) = commit.map(|c| c.committer_timestamp()).transpose()? {
            self.revs.insert(timestamp, id);
            if !is_common {
                self.non_common_revs += 1;
            }
        }
        Ok(())
    }

    fn mark_common(&mut self, id: ObjectId, mode: Mark, ancestors: Ancestors) -> Result<(), Error> {
        let mut is_common = false;
        if let Some(commit) = self
            .graph
            .try_lookup_and_insert(id, |current| is_common = current.contains(Flags::COMMON))?
            .filter(|_| !is_common)
        {
            let mut queue =
                gix_revision::PriorityQueue::from_iter(Some((commit.committer_timestamp()?, (id, 0_usize))));
            if let Mark::ThisCommitAndAncestors = mode {
                let current = self.graph.get_mut(&id).expect("just inserted");
                *current |= Flags::COMMON;
                if current.contains(Flags::SEEN) && !current.contains(Flags::POPPED) {
                    self.non_common_revs -= 1;
                }
            }
            let mut parents = SmallVec::new();
            while let Some((id, generation)) = queue.pop() {
                if self.graph.get(&id).map_or(true, |d| !d.contains(Flags::SEEN)) {
                    self.add_to_queue(id, Flags::SEEN)?;
                } else if matches!(ancestors, Ancestors::AllUnseen) || generation < 2 {
                    if let Some(commit) = self.graph.try_lookup_and_insert(id, |_| {})? {
                        collect_parents(commit.iter_parents(), &mut parents)?;
                        for parent_id in parents.drain(..) {
                            let mut prev_flags = Flags::default();
                            if let Some(parent) = self
                                .graph
                                .try_lookup_and_insert(parent_id, |d| {
                                    prev_flags = *d;
                                    *d |= Flags::COMMON;
                                })?
                                .filter(|_| !prev_flags.contains(Flags::COMMON))
                            {
                                if prev_flags.contains(Flags::SEEN) && !prev_flags.contains(Flags::POPPED) {
                                    self.non_common_revs -= 1;
                                }
                                queue.insert(parent.committer_timestamp()?, (parent_id, generation + 1))
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub(crate) fn collect_parents(
    parents: gix_revision::graph::commit::Parents<'_>,
    out: &mut SmallVec<[ObjectId; 2]>,
) -> Result<(), Error> {
    out.clear();
    for parent in parents {
        out.push(parent.map_err(|err| match err {
            gix_revision::graph::commit::iter_parents::Error::DecodeCommit(err) => Error::DecodeCommit(err),
            gix_revision::graph::commit::iter_parents::Error::DecodeCommitGraph(err) => Error::DecodeCommitInGraph(err),
        })?);
    }
    Ok(())
}

impl<'a> Negotiator for Algorithm<'a> {
    fn known_common(&mut self, id: ObjectId) -> Result<(), Error> {
        if self.graph.get(&id).map_or(true, |d| !d.contains(Flags::SEEN)) {
            self.add_to_queue(id, Flags::COMMON_REF | Flags::SEEN)?;
            self.mark_common(id, Mark::AncestorsOnly, Ancestors::DirectUnseen)?;
        }
        Ok(())
    }

    fn add_tip(&mut self, id: ObjectId) -> Result<(), Error> {
        self.add_to_queue(id, Flags::SEEN)
    }

    fn next_have(&mut self) -> Option<Result<ObjectId, Error>> {
        let mut parents = SmallVec::new();
        loop {
            let id = self.revs.pop().filter(|_| self.non_common_revs != 0)?;
            let flags = self.graph.get_mut(&id).expect("it was added to the graph by now");
            *flags |= Flags::POPPED;

            if !flags.contains(Flags::COMMON) {
                self.non_common_revs -= 1;
            }

            let (res, mark) = if flags.contains(Flags::COMMON) {
                (None, Flags::COMMON | Flags::SEEN)
            } else if flags.contains(Flags::COMMON_REF) {
                (Some(id), Flags::COMMON | Flags::SEEN)
            } else {
                (Some(id), Flags::SEEN)
            };

            let commit = match self.graph.try_lookup(&id) {
                Ok(c) => c.expect("it was found before, must still be there"),
                Err(err) => return Some(Err(err.into())),
            };
            if let Err(err) = collect_parents(commit.iter_parents(), &mut parents) {
                return Some(Err(err));
            }
            for parent_id in parents.drain(..) {
                if self.graph.get(&parent_id).map_or(true, |d| !d.contains(Flags::SEEN)) {
                    if let Err(err) = self.add_to_queue(parent_id, mark) {
                        return Some(Err(err));
                    }
                }
                if mark.contains(Flags::COMMON) {
                    if let Err(err) = self.mark_common(parent_id, Mark::AncestorsOnly, Ancestors::AllUnseen) {
                        return Some(Err(err));
                    }
                }
            }

            if let Some(id) = res {
                return Some(Ok(id));
            }
        }
    }

    fn in_common_with_remote(&mut self, id: ObjectId) -> Result<bool, Error> {
        let known_to_be_common = self.graph.get(&id).map_or(false, |d| d.contains(Flags::COMMON));
        self.mark_common(id, Mark::ThisCommitAndAncestors, Ancestors::DirectUnseen)?;
        Ok(known_to_be_common)
    }
}

enum Mark {
    AncestorsOnly,
    ThisCommitAndAncestors,
}

enum Ancestors {
    /// Traverse only the parents of a commit.
    DirectUnseen,
    /// Traverse all ancestors that weren't yet seen.
    AllUnseen,
}
