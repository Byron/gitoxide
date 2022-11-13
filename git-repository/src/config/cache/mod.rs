use super::{Cache, Error};

mod incubate;
pub(crate) use incubate::StageOne;

mod init;

impl std::fmt::Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cache").finish_non_exhaustive()
    }
}

mod access;

pub(crate) mod util;

pub(crate) use util::interpolate_context;
