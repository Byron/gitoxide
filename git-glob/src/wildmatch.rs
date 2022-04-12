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

    #[derive(Eq, PartialEq)]
    enum Result {
        Match,
        NoMatch,
        AbortAll,
        // AbortToStarStar,
    }

    const STAR: u8 = b'*';

    fn match_recursive(pattern: &BStr, text: &BStr, mode: Mode) -> Result {
        use self::Result::*;
        let possibly_lowercase = |c: &u8| {
            if mode.contains(Mode::IGNORE_CASE) {
                c.to_ascii_lowercase()
            } else {
                *c
            }
        };
        let mut p = pattern.iter().map(possibly_lowercase);
        let mut t = text.iter().map(possibly_lowercase);

        while let Some(mut p_ch) = p.next() {
            let t_ch = match t.next() {
                Some(c) => c,
                None if p_ch != STAR => return AbortAll,
                None => 0,
            };

            if p_ch == b'\\' {
                p_ch = match p.next() {
                    Some(c) => c,
                    None => return NoMatch,
                };
            }
            match p_ch {
                non_glob_ch => {
                    if non_glob_ch != t_ch {
                        return NoMatch;
                    } else {
                        continue;
                    }
                }
            }
        }
        t.next().map(|_| NoMatch).unwrap_or(Match)
    }

    pub fn wildmatch(pattern: &BStr, value: &BStr, mode: Mode) -> bool {
        match_recursive(pattern, value, mode) == Result::Match
    }
}
