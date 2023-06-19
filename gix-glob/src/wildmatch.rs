use bitflags::bitflags;
bitflags! {
    /// The match mode employed in [`Pattern::matches()`][crate::Pattern::matches()].
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
    pub struct Mode: u8 {
        /// Let globs like `*` and `?` not match the slash `/` literal, which is useful when matching paths.
        const NO_MATCH_SLASH_LITERAL = 1 << 0;
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
    const BRACKET_OPEN: u8 = b'[';
    const BRACKET_CLOSE: u8 = b']';
    const COLON: u8 = b':';

    const NEGATE_CLASS: u8 = b'!';

    fn match_recursive(pattern: &BStr, text: &BStr, mode: Mode) -> Result {
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
                None if p_ch != STAR => return Result::AbortAll,
                None => (text.len(), 0),
            };

            if p_ch == BACKSLASH {
                match p.next() {
                    Some((_p_idx, p_ch)) => {
                        if p_ch != t_ch {
                            return Result::NoMatch;
                        } else {
                            continue;
                        }
                    }
                    None => return Result::NoMatch,
                };
            }
            match p_ch {
                b'?' => {
                    if mode.contains(Mode::NO_MATCH_SLASH_LITERAL) && t_ch == SLASH {
                        return Result::NoMatch;
                    } else {
                        continue;
                    }
                }
                STAR => {
                    let mut match_slash = !mode.contains(Mode::NO_MATCH_SLASH_LITERAL);
                    match p.next() {
                        Some((next_p_idx, next_p_ch)) => {
                            let next;
                            if next_p_ch == STAR {
                                let leading_slash_idx = p_idx.checked_sub(1);
                                while p.next_if(|(_, c)| *c == STAR).is_some() {}
                                next = p.next();
                                if !mode.contains(Mode::NO_MATCH_SLASH_LITERAL) {
                                    match_slash = true;
                                } else if leading_slash_idx.map_or(true, |idx| pattern[idx] == SLASH)
                                    && next.map_or(true, |(_, c)| {
                                        c == SLASH || (c == BACKSLASH && p.peek().map(|t| t.1) == Some(SLASH))
                                    })
                                {
                                    if next.map_or(Result::NoMatch, |(idx, _)| {
                                        match_recursive(pattern[idx + 1..].as_bstr(), text[t_idx..].as_bstr(), mode)
                                    }) == Result::Match
                                    {
                                        return Result::Match;
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
                                        Result::NoMatch
                                    } else {
                                        Result::Match
                                    };
                                }
                                Some((next_p_idx, next_p_ch)) => {
                                    p_idx = next_p_idx;
                                    p_ch = next_p_ch;
                                    if !match_slash && p_ch == SLASH {
                                        match text[t_idx..].find_byte(SLASH) {
                                            Some(distance_to_slash) => {
                                                for _ in t.by_ref().take(distance_to_slash) {}
                                                continue;
                                            }
                                            None => return Result::NoMatch,
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            return if !match_slash && text[t_idx..].contains(&SLASH) {
                                Result::NoMatch
                            } else {
                                Result::Match
                            }
                        }
                    }

                    return loop {
                        if !crate::parse::GLOB_CHARACTERS.contains(&p_ch) {
                            loop {
                                if (!match_slash && t_ch == SLASH) || t_ch == p_ch {
                                    break;
                                }
                                match t.next() {
                                    Some(t) => {
                                        t_idx = t.0;
                                        t_ch = t.1;
                                    }
                                    None => break,
                                };
                            }
                            if t_ch != p_ch {
                                return Result::NoMatch;
                            }
                        }
                        let res = match_recursive(pattern[p_idx..].as_bstr(), text[t_idx..].as_bstr(), mode);
                        if res != Result::NoMatch {
                            if !match_slash || res != Result::AbortToStarStar {
                                return res;
                            }
                        } else if !match_slash && t_ch == SLASH {
                            return Result::AbortToStarStar;
                        }
                        match t.next() {
                            Some(t) => {
                                t_idx = t.0;
                                t_ch = t.1;
                            }
                            None => break Result::AbortAll,
                        };
                    };
                }
                BRACKET_OPEN => {
                    match p.next() {
                        Some(t) => {
                            p_idx = t.0;
                            p_ch = t.1;
                        }
                        None => return Result::AbortAll,
                    };

                    if p_ch == b'^' {
                        p_ch = NEGATE_CLASS;
                    }
                    let negated = p_ch == NEGATE_CLASS;
                    let mut next = if negated { p.next() } else { Some((p_idx, p_ch)) };
                    let mut prev_p_ch = 0;
                    let mut matched = false;
                    loop {
                        match next {
                            None => return Result::AbortAll,
                            Some((p_idx, mut p_ch)) => match p_ch {
                                BACKSLASH => match p.next() {
                                    Some((_, p_ch)) => {
                                        if p_ch == t_ch {
                                            matched = true
                                        } else {
                                            prev_p_ch = p_ch;
                                        }
                                    }
                                    None => return Result::AbortAll,
                                },
                                b'-' if prev_p_ch != 0
                                    && p.peek().is_some()
                                    && p.peek().map(|t| t.1) != Some(BRACKET_CLOSE) =>
                                {
                                    p_ch = p.next().expect("peeked").1;
                                    if p_ch == BACKSLASH {
                                        p_ch = match p.next() {
                                            Some(t) => t.1,
                                            None => return Result::AbortAll,
                                        };
                                    }
                                    if t_ch <= p_ch && t_ch >= prev_p_ch {
                                        matched = true;
                                    } else if mode.contains(Mode::IGNORE_CASE) && t_ch.is_ascii_lowercase() {
                                        let t_ch_upper = t_ch.to_ascii_uppercase();
                                        if (t_ch_upper <= p_ch.to_ascii_uppercase()
                                            && t_ch_upper >= prev_p_ch.to_ascii_uppercase())
                                            || (t_ch_upper <= prev_p_ch.to_ascii_uppercase()
                                                && t_ch_upper >= p_ch.to_ascii_uppercase())
                                        {
                                            matched = true;
                                        }
                                    }
                                    prev_p_ch = 0;
                                }
                                BRACKET_OPEN if matches!(p.peek(), Some((_, COLON))) => {
                                    p.next();
                                    while p.peek().map_or(false, |t| t.1 != BRACKET_CLOSE) {
                                        p.next();
                                    }
                                    let closing_bracket_idx = match p.next() {
                                        Some((idx, _)) => idx,
                                        None => return Result::AbortAll,
                                    };
                                    const BRACKET__COLON__BRACKET: usize = 3;
                                    if closing_bracket_idx - p_idx < BRACKET__COLON__BRACKET
                                        || pattern[closing_bracket_idx - 1] != COLON
                                    {
                                        if t_ch == BRACKET_OPEN {
                                            matched = true
                                        }
                                        p = pattern[p_idx + 1..]
                                            .iter()
                                            .map(possibly_lowercase)
                                            .enumerate()
                                            .peekable();
                                    } else {
                                        let class = &pattern.as_bytes()[p_idx + 2..closing_bracket_idx - 1];
                                        match class {
                                            b"alnum" => {
                                                if t_ch.is_ascii_alphanumeric() {
                                                    matched = true;
                                                }
                                            }
                                            b"alpha" => {
                                                if t_ch.is_ascii_alphabetic() {
                                                    matched = true;
                                                }
                                            }
                                            b"blank" => {
                                                if t_ch.is_ascii_whitespace() {
                                                    matched = true;
                                                }
                                            }
                                            b"cntrl" => {
                                                if t_ch.is_ascii_control() {
                                                    matched = true;
                                                }
                                            }
                                            b"digit" => {
                                                if t_ch.is_ascii_digit() {
                                                    matched = true;
                                                }
                                            }

                                            b"graph" => {
                                                if t_ch.is_ascii_graphic() {
                                                    matched = true;
                                                }
                                            }
                                            b"lower" => {
                                                if t_ch.is_ascii_lowercase() {
                                                    matched = true;
                                                }
                                            }
                                            b"print" => {
                                                if (0x20u8..=0x7e).contains(&t_ch) {
                                                    matched = true;
                                                }
                                            }
                                            b"punct" => {
                                                if t_ch.is_ascii_punctuation() {
                                                    matched = true;
                                                }
                                            }
                                            b"space" => {
                                                if t_ch == b' ' {
                                                    matched = true;
                                                }
                                            }
                                            b"upper" => {
                                                if t_ch.is_ascii_uppercase()
                                                    || mode.contains(Mode::IGNORE_CASE) && t_ch.is_ascii_lowercase()
                                                {
                                                    matched = true;
                                                }
                                            }
                                            b"xdigit" => {
                                                if t_ch.is_ascii_hexdigit() {
                                                    matched = true;
                                                }
                                            }
                                            _ => return Result::AbortAll,
                                        };
                                        prev_p_ch = 0;
                                    }
                                }
                                _ => {
                                    prev_p_ch = p_ch;
                                    if p_ch == t_ch {
                                        matched = true;
                                    }
                                }
                            },
                        };
                        next = p.next();
                        if let Some((_, BRACKET_CLOSE)) = next {
                            break;
                        }
                    }
                    if matched == negated || mode.contains(Mode::NO_MATCH_SLASH_LITERAL) && t_ch == SLASH {
                        return Result::NoMatch;
                    }
                    continue;
                }
                non_glob_ch => {
                    if non_glob_ch != t_ch {
                        return Result::NoMatch;
                    } else {
                        continue;
                    }
                }
            }
        }
        t.next().map_or(Result::Match, |_| Result::NoMatch)
    }

    /// Employ pattern matching to see if `value` matches `pattern`.
    ///
    /// `mode` can be used to adjust the way the matching is performed.
    pub fn wildmatch(pattern: &BStr, value: &BStr, mode: Mode) -> bool {
        match_recursive(pattern, value, mode) == Result::Match
    }
}
