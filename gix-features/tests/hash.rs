use gix_features::hash::Sha1;

#[cfg(not(feature = "fast-sha1"))]
#[test]
fn size_of_sha1() {
    assert_eq!(std::mem::size_of::<Sha1>(), 96)
}

#[cfg(feature = "fast-sha1")]
#[test]
fn size_of_sha1() {
    assert_eq!(
        std::mem::size_of::<Sha1>(),
        if cfg!(target_arch = "x86") { 96 } else { 104 }
    )
}
