use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use git_object::bstr::BStr;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Outcome<'a> {
    pub name: Cow<'a, BStr>,
    pub id: git_hash::ObjectId,
    pub depth: u32,
}

impl<'a> Outcome<'a> {
    pub fn into_format(self, hex_len: usize) -> Format<'a> {
        Format {
            name: self.name,
            id: self.id,
            hex_len,
            depth: self.depth,
            long: false,
            dirty_suffix: None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Format<'a> {
    pub name: Cow<'a, BStr>,
    pub id: git_hash::ObjectId,
    pub hex_len: usize,
    pub depth: u32,
    pub long: bool,
    pub dirty_suffix: Option<String>,
}

impl<'a> Format<'a> {
    pub fn is_exact_match(&self) -> bool {
        self.depth == 0
    }
    pub fn long(&mut self) -> &mut Self {
        self.long = true;
        self
    }
    pub fn short(&mut self) -> &mut Self {
        self.long = false;
        self
    }
}

impl<'a> Display for Format<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.long && self.is_exact_match() {
            self.name.fmt(f)?;
        } else {
            write!(
                f,
                "{}-{}-g{}",
                self.name,
                self.depth,
                self.id.to_hex_with_len(self.hex_len)
            )?;
        }
        if let Some(suffix) = &self.dirty_suffix {
            write!(f, "-{}", suffix)?;
        }
        Ok(())
    }
}

pub(crate) mod function {
    use std::{
        borrow::Cow,
        collections::{hash_map, HashMap, VecDeque},
        iter::FromIterator,
    };

    use git_hash::{oid, ObjectId};
    use git_object::{bstr::BStr, CommitRefIter};

    use super::Outcome;

    #[allow(clippy::result_unit_err)]
    pub fn describe<'a, Find, E>(
        commit: &oid,
        mut find: Find,
        name_set: &HashMap<ObjectId, Cow<'a, BStr>>,
    ) -> Result<Option<Outcome<'a>>, E>
    where
        Find: for<'b> FnMut(&oid, &'b mut Vec<u8>) -> Result<CommitRefIter<'b>, E>,
        E: std::error::Error + Send + Sync + 'static,
    {
        if let Some(name) = name_set.get(commit) {
            return Ok(Some(Outcome {
                name: name.clone(),
                id: commit.to_owned(),
                depth: 0,
            }));
        }

        let mut buf = Vec::new();
        let mut parent_buf = Vec::new();
        let mut parents = Vec::new();

        let mut queue = VecDeque::from_iter(Some(commit.to_owned()));
        let mut candidates = Vec::new();
        let mut seen_commits = 0;
        let mut _gave_up_on_commit = None;
        let mut seen = hash_hasher::HashedMap::default();
        seen.insert(commit.to_owned(), 0u32);

        const MAX_CANDIDATES: usize = std::mem::size_of::<Flags>() * 8;
        while let Some(commit) = queue.pop_front() {
            seen_commits += 1;
            if let Some(name) = name_set.get(&commit) {
                if candidates.len() < MAX_CANDIDATES {
                    let identity_bit = 1 << candidates.len();
                    candidates.push(Candidate {
                        name: name.clone(),
                        commits_in_its_future: seen_commits - 1,
                        identity_bit,
                        order: candidates.len(),
                    });
                    *seen.get_mut(&commit).expect("inserted") |= identity_bit;
                } else {
                    _gave_up_on_commit = Some(commit);
                    break;
                }
            }

            let flags = seen[&commit];
            for candidate in candidates
                .iter_mut()
                .filter(|c| (flags & c.identity_bit) != c.identity_bit)
            {
                candidate.commits_in_its_future += 1;
            }

            let commit_iter = find(&commit, &mut buf)?;
            parents.clear();
            for token in commit_iter {
                match token {
                    Ok(git_object::commit::ref_iter::Token::Tree { .. }) => continue,
                    Ok(git_object::commit::ref_iter::Token::Parent { id: parent_id }) => {
                        match seen.entry(parent_id) {
                            hash_map::Entry::Vacant(entry) => {
                                let mut parent = find(&parent_id, &mut parent_buf)?;
                                // TODO: figure out if not having a date is a hard error.
                                let parent_commit_date = parent
                                    .committer()
                                    .map(|committer| committer.time.seconds_since_unix_epoch)
                                    .unwrap_or_default();

                                entry.insert(flags);
                                parents.push((parent_id, parent_commit_date));
                            }
                            hash_map::Entry::Occupied(mut entry) => {
                                *entry.get_mut() |= flags;
                            }
                        }
                    }
                    Ok(_unused_token) => break,
                    Err(_err) => todo!("return a decode error"),
                }
            }

            if !parents.is_empty() {
                parents.sort_by(|a, b| a.1.cmp(&b.1).reverse());
                seen.extend(parents.iter().map(|e| (e.0, flags)));
                queue.extend(parents.iter().map(|e| e.0));
            }
        }

        if candidates.is_empty() {
            return Ok(None);
        }

        candidates.sort_by(|a, b| {
            a.commits_in_its_future
                .cmp(&b.commits_in_its_future)
                .then_with(|| a.order.cmp(&b.order))
        });
        Ok(candidates.into_iter().next().map(|c| Outcome {
            name: c.name,
            id: commit.to_owned(),
            depth: c.commits_in_its_future,
        }))
    }

    type Flags = u32;

    #[derive(Debug)]
    struct Candidate<'a> {
        name: Cow<'a, BStr>,
        commits_in_its_future: Flags,
        /// A single bit identifying this candidate uniquely in a bitset
        identity_bit: Flags,
        /// The order at which we found the candidate, first one has order = 0
        order: usize,
    }
}
