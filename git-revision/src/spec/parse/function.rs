use crate::spec;
use crate::spec::parse::delegate::SiblingBranch;
use crate::spec::parse::{delegate, Delegate, Error};
use bstr::{BStr, BString, ByteSlice, ByteVec};
use std::convert::TryInto;
use std::str::FromStr;

pub fn parse(mut input: &BStr, delegate: &mut impl Delegate) -> Result<(), Error> {
    let mut prev_kind = None;
    if let Some(b'^') = input.get(0) {
        input = next(input).1;
        delegate.kind(spec::Kind::Range).ok_or(Error::Delegate)?;
        prev_kind = spec::Kind::Range.into();
    }

    input = revision(input, delegate)?;
    if let Some((rest, kind)) = try_range(input) {
        if let Some(prev_kind) = prev_kind {
            return Err(Error::KindSetTwice { prev_kind, kind });
        }
        delegate.kind(kind).ok_or(Error::Delegate)?;
        input = revision(rest.as_bstr(), delegate)?;
    }

    if input.is_empty() {
        delegate.done();
        Ok(())
    } else {
        Err(Error::UnconsumedInput { input: input.into() })
    }
}

fn try_set_prefix(delegate: &mut impl Delegate, hex_name: &BStr) -> Option<()> {
    git_hash::Prefix::from_hex(hex_name.to_str().expect("hexadecimal only"))
        .ok()
        .and_then(|prefix| delegate.disambiguate_prefix(prefix))
}

fn long_describe_prefix(name: &BStr) -> Option<&BStr> {
    let mut iter = name.rsplit(|b| *b == b'-');
    let candidate = iter.by_ref().find_map(|substr| {
        if substr.get(0)? != &b'g' {
            return None;
        };
        let rest = substr.get(1..)?;
        rest.iter().all(|b| b.is_ascii_hexdigit()).then(|| rest.as_bstr())
    });
    iter.any(|token| !token.is_empty()).then(|| candidate).flatten()
}

fn short_describe_prefix(name: &BStr) -> Option<&BStr> {
    let mut iter = name.split(|b| *b == b'-');
    let candidate = iter
        .next()
        .and_then(|prefix| prefix.iter().all(|b| b.is_ascii_hexdigit()).then(|| prefix.as_bstr()));
    (iter.count() == 1).then(|| candidate).flatten()
}

fn parens(input: &[u8]) -> Result<Option<(std::borrow::Cow<'_, BStr>, &BStr, usize)>, Error> {
    if input.get(0) != Some(&b'{') {
        return Ok(None);
    }
    let mut open_braces = 0;
    let mut ignore_next = false;
    let mut skip_list = Vec::new();
    for (idx, b) in input.iter().enumerate() {
        match *b {
            b'{' => {
                if ignore_next {
                    ignore_next = false;
                } else {
                    open_braces += 1
                }
            }
            b'}' => {
                if ignore_next {
                    ignore_next = false;
                } else {
                    open_braces -= 1
                }
            }
            b'\\' => {
                if ignore_next {
                    ignore_next = false;
                    skip_list.push(idx);
                } else {
                    ignore_next = true;
                }
            }
            _ => ignore_next = false,
        }
        if open_braces == 0 {
            let inner: std::borrow::Cow<'_, _> = if skip_list.is_empty() {
                input[1..idx].as_bstr().into()
            } else {
                let mut from = 1;
                let mut buf = BString::default();
                for next in skip_list.into_iter() {
                    buf.push_str(&input[from..next]);
                    from = next + 1;
                }
                if let Some(rest) = input.get(from..idx) {
                    buf.push_str(rest);
                }
                buf.into()
            };
            return Ok(Some((inner, input[idx + 1..].as_bstr(), idx + 1)));
        }
    }
    Err(Error::UnclosedBracePair { input: input.into() })
}

fn try_parse<T: FromStr + PartialEq + Default>(input: &BStr) -> Result<Option<T>, Error> {
    input
        .to_str()
        .ok()
        .and_then(|n| {
            n.parse().ok().map(|n| {
                if n == T::default() && input[0] == b'-' {
                    return Err(Error::NegativeZero { input: input.into() });
                };
                Ok(n)
            })
        })
        .transpose()
}

fn revision<'a>(mut input: &'a BStr, delegate: &mut impl Delegate) -> Result<&'a BStr, Error> {
    let mut sep_pos = None;
    let mut consecutive_hex_chars = Some(0);
    {
        let mut cursor = input;
        let mut ofs = 0;
        while let Some((pos, b)) = cursor.iter().enumerate().find(|(_, b)| {
            if b"@~^:.".contains(b) {
                true
            } else {
                if let Some(num) = consecutive_hex_chars.as_mut() {
                    if b.is_ascii_hexdigit() {
                        *num += 1;
                    } else {
                        consecutive_hex_chars = None;
                    }
                }
                false
            }
        }) {
            if *b != b'.' || cursor.get(pos + 1) == Some(&b'.') {
                sep_pos = Some(ofs + pos);
                break;
            }
            ofs += pos + 1;
            cursor = &cursor[pos + 1..];
        }
    }

    let name = &input[..sep_pos.unwrap_or(input.len())].as_bstr();
    let mut sep = sep_pos.map(|pos| input[pos]);
    let mut has_ref_or_implied_name = name.is_empty();
    if name.is_empty() && sep == Some(b'@') && sep_pos.and_then(|pos| input.get(pos + 1)) != Some(&b'{') {
        delegate.find_ref("HEAD".into()).ok_or(Error::Delegate)?;
        sep_pos = sep_pos.map(|pos| pos + 1);
        sep = match sep_pos.and_then(|pos| input.get(pos).copied()) {
            None => return Ok("".into()),
            Some(pos) => Some(pos),
        };
    } else {
        (consecutive_hex_chars.unwrap_or(0) >= git_hash::Prefix::MIN_HEX_LEN)
            .then(|| try_set_prefix(delegate, name))
            .flatten()
            .or_else(|| {
                let prefix = long_describe_prefix(name).or_else(|| short_describe_prefix(name))?;
                try_set_prefix(delegate, prefix)
            })
            .or_else(|| {
                name.is_empty().then(|| ()).or_else(|| {
                    let res = delegate.find_ref(name)?;
                    has_ref_or_implied_name = true;
                    res.into()
                })
            })
            .ok_or(Error::Delegate)?;
    }

    input = {
        if let Some(b'@') = sep {
            let past_sep = input[sep_pos.map(|pos| pos + 1).unwrap_or(input.len())..].as_bstr();
            let (nav, rest, _consumed) = parens(past_sep)?.ok_or_else(|| Error::AtNeedsCurlyBrackets {
                input: input[sep_pos.unwrap_or(input.len())..].into(),
            })?;
            let nav = nav.as_ref();
            if let Some(n) = try_parse::<isize>(nav)? {
                if n < 0 {
                    if name.is_empty() {
                        delegate
                            .nth_checked_out_branch(n.abs().try_into().expect("non-negative isize fits usize"))
                            .ok_or(Error::Delegate)?;
                    } else {
                        return Err(Error::RefnameNeedsPositiveReflogEntries { nav: nav.into() });
                    }
                } else if has_ref_or_implied_name {
                    delegate
                        .reflog(delegate::ReflogLookup::Entry(
                            n.try_into().expect("non-negative isize fits usize"),
                        ))
                        .ok_or(Error::Delegate)?;
                } else {
                    return Err(Error::ReflogLookupNeedsRefName { name: (*name).into() });
                }
            } else if let Some(kind) = SiblingBranch::parse(nav) {
                if has_ref_or_implied_name {
                    delegate.sibling_branch(kind).ok_or(Error::Delegate)
                } else {
                    Err(Error::SiblingBranchNeedsBranchName { name: (*name).into() })
                }?
            } else if has_ref_or_implied_name {
                let time = git_date::parse(nav).ok_or_else(|| Error::Time { input: nav.into() })?;
                delegate
                    .reflog(delegate::ReflogLookup::Date(time))
                    .ok_or(Error::Delegate)?;
            } else {
                return Err(Error::ReflogLookupNeedsRefName { name: (*name).into() });
            }
            rest
        } else {
            input[sep_pos.unwrap_or(input.len())..].as_bstr()
        }
    };

    navigate(input, delegate)
}

fn navigate<'a>(input: &'a BStr, delegate: &mut impl Delegate) -> Result<&'a BStr, Error> {
    let mut cursor = 0;
    while let Some(b) = input.get(cursor) {
        cursor += 1;
        match *b {
            b'~' => todo!("~"),
            b'^' => {
                let past_sep = input.get(cursor..);
                if let Some((number, consumed)) = past_sep
                    .and_then(|past_sep| try_parse_usize(past_sep.as_bstr()).transpose())
                    .transpose()?
                {
                    if number == 0 {
                        delegate.peel_until(delegate::PeelTo::ObjectKind(git_object::Kind::Commit))
                    } else {
                        delegate.traverse(delegate::Traversal::NthParent(number))
                    }
                    .ok_or(Error::Delegate)?;
                    cursor += consumed;
                } else if let Some((kind, _rest, consumed)) =
                    past_sep.and_then(|past_sep| parens(past_sep).transpose()).transpose()?
                {
                    cursor += consumed;
                    let target = match kind.as_ref().as_ref() {
                        b"commit" => delegate::PeelTo::ObjectKind(git_object::Kind::Commit),
                        b"tag" => delegate::PeelTo::ObjectKind(git_object::Kind::Tag),
                        b"tree" => delegate::PeelTo::ObjectKind(git_object::Kind::Tree),
                        b"blob" => delegate::PeelTo::ObjectKind(git_object::Kind::Blob),
                        b"object" => delegate::PeelTo::ExistingObject,
                        b"" => delegate::PeelTo::RecursiveTagObject,
                        regex if regex.starts_with(b"/") => {
                            let (regex, negated) = match regex.strip_prefix(b"/!") {
                                Some(regex) if regex.get(0) == Some(&b'!') => (regex.as_bstr(), false),
                                Some(regex) if regex.get(0) == Some(&b'-') => (regex[1..].as_bstr(), true),
                                Some(_regex) => return Err(Error::UnspecifiedRegexModifier { regex: regex.into() }),
                                None => (regex[1..].as_bstr(), false),
                            };
                            if !regex.is_empty() {
                                delegate.find(regex, negated).ok_or(Error::Delegate)?;
                            }
                            continue;
                        }
                        invalid => return Err(Error::InvalidObject { input: invalid.into() }),
                    };
                    delegate.peel_until(target).ok_or(Error::Delegate)?;
                } else {
                    delegate
                        .traverse(delegate::Traversal::NthParent(1))
                        .ok_or(Error::Delegate)?;
                }
            }
            b':' => todo!(":"),
            _ => return Ok(input[cursor - 1..].as_bstr()),
        }
    }
    Ok("".into())
}

fn try_parse_usize(input: &BStr) -> Result<Option<(usize, usize)>, Error> {
    let mut bytes = input.iter().peekable();
    if bytes.peek().filter(|&&&b| b == b'-' || b == b'+').is_some() {
        return Err(Error::SignedNumber { input: input.into() });
    }
    let num_digits = bytes.take_while(|b| b.is_ascii_digit()).count();
    if num_digits == 0 {
        return Ok(None);
    }
    let number = try_parse(&input[..num_digits])?.expect("parse number if only digits");
    Ok(Some((number, num_digits)))
}

fn try_range(input: &BStr) -> Option<(&[u8], spec::Kind)> {
    input
        .strip_prefix(b"...")
        .map(|rest| (rest, spec::Kind::MergeBase))
        .or_else(|| input.strip_prefix(b"..").map(|rest| (rest, spec::Kind::Range)))
}

fn next(i: &BStr) -> (u8, &BStr) {
    let b = i[0];
    (b, i[1..].as_bstr())
}
