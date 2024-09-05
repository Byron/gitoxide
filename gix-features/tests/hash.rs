use gix_features::hash::Hasher;

#[cfg(not(feature = "fast-sha1"))]
#[test]
fn size_of_sha1() {
    assert_eq!(std::mem::size_of::<Hasher>(), 96);
}

#[cfg(feature = "fast-sha1")]
#[test]
fn size_of_sha1() {
    assert_eq!(
        std::mem::size_of::<Hasher>(),
        if cfg!(target_arch = "x86") { 96 } else { 104 }
    );
}
