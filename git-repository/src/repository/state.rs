use crate::RepositoryState;

impl crate::Repository {
    /// Returns the status of an in-progress operation on a repository or [`RepositoryState::None`]
    /// if nothing is happening.
    pub fn in_progress_state(&self) -> RepositoryState {
        let repo_path = self.path();

        // This is modeled on the logic from wt_status_get_state in git's wt-status.c

        if repo_path.join("rebase-apply/applying").is_file() {
            return RepositoryState::ApplyMailbox;
        } else if repo_path.join("rebase-apply").is_dir() {
            // Should this be a separate RebaseApplyMailbox or ApplyMailboxMerge?
            return RepositoryState::Rebase;
        } else if repo_path.join("rebase-merge/interactive").is_file() {
            return RepositoryState::RebaseInteractive;
        } else if repo_path.join("rebase-merge").is_dir() {
            // Should this be RebaseMerge?
            return RepositoryState::Rebase;
        } else if repo_path.join("CHERRY_PICK_HEAD").is_file() {
            if repo_path.join("todo").is_file() {
                return RepositoryState::CherryPickSequence;
            } else {
                return RepositoryState::CherryPick;
            }
        } else if repo_path.join("MERGE_HEAD").is_file() {
            return RepositoryState::Merge;
        } else if repo_path.join("BISECT_LOG").is_file() {
            return RepositoryState::Bisect;
        } else if repo_path.join("REVERT_HEAD").is_file() {
            if repo_path.join("todo").is_file() {
                return RepositoryState::RevertSequence;
            } else {
                return RepositoryState::Revert;
            }
        } else {
            return RepositoryState::None;
        }
    }
}
