#![allow(unused)]
use std::borrow::Borrow;

#[derive(Clone)]
pub enum Owned {
    Sha1([u8; 20]),
    Sha256([u8; 32]),
}

impl Owned {
    pub fn sha1() -> Self {
        Owned::Sha1([0; 20])
    }
    pub fn sha256() -> Self {
        Owned::Sha256([0; 32])
    }
}

#[repr(transparent)]
pub struct Borrowed {
    bytes: [u8],
}

impl Borrowed {
    #[inline]
    pub fn from_bytes(slice: &[u8]) -> &Borrowed {
        unsafe { &*(slice as *const [u8] as *const Borrowed) }
    }
}

impl AsRef<Borrowed> for &Borrowed {
    fn as_ref(&self) -> &Borrowed {
        self
    }
}

impl AsRef<Borrowed> for Owned {
    fn as_ref(&self) -> &Borrowed {
        match self {
            Owned::Sha1(b) => Borrowed::from_bytes(b.as_ref()),
            Owned::Sha256(b) => Borrowed::from_bytes(&b.as_ref()),
        }
    }
}

fn lookup(id: impl AsRef<Borrowed>) {}

fn call_lookup() {
    lookup(Owned::sha1());
    lookup(&Owned::sha256());
    lookup(Borrowed::from_bytes(b"hello"))
}

fn use_owned(_id: Owned) {}
fn use_by_ref(_id: &Owned) {}

fn use_by_ref_impl_as_ref(id: impl AsRef<Owned>) {
    use_by_ref(id.as_ref())
}

pub fn use_by_ref_impl_borrow(id: impl Borrow<Owned>) {
    use_by_ref(id.borrow());
}

fn call_use_owned() {
    use_owned(Owned::sha1())
}
fn call_by_ref() {
    use_by_ref(&Owned::sha1())
}

fn call_by_ref_impl_as_ref_with_owned() {
    // use_by_ref_impl_as_ref(Owned::sha1()) // does not work as it requires types like String/str, Vec/slice
}

fn call_by_ref_impl_as_ref_with_reference() {
    // use_by_ref_impl_as_ref(&Owned::sha1()) // does not work as it requires types like String/str, Vec/slice
}

fn call_by_ref_impl_borrow_with_owned() {
    use_by_ref_impl_borrow(Owned::sha1())
}

fn call_by_ref_impl_borrow_with_reference() {
    use_by_ref_impl_borrow(&Owned::sha1())
}
