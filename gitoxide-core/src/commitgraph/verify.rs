use anyhow::{Context as AnyhowContext, Result};
use std::{io, path::Path};

/// A general purpose context for many operations provided here
pub struct Context<W1: io::Write, W2: io::Write> {
    /// A stream to which to output operation results
    pub out: W1,
    /// A stream to which to errors
    pub err: W2,
}

impl Default for Context<Vec<u8>, Vec<u8>> {
    fn default() -> Self {
        Context {
            out: Vec::new(),
            err: Vec::new(),
        }
    }
}

pub fn graph_or_file<W1, W2>(
    path: impl AsRef<Path>,
    Context {
        out: mut _out,
        err: mut _err,
    }: Context<W1, W2>,
) -> Result<()>
where
    W1: io::Write,
    W2: io::Write,
{
    // TODO: Handle `path` being objects/info, objects/info/commit-graph,
    //   or objects/info/commit-graphs/graph-xyz.graph.
    let _g = git_commitgraph::Graph::from_info_dir(path).with_context(|| "Could not open commit graph")?;
    Err(anyhow::Error::msg("not implemented"))
}
