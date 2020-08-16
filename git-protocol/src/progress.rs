use bstr::BStr;
use nom::{
    bytes::complete::{take_till, take_till1},
    combinator::{map_res, opt},
    sequence::preceded,
};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Remote<'a> {
    pub action: &'a BStr,
    pub percent: Option<u32>,
    pub step: Option<usize>,
    pub max: Option<usize>,
}

impl<'a> Remote<'a> {
    pub fn from_bytes(line: &'a [u8]) -> Self {
        parse_progress(line)
            .ok()
            .and_then(|(_, r)| {
                if r.percent.is_none() && r.step.is_none() && r.max.is_none() {
                    None
                } else {
                    Some(r)
                }
            })
            .unwrap_or_else(|| Remote {
                action: line.into(),
                percent: None,
                step: None,
                max: None,
            })
    }
}

fn parse_number(i: &[u8]) -> nom::IResult<&[u8], usize> {
    map_res(take_till(|c: u8| !c.is_ascii_digit()), btoi::btoi)(i)
}

fn next_optional_number(i: &[u8]) -> nom::IResult<&[u8], Option<usize>> {
    opt(preceded(take_till(|c: u8| c.is_ascii_digit()), parse_number))(i)
}

fn parse_progress(line: &[u8]) -> nom::IResult<&[u8], Remote> {
    let (i, action) = take_till1(|c| c == b':')(line)?;
    let (i, step) = next_optional_number(i)?;
    let (i, max) = next_optional_number(i)?;
    Ok((
        i,
        Remote {
            action: action.into(),
            percent: None,
            step,
            max,
        },
    ))
}
