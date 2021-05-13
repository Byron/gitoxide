use bstr::ByteSlice;
use git_packetline::Writer;
use std::io::Write;

const MAX_DATA_LEN: usize = 65516;
const MAX_LINE_LEN: usize = 4 + MAX_DATA_LEN;

#[test]
fn each_write_results_in_one_line() -> crate::Result {
    let mut w = Writer::new(Vec::new());
    w.write_all(b"hello")?;
    w.write(b"world!")?;
    assert_eq!(w.inner.as_bstr(), b"0009hello000aworld!".as_bstr());
    Ok(())
}

#[test]
fn write_text_and_write_binary() -> crate::Result {
    let mut w = Writer::new(Vec::new()).text_mode();
    w.write_all(b"hello")?;
    w = w.binary_mode();
    w.write(b"world")?;
    assert_eq!(w.inner.as_bstr(), b"000ahello\n0009world".as_bstr());
    Ok(())
}

#[test]
fn huge_writes_are_split_into_lines() -> crate::Result {
    let data = {
        let mut v = Vec::new();
        v.resize(MAX_DATA_LEN * 2, 0);
        v
    };
    let mut w = Writer::new(Vec::new());
    w.write(&data)?;
    assert_eq!(w.inner.len(), MAX_LINE_LEN * 2);
    Ok(())
}

#[test]
fn empty_writes_fail_with_error() {
    assert_eq!(
        Writer::new(Vec::new()).write(&[]).unwrap_err().to_string(),
        "empty packet lines are not permitted as '0004' is invalid"
    )
}

#[test]
fn nothing_happens_on_into_read() {
    let mut out = Vec::new();
    let w = Writer::new(&mut out);
    drop(w);
    assert!(out.is_empty());
}
