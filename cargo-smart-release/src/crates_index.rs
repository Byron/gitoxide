pub struct Index {
    inner: Option<crates_index::GitIndex>,
}

impl Index {
    /// Like the original one, but doesn't create it if it doesn't exist
    pub fn new_cargo_default() -> Result<Index, crates_index::Error> {
        Ok(Index {
            inner: crates_index::GitIndex::try_new_cargo_default()?,
        })
    }

    pub fn exists(&self) -> bool {
        self.inner.is_some()
    }

    pub fn update(&mut self) -> Result<(), crates_index::Error> {
        self.inner.as_mut().expect("BUG: call only after exists check").update()
    }

    pub fn crate_(&self, name: &str) -> Option<crates_index::Crate> {
        self.inner.as_ref().and_then(|idx| idx.crate_(name))
    }
}
