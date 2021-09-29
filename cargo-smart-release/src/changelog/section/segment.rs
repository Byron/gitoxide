use bitflags::bitflags;

bitflags! {
    pub struct Selection: u32 {
        const CLIPPY = 1<<0;
        const COMMIT_DETAILS = 1<<1;
        const COMMIT_STATISTICS = 1<<2;
    }
}
