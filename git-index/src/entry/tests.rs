use crate::entry::at_rest;
use crate::Version;

#[test]
fn in_mem_flags_to_storage_flags_v2() {
    let flag_bytes = u16::from_be_bytes(*b"\x00\x01");
    let flags_at_rest = at_rest::Flags::from_bits(flag_bytes).unwrap();
    let in_memory_flags = flags_at_rest.to_memory();

    let output = in_memory_flags.to_storage(Version::V2);

    assert_eq!(output.bits(), flag_bytes);
}
