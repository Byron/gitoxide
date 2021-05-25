use crate::pack::fixture_path;
use std::convert::TryInto;

#[test]
fn encode_decode_roundtrip() -> crate::Result {
    let buf = std::fs::read(fixture_path(
        "objects/pack/pack-11fdfa9e156ab73caae3b6da867192221f2089c2.pack",
    ))?;
    let expected_encoded_header = &buf[..12];
    let (version, num_objects) = git_pack::pack::data::header::decode(expected_encoded_header.try_into()?)?;
    let actual_encoded_header = git_pack::pack::data::header::encode(version, num_objects);
    assert_eq!(actual_encoded_header, expected_encoded_header);
    Ok(())
}
