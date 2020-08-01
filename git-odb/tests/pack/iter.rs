use git_odb::pack;

#[test]
fn size_of_entry() {
    assert_eq!(
        std::mem::size_of::<pack::data::iter::Entry>(),
        104,
        "let's keep the size in check as we have many of them"
    );
}

mod new_from_header {
    use crate::{fixture_path, pack::SMALL_PACK, pack::V2_PACKS_AND_INDICES};
    use git_odb::{pack, pack::data::iter::Mode};
    use std::fs;

    #[test]
    fn header_encode() -> Result<(), Box<dyn std::error::Error>> {
        for (_, data_file) in V2_PACKS_AND_INDICES {
            let data = fs::read(fixture_path(data_file))?;
            for entry in pack::data::Iter::new_from_header(std::io::BufReader::new(data.as_slice()), Mode::AsIs)? {
                let entry = entry?;

                let mut buf = Vec::<u8>::new();
                entry.header.to_write(entry.decompressed.len() as u64, &mut buf)?;
                let pack::data::Entry {
                    header: new_header,
                    decompressed_size,
                    header_size,
                    ..
                } = pack::data::Header::from_bytes(&buf, entry.pack_offset);

                assert_eq!(header_size, buf.len() as u8, "it should consume all provided bytes");
                assert_eq!(
                    decompressed_size,
                    entry.decompressed.len() as u64,
                    "decoded size must match"
                );
                assert_eq!(new_header, entry.header, "headers match after roundtrip");
            }
        }
        Ok(())
    }

    #[test]
    fn generic_iteration() -> Result<(), Box<dyn std::error::Error>> {
        for trailer_mode in &[Mode::AsIs, Mode::Verify, Mode::Restore] {
            let mut iter = pack::data::Iter::new_from_header(
                std::io::BufReader::new(fs::File::open(fixture_path(SMALL_PACK))?),
                *trailer_mode,
            )?;

            let num_objects = iter.len();
            assert_eq!(iter.kind(), pack::data::Kind::V2);
            assert_eq!(num_objects, 42);
            assert_eq!(iter.by_ref().take(42 - 1).count(), num_objects - 1);
            assert_eq!(iter.len(), 1);
            assert_eq!(
                iter.next().expect("last object")?.trailer.expect("trailer id"),
                pack::data::File::at(fixture_path(SMALL_PACK))?.checksum(),
                "last object contains the trailer - a hash over all bytes in the pack"
            );
            assert_eq!(iter.len(), 0);
        }
        Ok(())
    }

    #[test]
    fn restore_missing_trailer() -> Result<(), Box<dyn std::error::Error>> {
        let pack = fs::read(fixture_path(SMALL_PACK))?;
        let mut iter =
            pack::data::Iter::new_from_header(std::io::BufReader::new(&pack[..pack.len() - 20]), Mode::Restore)?;
        let num_objects = iter.len();
        assert_eq!(iter.by_ref().take(42 - 1).count(), num_objects - 1);
        assert_eq!(
            iter.next().expect("last object")?.trailer.expect("trailer id"),
            pack::data::File::at(fixture_path(SMALL_PACK))?.checksum(),
            "the correct checksum should be restored"
        );
        Ok(())
    }

    #[test]
    fn restore_partial_pack() -> Result<(), Box<dyn std::error::Error>> {
        let pack = fs::read(fixture_path(SMALL_PACK))?;
        let mut iter =
            pack::data::Iter::new_from_header(std::io::BufReader::new(&pack[..pack.len() / 2]), Mode::Restore)?;
        let mut num_objects = 0;
        while let Some(entry) = iter.next() {
            let entry = entry?;
            num_objects += 1;
            assert!(
                entry.trailer.is_some(),
                "every entry has a trailer as we don't know when an object will fail - thus we never fail"
            );
        }
        assert_eq!(num_objects, 12);
        assert_eq!(
            iter.len(),
            0,
            "it will never return any more objects (right now), so nothing is left"
        );
        Ok(())
    }
}
