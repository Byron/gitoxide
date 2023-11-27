use gix_features::hash::Sha1;

#[cfg(not(feature = "fast-sha1"))]
#[test]
fn size_of_sha1() {
    assert_eq!(std::mem::size_of::<Sha1>(), 96)
}

#[cfg(all(feature = "fast-sha1", target_arch = "x86"))]
#[test]
fn size_of_sha1() {
    assert_eq!(std::mem::size_of::<Sha1>(), 96)
}

#[cfg(all(feature = "fast-sha1", not(target_arch = "x86")))]
#[test]
fn size_of_sha1() {
    assert_eq!(std::mem::size_of::<Sha1>(), 104)
}
