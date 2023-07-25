use bstr::ByteSlice;
use gix_actor::Identity;

#[test]
fn round_trip() -> gix_testtools::Result {
    static DEFAULTS: &[&[u8]] =     &[
        b"Sebastian Thiel <byronimo@gmail.com>",
        ".. ☺️Sebastian 王知明 Thiel🙌 .. <byronimo@gmail.com>".as_bytes(),
        b".. whitespace  \t  is explicitly allowed    - unicode aware trimming must be done elsewhere  <byronimo@gmail.com>"
    ];
    for input in DEFAULTS {
        let signature: Identity = gix_actor::IdentityRef::from_bytes::<()>(input).unwrap().into();
        let mut output = Vec::new();
        signature.write_to(&mut output)?;
        assert_eq!(output.as_bstr(), input.as_bstr());
    }
    Ok(())
}
