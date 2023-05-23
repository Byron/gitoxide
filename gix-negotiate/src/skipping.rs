use crate::consecutive::collect_parents;
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
        /// The remote let us know it has the object. We still have to tell the server we have this object or one of its descendants.
        /// We won't tell the server about its ancestors.
        const ADVERTISED = 1 << 1;
        /// The revision has at one point entered the priority queue (even though it might not be on it anymore).
        const SEEN = 1 << 2;
        /// The revision was popped off our priority queue.
        const POPPED = 1 << 3;
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Entry {
    flags: Flags,
    /// Only used if commit is not COMMON
    original_ttl: u16,
    ttl: u16,
}

pub(crate) struct Algorithm<'find> {
    graph: gix_revision::Graph<'find, Entry>,
    revs: gix_revision::PriorityQueue<CommitterTimestamp, ObjectId>,
    non_common_revs: usize,
}

impl<'a> Algorithm<'a> {
    pub fn new(graph: gix_revision::Graph<'a, Entry>) -> Self {
        Self {
            graph,
            revs: gix_revision::PriorityQueue::new(),
            non_common_revs: 0,
        }
    }

    /// Add `id` to our priority queue and *add* `flags` to it.
    fn add_to_queue(&mut self, id: ObjectId, mark: Flags) -> Result<(), Error> {
        let commit = self.graph.try_lookup_and_insert(id, |entry| {
            entry.flags |= mark | Flags::SEEN;
        })?;
        if let Some(timestamp) = commit.map(|c| c.committer_timestamp()).transpose()? {
            self.revs.insert(timestamp, id);
            if !mark.contains(Flags::COMMON) {
                self.non_common_revs += 1;
            }
        }
        Ok(())
    }

    fn mark_common(&mut self, id: ObjectId) -> Result<(), Error> {
        let mut is_common = false;
        if let Some(commit) = self
            .graph
            .try_lookup_and_insert(id, |entry| {
                is_common = entry.flags.contains(Flags::COMMON);
                entry.flags |= Flags::COMMON;
            })?
            .filter(|_| !is_common)
        {
            let mut queue = gix_revision::PriorityQueue::from_iter(Some((commit.committer_timestamp()?, id)));
            let mut parents = SmallVec::new();
            while let Some(id) = queue.pop() {
                // This is a bit of a problem as there is no representation of the `parsed` based skip, which probably
                // prevents this traversal from going on for too long. There is no equivalent here, but when artificially
                // limiting the traversal depth the tests fail as they actually require the traversal to happen.
                if self
                    .graph
                    .get(&id)
                    .map_or(false, |entry| entry.flags.contains(Flags::POPPED))
                {
                    self.non_common_revs = self.non_common_revs.saturating_sub(1);
                }
                if let Some(commit) = self.graph.try_lookup_and_insert(id, |entry| {
                    if !entry.flags.contains(Flags::POPPED) {
                        self.non_common_revs -= 1;
                    }
                })? {
                    collect_parents(commit.iter_parents(), &mut parents)?;
                    for parent_id in parents.drain(..) {
                        let mut was_unseen_or_common = false;
                        if let Some(parent) = self
                            .graph
                            .try_lookup_and_insert(parent_id, |entry| {
                                was_unseen_or_common =
                                    !entry.flags.contains(Flags::SEEN) || entry.flags.contains(Flags::COMMON);
                                entry.flags |= Flags::COMMON
                            })?
                            .filter(|_| !was_unseen_or_common)
                        {
                            queue.insert(parent.committer_timestamp()?, parent_id);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn push_parent(&mut self, entry: Entry, parent_id: ObjectId) -> Result<bool, Error> {
        let mut was_seen = false;
        if let Some(parent_entry) = self
            .graph
            .get(&parent_id)
            .map(|entry| {
                was_seen = entry.flags.contains(Flags::SEEN);
                entry
            })
            .filter(|_| was_seen)
        {
            if parent_entry.flags.contains(Flags::POPPED) {
                return Ok(false);
            }
        } else {
            self.add_to_queue(parent_id, Flags::default())?;
        }
        if entry.flags.intersects(Flags::COMMON | Flags::ADVERTISED) {
            self.mark_common(parent_id)?;
        } else {
            let new_original_ttl = if entry.ttl > 0 {
                entry.original_ttl
            } else {
                entry.original_ttl * 3 / 2 + 1
            };
            let new_ttl = if entry.ttl > 0 { entry.ttl - 1 } else { new_original_ttl };
            let parent_entry = self.graph.get_mut(&parent_id).expect("present or inserted");
            if parent_entry.original_ttl < new_original_ttl {
                parent_entry.original_ttl = new_original_ttl;
                parent_entry.ttl = new_ttl;
            }
        }
        Ok(true)
    }
}

impl<'a> Negotiator for Algorithm<'a> {
    fn known_common(&mut self, id: ObjectId) -> Result<(), Error> {
        if self
            .graph
            .get(&id)
            .map_or(false, |entry| entry.flags.contains(Flags::SEEN))
        {
            return Ok(());
        }
        self.add_to_queue(id, Flags::ADVERTISED)
    }

    fn add_tip(&mut self, id: ObjectId) -> Result<(), Error> {
        if self
            .graph
            .get(&id)
            .map_or(false, |entry| entry.flags.contains(Flags::SEEN))
        {
            return Ok(());
        }
        self.add_to_queue(id, Flags::default())
    }

    fn next_have(&mut self) -> Option<Result<ObjectId, Error>> {
        let mut parents = SmallVec::new();
        loop {
            let id = self.revs.pop().filter(|_| self.non_common_revs != 0)?;
            let entry = self.graph.get_mut(&id).expect("it was added to the graph by now");
            entry.flags |= Flags::POPPED;

            if !entry.flags.contains(Flags::COMMON) {
                self.non_common_revs -= 1;
            }
            let mut to_send = None;
            if !entry.flags.contains(Flags::COMMON) && entry.ttl == 0 {
                to_send = Some(id);
            }
            let entry = *entry;

            let commit = match self.graph.try_lookup(&id) {
                Ok(c) => c.expect("it was found before, must still be there"),
                Err(err) => return Some(Err(err.into())),
            };
            if let Err(err) = collect_parents(commit.iter_parents(), &mut parents) {
                return Some(Err(err));
            }
            let mut parent_pushed = false;
            for parent_id in parents.drain(..) {
                parent_pushed |= match self.push_parent(entry, parent_id) {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                }
            }

            if !entry.flags.contains(Flags::COMMON) && !parent_pushed {
                to_send = Some(id);
            }

            if let Some(to_send) = to_send {
                return Some(Ok(to_send));
            }
        }
    }

    fn in_common_with_remote(&mut self, id: ObjectId) -> Result<bool, Error> {
        let mut was_seen = false;
        let known_to_be_common = self.graph.get(&id).map_or(false, |entry| {
            was_seen = entry.flags.contains(Flags::SEEN);
            entry.flags.contains(Flags::COMMON)
        });
        assert!(
            was_seen,
            "Cannot receive ACK for commit we didn't send a HAVE for: {id}"
        );
        self.mark_common(id)?;
        Ok(known_to_be_common)
    }
}
