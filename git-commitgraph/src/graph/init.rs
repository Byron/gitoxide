use crate::{
    file::{self, File},
    Graph, MAX_COMMITS,
};
use git_object::HashKind;
use std::{
    convert::TryFrom,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{}", .path.display())]
    File {
        #[source]
        err: file::Error,
        path: PathBuf,
    },
    #[error("Commit-graph files mismatch: '{}' uses hash {hash1:?}, but '{}' uses hash {hash2:?}", .path1.display(), .path2.display())]
    HashVersionMismatch {
        path1: PathBuf,
        hash1: HashKind,
        path2: PathBuf,
        hash2: HashKind,
    },
    #[error("Did not find any files that look like commit graphs at '{}'", .0.display())]
    InvalidPath(PathBuf),
    #[error("Could not open commit-graph file at '{}'", .path.display())]
    Io {
        #[source]
        err: std::io::Error,
        path: PathBuf,
    },
    #[error(
        "Commit-graph files contain {0} commits altogether, but only {} commits are allowed",
        MAX_COMMITS
    )]
    TooManyCommits(u64),
}

/// Instantiate a `Graph` from various sources.
impl Graph {
    pub fn at(path: impl AsRef<Path>) -> Result<Self, Error> {
        Self::try_from(path.as_ref())
    }

    pub fn from_commit_graphs_dir(path: impl AsRef<Path>) -> Result<Self, Error> {
        let commit_graphs_dir = path.as_ref();
        let chain_file_path = commit_graphs_dir.join("commit-graph-chain");
        let chain_file = std::fs::File::open(&chain_file_path).map_err(|e| Error::Io {
            err: e,
            path: chain_file_path.clone(),
        })?;
        let mut files = Vec::new();
        for line in BufReader::new(chain_file).lines() {
            let hash = line.map_err(|e| Error::Io {
                err: e,
                path: chain_file_path.clone(),
            })?;
            let graph_file_path = commit_graphs_dir.join(format!("graph-{}.graph", hash));
            files.push(File::at(&graph_file_path).map_err(|e| Error::File {
                err: e,
                path: graph_file_path.clone(),
            })?);
        }
        Self::new(files)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let file = File::at(path).map_err(|e| Error::File {
            err: e,
            path: path.to_owned(),
        })?;
        Self::new(vec![file])
    }

    pub fn from_info_dir(info_dir: impl AsRef<Path>) -> Result<Self, Error> {
        Self::from_file(info_dir.as_ref().join("commit-graph"))
            .or_else(|_| Self::from_commit_graphs_dir(info_dir.as_ref().join("commit-graphs")))
    }

    pub fn new(files: Vec<File>) -> Result<Self, Error> {
        let num_commits: u64 = files.iter().map(|f| u64::from(f.num_commits())).sum();
        if num_commits > u64::from(MAX_COMMITS) {
            return Err(Error::TooManyCommits(num_commits));
        }

        for window in files.windows(2) {
            let f1 = &window[0];
            let f2 = &window[1];
            if f1.hash_kind() != f2.hash_kind() {
                return Err(Error::HashVersionMismatch {
                    path1: f1.path().to_owned(),
                    hash1: f1.hash_kind(),
                    path2: f2.path().to_owned(),
                    hash2: f2.hash_kind(),
                });
            }
        }

        Ok(Self { files })
    }
}

impl TryFrom<&Path> for Graph {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_file() {
            // Assume we are looking at `.git/objects/info/commit-graph` or
            // `.git/objects/info/commit-graphs/graph-*.graph`.
            Self::from_file(path)
        } else if path.is_dir() {
            if path.join("commit-graph-chain").is_file() {
                Self::from_commit_graphs_dir(path)
            } else {
                Self::from_info_dir(path)
            }
        } else {
            Err(Error::InvalidPath(path.to_owned()))
        }
    }
}
