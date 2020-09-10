use bstr::ByteSlice;
use nom::{
    bytes::complete::{tag, take_till, take_till1},
    combinator::{map_res, opt},
    sequence::{preceded, terminated},
};
use std::convert::TryFrom;

/// The information usually found in remote progress messages as sent by a git server during
/// fetch, clone and push.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoteProgress<'a> {
    #[cfg_attr(feature = "serde1", serde(borrow))]
    pub action: &'a bstr::BStr,
    pub percent: Option<u32>,
    pub step: Option<usize>,
    pub max: Option<usize>,
}

impl<'a> RemoteProgress<'a> {
    pub fn from_bytes(line: &[u8]) -> Option<RemoteProgress> {
        parse_progress(line).ok().and_then(|(_, r)| {
            if r.percent.is_none() && r.step.is_none() && r.max.is_none() {
                None
            } else {
                Some(r)
            }
        })
    }

    pub fn translate_to_progress(is_error: bool, text: &[u8], progress: &mut impl git_features::progress::Progress) {
        fn progress_name(current: Option<String>, action: &[u8]) -> String {
            match current {
                Some(current) => format!(
                    "{}: {}",
                    current.splitn(2, ':').next().expect("token"),
                    action.as_bstr()
                ),
                None => action.as_bstr().to_string(),
            }
        }
        if is_error {
            // ignore keep-alive packages sent with 'sideband-all'
            if !text.is_empty() {
                progress.fail(progress_name(None, text));
            }
        } else {
            match Self::from_bytes(text) {
                Some(RemoteProgress {
                    action,
                    percent: _,
                    step,
                    max,
                }) => {
                    progress.set_name(progress_name(progress.name(), action));
                    progress.init(max, git_features::progress::count("objects"));
                    if let Some(step) = step {
                        progress.set(step);
                    }
                }
                None => progress.set_name(progress_name(progress.name(), text)),
            };
        }
    }
}

fn parse_number(i: &[u8]) -> nom::IResult<&[u8], usize> {
    map_res(take_till(|c: u8| !c.is_ascii_digit()), btoi::btoi)(i)
}

fn next_optional_percentage(i: &[u8]) -> nom::IResult<&[u8], Option<u32>> {
    opt(terminated(
        preceded(
            take_till(|c: u8| c.is_ascii_digit()),
            map_res(parse_number, u32::try_from),
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
