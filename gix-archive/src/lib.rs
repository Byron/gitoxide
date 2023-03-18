#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Creating archive failed")]
    FailedToArchive,
    #[error("No such tree, commit or tag id")]
    InvalidTreeish,
}

pub enum Format {
    Tar,
    Zip,
    TarGz,
    Tgz,
    Custom(FromConfig),
}

pub struct FromConfig {
    name: String,
    command: String,
}

struct VirtualFile {
    path: PathBuf,
    content: Vec<u8>,
}

enum Extra {
    Zip(i8),
    Tar(i8),
}

struct Remote {
    repository: String,
    exec: Option<PathBuf>,
}

struct ArchiveBuilder {
    format: Format,
    verbose: bool,
    prefix: Option<PathBuf>,
    output: Output,
    add_files: Vec<PathBuf>,
    add_virtual_files: Vec<VirtualFile>,
    worktree_attributes: bool,
    modification_time: SystemTime,
    extra: Option<Extra>,
    remote: Option<Remote>,
    tree_ish: Treeish,
    include_paths: Vec<PathBuf>,
}

enum Treeish {
    Tree(String),
    Commit(String),
    Tag(String),
}

impl TryFrom<String> for Treeish {
    type Error = Error;

    fn try_from(id: String) -> Result<Self, Self::Error> {
        Ok(Self::Tree(id))
    }
}

enum Output {
    File(PathBuf),
    StdOut,
}

impl ArchiveBuilder {
    fn new(thee_ish_id: String) -> Result<Self, Error> {
        let mut builder = ArchiveBuilder {
            format: Format::Tar,
            verbose: false,
            prefix: None,
            output: Output::StdOut,
            add_files: Vec::new(),
            add_virtual_files: Vec::new(),
            worktree_attributes: false,
            modification_time: SystemTime::now(),
            extra: None,
            remote: None,
            tree_ish: Treeish::try_from(thee_ish_id)?,
            include_paths: Vec::new(),
        };

        if let Treeish::Commit(id) = &builder.tree_ish {
            builder.modification_time = get_commit_timestamp(id);
        }

        Ok(builder)
    }

    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    pub fn prefix(mut self, prefix: PathBuf) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn output(mut self, output_file: PathBuf) -> Self {
        self.output = Output::File(output_file);
        self
    }

    pub fn add_files(mut self, file_paths: Vec<PathBuf>) -> Self {
        if let Some(prefix) = self.prefix.clone() {
            let file_paths: Vec<PathBuf> = file_paths.into_iter().map(|path| prefix.join(path)).collect();
            self.add_files = file_paths;
        } else {
            self.add_files = file_paths;
        }
        self
    }

    // write archive to file or STDOUT
    pub fn archive(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub fn formats() -> Vec<Format> {
    use Format::*;
    let fmts = vec![Tar, Zip, TarGz, Tgz];
    // read config and append custom formats to fmts
    fmts
}

fn get_commit_timestamp(_tree_ish: &str) -> SystemTime {
    SystemTime::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_files_prepends_prefix() {
        let builder = ArchiveBuilder::new("80b8ff3e738f698e3801f8de1d6ec4220651a591".into())
            .expect("valid builder")
            .prefix("pre".into())
            .add_files(vec![PathBuf::from("a"), PathBuf::from("b")]);

        assert_eq!(builder.add_files, vec![PathBuf::from("pre/a"), PathBuf::from("pre/b")])
    }
}
