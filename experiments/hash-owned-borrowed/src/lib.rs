#[derive(Clone)]
pub enum Owned {
    Sha1([u8; 20]),
    Sha256([u8; 32]),
}
