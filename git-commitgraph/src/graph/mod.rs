mod access;
mod init;

use crate::file::File;
use std::fmt::{Display, Formatter};

/// A complete commit graph.
///
/// The data in the commit graph may come from a monolithic `objects/info/commit-graph` file, or it
/// may come from one or more `objects/info/commit-graphs/graph-*.graph` files. These files are
/// generated via `git commit-graph write ...` commands.
pub struct Graph {
    files: Vec<File>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GraphPosition(pub u32);

impl Display for GraphPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
