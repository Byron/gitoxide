use git_object::bstr::BStr;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome<'a> {
    pub name: Cow<'a, BStr>,
    pub id: git_hash::ObjectId,
    pub hex_len: usize,
    pub depth: usize,
    pub long: bool,
    pub dirty_suffix: Option<String>,
}

impl<'a> Outcome<'a> {
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

impl<'a> Display for Outcome<'a> {
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
    use super::Outcome;
    use git_hash::{oid, ObjectId};
    use git_object::bstr::BStr;
    use git_object::CommitRefIter;
    use std::borrow::Cow;
    use std::collections::{HashMap, VecDeque};
    use std::iter::FromIterator;

    #[allow(clippy::result_unit_err)]
    pub fn describe<'a, Find, E>(
        commit: &oid,
        mut find: Find,
        hex_len: usize,
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
                hex_len,
                depth: 0,
                long: false,
                dirty_suffix: None,
            }));
        }
        type Flags = usize;
        let mut queue = VecDeque::from_iter(Some((commit.to_owned(), 0 as Flags)));
        let mut candidates = Vec::new();
        let mut seen_commits = 0;
        let mut gave_up_on_commit = None;
        let mut seen = hash_hasher::HashedSet::default();
        let mut buf = Vec::new();

        const MAX_CANDIDATES: usize = std::mem::size_of::<Flags>() * 8;
        while let Some((commit, flags)) = queue.pop_front() {
            seen_commits += 1;
            assert!(seen.insert(commit), "BUG: shouldn't ever see dupes here");
            if let Some(name) = name_set.get(&commit) {
                if candidates.len() < MAX_CANDIDATES {
                    candidates.push(Candidate {
                        name: name.clone(),
                        commits_in_its_future: seen_commits - 1,
                        identity_bit: 1 << candidates.len(),
                        order: candidates.len(),
                    });
                } else {
                    gave_up_on_commit = Some(commit);
                    break;
                }
            }

            for candidate in candidates
                .iter_mut()
                .filter(|c| !((flags & c.identity_bit) == c.identity_bit))
            {
                candidate.commits_in_its_future += 1;
            }

            let commit_iter = find(&commit, &mut buf)?;
            for token in commit_iter {
                match token {
                    Ok(git_object::commit::ref_iter::Token::Tree { .. }) => continue,
                    Ok(git_object::commit::ref_iter::Token::Parent { id }) => {
                        let mut parent = find(id.as_ref(), &mut buf)?;

                        // TODO: figure out if not having a date is a hard error.
                        let parent_committer_date = parent
                            .committer()
                            .map(|committer| committer.time.time)
                            .unwrap_or_default();

                        // queue.binary_search_by_key()
                    }
                    Ok(_unused_token) => break,
                    Err(err) => todo!("return a decode error"),
                }
            }
        }
        todo!("actually search for it")
    }

    struct Candidate<'a> {
        name: Cow<'a, BStr>,
        commits_in_its_future: usize,
        /// A single bit identifying this candidate uniquely in a bitset
        identity_bit: usize,
        /// The order at which we found the candidate, first one has order = 0
        order: usize,
    }
}
