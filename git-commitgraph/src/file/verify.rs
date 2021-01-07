use crate::{
    file::{self, File},
    GENERATION_NUMBER_INFINITY, GENERATION_NUMBER_MAX,
};
use bstr::ByteSlice;
use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;
use git_object::{borrowed, owned};
use std::{
    cmp::{max, min},
    collections::HashMap,
    convert::TryFrom,
    path::Path,
};

/// The error used in [`File::traverse()`].
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error<E: std::error::Error + 'static> {
    #[error(transparent)]
    Commit(#[from] file::commit::Error),
    #[error("commit at file position {pos} has invalid ID {id}")]
    CommitId { id: owned::Id, pos: file::Position },
    #[error("commit at file position {pos} with ID {id} is out of order relative to its predecessor with ID {predecessor_id}")]
    CommitsOutOfOrder {
        id: owned::Id,
        pos: file::Position,
        predecessor_id: owned::Id,
    },
    #[error("commit-graph filename should be {0}")]
    Filename(String),
    #[error("commit {id} has invalid generation {generation}")]
    Generation { generation: u32, id: owned::Id },
    #[error("checksum mismatch: expected {expected}, got {actual}")]
    Mismatch { actual: owned::Id, expected: owned::Id },
    #[error("{0}")]
    Processor(#[source] E),
    #[error("commit {id} has invalid root tree ID {root_tree_id}")]
    RootTreeId { id: owned::Id, root_tree_id: owned::Id },
}

/// The positive result of [`File::traverse()`] providing some statistical information.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde1", derive(serde::Deserialize, serde::Serialize))]
pub struct Outcome {
    /// The highest encountered [`file::Commit`] generation number.
    pub max_generation: u32,
    /// The smallest encountered [`file::Commit`] generation number.
    pub min_generation: u32,
    /// The highest amount parents in a single [`file::Commit`].
    pub max_parents: u32,
    /// The total amount of [`commits`][file::Commit] seen in the iteration.
    pub num_commits: u32,
    /// A mapping of `parent-count -> amount of [commits][file::Commit] with that count`.
    pub parent_counts: HashMap<u32, u32>,
}

/// Verification
impl File {
    /// Returns the trailing checksum over the entire content of this file.
    pub fn checksum(&self) -> borrowed::Id<'_> {
        borrowed::Id::try_from(&self.data[self.data.len() - SHA1_SIZE..]).expect("file to be large enough for a hash")
    }

    /// Traverse all [commits][file::Commit] stored in this file and call `processor(commit) -> Result<(), Error>` on it.
    ///
    /// If the `processor` fails, the iteration will be stopped and the entire call results in the respective error.
    pub fn traverse<'a, E, Processor>(&'a self, mut processor: Processor) -> Result<Outcome, Error<E>>
    where
        E: std::error::Error + 'static,
        Processor: FnMut(&file::Commit<'a>) -> Result<(), E>,
    {
        self.verify_checksum()
            .map_err(|(actual, expected)| Error::Mismatch { actual, expected })?;
        verify_split_chain_filename_hash(&self.path, self.checksum()).map_err(Error::Filename)?;

        let null_id = borrowed::Id::null_sha1();

        let mut stats = Outcome {
            max_generation: 0,
            max_parents: 0,
            min_generation: GENERATION_NUMBER_INFINITY,
            num_commits: self.num_commits(),
            parent_counts: HashMap::new(),
        };

        // TODO: Verify self.fan values as we go.
        let mut prev_id: borrowed::Id<'a> = null_id;
        for commit in self.iter_commits() {
            if commit.id() <= prev_id {
                if commit.id() == null_id {
                    return Err(Error::CommitId {
                        pos: commit.position(),
                        id: commit.id().into(),
                    });
                }
                return Err(Error::CommitsOutOfOrder {
                    pos: commit.position(),
                    id: commit.id().into(),
                    predecessor_id: prev_id.into(),
                });
            }
            if commit.root_tree_id() == null_id {
                return Err(Error::RootTreeId {
                    id: commit.id().into(),
                    root_tree_id: commit.root_tree_id().into(),
                });
            }
            if commit.generation() > GENERATION_NUMBER_MAX {
                return Err(Error::Generation {
                    generation: commit.generation(),
                    id: commit.id().into(),
                });
            }

            processor(&commit).map_err(Error::Processor)?;

            stats.max_generation = max(stats.max_generation, commit.generation());
            stats.min_generation = min(stats.min_generation, commit.generation());
            let parent_count = commit
                .iter_parents()
                .try_fold(0u32, |acc, pos| pos.map(|_| acc + 1))
                .map_err(Error::Commit)?;
            *stats.parent_counts.entry(parent_count).or_insert(0) += 1;
            prev_id = commit.id();
        }

        if stats.min_generation == GENERATION_NUMBER_INFINITY {
            stats.min_generation = 0;
        }

        Ok(stats)
    }

    /// Assure the [`checksum`][File::checksum()] matches the actual checksum over all content of this file, excluding the trailing
    /// checksum itself.
    ///
    /// Return the actual checksum on success or `(actual checksum, expected checksum)` if there is a mismatch.
    pub fn verify_checksum(&self) -> Result<owned::Id, (owned::Id, owned::Id)> {
        // Even though we could use git_features::hash::bytes_of_file(…), this would require using our own
        // Error type to support io::Error and Mismatch. As we only gain progress, there probably isn't much value
        // as these files are usually small enough to process them in less than a second, even for the large ones.
        // But it's possible, once a progress instance is passed.
        let data_len_without_trailer = self.data.len() - SHA1_SIZE;
        let mut hasher = git_features::hash::Sha1::default();
        hasher.update(&self.data[..data_len_without_trailer]);
        let actual = owned::Id::new_sha1(hasher.digest());

        let expected = self.checksum();
        if actual.to_borrowed() == expected {
            Ok(actual)
        } else {
            Err((actual, expected.into()))
        }
    }
}

/// If the given path's filename matches "graph-{hash}.graph", check that `hash` matches the
/// expected hash.
fn verify_split_chain_filename_hash(path: impl AsRef<Path>, expected: borrowed::Id<'_>) -> Result<(), String> {
    let path = path.as_ref();
    path.file_name()
        .and_then(|filename| filename.to_str())
        .and_then(|filename| filename.strip_suffix(".graph"))
        .and_then(|stem| stem.strip_prefix("graph-"))
        .map_or(Ok(()), |hex| match owned::Id::from_40_bytes_in_hex(hex.as_bytes()) {
            Ok(actual) if actual.to_borrowed() == expected => Ok(()),
            _ => Err(format!("graph-{}.graph", expected.to_sha1_hex().as_bstr())),
        })
}
