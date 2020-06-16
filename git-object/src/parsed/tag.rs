use super::{
    util::{parse_timezone_offset, split2_at_space},
    Error,
};
use crate::{parsed::Signature, Sign, Time};
use bstr::{BStr, ByteSlice};
use btoi::btoi;
use hex::FromHex;
use nom::bytes::complete::{is_not, take_till};
use nom::sequence::delimited;
use nom::{
    branch::alt,
    bytes::complete::take,
    bytes::complete::{tag, take_until, take_while1, take_while_m_n},
    character::{is_alphabetic, is_digit},
    combinator::opt,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Tag<'data> {
    pub target: &'data BStr,
    pub name: &'data BStr,
    pub target_kind: crate::Kind,
    pub message: &'data BStr,
    pub signature: Signature<'data>,
    pub pgp_signature: Option<&'data BStr>,
}

fn parse_signature(d: &[u8]) -> Result<Signature, Error> {
    const ONE_SPACE: usize = 1;
    let email_begin = d
        .iter()
        .position(|&b| b == b'<')
        .ok_or_else(|| {
            Error::ParseError(
                "Could not find beginning of email marked by '<'",
                d.to_owned(),
            )
        })
        .and_then(|pos| {
            if pos == 0 {
                Err(Error::ParseError(
                    "Email found in place of author name",
                    d.to_owned(),
                ))
            } else {
                Ok(pos)
            }
        })?;
    let email_end = email_begin
        + d.iter()
            .skip(email_begin)
            .position(|&b| b == b'>')
            .ok_or_else(|| {
                Error::ParseError("Could not find end of email marked by '>'", d.to_owned())
            })
            .and_then(|pos| {
                if pos >= d.len() - 1 - ONE_SPACE {
                    Err(Error::ParseError(
                        "There is no time after email",
                        d.to_owned(),
                    ))
                } else {
                    Ok(pos)
                }
            })?;
    let (time_in_seconds, tzofz) = split2_at_space(&d[email_end + ONE_SPACE + 1..], |_, _| true)?;
    let (offset, sign) = parse_timezone_offset(tzofz)?;

    Ok(Signature {
        name: (&d[..email_begin - ONE_SPACE]).as_bstr(),
        email: (&d[email_begin + 1..email_end]).as_bstr(),
        time: Time {
            time: btoi::<u32>(time_in_seconds).map_err(|e| {
                Error::ParseIntegerError("Could parse to seconds", time_in_seconds.to_owned(), e)
            })?,
            offset,
            sign,
        },
    })
}

fn is_hex_digit_lc(b: u8) -> bool {
    match b {
        b'0'..=b'9' => true,
        b'a'..=b'f' => true,
        _ => false,
    }
}

const NL: &[u8] = b"\n";
const SPACE: &[u8] = b" ";
pub(crate) fn parse_signature_nom(i: &[u8]) -> IResult<&[u8], Signature, Error> {
    let (i, (name, email, time_in_seconds, tzsign, tzhour, tzminute)) = tuple((
        terminated(take_until(&b" <"[..]), take(2usize)),
        terminated(take_until(&b"> "[..]), take(2usize)),
        terminated(take_until(SPACE), take(1usize)),
        alt((tag(b"-"), tag(b"+"))),
        take_while_m_n(2usize, 2, |b| is_digit(b)),
        take_while_m_n(2usize, 2, |b| is_digit(b)),
    ))(i)
    .map_err(Error::context(
        "tagger <name> <<email>> <time seconds since epoch> <+|-><HHMM>",
    ))?;

    let sign = if tzsign[0] == b'-' {
        Sign::Minus
    } else {
        Sign::Plus
    };
    let hours = btoi::<i32>(&tzhour).map_err(|e| {
        nom::Err::Error(Error::ParseIntegerError(
            "invalid 'hours' string",
            tzhour.to_owned(),
            e,
        ))
    })?;
    let minutes = btoi::<i32>(&tzminute).map_err(|e| {
        nom::Err::Error(Error::ParseIntegerError(
            "invalid 'minutes' string",
            tzminute.to_owned(),
            e,
        ))
    })?;
    let offset = hours * 3600 + minutes * 60;

    Ok((
        i,
        Signature {
            name: name.as_bstr(),
            email: email.as_bstr(),
            time: Time {
                time: btoi::<u32>(time_in_seconds).map_err(|e| {
                    nom::Err::Error(Error::ParseIntegerError(
                        "Could parse to seconds",
                        time_in_seconds.to_owned(),
                        e,
                    ))
                })?,
                offset,
                sign,
            },
        },
    ))
}

pub(crate) fn parse_tag_nom(i: &[u8]) -> IResult<&[u8], Tag, Error> {
    let (i, target) = terminated(
        preceded(
            tag(b"object "),
            take_while_m_n(40usize, 40, is_hex_digit_lc),
        ),
        tag(NL),
    )(i)
    .map_err(Error::context("object <40 lowercase hex char>"))?;

    let (i, kind) = terminated(preceded(tag(b"type "), take_while1(is_alphabetic)), tag(NL))(i)
        .map_err(Error::context("type <object kind>"))?;
    let kind =
        crate::Kind::from_bytes(kind).map_err(|e| nom::Err::Error(Error::ParseKindError(e)))?;

    let (i, tag_version) =
        terminated(preceded(tag(b"tag "), take_while1(|b| b != NL[0])), tag(NL))(i)
            .map_err(Error::context("tag <version>"))?;

    let (i, signature) = terminated(preceded(tag(b"tagger "), parse_signature_nom), tag(NL))(i)
        .map_err(Error::context("tagger <signature>"))?;
    let (i, (message, pgp_signature)) = parse_message_nom(i)?;
    Ok((
        i,
        Tag {
            target: target.as_bstr(),
            name: tag_version.as_bstr(),
            target_kind: kind,
            message,
            signature,
            pgp_signature,
        },
    ))
}

pub(crate) fn parse_message_nom(i: &[u8]) -> IResult<&[u8], (&BStr, Option<&BStr>), Error> {
    let NLNL: &[u8] = b"\n\n";
    const PGP_SIGNATURE_BEGIN: &[u8] = b"-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

    let (i, _) = tag(NL)(i)?;
    if i.is_empty() {
        return Err(nom::Err::Error(Error::Nom(i.into(), "Missing tag message")));
    }
    fn all_but_trailing_newline(i: &[u8]) -> IResult<&[u8], (&[u8], &[u8]), Error> {
        if i.len() < 2 {
            return Err(nom::Err::Error(Error::Nom(
                i.into(),
                "tag message is missing",
            )));
        }
        tag(NL)(&i[i.len() - 1..]).map_err(Error::context("tag message must end with newline"))?;
        Ok((&[], (&i[..i.len() - 1], &[])))
    }
    let (i, (message, signature)) = alt((
        tuple((
            take_until(PGP_SIGNATURE_BEGIN),
            delimited(
                tag(PGP_SIGNATURE_BEGIN),
                take_until(PGP_SIGNATURE_END),
                tag(PGP_SIGNATURE_END),
            ),
        )),
        all_but_trailing_newline,
    ))(i)?;
    Ok((
        i,
        (
            message.as_bstr(),
            if signature.is_empty() {
                None
            } else {
                Some(signature.as_bstr())
            },
        ),
    ))
}

fn parse_message<'data>(
    d: &'data [u8],
    mut lines: impl Iterator<Item = &'data [u8]>,
) -> Result<(&'data BStr, Option<&'data BStr>), Error> {
    const PGP_SIGNATURE_BEGIN: &[u8] = b"-----BEGIN PGP SIGNATURE-----";
    const PGP_SIGNATURE_END: &[u8] = b"-----END PGP SIGNATURE-----";

    Ok(match lines.next() {
        Some(l) if l.is_empty() => {
            let msg_begin = 0; // TODO: use nom to parse this or do it without needing nightly
            if msg_begin >= d.len() {
                return Err(Error::ParseError(
                    "Message separator was not followed by message",
                    d.to_owned(),
                ));
            }
            let mut msg_end = d.len();
            let mut pgp_signature = None;
            if let Some(_pgp_begin_line) = lines.find(|l| l.starts_with(PGP_SIGNATURE_BEGIN)) {
                match lines.find(|l| l.starts_with(PGP_SIGNATURE_END)) {
                    None => {
                        return Err(Error::ParseError(
                            "Didn't find end of signature marker",
                            d.to_owned(),
                        ))
                    }
                    Some(_) => {
                        msg_end = d.len(); // TODO: use nom to parse this or do it without needing nightly
                        pgp_signature = Some((&d[msg_end..]).as_bstr())
                    }
                }
            }
            ((&d[msg_begin..msg_end]).as_bstr(), pgp_signature)
        }
        Some(l) => {
            return Err(Error::ParseError(
                "Expected empty newline to separate message",
                l.to_owned(),
            ))
        }
        None => (b"".as_bstr(), None),
    })
}

impl<'data> Tag<'data> {
    pub fn target(&self) -> crate::Id {
        <[u8; 20]>::from_hex(self.target).expect("prior validation")
    }
    pub fn from_bytes(d: &'data [u8]) -> Result<Tag<'data>, Error> {
        let mut lines = d.split(|&b| b == b'\n');
        let (target, target_kind, name, signature) =
            match (lines.next(), lines.next(), lines.next(), lines.next()) {
                (Some(target), Some(kind), Some(name), Some(tagger)) => {
                    let (_, target) = split2_at_space(target, |f, v| {
                        f == b"object" && v.len() == 40 && <[u8; 20]>::from_hex(v).is_ok()
                    })?;
                    let kind = split2_at_space(kind, |f, _v| f == b"type")
                        .and_then(|(_, kind)| crate::Kind::from_bytes(kind).map_err(Into::into))?;
                    let (_, name) = split2_at_space(name, |f, _v| f == b"tag")?;
                    let (_, rest) = split2_at_space(tagger, |f, _v| f == b"tagger")?;
                    (
                        target.as_bstr(),
                        kind,
                        name.as_bstr(),
                        parse_signature(rest)?,
                    )
                }
                _ => {
                    return Err(Error::ParseError(
                        "Expected four lines: target, type, tag and tagger",
                        d.to_owned(),
                    ))
                }
            };

        let (message, pgp_signature) = parse_message(d, &mut lines)?;

        Ok(Tag {
            target,
            name,
            target_kind,
            message,
            signature,
            pgp_signature,
        })
    }
}
