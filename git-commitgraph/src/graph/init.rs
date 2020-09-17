use crate::{graph_file, Graph, GraphFile, MAX_COMMITS};
use git_object::HashKind;
use quick_error::quick_error;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Corrupt(msg: String, path: PathBuf) {
            display("{}", msg)
        }
        GraphFile(err: graph_file::Error, path: PathBuf) {
            display("{}", path.display())
            source(err)
        }
        HashVersionMismatch(path1: PathBuf, hash1: HashKind, path2: PathBuf, hash2: HashKind) {
            display(
                "Commit-graph files mismatch: '{}' uses hash {:?}, but '{}' uses hash {:?}",
                path1.display(),
                hash1,
                path2.display(),
                hash2,
            )
        }
        Io(err: std::io::Error, path: PathBuf) {
            display("Could not open commit-graph graph_file at '{}'", path.display())
            source(err)
        }
        TooManyCommits(num_commits: u64) {
            display(
                "Commit-graph files contain {} commits altogether, but only {} commits are allowed",
                num_commits,
                MAX_COMMITS,
            )
        }
        // This might actually be legal...
        VersionMismatch(path1: PathBuf, version1: graph_file::Kind, path2: PathBuf, version2: graph_file::Kind) {
            display(
                "Commit-graph files mismatch: '{}' is version {:?}, but '{}' is version {:?}",
                path1.display(),
                version1,
                path2.display(),
                version2,
            )
        }
    }
}

impl Graph {
    pub fn from_info_dir(info_dir: impl AsRef<Path>) -> Result<Self, Error> {
        Self::from_single_file(info_dir.as_ref())
            .or_else(|_| Self::from_split_chain(info_dir.as_ref().join("commit-graphs")))
    }

    pub fn from_single_file(info_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let single_graph_file = info_dir.as_ref().join("commit-graph");
        let file = GraphFile::at(&single_graph_file).map_err(|e| Error::GraphFile(e, single_graph_file.clone()))?;
        Self::new(vec![file])
    }

    pub fn from_split_chain(commit_graphs_dir: impl AsRef<Path>) -> Result<Self, Error> {
        let commit_graphs_dir = commit_graphs_dir.as_ref();
        let chain_file_path = commit_graphs_dir.join("commit-graph-chain");
        let chain_file = std::fs::File::open(&chain_file_path).map_err(|e| Error::Io(e, chain_file_path.clone()))?;
        let mut files: Vec<GraphFile> = Vec::new();
        for line in BufReader::new(chain_file).lines() {
            let line = line.map_err(|e| Error::Io(e, chain_file_path.clone()))?;
            let graph_filename = format!("graph-{}.graph", line);
            let graph_file_path = commit_graphs_dir.join(graph_filename);
            files.push(GraphFile::at(&graph_file_path).map_err(|e| Error::GraphFile(e, graph_file_path.clone()))?);
        }
        Self::new(files)
    }

    pub fn new(files: Vec<GraphFile>) -> Result<Self, Error> {
        let num_commits: u64 = files.iter().map(|f| f.num_commits() as u64).sum();
        if num_commits > MAX_COMMITS as u64 {
            return Err(Error::TooManyCommits(num_commits));
        }

        for window in files.windows(2) {
            let f1 = &window[0];
            let f2 = &window[1];
            if f1.kind() != f2.kind() {
                return Err(Error::VersionMismatch(
                    f1.path().to_owned(),
                    f1.kind(),
                    f2.path().to_owned(),
                    f2.kind(),
                ));
            }
            if f1.hash_kind() != f2.hash_kind() {
                return Err(Error::HashVersionMismatch(
                    f1.path().to_owned(),
                    f1.hash_kind(),
                    f2.path().to_owned(),
                    f2.hash_kind(),
                ));
            }
        }

        Ok(Self { files })
    }
}
