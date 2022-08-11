use crate::{write, File};
use git_features::hash;

impl File {
    pub fn write_to(&self, mut out: &mut impl std::io::Write, options: write::Options) -> std::io::Result<()> {
        let mut hasher = hash::Write::new(&mut out, options.hash_kind);
        self.state.write_to(&mut hasher, options)?;

        let hash = hasher.hash.digest();
        out.write_all(&hash)
    }
}
