use std::{convert::TryFrom, str::FromStr};

use git_repository::bstr::ByteSlice;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_while, take_while_m_n},
    combinator::{all_consuming, map, map_res, opt},
    error::{FromExternalError, ParseError},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use crate::{changelog, ChangeLog};

impl ChangeLog {
    /// Obtain as much information as possible from `input` and keep everything we didn't understand in respective sections.
    pub fn from_markdown(input: &str) -> ChangeLog {
        let mut sections = Vec::new();
        let mut plain_text = String::new();
        let mut previous_headline = None;
        for line in input.as_bytes().as_bstr().lines_with_terminator() {
            let line = line.to_str().expect("valid UTF-8");
            match Headline::try_from(line) {
                Ok(headline) => {
                    match previous_headline {
                        Some(headline) => {
                            sections.push(changelog::Section::from_headline_and_body(headline, &mut plain_text));
                        }
                        None => sections.push(changelog::Section::Verbatim {
                            text: std::mem::take(&mut plain_text),
                            generated: false,
                        }),
                    };
                    previous_headline = Some(headline);
                }
                Err(()) => {
                    plain_text.push_str(line);
                }
            }
        }

        match previous_headline {
            Some(headline) => {
                sections.push(changelog::Section::from_headline_and_body(headline, &mut plain_text));
            }
            None => sections.push(changelog::Section::Verbatim {
                text: plain_text,
                generated: false,
            }),
        }
        ChangeLog { sections }
    }
}

impl changelog::Section {
    fn from_headline_and_body(Headline { level, version, date }: Headline, body: &mut String) -> Self {
        // TODO: parse body
        body.clear();
        changelog::Section::Release {
            name: match version {
                Some(version) => changelog::Version::Semantic(version),
                None => changelog::Version::Unreleased,
            },
            date,
            heading_level: level,
        }
    }
}

struct Headline {
    level: usize,
    version: Option<semver::Version>,
    date: Option<time::OffsetDateTime>,
}

impl<'a> TryFrom<&'a str> for Headline {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        all_consuming(headline::<()>)(value).finish().map(|(_, h)| h)
    }
}

fn headline<'a, E: ParseError<&'a str> + FromExternalError<&'a str, ()>>(i: &'a str) -> IResult<&'a str, Headline, E> {
    let hashes = take_while(|c: char| c == '#');
    let greedy_whitespace = |i| take_while(|c: char| c.is_whitespace())(i);
    let take_n_digits = |n: usize| {
        map_res(take_while_m_n(n, n, |c: char| c.is_digit(10)), |num| {
            u32::from_str(num).map_err(|_| ())
        })
    };
    map(
        terminated(
            tuple((
                separated_pair(
                    hashes,
                    greedy_whitespace,
                    alt((
                        preceded(
                            tag("v"),
                            map_res(take_till(|c: char| c.is_whitespace()), |v| {
                                semver::Version::parse(v).map_err(|_| ()).map(Some)
                            }),
                        ),
                        map(tag_no_case("unreleased"), |_| None),
                    )),
                ),
                opt(preceded(
                    greedy_whitespace,
                    delimited(
                        tag("("),
                        map_res(
                            tuple((take_n_digits(4), tag("-"), take_n_digits(2), tag("-"), take_n_digits(2))),
                            |(year, _, month, _, day)| {
                                time::Month::try_from(month as u8).map_err(|_| ()).and_then(|month| {
                                    time::Date::from_calendar_date(year as i32, month, day as u8)
                                        .map_err(|_| ())
                                        .map(|d| d.midnight().assume_utc())
                                })
                            },
                        ),
                        tag(")"),
                    ),
                )),
            )),
            greedy_whitespace,
        ),
        |((hashes, version), date)| Headline {
            level: hashes.len(),
            version,
            date,
        },
    )(i)
}
