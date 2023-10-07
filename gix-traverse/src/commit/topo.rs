//! Topological commit traversal
//!
//! Example:
//! ```
//! let repo = gix::discover(".").uwnrap();
//! let spec = repo.rev_parse_single("HEAD").unwrap();
//! let walk = Builder::from_specs(&repo.objects, std::iter::once(spec)).unwrap();
//! for commit_info in walk {
//!     println!("{}", commit_info.id);
//! }
//! ```

use gix_hash::{oid, ObjectId};
use gix_revwalk::{graph::IdMap, PriorityQueue};

use bitflags::bitflags;

use smallvec::SmallVec;

use super::{find, Either, Info, Parents};

#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
/// The errors that can occur during creation and iteration.
pub enum Error {
    #[error("Calculated indegree missing")]
    MissingIndegree,
    #[error("Internal state not found")]
    MissingState,
    #[error(transparent)]
    CommitGraphFile(#[from] gix_commitgraph::file::commit::Error),
    #[error(transparent)]
    ObjectDecode(#[from] gix_object::decode::Error),
    #[error(transparent)]
    Find(#[from] gix_object::find::existing_iter::Error),
}

bitflags! {
    /// Set of flags to describe the state of a particular commit while iterating.
    // NOTE: The names correspond to the names of the flags in revision.h
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WalkFlags: u32 {
        /// Commit has been seen
        const Seen = 0b000001;
        /// Commit has been processed by the Explore walk
        const Explored = 0b000010;
        /// Commit has been processed by the Indegree walk
        const InDegree = 0b000100;
        /// Commit is deemed uninteresting for whatever reason
        const Uninteresting = 0b001000;
        /// Commit marks the end of a walk, like `foo` in `git rev-list foo..bar`
        const Bottom = 0b010000;
        /// Parents have been processed
        const Added = 0b100000;
    }
}

/// Sorting to use for the topological walk
#[derive(Clone, Copy, Debug, Default)]
pub enum Sorting {
    /// Show no parents before all of its children are shown, but otherwise show
    /// commits in the commit timestamp order.
    #[default]
    DateOrder,

    /// Show no parents before all of its children are shown, and avoid
    /// showing commits on multiple lines of history intermixed.
    TopoOrder,
}

// Git's priority queue works as a LIFO stack if no compare function is set,
// which is the case for --topo-order. However, even in that case the initial
// items of the queue are sorted according to the commit time before beginning
// the walk.
#[derive(Debug)]
enum Queue {
    Date(PriorityQueue<i64, Info>),
    Topo(Vec<(i64, Info)>),
}

impl Queue {
    fn new(s: Sorting) -> Self {
        match s {
            Sorting::DateOrder => Self::Date(PriorityQueue::new()),
            Sorting::TopoOrder => Self::Topo(vec![]),
        }
    }

    fn push(&mut self, commit_time: i64, info: Info) {
        match self {
            Self::Date(q) => q.insert(commit_time, info),
            Self::Topo(q) => q.push((commit_time, info)),
        }
    }

    fn pop(&mut self) -> Option<Info> {
        match self {
            Self::Date(q) => q.pop().map(|(_, info)| info),
            Self::Topo(q) => q.pop().map(|(_, info)| info),
        }
    }

    fn initial_sort(&mut self) {
        if let Self::Topo(ref mut inner_vec) = self {
            inner_vec.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }
}

type GenAndCommitTime = (u32, i64);

/// Builder for [`Walk`]
pub struct Builder<Find, Predicate> {
    commit_graph: Option<gix_commitgraph::Graph>,
    find: Find,
    predicate: Predicate,
    sorting: Sorting,
    parents: Parents,
    tips: Vec<ObjectId>,
    ends: Vec<ObjectId>,
}

impl<Find> Builder<Find, fn(&oid) -> bool>
where
    Find: gix_object::Find,
{
    /// Create a new `Builder` for a [`Walk`] that walks the given repository,
    /// starting at the tips and ending at the ends. Like `git rev-list
    /// --topo-order ^ends... tips...`
    pub fn from_iters(
        find: Find,
        tips: impl IntoIterator<Item = impl Into<ObjectId>>,
        ends: Option<impl IntoIterator<Item = impl Into<ObjectId>>>,
    ) -> Self {
        let tips = tips.into_iter().map(Into::into).collect::<Vec<_>>();
        let ends = ends
            .map(|e| e.into_iter().map(Into::into).collect::<Vec<_>>())
            .unwrap_or_default();

        Self {
            commit_graph: Default::default(),
            find,
            sorting: Default::default(),
            parents: Default::default(),
            tips,
            ends,
            predicate: |_| true,
        }
    }

    /// Create a new `Builder` for a [`Walk`] that walks the given repository
    /// from an iterator of `Specs`, given by the [`Spec`s](gix_revision::Spec)
    pub fn from_specs(find: Find, specs: impl IntoIterator<Item = gix_revision::Spec>) -> Self {
        let mut tips = vec![];
        let mut ends = vec![];

        for spec in specs {
            use gix_revision::Spec as S;
            match spec {
                S::Include(i) => tips.push(i),
                S::Exclude(e) => ends.push(e),
                S::Range { from, to } => {
                    tips.push(to);
                    ends.push(from)
                }
                S::Merge { .. } => todo!(),
                S::IncludeOnlyParents(_) => todo!(),
                S::ExcludeParents(_) => todo!(),
            }
        }

        Self {
            commit_graph: Default::default(),
            find,
            sorting: Default::default(),
            parents: Default::default(),
            tips,
            ends,
            predicate: |_| true,
        }
    }

    /// Set a predicate to filter out revisions from the walk. Can be used to
    /// implement e.g. filtering on paths or time. This does *not* exclude the
    /// parent(s) of a revision that is excluded. Specify a revision as an end
    /// if you want that behavior.
    pub fn with_predicate<Predicate>(self, predicate: Predicate) -> Builder<Find, Predicate>
    where
        Predicate: FnMut(&oid) -> bool,
    {
        Builder {
            commit_graph: self.commit_graph,
            find: self.find,
            sorting: self.sorting,
            parents: self.parents,
            tips: self.tips,
            ends: self.ends,
            predicate,
        }
    }
}
impl<Find, Predicate> Builder<Find, Predicate>
where
    Find: gix_object::Find,
    Predicate: FnMut(&oid) -> bool,
{
    /// Set the [`Sorting`] to use for the topological walk
    pub fn sorting(mut self, sorting: Sorting) -> Self {
        self.sorting = sorting;
        self
    }

    /// Specify how to handle commit parents during traversal.
    pub fn parents(mut self, parents: Parents) -> Self {
        self.parents = parents;
        self
    }

    /// Set or unset the commit-graph to use for the iteration.
    pub fn with_commit_graph(mut self, commit_graph: Option<gix_commitgraph::Graph>) -> Self {
        self.commit_graph = commit_graph;
        self
    }

    /// Build a new [`Walk`] instance. Note that merely building an instance is
    /// currently expensive.
    pub fn build(self) -> Result<Walk<Find, Predicate>, Error> {
        let mut w = Walk {
            commit_graph: self.commit_graph,
            find: self.find,
            predicate: self.predicate,
            indegrees: IdMap::default(),
            states: IdMap::default(),
            explore_queue: PriorityQueue::new(),
            indegree_queue: PriorityQueue::new(),
            topo_queue: Queue::new(self.sorting),
            parents: self.parents,
            min_gen: gix_commitgraph::GENERATION_NUMBER_INFINITY,
            buf: vec![],
        };

        // Initial flags for the states of the tips and ends. All of them are
        // seen and added to the explore and indegree queues. The ends are by
        // definition (?) uninteresting and bottom.
        let tip_flags = WalkFlags::Seen | WalkFlags::Explored | WalkFlags::InDegree;
        let end_flags = tip_flags | WalkFlags::Uninteresting | WalkFlags::Bottom;

        for (id, flags) in self
            .tips
            .iter()
            .map(|id| (id, tip_flags))
            .chain(self.ends.iter().map(|id| (id, end_flags)))
        {
            *w.indegrees.entry(*id).or_default() = 1;

            let commit = find(w.commit_graph.as_ref(), &w.find, id, &mut w.buf)?;

            let (gen, time) = gen_and_commit_time(commit)?;

            if gen < w.min_gen {
                w.min_gen = gen;
            }

            w.states.insert(*id, flags);
            w.explore_queue.insert((gen, time), *id);
            w.indegree_queue.insert((gen, time), *id);
        }

        // NOTE: Parents of the ends must also be marked uninteresting for some
        // reason. See handle_commit()
        for id in &self.ends {
            let parents = w.collect_all_parents(id)?;
            for (id, _) in parents {
                w.states
                    .entry(id)
                    .and_modify(|s| *s |= WalkFlags::Uninteresting)
                    .or_insert(WalkFlags::Uninteresting | WalkFlags::Seen);
            }
        }

        w.compute_indegrees_to_depth(w.min_gen)?;

        // NOTE: in Git the ends are also added to the topo_queue in addition to
        // the tips, but then in simplify_commit() Git is told to ignore it. For
        // now the tests pass.
        for id in self.tips.iter() {
            let i = w.indegrees.get(id).ok_or(Error::MissingIndegree)?;

            if *i == 1 {
                let commit = find(w.commit_graph.as_ref(), &w.find, id, &mut w.buf)?;

                let (_, time) = gen_and_commit_time(commit)?;

                let parent_ids = w.collect_all_parents(id)?.into_iter().map(|e| e.0).collect();

                w.topo_queue.push(
                    time,
                    Info {
                        id: *id,
                        parent_ids,
                        commit_time: Some(time),
                    },
                );
            }
        }

        w.topo_queue.initial_sort();

        Ok(w)
    }
}

/// A commit walker that walks in topographical order, like `git rev-list
/// --topo-order` or `--date-order` depending on the chosen [`Sorting`]
pub struct Walk<Find, Predicate> {
    commit_graph: Option<gix_commitgraph::Graph>,
    find: Find,
    predicate: Predicate,
    indegrees: IdMap<i32>,
    states: IdMap<WalkFlags>,
    explore_queue: PriorityQueue<GenAndCommitTime, ObjectId>,
    indegree_queue: PriorityQueue<GenAndCommitTime, ObjectId>,
    topo_queue: Queue,
    parents: Parents,
    min_gen: u32,
    buf: Vec<u8>,
}

impl<Find, Predicate> Walk<Find, Predicate>
where
    Find: gix_object::Find,
{
    fn compute_indegrees_to_depth(&mut self, gen_cutoff: u32) -> Result<(), Error> {
        while let Some(((gen, _), _)) = self.indegree_queue.peek() {
            if *gen >= gen_cutoff {
                self.indegree_walk_step()?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn indegree_walk_step(&mut self) -> Result<(), Error> {
        if let Some(((gen, _), id)) = self.indegree_queue.pop() {
            self.explore_to_depth(gen)?;

            let parents = self.collect_parents(&id)?;

            for (id, gen_time) in parents {
                self.indegrees.entry(id).and_modify(|e| *e += 1).or_insert(2);

                let state = self.states.get_mut(&id).ok_or(Error::MissingState)?;

                if !state.contains(WalkFlags::InDegree) {
                    *state |= WalkFlags::InDegree;
                    self.indegree_queue.insert(gen_time, id);
                }
            }
        }

        Ok(())
    }

    fn explore_to_depth(&mut self, gen_cutoff: u32) -> Result<(), Error> {
        while let Some(((gen, _), _)) = self.explore_queue.peek() {
            if *gen >= gen_cutoff {
                self.explore_walk_step()?;
            } else {
                break;
            }
        }

        Ok(())
    }

    fn explore_walk_step(&mut self) -> Result<(), Error> {
        if let Some((_, id)) = self.explore_queue.pop() {
            let parents = self.collect_parents(&id)?;

            self.process_parents(&id, &parents)?;

            for (id, gen_time) in parents {
                let state = self.states.get_mut(&id).ok_or(Error::MissingState)?;

                if !state.contains(WalkFlags::Explored) {
                    *state |= WalkFlags::Explored;
                    self.explore_queue.insert(gen_time, id);
                }
            }
        }

        Ok(())
    }

    fn expand_topo_walk(&mut self, id: &oid) -> Result<(), Error> {
        let parents = self.collect_parents(id)?;

        self.process_parents(id, &parents)?;

        for (pid, (parent_gen, parent_commit_time)) in parents {
            let parent_state = self.states.get(&pid).ok_or(Error::MissingState)?;

            if parent_state.contains(WalkFlags::Uninteresting) {
                continue;
            }

            if parent_gen < self.min_gen {
                self.min_gen = parent_gen;
                self.compute_indegrees_to_depth(self.min_gen)?;
            }

            let i = self.indegrees.get_mut(&pid).ok_or(Error::MissingIndegree)?;

            *i -= 1;

            if *i == 1 {
                let parent_ids = self.collect_all_parents(&pid)?.into_iter().map(|e| e.0).collect();

                self.topo_queue.push(
                    parent_commit_time,
                    Info {
                        id: pid,
                        parent_ids,
                        commit_time: Some(parent_commit_time),
                    },
                );
            }
        }

        Ok(())
    }

    fn process_parents(&mut self, id: &oid, parents: &[(ObjectId, GenAndCommitTime)]) -> Result<(), Error> {
        let state = self.states.get_mut(id).ok_or(Error::MissingState)?;

        if state.contains(WalkFlags::Added) {
            return Ok(());
        }

        *state |= WalkFlags::Added;

        // If the current commit is uninteresting we pass that on to ALL
        // parents, otherwise we set the Seen flag.
        let (pass, insert) = if state.contains(WalkFlags::Uninteresting) {
            let flags = WalkFlags::Uninteresting.into();

            for (id, _) in parents {
                let grand_parents = self.collect_all_parents(id)?;

                for (id, _) in &grand_parents {
                    self.states
                        .entry(*id)
                        .and_modify(|s| *s |= WalkFlags::Uninteresting)
                        .or_insert(WalkFlags::Uninteresting | WalkFlags::Seen);
                }
            }

            (flags, flags)
        } else {
            // NOTE: git sets SEEN like we do but keeps the SYMMETRIC_LEFT and
            // ANCENSTRY_PATH if they are set, but they have no purpose here.
            let flags = WalkFlags::empty();
            (flags, flags | WalkFlags::Seen)
        };

        for (id, _) in parents {
            self.states.entry(*id).and_modify(|s| *s |= pass).or_insert(insert);
        }

        Ok(())
    }

    fn collect_parents(&mut self, id: &oid) -> Result<SmallVec<[(ObjectId, GenAndCommitTime); 1]>, Error> {
        collect_parents(
            self.commit_graph.as_ref(),
            &self.find,
            id,
            matches!(self.parents, Parents::First),
            &mut self.buf,
        )
    }

    // Same as collect_parents but disregards the first_parent flag
    fn collect_all_parents(&mut self, id: &oid) -> Result<SmallVec<[(ObjectId, GenAndCommitTime); 1]>, Error> {
        collect_parents(self.commit_graph.as_ref(), &self.find, id, false, &mut self.buf)
    }

    fn pop_commit(&mut self) -> Option<Result<Info, Error>> {
        let id = self.topo_queue.pop()?;

        let i = match self.indegrees.get_mut(&id.id) {
            Some(i) => i,
            None => {
                return Some(Err(Error::MissingIndegree));
            }
        };

        *i = 0;

        match self.expand_topo_walk(&id.id) {
            Ok(_) => (),
            Err(e) => {
                return Some(Err(e));
            }
        };

        Some(Ok(id))
    }
}

impl<Find, Predicate> Iterator for Walk<Find, Predicate>
where
    Find: gix_object::Find,
    Predicate: FnMut(&oid) -> bool,
{
    type Item = Result<Info, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pop_commit()? {
            Ok(id) => {
                if (self.predicate)(&id.id) {
                    Some(Ok(id))
                } else {
                    self.next()
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

fn collect_parents<Find>(
    cache: Option<&gix_commitgraph::Graph>,
    f: Find,
    id: &oid,
    first_only: bool,
    buf: &mut Vec<u8>,
) -> Result<SmallVec<[(ObjectId, GenAndCommitTime); 1]>, Error>
where
    Find: gix_object::Find,
{
    let mut parents = SmallVec::<[(ObjectId, GenAndCommitTime); 1]>::new();

    match find(cache, &f, id, buf)? {
        Either::CommitRefIter(c) => {
            for token in c {
                use gix_object::commit::ref_iter::Token as T;
                match token {
                    Ok(T::Tree { .. }) => continue,
                    Ok(T::Parent { id }) => {
                        parents.push((id, (0, 0))); // Dummy numbers to be filled in
                        if first_only {
                            break;
                        }
                    }
                    Ok(_past_parents) => break,
                    Err(err) => return Err(err.into()),
                }
            }
            // Need to check the cache again. That a commit is not in the cache
            // doesn't mean a parent is not.
            for (id, gen_time) in parents.iter_mut() {
                let commit = find(cache, &f, id, buf)?;
                *gen_time = gen_and_commit_time(commit)?;
            }
        }
        Either::CachedCommit(c) => {
            for pos in c.iter_parents() {
                let parent_commit = cache
                    .expect("cache exists if CachedCommit was returned")
                    .commit_at(pos?);
                parents.push((
                    parent_commit.id().into(),
                    (parent_commit.generation(), parent_commit.committer_timestamp() as i64),
                ));
                if first_only {
                    break;
                }
            }
        }
    };

    Ok(parents)
}

fn gen_and_commit_time(c: Either<'_, '_>) -> Result<GenAndCommitTime, Error> {
    match c {
        Either::CommitRefIter(c) => {
            let mut commit_time = 0;
            for token in c {
                use gix_object::commit::ref_iter::Token as T;
                match token {
                    Ok(T::Tree { .. }) => continue,
                    Ok(T::Parent { .. }) => continue,
                    Ok(T::Author { .. }) => continue,
                    Ok(T::Committer { signature }) => {
                        commit_time = signature.time.seconds;
                        break;
                    }
                    Ok(_unused_token) => break,
                    Err(err) => return Err(err.into()),
                }
            }
            Ok((gix_commitgraph::GENERATION_NUMBER_INFINITY, commit_time))
        }
        Either::CachedCommit(c) => Ok((c.generation(), c.committer_timestamp() as i64)),
    }
}
