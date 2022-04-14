use crate::RepositoryState;

impl crate::Repository {
    /// Returns the status of an in progress operation on a repository or [`None`]
    /// if nothing is happening.
    pub fn in_progress_operation(&self) -> Option<RepositoryState> {
        let repo_path = self.path();

        // This is modeled on the logic from wt_status_get_state in git's wt-status.c and
        // ps1 from git-prompt.sh.

        if repo_path.join("rebase-apply/applying").is_file() {
            Some(RepositoryState::ApplyMailbox)
        } else if repo_path.join("rebase-apply/rebasing").is_file() {
            Some(RepositoryState::Rebase)
        } else if repo_path.join("rebase-apply").is_dir() {
            Some(RepositoryState::ApplyMailboxRebase)
        } else if repo_path.join("rebase-merge/interactive").is_file() {
            Some(RepositoryState::RebaseInteractive)
        } else if repo_path.join("rebase-merge").is_dir() {
            Some(RepositoryState::Rebase)
        } else if repo_path.join("CHERRY_PICK_HEAD").is_file() {
            if repo_path.join("todo").is_file() {
                Some(RepositoryState::CherryPickSequence)
            } else {
                Some(RepositoryState::CherryPick)
            }
        } else if repo_path.join("MERGE_HEAD").is_file() {
            Some(RepositoryState::Merge)
        } else if repo_path.join("BISECT_LOG").is_file() {
            Some(RepositoryState::Bisect)
        } else if repo_path.join("REVERT_HEAD").is_file() {
            if repo_path.join("todo").is_file() {
                Some(RepositoryState::RevertSequence)
            } else {
                Some(RepositoryState::Revert)
            }
        } else {
            None
        }
    }
}
