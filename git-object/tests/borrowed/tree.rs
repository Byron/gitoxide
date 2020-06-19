mod parse {
    use crate::{borrowed::fixture_bytes, hex_to_id};
    use bstr::ByteSlice;
    use git_object::borrowed::{Tree, TreeEntry as Entry, TreeMode as Mode};

    #[test]
    fn everything() {
        assert_eq!(
            Tree::from_bytes(&fixture_bytes("tree", "everything.tree")).unwrap(),
            Tree(vec![
                Entry {
                    mode: Mode::BlobExecutable,
                    filename: b"exe".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")[..]
                },
                Entry {
                    mode: Mode::Blob,
                    filename: b"file".as_bstr(),
                    oid: &hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")[..]
                },
                Entry {
                    mode: Mode::Commit,
                    filename: b"grit-submodule".as_bstr(),
                    oid: &hex_to_id("b2d1b5d684bdfda5f922b466cc13d4ce2d635cf8")[..]
                },
                Entry {
                    mode: Mode::Tree,
                    filename: b"subdir".as_bstr(),
                    oid: &hex_to_id("4d5fcadc293a348e88f777dc0920f11e7d71441c")[..]
                },
                Entry {
                    mode: Mode::Link,
                    filename: b"symlink".as_bstr(),
                    oid: &hex_to_id("1a010b1c0f081b2e8901d55307a15c29ff30af0e")[..]
                }
            ])
        );
    }
}
