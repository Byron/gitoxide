use git_object::owned;

#[derive(Clone)]
pub enum ObjectKind {
    Base(git_object::Kind),
    OfsDelta,
}

impl ObjectKind {
    pub fn to_kind(&self) -> Option<git_object::Kind> {
        match self {
            ObjectKind::Base(kind) => Some(*kind),
            ObjectKind::OfsDelta => None,
        }
    }
}

pub struct TreeEntry {
    pub id: owned::Id,
    pub kind: ObjectKind,
    pub crc32: u32,
}

impl Default for TreeEntry {
    fn default() -> Self {
        TreeEntry {
            id: owned::Id::null(),
            kind: ObjectKind::OfsDelta,
            crc32: 0,
        }
    }
}
