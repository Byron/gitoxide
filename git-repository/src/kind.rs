use crate::Kind;

impl Kind {
    /// Returns true if this is a bare repository, one without a work tree.
    pub fn is_bare(&self) -> bool {
        matches!(self, Kind::Bare)
    }
}

impl From<git_discover::repository::Kind> for Kind {
    fn from(v: git_discover::repository::Kind) -> Self {
        match v {
            git_discover::repository::Kind::Submodule { .. } | git_discover::repository::Kind::SubmoduleGitDir => {
                Kind::WorkTree { is_linked: false }
            }
            git_discover::repository::Kind::Bare => Kind::Bare,
            git_discover::repository::Kind::WorkTreeGitDir { .. } => Kind::WorkTree { is_linked: true },
            git_discover::repository::Kind::WorkTree { linked_git_dir } => Kind::WorkTree {
                is_linked: linked_git_dir.is_some(),
            },
        }
    }
}
