use gix_object::{bstr::ByteSlice, CommitRef};

use crate::{
    commit::{OTHER_SIGNATURE, SIGNATURE},
    fixture_name,
};

fn extract_signed_helper(
    fixture: &str,
    signature: &[u8],
    signature_lines: std::ops::RangeInclusive<usize>,
) -> crate::Result {
    let fixture_data = fixture_name("commit", fixture);
    let mut signed_data = Vec::new();
    {
        let sig = CommitRef::extract_signature(&fixture_data, &mut signed_data)?.unwrap();
        assert_eq!(sig.to_str(), signature.to_str());
    }

    let ends_with_newline = fixture_data[fixture_data.len() - 1] == b'\n';
    let mut expected = Vec::new();
    for (i, line) in String::from_utf8(fixture_data).unwrap().lines().enumerate() {
        if signature_lines.contains(&i) {
            if *signature_lines.end() == i {
                expected.push(b'\n');
            }
            continue;
        }
        expected.extend_from_slice(line.as_bytes());
        expected.push(b'\n');
    }

    if !ends_with_newline {
        expected.pop();
    }

    assert_eq!(signed_data.to_str(), expected.to_str());
    Ok(())
}

#[test]
fn extract_sig_singleline() -> crate::Result {
    extract_signed_helper("signed-singleline.txt", b"magic:signature", 4..=4)
}

#[test]
fn extract_sig_signed() -> crate::Result {
    extract_signed_helper("signed.txt", b"-----BEGIN PGP SIGNATURE-----\n\niQEzBAABCAAdFiEEdjYp/sh4j8NRKLX27gKdHl60AwAFAl7p9tgACgkQ7gKdHl60\nAwBpegf+KQciv9AOIN7+yPmowecGxBnSfpKWTDzFxnyGR8dq63SpWT8WEKG5mf3a\nG6iUqpsDWaMHlzihaMKRvgRpZxFRbjnNPFBj6F4RRqfE+5R7k6DRSLUV5PqnsdSH\nuccfIDWi1imhsm7AaP5trwl1t+83U2JhHqPcPVFLMODYwWeO6NLR/JCzGSTQRa8t\nRgaVMKI19O/fge5OT5Ua8D47VKEhsJX0LfmkP5RfZQ8JJvNd40TupqKRdlv0sAzP\nya7NXkSHXCavHNR6kA+KpWxn900UoGK8/IDlwU6MeOkpPVawb3NFMqnc7KJDaC2p\nSMzpuEG8LTrCx2YSpHNLqHyzvQ1CZA==\n=5ITV\n-----END PGP SIGNATURE-----", 4..=14)
}

#[test]
fn extract_sig_with_encoding() -> crate::Result {
    extract_signed_helper("signed-with-encoding.txt", SIGNATURE, 5..=15)
}

#[test]
fn extract_sig_with_msg_footer() -> crate::Result {
    extract_signed_helper("message-with-footer.txt", b"-----BEGIN PGP SIGNATURE-----\n\niHUEABYIAB0WIQSuZwcGWSQItmusNgR5URpSUCnwXQUCYT7xpAAKCRB5URpSUCnw\nXWB3AP9q323HlxnI8MyqszNOeYDwa7Y3yEZaUM2y/IRjz+z4YQEAq0yr1Syt3mrK\nOSFCqL2vDm3uStP+vF31f6FnzayhNg0=\n=Mhpp\n-----END PGP SIGNATURE-----", 4..=10)
}

#[test]
fn extract_sig_whitespace() -> crate::Result {
    extract_signed_helper("signed-whitespace.txt", OTHER_SIGNATURE, 5..=15)
}
