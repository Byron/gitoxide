use std::{
    convert::TryFrom,
    iter::{FromIterator, Peekable},
    ops::Range,
    str::FromStr,
};

use git_repository::bstr::ByteSlice;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_while, take_while_m_n},
    combinator::{all_consuming, map, map_res, opt},
    error::{FromExternalError, ParseError},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};
use pulldown_cmark::{CowStr, Event, OffsetIter, Tag};

use crate::{
    changelog,
    changelog::{
        section,
        section::{
            segment::{conventional::as_headline, Conventional},
            Segment,
        },
        Section,
    },
    ChangeLog,
};

impl ChangeLog {
    /// Obtain as much information as possible from `input` and keep everything we didn't understand in respective sections.
    pub fn from_markdown(input: &str) -> ChangeLog {
        let mut sections = Vec::new();
        let mut section_body = String::new();
        let mut previous_headline = None::<Headline>;
        let mut first_heading_level = None;
        for line in input.as_bytes().as_bstr().lines_with_terminator() {
            let line = line.to_str().expect("valid UTF-8");
            match Headline::try_from(line) {
                Ok(headline) => {
                    first_heading_level.get_or_insert(headline.level);
                    match previous_headline {
                        Some(mut headline) => {
                            headline.level = first_heading_level.expect("set first");
                            sections.push(Section::from_headline_and_body(
                                headline,
                                std::mem::take(&mut section_body),
                            ));
                        }
                        None => {
                            if !section_body.is_empty() {
                                sections.push(Section::Verbatim {
                                    text: std::mem::take(&mut section_body),
                                    generated: false,
                                })
                            }
                        }
                    };
                    previous_headline = Some(headline);
                }
                Err(()) => {
                    section_body.push_str(line);
                }
            }
        }

        match previous_headline {
            Some(headline) => {
                sections.push(Section::from_headline_and_body(
                    headline,
                    std::mem::take(&mut section_body),
                ));
            }
            None => sections.push(Section::Verbatim {
                text: section_body,
                generated: false,
            }),
        }

        let insert_sorted_at_pos = sections
            .first()
            .map(|s| match s {
                Section::Verbatim { .. } => 1,
                Section::Release { .. } => 0,
            })
            .unwrap_or(0);
        let mut non_release_sections = Vec::new();
        let mut release_sections = Vec::new();
        for section in sections {
            match section {
                Section::Verbatim { .. } => non_release_sections.push(section),
                Section::Release { .. } => release_sections.push(section),
            }
        }
        release_sections.sort_by(|lhs, rhs| match (lhs, rhs) {
            (Section::Release { name: lhs, .. }, Section::Release { name: rhs, .. }) => lhs.cmp(rhs).reverse(),
            _ => unreachable!("BUG: there are only release sections here"),
        });
        let mut sections = Vec::from_iter(non_release_sections.drain(..insert_sorted_at_pos));
        sections.append(&mut release_sections);
        sections.append(&mut non_release_sections);
        ChangeLog { sections }
    }
}

impl Section {
    fn from_headline_and_body(Headline { level, version, date }: Headline, body: String) -> Self {
        let mut events = pulldown_cmark::Parser::new_ext(&body, pulldown_cmark::Options::all())
            .into_offset_iter()
            .peekable();
        let mut unknown = String::new();
        let mut segments = Vec::new();

        let mut unknown_range = None;
        let mut removed_messages = Vec::new();
        while let Some((e, range)) = events.next() {
            match e {
                Event::Html(text) if text.starts_with(Section::UNKNOWN_TAG_START) => {
                    record_unknown_range(&mut segments, unknown_range.take(), &body);
                    for (event, _range) in events.by_ref().take_while(
                        |(e, _range)| !matches!(e, Event::Html(text) if text.starts_with(Section::UNKNOWN_TAG_END)),
                    ) {
                        track_unknown_event(event, &mut unknown);
                    }
                }
                Event::Html(text) if text.starts_with(section::segment::Conventional::REMOVED_HTML_PREFIX) => {
                    if let Some(id) = parse_message_id(text.as_ref()) {
                        removed_messages.push(id);
                    }
                }
                Event::Start(Tag::Heading(indent)) => {
                    record_unknown_range(&mut segments, unknown_range.take(), &body);
                    enum State {
                        ParseConventional { title: String },
                        SkipGenerated,
                        ConsiderUserAuthored,
                    }
                    let state = match events.next() {
                        Some((Event::Text(title), _range))
                            if title.starts_with(section::segment::ThanksClippy::TITLE) =>
                        {
                            segments.push(Segment::Clippy(section::Data::Parsed));
                            State::SkipGenerated
                        }
                        Some((Event::Text(title), _range))
                            if title.starts_with(section::segment::CommitStatistics::TITLE) =>
                        {
                            segments.push(Segment::Statistics(section::Data::Parsed));
                            State::SkipGenerated
                        }
                        Some((Event::Text(title), _range)) if title.starts_with(section::segment::Details::TITLE) => {
                            segments.push(Segment::Details(section::Data::Parsed));
                            State::SkipGenerated
                        }
                        Some((Event::Text(title), _range))
                            if title.starts_with(as_headline("feat").expect("valid"))
                                || title.starts_with(as_headline("add").expect("valid"))
                                || title.starts_with(as_headline("revert").expect("valid"))
                                || title.starts_with(as_headline("remove").expect("valid"))
                                || title.starts_with(as_headline("change").expect("valid"))
                                || title.starts_with(as_headline("docs").expect("valid"))
                                || title.starts_with(as_headline("perf").expect("valid"))
                                || title.starts_with(as_headline("fix").expect("valid")) =>
                        {
                            State::ParseConventional {
                                title: title.into_string(),
                            }
                        }
                        Some((_event, next_range)) => {
                            update_unknown_range(&mut unknown_range, range);
                            update_unknown_range(&mut unknown_range, next_range);
                            State::ConsiderUserAuthored
                        }
                        None => State::ConsiderUserAuthored,
                    };

                    events
                        .by_ref()
                        .take_while(|(e, range)| {
                            if matches!(state, State::ConsiderUserAuthored) {
                                update_unknown_range(&mut unknown_range, range.clone());
                            }
                            !matches!(e, Event::End(Tag::Heading(_)))
                        })
                        .count();
                    match state {
                        State::ParseConventional { title } => {
                            segments.push(parse_conventional_to_next_section_title(
                                &body,
                                title,
                                &mut events,
                                indent,
                                &mut unknown,
                            ));
                        }
                        State::SkipGenerated => {
                            skip_to_next_section_title(&mut events, indent);
                        }
                        State::ConsiderUserAuthored => {}
                    }
                }
                _unknown_event => update_unknown_range(&mut unknown_range, range),
            };
        }
        record_unknown_range(&mut segments, unknown_range.take(), &body);
        Section::Release {
            name: match version {
                Some(version) => changelog::Version::Semantic(version),
                None => changelog::Version::Unreleased,
            },
            date,
            removed_messages,
            heading_level: level,
            segments,
            unknown,
        }
    }
}

fn parse_conventional_to_next_section_title(
    markdown: &str,
    title: String,
    events: &mut Peekable<OffsetIter<'_>>,
    level: u32,
    unknown: &mut String,
) -> Segment {
    let is_breaking = title.ends_with(section::segment::Conventional::BREAKING_TITLE);
    let kind = ["fix", "add", "feat", "revert", "remove", "change", "docs", "perf"]
        .iter()
        .find(|kind| title.starts_with(section::segment::conventional::as_headline(*kind).expect("valid")))
        .expect("BUG: this list needs an update too if new kinds of conventional messages are added");

    let mut conventional = section::segment::Conventional {
        kind: *kind,
        is_breaking,
        removed: vec![],
        messages: vec![],
    };
    while let Some((event, _range)) = events.peek() {
        match event {
            Event::Start(Tag::Heading(indent)) if *indent == level => break,
            _ => {
                let (event, _range) = events.next().expect("peeked before so event is present");
                match event {
                    Event::Html(ref tag) => match parse_message_id(tag.as_ref()) {
                        Some(id) => conventional.removed.push(id),
                        None => track_unknown_event(event, unknown),
                    },
                    Event::Start(Tag::List(_)) => {
                        while let Some((event, item_range)) = events.next() {
                            match event {
                                Event::Start(Tag::Item) => {
                                    if let Some((possibly_html, _)) = events.next() {
                                        match possibly_html {
                                            Event::Start(Tag::Paragraph) => {
                                                if let Some((possibly_html, _)) = events.next() {
                                                    match possibly_html {
                                                        Event::Html(tag) => {
                                                            parse_id_fallback_to_user_message(
                                                                markdown,
                                                                events,
                                                                &mut conventional,
                                                                item_range,
                                                                tag,
                                                            );
                                                        }
                                                        _other_event => make_user_message_and_consume_item(
                                                            markdown,
                                                            events,
                                                            &mut conventional,
                                                            item_range,
                                                        ),
                                                    }
                                                }
                                            }
                                            Event::Html(tag) => {
                                                parse_id_fallback_to_user_message(
                                                    markdown,
                                                    events,
                                                    &mut conventional,
                                                    item_range,
                                                    tag,
                                                );
                                            }
                                            _other_event => make_user_message_and_consume_item(
                                                markdown,
                                                events,
                                                &mut conventional,
                                                item_range,
                                            ),
                                        }
                                    }
                                }
                                Event::End(Tag::List(_)) => break,
                                event => track_unknown_event(event, unknown),
                            }
                        }
                    }
                    event => track_unknown_event(event, unknown),
                }
                continue;
            }
        }
    }
    section::Segment::Conventional(conventional)
}

fn parse_id_fallback_to_user_message(
    markdown: &str,
    events: &mut Peekable<OffsetIter<'_>>,
    mut conventional: &mut Conventional,
    item_range: Range<usize>,
    tag: CowStr<'_>,
) {
    match parse_message_id(tag.as_ref()) {
        Some(id) => {
            let mut events = events
                .by_ref()
                .take_while(|(e, _r)| !matches!(e, Event::End(Tag::Item)))
                .map(|(_, r)| r);
            let start = events.next();
            let end = events.last();
            if let Some((start, end)) = start.map(|r| r.start).and_then(|start| end.map(|r| (start, r.end))) {
                conventional
                    .messages
                    .push(section::segment::conventional::Message::Generated {
                        id,
                        title: markdown[start..end].trim().to_owned(),
                    })
            }
        }
        None => make_user_message_and_consume_item(markdown, events, &mut conventional, item_range),
    };
}

fn make_user_message_and_consume_item(
    markdown: &str,
    events: &mut Peekable<OffsetIter<'_>>,
    conventional: &mut Conventional,
    range: Range<usize>,
) {
    conventional
        .messages
        .push(section::segment::conventional::Message::User {
            markdown: markdown[range].trim_end().to_owned(),
        });
    events.take_while(|(e, _)| !matches!(e, Event::End(Tag::Item))).count();
}

fn parse_message_id(html: &str) -> Option<git_repository::hash::ObjectId> {
    let html = html.strip_prefix(section::segment::Conventional::REMOVED_HTML_PREFIX)?;
    let end_of_hex = html.find(|c| {
        !matches!(c,
            'a'..='f' | '0'..='9'
        )
    })?;
    git_repository::hash::ObjectId::from_hex(html[..end_of_hex].as_bytes()).ok()
}

fn update_unknown_range(target: &mut Option<Range<usize>>, source: Range<usize>) {
    match target {
        Some(range_thus_far) => {
            if source.end > range_thus_far.end {
                range_thus_far.end = source.end;
            }
        }
        None => *target = source.into(),
    }
}

fn record_unknown_range(out: &mut Vec<section::Segment>, range: Option<Range<usize>>, markdown: &str) {
    if let Some(range) = range {
        out.push(Segment::User {
            markdown: markdown[range].to_owned(),
        })
    }
}

fn track_unknown_event(unknown_event: Event<'_>, unknown: &mut String) {
    log::trace!("Cannot handle {:?}", unknown_event);
    match unknown_event {
        Event::Html(text)
        | Event::Code(text)
        | Event::Text(text)
        | Event::FootnoteReference(text)
        | Event::Start(Tag::FootnoteDefinition(text))
        | Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(text)))
        | Event::Start(Tag::Link(_, text, _))
        | Event::Start(Tag::Image(_, text, _)) => unknown.push_str(text.as_ref()),
        _ => {}
    }
}

fn skip_to_next_section_title(events: &mut Peekable<OffsetIter<'_>>, level: u32) {
    while let Some((event, _range)) = events.peek() {
        match event {
            Event::Start(Tag::Heading(indent)) if *indent == level => break,
            _ => {
                events.next();
                continue;
            }
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
