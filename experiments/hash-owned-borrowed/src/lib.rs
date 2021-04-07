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
