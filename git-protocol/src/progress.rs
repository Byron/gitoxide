use bstr::ByteSlice;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Remote<'a> {
    pub action: &'a [u8],
    pub percent: Option<u32>,
    pub step: Option<usize>,
    pub max: Option<usize>,
}

pub fn decode(line: &[u8]) -> Remote {
    line.find_byte(b':')
        .map(|pos| (&line[..pos], pos))
        .and_then(|(action, pos)| line.get(pos..).map(|last_part| (action, pos, last_part)));
    unimplemented!("progress decode")
}
