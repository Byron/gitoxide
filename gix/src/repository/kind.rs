use crate::repository::Kind;

impl Kind {
    /// Returns true if this is a bare repository, one without a work tree.
    pub fn is_bare(&self) -> bool {
        matches!(self, Kind::Bare)
    }
}

impl From<gix_discover::repository::Kind> for Kind {
    fn from(v: gix_discover::repository::Kind) -> Self {
        match v {
            gix_discover::repository::Kind::Submodule { .. } | gix_discover::repository::Kind::SubmoduleGitDir => {
                Kind::WorkTree { is_linked: false }
            }
            gix_discover::repository::Kind::PossiblyBare => Kind::Bare,
            gix_discover::repository::Kind::WorkTreeGitDir { .. } => Kind::WorkTree { is_linked: true },
            gix_discover::repository::Kind::WorkTree { linked_git_dir } => Kind::WorkTree {
                is_linked: linked_git_dir.is_some(),
            },
        }
    }
}
