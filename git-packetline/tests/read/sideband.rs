#[cfg(all(not(feature = "blocking-io"), feature = "async-io"))]
use futures_lite::io::AsyncReadExt;
use git_packetline::PacketLine;
#[cfg(feature = "blocking-io")]
use std::io::{BufRead, Read};

#[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
async fn peek_past_an_actual_eof_is_an_error() -> crate::Result {
    let input = b"0009ERR e";
    let mut rd = git_packetline::StreamingPeekableIter::new(&input[..], &[]);
    let mut reader = rd.as_read();
    let res = reader.peek_data_line().await;
    assert_eq!(res.expect("one line")??, b"ERR e");

    let mut buf = String::new();
    reader.read_line(&mut buf).await?;
    assert_eq!(
        buf, "ERR e",
        "by default ERR lines won't propagate as failure but are merely text"
    );

    let res = reader.peek_data_line().await;
    assert_eq!(
        res.expect("an err").expect_err("foo").kind(),
        std::io::ErrorKind::UnexpectedEof,
        "peeking past the end is not an error as the caller should make sure we dont try 'invalid' reads"
    );
    Ok(())
}

#[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
async fn peek_past_a_delimiter_is_no_error() -> crate::Result {
    let input = b"0009hello0000";
    let mut rd = git_packetline::StreamingPeekableIter::new(&input[..], &[PacketLine::Flush]);
    let mut reader = rd.as_read();
    let res = reader.peek_data_line().await;
    assert_eq!(res.expect("one line")??, b"hello");

    let mut buf = String::new();
    reader.read_line(&mut buf).await?;
    assert_eq!(buf, "hello");

    let res = reader.peek_data_line().await;
    assert!(
        res.is_none(),
        "peeking past a flush packet is a 'natural' event that shold not cause an error"
    );
    Ok(())
}

#[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
async fn handling_of_err_lines() {
    let input = b"0009ERR e0009ERR x0000";
    let mut rd = git_packetline::StreamingPeekableIter::new(&input[..], &[]);
    rd.fail_on_err_lines(true);
    let mut buf = [0u8; 2];
    let mut reader = rd.as_read();
    let res = reader.read(buf.as_mut()).await;
    assert_eq!(
        res.unwrap_err().to_string(),
        "e",
        "it respects errors and passes them on"
    );
    let res = reader.read(buf.as_mut()).await;
    assert_eq!(
        res.expect("read to succeed - EOF"),
        0,
        "it stops reading after an error despite there being more to read"
    );
    reader.reset_with(&[PacketLine::Flush]);
    let res = reader.read(buf.as_mut()).await;
    assert_eq!(
        res.unwrap_err().to_string(),
        "x",
        "after a reset it continues reading, but retains the 'fail_on_err_lines' setting"
    );
    assert_eq!(
        reader.stopped_at(),
        None,
        "An error can also be the reason, which is not distinguishable from an EOF"
    );
}
