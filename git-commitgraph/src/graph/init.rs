use crate::{
    file::{self, File},
    Graph, MAX_COMMITS,
};
use git_object::HashKind;
use std::{
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
    #[error("Could not open commit-graph file at '{}'", path.display())]
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

/// Instantiate a `Graph` from various sources
impl Graph {
    pub fn from_info_dir(info_dir: impl AsRef<Path>) -> Result<Self, Error> {
        Self::from_single_file(info_dir.as_ref())
            .or_else(|_| Self::from_split_chain(info_dir.as_ref().join("commit-graphs")))
    }

    pub fn from_single_file(info_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let single_graph_file = info_dir.as_ref().join("commit-graph");
        let file = File::at(&single_graph_file).map_err(|e| Error::File {
            err: e,
            path: single_graph_file.clone(),
        })?;
        Self::new(vec![file])
    }

    pub fn from_split_chain(commit_graphs_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let commit_graphs_dir = commit_graphs_dir.as_ref();
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

    pub fn new(files: Vec<File>) -> Result<Self, Error> {
        let num_commits: u64 = files.iter().map(|f| f.num_commits() as u64).sum();
        if num_commits > MAX_COMMITS as u64 {
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
