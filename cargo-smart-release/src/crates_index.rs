use std::path::{Path, PathBuf};

pub struct Index {
    inner: Option<crates_index::Index>,
}

impl Index {
    /// Like the original one, but doesn't create it if it doesn't exist
    pub fn new_cargo_default() -> Result<Index, crates_index::Error> {
        let path = default_path();
        Ok(Index {
            inner: if path.is_dir() {
                crates_index::Index::new_cargo_default()?.into()
            } else {
                None
            },
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

fn default_path() -> PathBuf {
    home::cargo_home()
        .ok()
        .or_else(|| {
            std::env::var_os("CARGO_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|dir| Path::new(&dir).join(".cargo")))
        })
        .expect("one of these paths works")
        .join("registry/index")
        .join("github.com-1ecc6299db9ec823")
}
