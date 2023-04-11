use gix_hashtable::hash::Hasher as GixHasher;
use std::hash::Hasher;

#[test]
fn write_works() {
    let mut hasher = GixHasher::default();
    hasher.write(u64::to_ne_bytes(0x0a0a9f2a7b7e0367).as_ref());
    assert_eq!(hasher.finish(), 0x0a0a9f2a7b7e0367);
}

#[test]
#[should_panic]
fn other_methods_panic() {
    let mut hasher = GixHasher::default();
    hasher.write_usize(4);
}
