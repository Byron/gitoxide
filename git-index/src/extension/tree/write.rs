use crate::extension::{tree, Tree};
use std::convert::TryFrom;

impl Tree {
    /// Serialize this instance to `out`.
    pub fn write_to(&self, mut out: impl std::io::Write) -> Result<(), std::io::Error> {
        fn tree_entry(out: &mut impl std::io::Write, tree: &Tree) -> Result<(), std::io::Error> {
            let num_entries_ascii = tree.num_entries.to_string();
            let num_children_ascii = tree.children.len().to_string();

            out.write_all(tree.name.as_slice())?;
            out.write_all(b"\0")?;
            out.write_all(num_entries_ascii.as_bytes())?;
            out.write_all(b" ")?;
            out.write_all(num_children_ascii.as_bytes())?;
            out.write_all(b"\n")?;
            out.write_all(tree.id.as_bytes())?;

            for child in &tree.children {
                tree_entry(out, child)?;
            }

            Ok(())
        }

        let signature = tree::SIGNATURE;

        let estimated_size = self.num_entries * (300 + 3 + 1 + 3 + 1 + 20);
        let mut entries: Vec<u8> = Vec::with_capacity(estimated_size as usize);
        tree_entry(&mut entries, self)?;

        out.write_all(&signature)?;
        out.write_all(&(u32::try_from(entries.len()).expect("less than 4GB tree extension")).to_be_bytes())?;
        out.write_all(&entries)?;

        Ok(())
    }
}
