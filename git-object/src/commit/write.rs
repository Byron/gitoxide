use bstr::ByteSlice;
use std::io;

use crate::{encode, encode::NL, Commit, CommitRef, Kind};

impl crate::WriteTo for Commit {
    /// Serializes this instance to `out` in the git serialization format.
    fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        encode::trusted_header_id(b"tree", &self.tree, &mut out)?;
        for parent in &self.parents {
            encode::trusted_header_id(b"parent", parent, &mut out)?;
        }
        encode::trusted_header_signature(b"author", &self.author.to_ref(), &mut out)?;
        encode::trusted_header_signature(b"committer", &self.committer.to_ref(), &mut out)?;
        if let Some(encoding) = self.encoding.as_ref() {
            encode::header_field(b"encoding", encoding, &mut out)?;
        }
        for (name, value) in &self.extra_headers {
            encode::header_field_multi_line(name, value, &mut out)?;
        }
        out.write_all(NL)?;
        out.write_all(&self.message)
    }

    fn size(&self) -> usize {
        let hashsize = self.tree.kind().len_in_hex();
        b"tree".len()
            + 1
            + hashsize
            + 1
            + self.parents.iter().count() * (b"parent".len() + 1 + hashsize + 1)
            + b"author".len()
            + 1
            + self.author.size()
            + 1
            + b"committer".len()
            + 1
            + self.committer.size()
            + 1
            + self
                .encoding
                .as_ref()
                .map(|e| b"encoding".len() + 1 + e.len() + 1)
                .unwrap_or(0)
            + self
                .extra_headers
                .iter()
                .map(|(name, value)| {
                    // each header *value* is preceded by a space and followed by a newline
                    name.len() + value.split_str("\n").map(|s| s.len() + 2).sum::<usize>()
                })
                .sum::<usize>()
            + 1
            + self.message.len()
    }

    fn kind(&self) -> Kind {
        Kind::Commit
    }
}

impl<'a> crate::WriteTo for CommitRef<'a> {
    /// Serializes this instance to `out` in the git serialization format.
    fn write_to(&self, mut out: impl io::Write) -> io::Result<()> {
        encode::trusted_header_id(b"tree", &self.tree(), &mut out)?;
        for parent in self.parents() {
            encode::trusted_header_id(b"parent", &parent, &mut out)?;
        }
        encode::trusted_header_signature(b"author", &self.author, &mut out)?;
        encode::trusted_header_signature(b"committer", &self.committer, &mut out)?;
        if let Some(encoding) = self.encoding.as_ref() {
            encode::header_field(b"encoding", encoding, &mut out)?;
        }
        for (name, value) in &self.extra_headers {
            encode::header_field_multi_line(name, value, &mut out)?;
        }
        out.write_all(NL)?;
        out.write_all(self.message)
    }

    fn size(&self) -> usize {
        let hashsize = self.tree().kind().len_in_hex();
        b"tree".len()
            + 1
            + hashsize
            + 1
            + self.parents.iter().count() * (b"parent".len() + 1 + hashsize + 1)
            + b"author".len()
            + 1
            + self.author.size()
            + 1
            + b"committer".len()
            + 1
            + self.committer.size()
            + 1
            + self
                .encoding
                .as_ref()
                .map(|e| b"encoding".len() + 1 + e.len() + 1)
                .unwrap_or(0)
            + self
                .extra_headers
                .iter()
                .map(|(name, value)| {
                    // each header *value* is preceded by a space and followed by a newline
                    name.len() + value.split_str("\n").map(|s| s.len() + 2).sum::<usize>()
                })
                .sum::<usize>()
            + 1
            + self.message.len()
    }

    fn kind(&self) -> Kind {
        Kind::Commit
    }
}
