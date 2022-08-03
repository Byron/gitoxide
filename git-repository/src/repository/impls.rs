impl Clone for crate::Repository {
    fn clone(&self) -> Self {
        crate::Repository::from_refs_and_objects(
            self.refs.clone(),
            self.objects.clone(),
            self.work_tree.clone(),
            self.common_dir.clone(),
            self.config.clone(),
            self.options.clone(),
            self.index.clone(),
        )
    }
}

impl std::fmt::Debug for crate::Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Repository")
            .field("kind", &self.kind())
            .field("git_dir", &self.git_dir())
            .field("work_dir", &self.work_dir())
            .finish()
    }
}

impl PartialEq<crate::Repository> for crate::Repository {
    fn eq(&self, other: &crate::Repository) -> bool {
        self.git_dir().canonicalize().ok() == other.git_dir().canonicalize().ok()
            && self.work_tree.as_deref().and_then(|wt| wt.canonicalize().ok())
                == other.work_tree.as_deref().and_then(|wt| wt.canonicalize().ok())
    }
}

impl From<&crate::ThreadSafeRepository> for crate::Repository {
    fn from(repo: &crate::ThreadSafeRepository) -> Self {
        crate::Repository::from_refs_and_objects(
            repo.refs.clone(),
            repo.objects.to_handle().into(),
            repo.work_tree.clone(),
            repo.common_dir.clone(),
            repo.config.clone(),
            repo.linked_worktree_options.clone(),
            repo.index.clone(),
        )
    }
}

impl From<crate::ThreadSafeRepository> for crate::Repository {
    fn from(repo: crate::ThreadSafeRepository) -> Self {
        crate::Repository::from_refs_and_objects(
            repo.refs,
            repo.objects.to_handle().into(),
            repo.work_tree,
            repo.common_dir,
            repo.config,
            repo.linked_worktree_options,
            repo.index,
        )
    }
}

impl From<crate::Repository> for crate::ThreadSafeRepository {
    fn from(r: crate::Repository) -> Self {
        crate::ThreadSafeRepository {
            refs: r.refs,
            objects: r.objects.into_inner().store(),
            work_tree: r.work_tree,
            common_dir: r.common_dir,
            config: r.config,
            linked_worktree_options: r.options,
            index: r.index,
        }
    }
}
