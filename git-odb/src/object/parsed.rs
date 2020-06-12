use crate::{
    object::{Id, Kind},
    Sign, Time,
};
use failure::{err_msg, Error, ResultExt};
use hex::FromHex;
use std::str;

const PGP_SIGNATURE_BEGIN: &'static [u8] = b"-----BEGIN PGP SIGNATURE-----";
const PGP_SIGNATURE_END: &'static [u8] = b"-----END PGP SIGNATURE-----";

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Object<'data> {
    Tag(Tag<'data>),
}

impl<'data> Object<'data> {
    pub fn kind(&self) -> Kind {
        match self {
            Object::Tag(_) => Kind::Tag,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Signature<'data> {
    pub name: &'data [u8],
    pub email: &'data [u8],
    pub time: Time,
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Tag<'data> {
    pub target_raw: &'data [u8],
    pub name_raw: &'data [u8],
    pub target_kind: Kind,
    pub message: Option<&'data [u8]>,
    pub pgp_signature: Option<&'data [u8]>,
    pub signature: Signature<'data>,
}

fn split2_at_space(
    d: &[u8],
    v: impl FnOnce(&[u8], &[u8]) -> bool,
) -> Result<(&[u8], &[u8]), Error> {
    let mut t = d.splitn(2, |&b| b == b' ');
    Ok(match (t.next(), t.next()) {
        (Some(t1), Some(t2)) => {
            if !v(t1, t2) {
                bail!("Tokens in {:?} are invalid", str::from_utf8(d))
            }
            (t1, t2)
        }
        _ => bail!(
            "didnt find two tokens separated by space in {:?}'",
            str::from_utf8(d)
        ),
    })
}

fn parse_timezone_offset(d: &str) -> Result<(i32, Sign), Error> {
    let db = d.as_bytes();
    if d.len() < 5 || !(db[0] == b'+' || db[0] == b'-') {
        bail!("Invalid timezone offset: '{}'", d);
    }
    let sign = if db[0] == b'-' {
        Sign::Minus
    } else {
        Sign::Plus
    };
    let hours = str::from_utf8(&db[..3])
        .expect("valid utf8")
        .parse::<i32>()?;
    let minutes = str::from_utf8(&db[3..])
        .expect("valid utf8")
        .parse::<i32>()?;
    Ok((hours * 3600 + minutes * 60, sign))
}

fn parse_signature(d: &[u8]) -> Result<Signature, Error> {
    const ONE_SPACE: usize = 1;
    let email_begin = d
        .iter()
        .position(|&b| b == b'<')
        .ok_or_else(|| err_msg("Could not find beginning of email marked by '<'"))
        .and_then(|pos| {
            if pos == 0 {
                Err(err_msg("Email found in place of author name"))
            } else {
                Ok(pos)
            }
        })?;
    let email_end = email_begin
        + d.iter()
            .skip(email_begin)
            .position(|&b| b == b'>')
            .ok_or_else(|| err_msg("Could not find end of email marked by '>'"))
            .and_then(|pos| {
                if pos >= d.len() - 1 - ONE_SPACE {
                    Err(err_msg("There is no time after email"))
                } else {
                    Ok(pos)
                }
            })?;
    let (time_in_seconds, tzofz) = split2_at_space(&d[email_end + ONE_SPACE + 1..], |_, _| true)
        .and_then(|(t1, t2)| {
            str::from_utf8(t1)
                .map_err(Into::into)
                .and_then(|t1| str::from_utf8(t2).map_err(Into::into).map(|t2| (t1, t2)))
        })
        .with_context(|_| "Could not convert time to utf8, even though it should be ascii")?;
    let (offset, sign) = parse_timezone_offset(tzofz)
        .with_context(|_| format!("Timezone parsing failed for '{}'", tzofz))?;

    Ok(Signature {
        name: &d[..email_begin - ONE_SPACE],
        email: &d[email_begin + 1..email_end],
        time: Time {
            time: time_in_seconds
                .parse::<u32>()
                .with_context(|_| format!("Could not parse '{}' to seconds", time_in_seconds))?,
            offset,
            sign,
        },
    })
}

fn parse_message<'data>(
    d: &'data [u8],
    mut lines: impl Iterator<Item = &'data [u8]>,
) -> Result<(Option<&'data [u8]>, Option<&'data [u8]>), Error> {
    Ok(match lines.next() {
        Some(l) if l.len() == 0 => {
            let msg_begin = 0; // TODO: use nom to parse this or do it without needing nightly
            if msg_begin >= d.len() {
                bail!("Message separator was not followed by message")
            }
            let mut msg_end = d.len();
            let mut pgp_signature = None;
            if let Some(_pgp_begin_line) = lines.find(|l| l.starts_with(PGP_SIGNATURE_BEGIN)) {
                match lines.find(|l| l.starts_with(PGP_SIGNATURE_END)) {
                    None => bail!("Didn't find end of signature marker"),
                    Some(_) => {
                        msg_end = d.len(); // TODO: use nom to parse this or do it without needing nightly
                        pgp_signature = Some(&d[msg_end..])
                    }
                }
            }
            (Some(&d[msg_begin..msg_end]), pgp_signature)
        }
        Some(l) => bail!(
            "Expected empty newline to separate message, got {:?}",
            str::from_utf8(l),
        ),
        None => (None, None),
    })
}

impl<'data> Tag<'data> {
    pub fn target(&self) -> Id {
        <[u8; 20]>::from_hex(self.target_raw).expect("prior validation")
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
                        .and_then(|(_, kind)| Kind::from_bytes(kind))?;
                    let (_, name) = split2_at_space(name, |f, _v| f == b"tag")?;
                    let (_, tagger) = split2_at_space(tagger, |f, _v| f == b"tagger")?;
                    (target, kind, name, parse_signature(tagger)?)
                }
                _ => bail!("Expected four lines: target, type, tag and tagger"),
            };

        let (message, pgp_signature) = parse_message(d, &mut lines)?;

        Ok(Tag {
            target_raw: target,
            name_raw: name,
            target_kind,
            message,
            signature,
            pgp_signature,
        })
    }
}
