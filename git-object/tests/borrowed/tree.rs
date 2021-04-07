mod from_bytes {
    use crate::borrowed::fixture_bytes;
    use git_object::{
        borrowed::{tree::Entry, Tree},
        bstr::ByteSlice,
        tree,
    };
    use hex::FromHex;

    pub fn hex_to_id(hex: &str) -> [u8; 20] {
        <[u8; 20]>::from_hex(hex).expect("40 bytes hex sha")
    }

    pub fn as_id(id: &[u8; 20]) -> &git_hash::oid {
        id.into()
    }

    #[test]
    fn everything() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Tree::from_bytes(&fixture_bytes("tree", "everything.tree"))?,
            Tree {
                entries: vec![
                    Entry {
                        mode: tree::Mode::BlobExecutable,
                        filename: b"exe".as_bstr(),
                        oid: as_id(&hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"))
                    },
                    Entry {
                        mode: tree::Mode::Blob,
                        filename: b"file".as_bstr(),
                        oid: as_id(&hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"))
                    },
                    Entry {
                        mode: tree::Mode::Commit,
                        filename: b"grit-submodule".as_bstr(),
                        oid: as_id(&hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8"))
                    },
                    Entry {
                        mode: tree::Mode::Tree,
                        filename: b"subdir".as_bstr(),
                        oid: as_id(&hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c"))
                    },
                    Entry {
                        mode: tree::Mode::Link,
                        filename: b"symlink".as_bstr(),
                        oid: as_id(&hex_to_id("1a010b1c0f081b2e8901d55307a15c29ff30af0e"))
                    }
                ]
            }
        );
        Ok(())
    }

    #[test]
    fn maybe_special() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Tree::from_bytes(&fixture_bytes("tree", "maybe-special.tree"))?
                .entries
                .len(),
            160
        );
        Ok(())
    }

    #[test]
    fn definitely_special() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Tree::from_bytes(&fixture_bytes("tree", "definitely-special.tree"))?
                .entries
                .len(),
            19
        );
        Ok(())
    }
}
