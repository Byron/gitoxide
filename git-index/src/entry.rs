use bitflags::bitflags;

bitflags! {
    pub struct Mode: u32 {
        const IFDIR = 0o040000;
    }
}

pub(crate) mod mode {
    impl super::Mode {
        pub fn is_sparse(&self) -> bool {
            *self == Self::IFDIR
        }
    }
}

pub(crate) mod flags {
    pub const EXTENDED: u32 = 0x4000;
    pub const INTENT_TO_ADD: u32 = 1 << 29;
    pub const SKIP_WORKTREE: u32 = 1 << 30;
}

pub(crate) mod mask {
    pub const PATH_LEN: u32 = 0x0fff;
}

pub struct Time {
    pub secs: u32,
    pub nsecs: u32,
}

pub struct Stat {
    pub mtime: Time,
    pub ctime: Time,
    pub dev: u32,
    pub ino: u32,
    pub uid: u32,
    pub gid: u32,
    /// The size of bytes on disk. Capped to u32 so files bigger than that will need thorough checking (and hopefully never make it)
    pub size: u32,
}

mod access {
    use bstr::{BStr, ByteSlice};

    use crate::{Entry, State};

    impl Entry {
        pub fn path<'a>(&self, state: &'a State) -> &'a BStr {
            (&state.path_backing[self.path.clone()]).as_bstr()
        }
    }
}
