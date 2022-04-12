use bitflags::bitflags;
bitflags! {
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Mode: u8 {
        /// Let globs not match the slash `/` literal.
        const SLASH_IS_LITERAL = 1 << 0;
        /// Match case insensitively for ascii characters only.
        const IGNORE_CASE = 1 << 1;
    }
}

pub(crate) mod function {
    use bstr::BStr;

    use crate::wildmatch::Mode;

    pub fn wildmatch(pattern: &BStr, value: &BStr, _mode: Mode) -> bool {
        todo!("actual wildcard match for '{}' ~= '{}'", pattern, value)
    }
}
