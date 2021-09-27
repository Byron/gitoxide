use crate::{
    changelog,
    changelog::{section, Section},
    ChangeLog,
};

impl std::fmt::Display for changelog::Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            changelog::Version::Unreleased => f.write_str("Unreleased"),
            changelog::Version::Semantic(v) => write!(f, "v{}", v),
        }
    }
}

impl Section {
    pub const UNKNOWN_TAG_START: &'static str = "<csr-unknown>";
    pub const UNKNOWN_TAG_END: &'static str = "<csr-unknown/>";
    pub const READONLY_TAG: &'static str = "<csr-read-only-do-not-edit/>";

    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Section::Verbatim { text, .. } => out.write_all(text.as_bytes()),
            Section::Release {
                name,
                date,
                heading_level,
                segments,
                unknown,
            } => {
                write!(out, "{} {}", heading(*heading_level), name)?;
                match date {
                    None => out.write_all(b"\n\n"),
                    Some(date) => writeln!(
                        out,
                        " ({:04}-{:02}-{:02})\n",
                        date.year(),
                        date.month() as u32,
                        date.day()
                    ),
                }?;
                let section_level = *heading_level + 1;
                for segment in segments {
                    segment.write_to(section_level, &mut out)?;
                }
                if !unknown.is_empty() {
                    writeln!(out, "{}", Section::UNKNOWN_TAG_START)?;
                    out.write_all(unknown.as_bytes())?;
                    writeln!(out, "{}", Section::UNKNOWN_TAG_END)?;
                }
                Ok(())
            }
        }
    }
}

fn heading(level: usize) -> String {
    "#".repeat(level)
}

impl ChangeLog {
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        for section in &self.sections {
            section.write_to(&mut out)?;
        }
        Ok(())
    }
}

impl section::Segment {
    pub fn write_to(&self, section_level: usize, mut out: impl std::io::Write) -> std::io::Result<()> {
        match self {
            section::Segment::User { text } => out.write_all(text.as_bytes())?,
            section::Segment::Statistics(section::Data::Generated(section::CommitStatistics {
                count,
                duration,
                conventional_count,
                unique_issues_count,
            })) => {
                writeln!(out, "{} {}\n", heading(section_level), section::CommitStatistics::TITLE)?;
                writeln!(out, "{}", Section::READONLY_TAG)?;
                writeln!(
                    out,
                    " - {} {} contributed to the release{}",
                    count,
                    if *count == 1 { "commit" } else { "commits" },
                    match duration {
                        Some(duration) if duration.whole_days() > 0 => format!(
                            " over the course of {} calendar {}.",
                            duration.whole_days(),
                            if duration.whole_days() == 1 { "day" } else { "days" }
                        ),
                        _ => ".".into(),
                    }
                )?;
                writeln!(
                    out,
                    " - {} {} where understood as [conventional](https://www.conventionalcommits.org).",
                    conventional_count,
                    if *conventional_count == 1 { "commit" } else { "commits" }
                )?;
                writeln!(
                    out,
                    " - {} unique {} {} worked on",
                    unique_issues_count,
                    if *unique_issues_count == 1 { "issue" } else { "issues" },
                    if *unique_issues_count == 1 { "was" } else { "were" },
                )?;
                writeln!(out)?;
            }
            section::Segment::Clippy(section::Data::Generated(section::ThanksClippy { count })) if *count > 0 => {
                writeln!(out, "{} {}\n", heading(section_level), section::ThanksClippy::TITLE)?;
                writeln!(out, "{}", Section::READONLY_TAG)?;
                writeln!(
                    out,
                    "[Clippy](https://github.com/rust-lang/rust-clippy) helped {} {} to make code idiomatic. \n",
                    count,
                    if *count > 1 { "times" } else { "time" }
                )?;
            }
            section::Segment::Clippy(_) => {}
            section::Segment::Statistics(_) => {}
        };
        Ok(())
    }
}
