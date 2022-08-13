use crate::write::util::CountBytes;
use crate::{extension, State, Version};
use std::convert::TryInto;
use std::io::Write;

/// The options for use when [writing an index][State::write_to()].
///
/// Note that default options write
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Options {
    /// The hash kind to use when writing the index file.
    ///
    /// It is not always possible to infer the hash kind when reading an index, so this is required.
    pub hash_kind: git_hash::Kind,
    /// The index version to write. Note that different versions affect the format and ultimately the size.
    pub version: Version,

    /// If true, write the tree-cache extension, if present.
    // TODO: should we not write all we have by default to be lossless, but provide options to those who seek them?
    pub tree_cache_extension: bool,
    /// If true, write the end-of-index-entry extension.
    // TODO: figure out if this is implied by other options, for instance multi-threading.
    pub end_of_index_entry_extension: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            hash_kind: git_hash::Kind::default(),
            /// TODO: make this 'automatic' by default to determine the correct index version - not all versions can represent all in-memory states.
            version: Version::V2,
            tree_cache_extension: true,
            end_of_index_entry_extension: true,
        }
    }
}

impl State {
    /// Serialize this instance to `out` with [`options`][Options].
    pub fn write_to(
        &self,
        out: &mut impl std::io::Write,
        Options {
            hash_kind,
            version,
            tree_cache_extension,
            end_of_index_entry_extension,
        }: Options,
    ) -> std::io::Result<()> {
        assert_eq!(
            version,
            Version::V2,
            "can only write V2 at the moment, please come back later"
        );

        let mut write = CountBytes::new(out);
        let num_entries = self
            .entries()
            .len()
            .try_into()
            .expect("definitely not 4billion entries");

        let offset_to_entries = header(&mut write, version, num_entries)?;
        let offset_to_extensions = entries(&mut write, self, offset_to_entries)?;

        let extensions = {
            type WriteExtFn<'a> = &'a dyn Fn(&mut dyn std::io::Write) -> Option<std::io::Result<extension::Signature>>;
            let extensions: &[WriteExtFn<'_>] = &[&|write| {
                tree_cache_extension
                    .then(|| self.tree())
                    .flatten()
                    .map(|tree| tree.write_to(write).map(|_| extension::tree::SIGNATURE))
            }];

            let mut offset_to_previous_ext = offset_to_extensions;
            let mut out = Vec::with_capacity(5);
            for write_ext in extensions {
                if let Some(signature) = write_ext(&mut write).transpose()? {
                    let offset_past_ext = write.count;
                    let ext_size = offset_past_ext - offset_to_previous_ext - (extension::MIN_SIZE as u32);
                    offset_to_previous_ext = offset_past_ext;
                    out.push((signature, ext_size));
                }
            }
            out
        };

        if num_entries > 0 && end_of_index_entry_extension && !extensions.is_empty() {
            extension::end_of_index_entry::write_to(write.inner, hash_kind, offset_to_extensions, &extensions)?
        }

        Ok(())
    }
}

fn header<T: std::io::Write>(
    out: &mut CountBytes<'_, T>,
    version: Version,
    num_entries: u32,
) -> Result<u32, std::io::Error> {
    let signature = b"DIRC";

    let version = match version {
        Version::V2 => 2_u32.to_be_bytes(),
        Version::V3 => 3_u32.to_be_bytes(),
        Version::V4 => 4_u32.to_be_bytes(),
    };

    out.write_all(signature)?;
    out.write_all(&version)?;
    out.write_all(&num_entries.to_be_bytes())?;

    Ok(out.count)
}

fn entries<T: std::io::Write>(
    out: &mut CountBytes<'_, T>,
    state: &State,
    header_size: u32,
) -> Result<u32, std::io::Error> {
    for entry in state.entries() {
        entry.write_to(&mut *out, state)?;
        match (out.count - header_size) % 8 {
            0 => {}
            n => {
                let eight_null_bytes = [0u8; 8];
                out.write_all(&eight_null_bytes[n as usize..])?;
            }
        };
    }

    Ok(out.count)
}

mod util {
    use std::convert::TryFrom;

    pub struct CountBytes<'a, T> {
        pub count: u32,
        pub inner: &'a mut T,
    }

    impl<'a, T> CountBytes<'a, T>
    where
        T: std::io::Write,
    {
        pub fn new(inner: &'a mut T) -> Self {
            CountBytes { inner, count: 0 }
        }
    }

    impl<'a, T> std::io::Write for CountBytes<'a, T>
    where
        T: std::io::Write,
    {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let written = self.inner.write(buf)?;
            self.count = self
                .count
                .checked_add(u32::try_from(written).expect("we don't write 4GB buffers"))
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Cannot write indices larger than 4 gigabytes",
                    )
                })?;
            Ok(written)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.inner.flush()
        }
    }
}
