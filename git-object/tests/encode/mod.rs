macro_rules! round_trip {
    ($owned:ty, $borrowed:ty, $( $files:literal ), +) => {
        #[test]
        fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
            use crate::fixture_bytes;
            use git_object::{ObjectRef, Object};
            use bstr::ByteSlice;
            for input in &[
                $( $files ),*
            ] {
                use git_object::WriteTo;
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
            }
            Ok(())
        }
    };
}

mod tag {
    round_trip!(
        git_object::Tag,
        git_object::TagRef,
        "tag/empty.txt",
        "tag/no-tagger.txt",
        "tag/whitespace.txt",
        "tag/with-newlines.txt",
        "tag/signed.txt"
    );
}

mod commit {
    round_trip!(
        git_object::Commit,
        git_object::CommitRef,
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
    round_trip!(git_object::Tree, git_object::TreeRef, "tree/everything.tree");
}

mod blob {
    // It doesn't matter which data we use - it's not interpreted.
    round_trip!(git_object::Blob, git_object::BlobRef, "tree/everything.tree");
}
