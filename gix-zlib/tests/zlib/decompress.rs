use gix_zlib::{Compression, Decompress, FlushDecompress, Status};

use crate::stream::deflate::compressed;

#[test]
fn lifecycle_counters_and_flush_modes() -> gix_error::TestResult {
    let expected = b"hello decompressor";
    let input = compressed(expected, Compression::BEST_SPEED);
    let mut state = Decompress::default();
    assert_eq!(state.total_in(), 0);
    assert_eq!(state.total_out(), 0);
    assert_eq!(state.error_message(), None);

    let mut output = [0; 64];
    let status = state.decompress(&input, &mut output, FlushDecompress::Finish)?;
    assert_eq!(status, Status::StreamEnd);
    assert_eq!(state.total_in(), input.len() as u64);
    assert_eq!(state.total_out(), expected.len() as u64);
    assert_eq!(&output[..state.total_out() as usize], expected);

    state.reset();
    assert_eq!(state.total_in(), 0);
    assert_eq!(state.total_out(), 0);
    assert_eq!(
        state.decompress(&input, &mut output, FlushDecompress::Sync)?,
        Status::StreamEnd
    );
    Ok(())
}
