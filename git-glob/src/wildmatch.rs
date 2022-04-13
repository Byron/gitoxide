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
    use bstr::{BStr, ByteSlice};

    use crate::wildmatch::Mode;

    #[derive(Eq, PartialEq)]
    enum Result {
        Match,
        NoMatch,
        AbortAll,
        AbortToStarStar,
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
        let mut p = pattern.iter().map(possibly_lowercase).enumerate().peekable();
        let mut t = text.iter().map(possibly_lowercase).enumerate();

        while let Some((mut p_idx, mut p_ch)) = p.next() {
            let (mut t_idx, mut t_ch) = match t.next() {
                Some(c) => c,
                None if p_ch != STAR => return AbortAll,
                None => (text.len(), 0),
            };

            if p_ch == BACKSLASH {
                match p.next() {
                    Some((_p_idx, p_ch)) => {
                        if p_ch != t_ch {
                            return NoMatch;
                        } else {
                            continue;
                        }
                    }
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
                    let mut match_slash = mode.contains(Mode::SLASH_IS_LITERAL).then(|| false).unwrap_or(true);
                    match p.next() {
                        Some((next_p_idx, next_p_ch)) => {
                            let next;
                            if next_p_ch == STAR {
                                let leading_slash_idx = p_idx.checked_sub(1);
                                while p.next_if(|(_, c)| *c == STAR).is_some() {}
                                next = p.next();
                                if !mode.contains(Mode::SLASH_IS_LITERAL) {
                                    match_slash = true;
                                } else if leading_slash_idx.map_or(true, |idx| pattern[idx] == SLASH)
                                    && next.map_or(true, |(_, c)| {
                                        c == SLASH || (c == BACKSLASH && p.peek().map(|t| t.1) == Some(SLASH))
                                    })
                                    && next.map_or(false, |t| t.1 == SLASH)
                                {
                                    if match_recursive(
                                        pattern[next.expect("checked prior").0 + 1..].as_bstr(),
                                        text[t_idx..].as_bstr(),
                                        mode,
                                    ) == Match
                                    {
                                        return Match;
                                    }
                                    match_slash = true;
                                } else {
                                    match_slash = false;
                                }
                            } else {
                                next = Some((next_p_idx, next_p_ch));
                            }

                            match next {
                                None => {
                                    return if !match_slash && text[t_idx..].contains(&SLASH) {
                                        NoMatch
                                    } else {
                                        Match
                                    };
                                }
                                Some((next_p_idx, next_p_ch)) => {
                                    (p_idx, p_ch) = (next_p_idx, next_p_ch);
                                    if !match_slash && p_ch == SLASH {
                                        match text[t_idx..].find_byte(SLASH) {
                                            Some(distance_to_slash) => {
                                                for _ in t.by_ref().take(distance_to_slash) {}
                                                continue;
                                            }
                                            None => return NoMatch,
                                        }
                                    }
                                }
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

                    return loop {
                        if !crate::parse::GLOB_CHARACTERS.contains(&p_ch) {
                            loop {
                                if (!match_slash && t_ch == SLASH) || t_ch == p_ch {
                                    break;
                                }
                                (t_idx, t_ch) = match t.next() {
                                    Some(t) => (t.0, t.1),
                                    None => break,
                                };
                            }
                            if t_ch != p_ch {
                                return NoMatch;
                            }
                        }
                        let res = match_recursive(pattern[p_idx..].as_bstr(), text[t_idx..].as_bstr(), mode);
                        if res != NoMatch {
                            if !match_slash || res != AbortToStarStar {
                                return res;
                            }
                        } else if !match_slash && t_ch == SLASH {
                            return AbortToStarStar;
                        }
                        (t_idx, t_ch) = match t.next() {
                            Some(t) => (t.0, t.1),
                            None => break AbortAll,
                        };
                    };
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
