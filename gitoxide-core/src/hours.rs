use git_features::progress::Progress;
use std::{ffi::OsStr, io, path::Path};

/// Additional configuration for the hours estimation functionality.
pub struct Context<W> {
    /// Show personally identifiable information before the summary. Includes names and email addresses.
    pub show_pii: bool,
    /// Omit unifying identities by name and email which can lead to the same author appear multiple times
    /// due to using different names or email addresses.
    pub omit_unify_identities: bool,
    /// Where to write our output to
    pub out: W,
}

/// * _working_dir_ - The directory containing a '.git/' folder.
/// * _refname_ - The name of the ref like 'main' or 'master' at which to start iterating the commit graph.
pub fn estimate<W, P>(
    working_dir: &Path,
    refname: &OsStr,
    progress: P,
    Context {
        show_pii,
        omit_unify_identities,
        out: output,
    }: Context<W>,
) -> anyhow::Result<()>
where
    W: io::Write,
    P: Progress,
{
    Ok(())
}
