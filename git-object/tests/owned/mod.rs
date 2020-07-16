macro_rules! round_trip {
    ($owned:ty, $borrowed:ty, $( $files:literal ), +) => {
        // Git checks out text files with different line feeds, which causes parsing failure.
        // No way to configure this in the checkout action :/
        #[cfg_attr(windows, ignore)]
        #[test]
        fn round_trip() {
            use crate::fixture_bytes;
            use bstr::ByteSlice;
            for input in &[
                $( $files ),*
            ] {
                let input = fixture_bytes(input);
                let item: $owned = <$borrowed>::from_bytes(&input).unwrap().into();
                let mut output = Vec::new();
                item.write_to(&mut output).unwrap();
                assert_eq!(output.as_bstr(), input.as_bstr());
            }
        }
    };
}

mod object;
mod tag {
    use git_object::{borrowed, owned};

    round_trip!(
        owned::Tag,
        borrowed::Tag,
        "tag/empty.txt",
        "tag/whitespace.txt",
        "tag/with-newlines.txt",
        "tag/signed.txt"
    );
}

mod commit {
    use git_object::{borrowed, owned};

    round_trip!(
        owned::Commit,
        borrowed::Commit,
        "commit/merge.txt",
        "commit/signed.txt",
        "commit/signed-singleline.txt",
        "commit/unsigned.txt",
        "commit/whitespace.txt",
        "commit/with-encoding.txt"
    );
}
