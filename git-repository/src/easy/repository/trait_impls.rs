use crate::{easy, sync};

impl Clone for easy::Repository {
    fn clone(&self) -> Self {
        easy::Repository::from_refs_and_objects(
            self.refs.clone(),
            self.objects.clone(),
            self.object_hash,
            self.work_tree.clone(),
            self.config.clone(),
        )
    }
}

impl std::fmt::Debug for easy::Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Repository(git = '{}', working_tree: {:?}",
            self.git_dir().display(),
            self.work_tree
        )
    }
}

impl PartialEq<easy::Repository> for easy::Repository {
    fn eq(&self, other: &easy::Repository) -> bool {
        self.git_dir() == other.git_dir() && self.work_tree == other.work_tree
    }
}

impl From<&sync::Handle> for easy::Repository {
    fn from(repo: &sync::Handle) -> Self {
        easy::Repository::from_refs_and_objects(
            repo.refs.clone(),
            repo.objects.to_handle().into(),
            repo.object_hash,
            repo.work_tree.clone(),
            repo.config.clone(),
        )
    }
}

impl From<sync::Handle> for easy::Repository {
    fn from(repo: sync::Handle) -> Self {
        easy::Repository::from_refs_and_objects(
            repo.refs,
            repo.objects.to_handle().into(),
            repo.object_hash,
            repo.work_tree,
            repo.config,
        )
    }
}

impl From<easy::Repository> for sync::Handle {
    fn from(r: easy::Repository) -> Self {
        sync::Handle {
            refs: r.refs,
            objects: r.objects.into_inner().store(),
            work_tree: r.work_tree,
            object_hash: r.object_hash,
            config: r.config,
        }
    }
}
