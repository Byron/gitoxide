/// Because the TryFrom implementations don't return proper errors
/// on failure
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("")]
    TryFromError,
}

macro_rules! round_trip {
    ($owned:ty, $borrowed:ty, $( $files:literal ), +) => {
        #[test]
        fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
            use std::convert::TryFrom;
            use std::io::Write;
            use crate::fixture_bytes;
            use gix_object::{ObjectRef, Object, WriteTo};
            use bstr::ByteSlice;

            for input in &[
                $( $files ),*
            ] {
                let input = fixture_bytes(input);
                // Test the parse->borrowed->owned->write chain for an object kind
                let mut output = Vec::new();
                let item = <$borrowed>::from_bytes(&input)?;
                item.write_to(&mut output)?;
                assert_eq!(output.as_bstr(), input.as_bstr());

                let item: $owned = item.into();
                output.clear();
                item.write_to(&mut output)?;
                assert_eq!(output.as_bstr(), input.as_bstr());

                // Test the parse->borrowed->owned->write chain for the top-level objects
                let item = ObjectRef::from(<$borrowed>::from_bytes(&input)?);
                output.clear();
                item.write_to(&mut output)?;
                assert_eq!(output.as_bstr(), input.as_bstr());

                let item: Object = item.into();
                output.clear();
                item.write_to(&mut output)?;
                assert_eq!(output.as_bstr(), input.as_bstr());

                // Test the loose serialisation -> parse chain for an object kind
                let item = <$borrowed>::from_bytes(&input)?;
                output.clear();
                // serialise to a tagged loose object
                let w = &mut output;
                w.write_all(&item.loose_header())?;
                item.write_to(w)?;
                let parsed = ObjectRef::from_loose(&output)?;
                let item2 = <$borrowed>::try_from(parsed).or(Err(super::Error::TryFromError))?;
                assert_eq!(item2, item);
            }
            Ok(())
        }
    };
}

mod tag {
    round_trip!(
        gix_object::Tag,
        gix_object::TagRef,
        "tag/empty.txt",
        "tag/no-tagger.txt",
        "tag/whitespace.txt",
        "tag/with-newlines.txt",
        "tag/signed.txt"
    );
}

mod commit {
    round_trip!(
        gix_object::Commit,
        gix_object::CommitRef,
        "commit/signed-whitespace.txt",
        "commit/two-multiline-headers.txt",
        "commit/mergetag.txt",
        "commit/merge.txt",
        "commit/signed.txt",
        "commit/signed-singleline.txt",
        "commit/signed-with-encoding.txt",
        "commit/unsigned.txt",
        "commit/whitespace.txt",
        "commit/with-encoding.txt"
    );
}

mod tree {
    round_trip!(gix_object::Tree, gix_object::TreeRef, "tree/everything.tree");
}

mod blob {
    // It doesn't matter which data we use - it's not interpreted.
    round_trip!(gix_object::Blob, gix_object::BlobRef, "tree/everything.tree");
}
