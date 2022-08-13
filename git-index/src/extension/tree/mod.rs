use crate::extension::Signature;

/// The signature for tree extensions
pub const SIGNATURE: Signature = *b"TREE";

///
pub mod verify;

mod decode;
pub use decode::decode;

mod write;

#[cfg(test)]
mod tests {

    #[test]
    fn size_of_tree() {
        assert_eq!(std::mem::size_of::<crate::extension::Tree>(), 88);
    }
}
