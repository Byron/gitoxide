macro_rules! round_trip {
    ($owned:ty, $borrowed:ty, $( $files:literal ), +) => {
        #[test]
        fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
            use crate::fixture_bytes;
            use git_object::{mutable, immutable};
            use bstr::ByteSlice;
            for input in &[
                $( $files ),*
            ] {
                let input = fixture_bytes(input);
                // Test the parse->borrowed->owned->write chain for an object kind
                let item: $owned = <$borrowed>::from_bytes(&input)?.into();
                let mut output = Vec::new();
                item.write_to(&mut output)?;
                assert_eq!(output.as_bstr(), input.as_bstr());

                // Test the parse->borrowed->owned->write chain for the top-level objects
                let item: mutable::Object = immutable::Object::from(<$borrowed>::from_bytes(&input)?).into();
                output.clear();
                item.write_to(&mut output)?;
                assert_eq!(output.as_bstr(), input.as_bstr());
            }
            Ok(())
        }
    };
}

mod object;
mod tag {
    round_trip!(
        mutable::Tag,
        immutable::Tag,
        "tag/empty.txt",
        "tag/no-tagger.txt",
        "tag/whitespace.txt",
        "tag/with-newlines.txt",
        "tag/signed.txt"
    );
}

mod commit {
    round_trip!(
        mutable::Commit,
        immutable::Commit,
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
    round_trip!(mutable::Tree, immutable::Tree, "tree/everything.tree");
}

mod blob {
    // It doesn't matter which data we use - it's not interpreted.
    round_trip!(mutable::Blob, immutable::Blob, "tree/everything.tree");
}
