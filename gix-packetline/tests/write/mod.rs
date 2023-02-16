#[cfg(feature = "blocking-io")]
use std::io::Write;

use bstr::ByteSlice;
#[cfg(all(feature = "async-io", not(feature = "blocking-io")))]
use futures_lite::prelude::*;
use git_packetline::Writer;

const MAX_DATA_LEN: usize = 65516;
const MAX_LINE_LEN: usize = 4 + MAX_DATA_LEN;

#[allow(clippy::unused_io_amount)] // under test
#[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
async fn each_write_results_in_one_line() -> crate::Result {
    let mut w = Writer::new(Vec::new());
    w.write_all(b"hello").await?;
    w.write(b"world!").await?;
    let buf = w.into_inner();
    assert_eq!(buf.as_bstr(), b"0009hello000aworld!".as_bstr());
    Ok(())
}

#[allow(clippy::unused_io_amount)] // under test
#[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
async fn write_text_and_write_binary() -> crate::Result {
    let buf = {
        let mut w = Writer::new(Vec::new()).text_mode();
        w.write_all(b"hello").await?;
        w = w.binary_mode();
        w.write(b"world").await?;
        w.into_inner()
    };
    assert_eq!(buf.as_bstr(), b"000ahello\n0009world".as_bstr());
    Ok(())
}

#[allow(clippy::unused_io_amount)] // under test
#[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
async fn huge_writes_are_split_into_lines() -> crate::Result {
    let buf = {
        let data = {
            let mut v = Vec::new();
            v.resize(MAX_DATA_LEN * 2, 0);
            v
        };
        let mut w = Writer::new(Vec::new());
        w.write(&data).await?;
        w.into_inner()
    };
    assert_eq!(buf.len(), MAX_LINE_LEN * 2);
    Ok(())
}

#[maybe_async::test(feature = "blocking-io", async(feature = "async-io", async_std::test))]
async fn empty_writes_fail_with_error() {
    let res = Writer::new(Vec::new()).write(&[]).await;
    assert_eq!(
        res.unwrap_err().to_string(),
        "empty packet lines are not permitted as '0004' is invalid"
    );
}
