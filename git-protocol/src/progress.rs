use bstr::BStr;
use nom::{
    bytes::complete::{tag, take_till, take_till1},
    combinator::{map_res, opt},
    sequence::{preceded, terminated},
};
use std::convert::TryFrom;

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoteProgress<'a> {
    pub action: &'a BStr,
    pub percent: Option<u32>,
    pub step: Option<usize>,
    pub max: Option<usize>,
}

impl<'a> RemoteProgress<'a> {
    pub fn from_bytes(line: &'a [u8]) -> Option<Self> {
        parse_progress(line).ok().and_then(|(_, r)| {
            if r.percent.is_none() && r.step.is_none() && r.max.is_none() {
                None
            } else {
                Some(r)
            }
        })
    }
}

fn parse_number(i: &[u8]) -> nom::IResult<&[u8], usize> {
    map_res(take_till(|c: u8| !c.is_ascii_digit()), btoi::btoi)(i)
}

fn next_optional_percentage(i: &[u8]) -> nom::IResult<&[u8], Option<u32>> {
    opt(terminated(
        preceded(
            take_till(|c: u8| c.is_ascii_digit()),
            map_res(parse_number, |num| u32::try_from(num)),
        ),
        tag(b"%"),
    ))(i)
}

fn next_optional_number(i: &[u8]) -> nom::IResult<&[u8], Option<usize>> {
    opt(preceded(take_till(|c: u8| c.is_ascii_digit()), parse_number))(i)
}

fn parse_progress(line: &[u8]) -> nom::IResult<&[u8], RemoteProgress> {
    let (i, action) = take_till1(|c| c == b':')(line)?;
    let (i, percent) = next_optional_percentage(i)?;
    let (i, step) = next_optional_number(i)?;
    let (i, max) = next_optional_number(i)?;
    Ok((
        i,
        RemoteProgress {
            action: action.into(),
            percent,
            step,
            max,
        },
    ))
}
