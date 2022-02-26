impl Clone for crate::Repository {
    fn clone(&self) -> Self {
        crate::Repository::from_refs_and_objects(
            self.refs.clone(),
            self.objects.clone(),
            self.object_hash,
            self.work_tree.clone(),
            self.config.clone(),
        )
    }
}

impl std::fmt::Debug for crate::Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Repository(git = '{}', working_tree: {:?}",
            self.git_dir().display(),
            self.work_tree
        )
    }
}

impl PartialEq<crate::Repository> for crate::Repository {
    fn eq(&self, other: &crate::Repository) -> bool {
        self.git_dir() == other.git_dir() && self.work_tree == other.work_tree
    }
}

impl From<&crate::ThreadSafeRepository> for crate::Repository {
    fn from(repo: &crate::ThreadSafeRepository) -> Self {
        crate::Repository::from_refs_and_objects(
            repo.refs.clone(),
            repo.objects.to_handle().into(),
            repo.object_hash,
            repo.work_tree.clone(),
            repo.config.clone(),
        )
    }
}

impl From<crate::ThreadSafeRepository> for crate::Repository {
    fn from(repo: crate::ThreadSafeRepository) -> Self {
        crate::Repository::from_refs_and_objects(
            repo.refs,
            repo.objects.to_handle().into(),
            repo.object_hash,
            repo.work_tree,
            repo.config,
        )
    }
}

impl From<crate::Repository> for crate::ThreadSafeRepository {
    fn from(r: crate::Repository) -> Self {
        crate::ThreadSafeRepository {
            refs: r.refs,
            objects: r.objects.into_inner().store(),
            work_tree: r.work_tree,
            object_hash: r.object_hash,
            config: r.config,
        }
    }
}
