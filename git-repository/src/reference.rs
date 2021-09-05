///
pub mod log {
    use bstr::{BString, ByteSlice, ByteVec};
    use git_object::Commit;

    use crate::commit;

    /// Generate a message typical for git commit logs based on the given `operation`
    pub fn message(operation: &str, commit: &Commit) -> BString {
        let mut out = BString::from(operation);
        if let Some(commit_type) = commit_type_by_parents(commit.parents.len()) {
            out.push_str(b" (");
            out.extend_from_slice(commit_type.as_bytes());
            out.push_byte(b')');
        }
        out.push_str(b": ");
        out.extend_from_slice(&commit::summary(commit.message.as_bstr()));
        out
    }

    pub(crate) fn commit_type_by_parents(count: usize) -> Option<&'static str> {
        Some(match count {
            0 => "initial",
            1 => return None,
            _two_or_more => "merge",
        })
    }
}
