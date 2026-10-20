use gix_zlib::{Compression, Inflate, Status};

use crate::stream::deflate::compressed;

#[test]
fn once_reports_progress_and_reset_allows_reuse() {
    let input = compressed(b"inflate once", Compression::BEST_SPEED);
    let mut inflate = Inflate::default();
    let mut output = [0; 32];

    let (status, consumed, written) = inflate.once(&input, &mut output).expect("valid stream");
    assert_eq!(status, Status::StreamEnd);
    assert_eq!(consumed, input.len());
    assert_eq!(&output[..written], b"inflate once");

    inflate.reset();
    let (_, consumed, written) = inflate.once(&input, &mut output).expect("valid stream after reset");
    assert_eq!(consumed, input.len());
    assert_eq!(written, b"inflate once".len());
}

#[test]
fn once_into_a_short_buffer_returns_the_beginning_of_the_stream() {
    let input = compressed(b"more than fits into the output", Compression::BEST_SPEED);
    let mut output = [0; 9];
    let (status, _consumed, written) = Inflate::default().once(&input, &mut output).expect("valid stream");
    assert_ne!(status, Status::StreamEnd, "the stream continues past the output buffer");
    assert_eq!(&output[..written], b"more than");
}

#[test]
fn prefix_can_be_continued_with_rest() -> gix_error::TestResult {
    let sizes: Vec<usize> = (0..=16)
        .chain([
            31, 32, 33, 257, 258, 259, 4095, 4096, 4097, 32767, 32768, 32769, 65535, 65536, 65537, 131073,
        ])
        .collect();
    let mut seed = 0x1234_5678_u32;
    let random: Vec<u8> = (0..131073)
        .map(|_| {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            seed as u8
        })
        .collect();
    let repeated: Vec<u8> = b"header and then the body\0"
        .iter()
        .copied()
        .cycle()
        .take(random.len())
        .collect();
    let zeros = vec![0; random.len()];

    for (pattern, data) in [("zeros", &zeros), ("repeated", &repeated), ("random", &random)] {
        for size in sizes.iter().copied() {
            let data = &data[..size];
            for level in [Compression::NONE, Compression::BEST_SPEED, Compression::DEFAULT] {
                let input = compressed(data, level);
                // Exhaust tiny splits; sample larger ones around match/window boundaries and throughout the stream.
                let mut prefixes: Vec<usize> = if size <= 16 {
                    (0..=size + 1).collect()
                } else {
                    [0, 1, 257, 258, 259, 32767, 32768, 32769]
                        .into_iter()
                        .filter(|len| *len <= size)
                        .chain([size / 2, size - 1, size, size + 1])
                        .collect()
                };
                prefixes.extend(random[..2].iter().map(|byte| usize::from(*byte) * size / 255));
                prefixes.sort_unstable();
                prefixes.dedup();
                for prefix_len in prefixes {
                    assert_prefix_rest(
                        data,
                        &input,
                        prefix_len,
                        &format!("{pattern}, size={size}, level={level:?}"),
                    )?;
                }
            }
        }
    }
    Ok(())
}

#[test]
fn rest_after_completed_prefix_does_not_start_following_stream() -> gix_error::TestResult {
    let data = b"complete object";
    let mut input = compressed(data, Compression::BEST_SPEED);
    input.extend_from_slice(&compressed(
        b"another object, not part of the first",
        Compression::BEST_SPEED,
    ));
    assert_prefix_rest(
        data,
        &input,
        data.len() + 1,
        "completed stream followed by another stream",
    )
}

fn assert_prefix_rest(data: &[u8], input: &[u8], prefix_len: usize, case: &str) -> gix_error::TestResult {
    use gix_error::{ResultExt, message};

    let expected_written = prefix_len.min(data.len());
    let mut skips: Vec<usize> = if data.len() <= 16 {
        (0..=expected_written).collect()
    } else {
        vec![0, expected_written / 2, expected_written]
    };
    skips.sort_unstable();
    skips.dedup();
    let mut inflate = Inflate::default();
    let mut prefix = vec![0; prefix_len];
    let mut out = Vec::new();
    for skip in skips {
        for spare in [0, 17] {
            let context = format!("{case}, prefix={prefix_len}, skip={skip}, spare={spare}");
            inflate.reset();
            let (status, consumed, written) = inflate
                .prefix(input, &mut prefix)
                .or_raise(|| message!("prefix: {context}"))?;
            assert_eq!(
                written, expected_written,
                "prefix fills the buffer or finishes: {context}"
            );
            assert_eq!(
                &prefix[..written],
                &data[..written],
                "prefix matches the original: {context}"
            );
            assert!(
                consumed <= input.len(),
                "prefix consumes only supplied input: {context}"
            );
            if written < data.len() {
                assert_ne!(status, Status::StreamEnd, "unwritten data remains: {context}");
            }

            // Reuse a nonempty buffer to ensure rest replaces, rather than appends to, its contents.
            out.resize(data.len() + 23, 0xa5);
            let n = inflate
                .rest(
                    &input[consumed..],
                    &prefix[..written],
                    skip,
                    &mut out,
                    data.len() - skip + spare,
                )
                .or_raise(|| message!("rest: {context}"))?;
            assert_eq!(
                n,
                data.len() - skip,
                "rest reports the entire unskipped object: {context}"
            );
            assert_eq!(out.len(), n, "rest truncates unused output space: {context}");
            assert_eq!(
                out,
                &data[skip..],
                "rest preserves the prefix tail and decodes the remainder: {context}"
            );
        }
    }
    Ok(())
}

#[test]
fn corrupt_streams_keep_classification_and_context() {
    let mut input = compressed(b"the zlib header protects this stream", Compression::BEST_SPEED);
    input[0] = 0xff;
    let err = Inflate::default()
        .once(&input, &mut [0; 64])
        .expect_err("the corrupt header must be rejected");

    insta::assert_debug_snapshot!(err, "corrupt streams keep classification and context", @"
    Could not decode zip stream

    Caused by:
        0: Invalid input data
    ");
    assert!(
        err.is_corrupted(),
        "the underlying invalid stream should be classified as corruption"
    );
}
