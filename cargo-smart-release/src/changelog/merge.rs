use crate::ChangeLog;

impl ChangeLog {
    /// Bring `generated` into `self` in such a way that `self` preserves everything while enriching itself from `generated`.
    /// Thus we clearly assume that `self` is parsed and `generated` is generated.
    pub fn merge_generated(self, _lhs: Self) -> Self {
        self
    }
}
