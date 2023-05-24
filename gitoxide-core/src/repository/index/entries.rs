#[derive(Debug)]
pub struct Options {
    pub format: crate::OutputFormat,
    /// If true, also show attributes
    pub attributes: Option<Attributes>,
    pub statistics: bool,
}

#[derive(Debug)]
pub enum Attributes {
    /// Look at worktree attributes and index as fallback.
    WorktreeAndIndex,
    /// Look at attributes from index files only.
    Index,
}

pub(crate) mod function {
    use crate::repository::attributes::query::index_on_demand;
    use crate::repository::index::entries::{Attributes, Options};
    use gix::odb::FindExt;
    use std::borrow::Cow;
    use std::io::{BufWriter, Write};

    pub fn entries(
        repo: gix::Repository,
        out: impl std::io::Write,
        mut err: impl std::io::Write,
        Options {
            format,
            attributes,
            statistics,
        }: Options,
    ) -> anyhow::Result<()> {
        use crate::OutputFormat::*;
        let index = index_on_demand(&repo)?;
        let mut cache = attributes
            .map(|attrs| {
                repo.attributes(
                    &index,
                    match attrs {
                        Attributes::WorktreeAndIndex => {
                            if repo.is_bare() {
                                gix::worktree::cache::state::attributes::Source::IdMapping
                            } else {
                                gix::worktree::cache::state::attributes::Source::WorktreeThenIdMapping
                            }
                        }
                        Attributes::Index => gix::worktree::cache::state::attributes::Source::IdMapping,
                    },
                    match attrs {
                        Attributes::WorktreeAndIndex => {
                            if repo.is_bare() {
                                gix::worktree::cache::state::ignore::Source::IdMapping
                            } else {
                                gix::worktree::cache::state::ignore::Source::WorktreeThenIdMappingIfNotSkipped
                            }
                        }
                        Attributes::Index => gix::worktree::cache::state::ignore::Source::IdMapping,
                    },
                    None,
                )
                .map(|cache| (cache.attribute_matches(), cache))
            })
            .transpose()?;
        let mut stats = Statistics {
            entries: index.entries().len(),
            ..Default::default()
        };

        let mut out = BufWriter::new(out);
        #[cfg(feature = "serde")]
        if let Json = format {
            out.write_all(b"[\n")?;
        }
        let mut entries = index.entries().iter().peekable();
        while let Some(entry) = entries.next() {
            let attrs = cache
                .as_mut()
                .map(|(attrs, cache)| {
                    cache
                        .at_entry(entry.path(&index), None, |id, buf| repo.objects.find_blob(id, buf))
                        .map(|entry| {
                            let is_excluded = entry.is_excluded();
                            stats.excluded += usize::from(is_excluded);
                            let attributes: Vec<_> = {
                                entry.matching_attributes(attrs);
                                attrs.iter().map(|m| m.assignment.to_owned()).collect()
                            };
                            stats.with_attributes += usize::from(!attributes.is_empty());
                            Attrs {
                                is_excluded,
                                attributes,
                            }
                        })
                })
                .transpose()?;
            match format {
                Human => to_human(&mut out, &index, entry, attrs)?,
                #[cfg(feature = "serde")]
                Json => to_json(&mut out, &index, entry, attrs, entries.peek().is_none())?,
            }
        }

        #[cfg(feature = "serde")]
        if format == Json {
            out.write_all(b"]\n")?;
            out.flush()?;
            if statistics {
                serde_json::to_writer_pretty(&mut err, &stats)?;
            }
        }
        if format == Human && statistics {
            out.flush()?;
            stats.cache = cache.map(|c| *c.1.statistics());
            writeln!(err, "{:#?}", stats)?;
        }
        Ok(())
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    struct Attrs {
        is_excluded: bool,
        attributes: Vec<gix::attrs::Assignment>,
    }

    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[derive(Default, Debug)]
    struct Statistics {
        #[allow(dead_code)] // Not really dead, but Debug doesn't count for it even though it's crucial.
        pub entries: usize,
        pub excluded: usize,
        pub with_attributes: usize,
        pub cache: Option<gix::worktree::cache::Statistics>,
    }

    #[cfg(feature = "serde")]
    fn to_json(
        mut out: &mut impl std::io::Write,
        index: &gix::index::File,
        entry: &gix::index::Entry,
        attrs: Option<Attrs>,
        is_last: bool,
    ) -> anyhow::Result<()> {
        use gix::bstr::ByteSlice;
        #[derive(serde::Serialize)]
        struct Entry<'a> {
            stat: &'a gix::index::entry::Stat,
            hex_id: String,
            flags: u32,
            mode: u32,
            path: std::borrow::Cow<'a, str>,
            meta: Option<Attrs>,
        }

        serde_json::to_writer(
            &mut out,
            &Entry {
                stat: &entry.stat,
                hex_id: entry.id.to_hex().to_string(),
                flags: entry.flags.bits(),
                mode: entry.mode.bits(),
                path: entry.path(index).to_str_lossy(),
                meta: attrs,
            },
        )?;

        if is_last {
            out.write_all(b"\n")?;
        } else {
            out.write_all(b",\n")?;
        }
        Ok(())
    }

    fn to_human(
        out: &mut impl std::io::Write,
        file: &gix::index::File,
        entry: &gix::index::Entry,
        attrs: Option<Attrs>,
    ) -> std::io::Result<()> {
        writeln!(
            out,
            "{} {}{:?} {} {}{}",
            match entry.flags.stage() {
                0 => "BASE   ",
                1 => "OURS   ",
                2 => "THEIRS ",
                _ => "UNKNOWN",
            },
            if entry.flags.is_empty() {
                "".to_string()
            } else {
                format!("{:?} ", entry.flags)
            },
            entry.mode,
            entry.id,
            entry.path(file),
            attrs
                .map(|a| {
                    let mut buf = String::new();
                    if a.is_excluded {
                        buf.push_str(" ❌");
                    }
                    if !a.attributes.is_empty() {
                        buf.push_str(" (");
                        for assignment in a.attributes {
                            use std::fmt::Write;
                            write!(&mut buf, "{}", assignment.as_ref()).ok();
                            buf.push_str(", ");
                        }
                        buf.pop();
                        buf.pop();
                        buf.push(')');
                    }
                    buf.into()
                })
                .unwrap_or(Cow::Borrowed(""))
        )
    }
}
