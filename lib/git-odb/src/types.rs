#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Time {
    /// time in seconds from epoch
    pub time: u32,
    /// time offset in seconds
    pub offset: i32,
    /// the sign seen in front of -0000
    pub sign: Sign,
}
