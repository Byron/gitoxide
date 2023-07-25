use std::{
    convert::TryFrom,
    iter::{FromIterator, Peekable},
    ops::Range,
    str::FromStr,
};

use gix::bstr::ByteSlice;
use pulldown_cmark::{CowStr, Event, HeadingLevel, OffsetIter, Tag};
use winnow::{
    combinator::alt,
    combinator::opt,
    combinator::{delimited, preceded, separated_pair, terminated},
    error::{FromExternalError, ParserError},
    prelude::*,
    token::{tag_no_case, take_till0, take_while},
};

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

        let insert_sorted_at_pos = sections.first().map_or(0, |s| match s {
            Section::Verbatim { .. } => 1,
            Section::Release { .. } => 0,
        });
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
    fn from_headline_and_body(
        Headline {
            level,
            version_prefix,
            version,
            date,
        }: Headline,
        body: String,
    ) -> Self {
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
                        if !removed_messages.contains(&id) {
                            removed_messages.push(id);
                        }
                    }
                }
                Event::Start(Tag::Heading(indent, _, _)) => {
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
                                || title.starts_with("refactor")
                                || title.starts_with("other")
                                || title.starts_with("style")
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
                            !matches!(e, Event::End(Tag::Heading(_, _, _)))
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
            version_prefix,
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
    events: &mut Peekable<OffsetIter<'_, '_>>,
    level: HeadingLevel,
    unknown: &mut String,
) -> Segment {
    let is_breaking = title.ends_with(section::segment::Conventional::BREAKING_TITLE_ENCLOSED);
    let kind = [
        "fix", "add", "feat", "revert", "remove", "change", "docs", "perf", "refactor", "other", "style",
    ]
    .iter()
    .find(|kind| {
        let headline = section::segment::conventional::as_headline(kind).unwrap_or(*kind);
        let common_len = headline.len().min(title.len());
        title
            .get(..common_len)
            .and_then(|t| headline.get(..common_len).map(|h| t.eq_ignore_ascii_case(h)))
            .unwrap_or(false)
    })
    .expect("BUG: this list needs an update too if new kinds of conventional messages are added");

    let mut conventional = section::segment::Conventional {
        kind,
        is_breaking,
        removed: vec![],
        messages: vec![],
    };
    while let Some((event, _range)) = events.peek() {
        match event {
            Event::Start(Tag::Heading(indent, _, _)) if *indent == level => break,
            _ => {
                let (event, _range) = events.next().expect("peeked before so event is present");
                match event {
                    Event::Html(ref tag) => match parse_message_id(tag.as_ref()) {
                        Some(id) => {
                            if !conventional.removed.contains(&id) {
                                conventional.removed.push(id)
                            }
                        }
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
    events: &mut Peekable<OffsetIter<'_, '_>>,
    conventional: &mut Conventional,
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
            let end = events.last().or_else(|| start.clone());
            if let Some(title_and_body) = start
                .map(|r| r.start)
                .and_then(|start| end.map(|r| markdown[start..r.end].trim()))
            {
                let mut lines = title_and_body
                    .as_bytes()
                    .as_bstr()
                    .lines_with_terminator()
                    .map(|b| b.to_str().expect("always valid as source is UTF-8"));
                conventional
                    .messages
                    .push(section::segment::conventional::Message::Generated {
                        id,
                        title: lines.next().map_or("", |l| l.trim()).to_owned(),
                        body: lines
                            .map(|l| {
                                match l
                                    .chars()
                                    .take_while(|c| *c == ' ' || *c == '\t')
                                    .enumerate()
                                    .map(|(idx, _)| idx)
                                    .last()
                                {
                                    Some(last_pos_to_truncate) => &l[last_pos_to_truncate + 1..],
                                    None => l,
                                }
                            })
                            .fold(None::<String>, |mut acc, l| {
                                acc.get_or_insert_with(String::new).push_str(l);
                                acc
                            }),
                    });
            }
        }
        None => make_user_message_and_consume_item(markdown, events, conventional, item_range),
    };
}

fn make_user_message_and_consume_item(
    markdown: &str,
    events: &mut Peekable<OffsetIter<'_, '_>>,
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

fn parse_message_id(html: &str) -> Option<gix::hash::ObjectId> {
    let html = html.strip_prefix(section::segment::Conventional::REMOVED_HTML_PREFIX)?;
    let end_of_hex = html.find(|c| {
        !matches!(c,
            'a'..='f' | '0'..='9'
        )
    })?;
    gix::hash::ObjectId::from_hex(html[..end_of_hex].as_bytes()).ok()
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
        | Event::Start(
            Tag::FootnoteDefinition(text)
            | Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(text))
            | Tag::Link(_, text, _)
            | Tag::Image(_, text, _),
        ) => unknown.push_str(text.as_ref()),
        _ => {}
    }
}

fn skip_to_next_section_title(events: &mut Peekable<OffsetIter<'_, '_>>, level: HeadingLevel) {
    while let Some((event, _range)) = events.peek() {
        match event {
            Event::Start(Tag::Heading(indent, _, _)) if *indent == level => break,
            _ => {
                events.next();
                continue;
            }
        }
    }
}

struct Headline {
    level: usize,
    version_prefix: String,
    version: Option<semver::Version>,
    date: Option<time::OffsetDateTime>,
}

impl<'a> TryFrom<&'a str> for Headline {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        headline::<()>.parse(value).map_err(|err| err.into_inner())
    }
}

fn headline<'a, E: ParserError<&'a str> + FromExternalError<&'a str, ()>>(i: &mut &'a str) -> PResult<Headline, E> {
    let hashes = take_while(0.., |c: char| c == '#');
    let greedy_whitespace = |i: &mut &'a str| take_while(0.., char::is_whitespace).parse_next(i);
    let take_n_digits =
        |n: usize| take_while(n, |c: char| c.is_ascii_digit()).try_map(|num| u32::from_str(num).map_err(|_| ()));

    terminated(
        (
            separated_pair(
                hashes,
                greedy_whitespace,
                alt((
                    (
                        opt("v"),
                        take_till0(char::is_whitespace)
                            .try_map(|v| semver::Version::parse(v).map_err(|_| ()).map(Some)),
                    ),
                    tag_no_case("unreleased").map(|_| (None, None)),
                )),
            ),
            opt(preceded(
                greedy_whitespace,
                delimited(
                    "(",
                    (take_n_digits(4), "-", take_n_digits(2), "-", take_n_digits(2)).try_map(
                        |(year, _, month, _, day)| {
                            time::Month::try_from(month as u8).map_err(|_| ()).and_then(|month| {
                                time::Date::from_calendar_date(year as i32, month, day as u8)
                                    .map_err(|_| ())
                                    .map(|d| d.midnight().assume_utc())
                            })
                        },
                    ),
                    ")",
                ),
            )),
        ),
        greedy_whitespace,
    )
    .map(|((hashes, (prefix, version)), date)| Headline {
        level: hashes.len(),
        version_prefix: prefix.map_or_else(String::new, ToOwned::to_owned),
        version,
        date,
    })
    .parse_next(i)
}
