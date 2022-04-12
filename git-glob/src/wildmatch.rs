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
    const BACKSLASH: u8 = b'\\';
    const SLASH: u8 = b'/';

    fn match_recursive(pattern: &BStr, text: &BStr, mode: Mode) -> Result {
        use self::Result::*;
        let possibly_lowercase = |c: &u8| {
            if mode.contains(Mode::IGNORE_CASE) {
                c.to_ascii_lowercase()
            } else {
                *c
            }
        };
        let mut p = pattern.iter().map(possibly_lowercase).enumerate();
        let mut t = text.iter().map(possibly_lowercase).enumerate();

        while let Some((mut _p_idx, mut p_ch)) = p.next() {
            let (t_idx, t_ch) = match t.next() {
                Some(c) => c,
                None if p_ch != STAR => return AbortAll,
                None => (text.len(), 0), // out of bounds, like in C, can we do better?
            };

            if p_ch == BACKSLASH {
                (_p_idx, p_ch) = match p.next() {
                    Some(c) => c,
                    None => return NoMatch,
                };
            }
            match p_ch {
                b'?' => {
                    if mode.contains(Mode::SLASH_IS_LITERAL) && t_ch == SLASH {
                        return NoMatch;
                    } else {
                        continue;
                    }
                }
                STAR => {
                    let match_slash = mode.contains(Mode::SLASH_IS_LITERAL).then(|| false).unwrap_or(true);
                    match p.next() {
                        Some((_next_p_idx, next_p_ch)) => {
                            if next_p_ch == STAR {
                                // check for '/**/' or '/**'
                                todo!("double star")
                            }
                        }
                        None => {
                            return if !match_slash && text[t_idx..].contains(&SLASH) {
                                NoMatch
                            } else {
                                Match
                            }
                        }
                    }
                }
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
