use crate::{changelog, changelog::Section, ChangeLog};

impl std::fmt::Display for changelog::Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            changelog::Version::Unreleased => f.write_str("Unreleased"),
            changelog::Version::Semantic(v) => write!(f, "v{}", v),
        }
    }
}

impl Section {
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Section::Verbatim { text, .. } => out.write_all(text.as_bytes()),
            Section::Release {
                name,
                date,
                heading_level,
            } => {
                out.write_all(format!("{} {}", "#".repeat(*heading_level), name).as_bytes())?;
                match date {
                    None => out.write_all(b"\n"),
                    Some(date) => writeln!(
                        out,
                        " ({:04}-{:02}-{:02})",
                        date.year(),
                        date.month() as u32,
                        date.day()
                    ),
                }?;
                out.write_all(b"\n")
            }
        }
    }
}

impl ChangeLog {
    pub fn write_to(&self, mut out: impl std::io::Write) -> std::io::Result<()> {
        for section in &self.sections {
            section.write_to(&mut out)?;
        }
        Ok(())
    }
}
