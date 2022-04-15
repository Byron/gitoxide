use crate::RepositoryState;

impl crate::Repository {
    /// Returns the status of an in progress operation on a repository or [`None`]
    /// if nothing is happening.
    pub fn in_progress_operation(&self) -> Option<RepositoryState> {
        let git_dir = self.path();

        // This is modeled on the logic from wt_status_get_state in git's wt-status.c and
        // ps1 from git-prompt.sh.

        if git_dir.join("rebase-apply/applying").is_file() {
            Some(RepositoryState::ApplyMailbox)
        } else if git_dir.join("rebase-apply/rebasing").is_file() {
            Some(RepositoryState::Rebase)
        } else if git_dir.join("rebase-apply").is_dir() {
            Some(RepositoryState::ApplyMailboxRebase)
        } else if git_dir.join("rebase-merge/interactive").is_file() {
            Some(RepositoryState::RebaseInteractive)
        } else if git_dir.join("rebase-merge").is_dir() {
            Some(RepositoryState::Rebase)
        } else if git_dir.join("CHERRY_PICK_HEAD").is_file() {
            if git_dir.join("todo").is_file() {
                Some(RepositoryState::CherryPickSequence)
            } else {
                Some(RepositoryState::CherryPick)
            }
        } else if git_dir.join("MERGE_HEAD").is_file() {
            Some(RepositoryState::Merge)
        } else if git_dir.join("BISECT_LOG").is_file() {
            Some(RepositoryState::Bisect)
        } else if git_dir.join("REVERT_HEAD").is_file() {
            if git_dir.join("todo").is_file() {
                Some(RepositoryState::RevertSequence)
            } else {
                Some(RepositoryState::Revert)
            }
        } else {
            None
        }
    }
}
