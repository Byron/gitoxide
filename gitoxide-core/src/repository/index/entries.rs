#[derive(Debug)]
pub struct Options {
    pub format: crate::OutputFormat,
    /// If true, also show attributes
    pub attributes: Option<Attributes>,
    pub statistics: bool,
    pub simple: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Attributes {
    /// Look at worktree attributes and index as fallback.
    WorktreeAndIndex,
    /// Look at attributes from index files only.
    Index,
}

pub(crate) mod function {
    use gix::bstr::BString;
    use std::collections::BTreeSet;
    use std::{
        borrow::Cow,
        io::{BufWriter, Write},
    };

    use gix::odb::FindExt;

    use crate::repository::index::entries::{Attributes, Options};

    pub fn entries(
        repo: gix::Repository,
        pathspecs: Vec<BString>,
        out: impl std::io::Write,
        mut err: impl std::io::Write,
        Options {
            simple,
            format,
            attributes,
            statistics,
        }: Options,
    ) -> anyhow::Result<()> {
        use crate::OutputFormat::*;
        let index = repo.index_or_load_from_head()?;
        let pathspec = repo.pathspec(pathspecs, false, &index)?;
        let mut cache = attributes
            .or_else(|| {
                pathspec
                    .search()
                    .patterns()
                    .any(|spec| !spec.attributes.is_empty())
                    .then_some(Attributes::Index)
            })
            .map(|attrs| {
                repo.attributes(
                    &index,
                    match attrs {
                        Attributes::WorktreeAndIndex => {
                            if repo.is_bare() {
                                gix::worktree::stack::state::attributes::Source::IdMapping
                            } else {
                                gix::worktree::stack::state::attributes::Source::WorktreeThenIdMapping
                            }
                        }
                        Attributes::Index => gix::worktree::stack::state::attributes::Source::IdMapping,
                    },
                    match attrs {
                        Attributes::WorktreeAndIndex => {
                            if repo.is_bare() {
                                gix::worktree::stack::state::ignore::Source::IdMapping
                            } else {
                                gix::worktree::stack::state::ignore::Source::WorktreeThenIdMappingIfNotSkipped
                            }
                        }
                        Attributes::Index => gix::worktree::stack::state::ignore::Source::IdMapping,
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

        let mut out = BufWriter::with_capacity(64 * 1024, out);
        #[cfg(feature = "serde")]
        if let Json = format {
            out.write_all(b"[\n")?;
        }
        let (mut search, _cache) = pathspec.into_parts();
        let mut all_attrs = statistics.then(BTreeSet::new);
        if let Some(entries) = index.prefixed_entries(search.common_prefix()) {
            stats.entries_after_prune = entries.len();
            let mut entries = entries.iter().peekable();
            while let Some(entry) = entries.next() {
                let mut last_match = None;
                let attrs = cache
                    .as_mut()
                    .and_then(|(attrs, cache)| {
                        // If the user wants to see assigned attributes, we always have to match.
                        attributes.is_some().then(|| {
                            cache
                                .at_entry(entry.path(&index), None, |id, buf| repo.objects.find_blob(id, buf))
                                .map(|entry| {
                                    let is_excluded = entry.is_excluded();
                                    stats.excluded += usize::from(is_excluded);
                                    let attributes: Vec<_> = {
                                        last_match = Some(entry.matching_attributes(attrs));
                                        attrs.iter().map(|m| m.assignment.to_owned()).collect()
                                    };
                                    stats.with_attributes += usize::from(!attributes.is_empty());
                                    stats.max_attributes_per_path = stats.max_attributes_per_path.max(attributes.len());
                                    if let Some(attrs) = all_attrs.as_mut() {
                                        attributes.iter().for_each(|attr| {
                                            attrs.insert(attr.clone());
                                        });
                                    }
                                    Attrs {
                                        is_excluded,
                                        attributes,
                                    }
                                })
                        })
                    })
                    .transpose()?;

                // Note that we intentionally ignore `_case` so that we act like git does, attribute matching case is determined
                // by the repository, not the pathspec.
                if search
                    .pattern_matching_relative_path(entry.path(&index), Some(false), |rela_path, _case, is_dir, out| {
                        cache
                            .as_mut()
                            .map(|(attrs, cache)| {
                                match last_match {
                                    // The user wants the attributes for display, so the match happened already.
                                    Some(matched) => {
                                        attrs.copy_into(cache.attributes_collection(), out);
                                        matched
                                    }
                                    // The user doesn't want attributes, so we set the cache position on demand only
                                    None => cache
                                        .at_entry(rela_path, Some(is_dir), |id, buf| repo.objects.find_blob(id, buf))
                                        .ok()
                                        .map(|platform| platform.matching_attributes(out))
                                        .unwrap_or_default(),
                                }
                            })
                            .unwrap_or_default()
                    })
                    .map_or(true, |m| m.is_excluded())
                {
                    continue;
                }
                match format {
                    Human => {
                        if simple {
                            to_human_simple(&mut out, &index, entry, attrs)
                        } else {
                            to_human(&mut out, &index, entry, attrs)
                        }?
                    }
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
                writeln!(err, "{stats:#?}")?;
                if let Some(attrs) = all_attrs.filter(|a| !a.is_empty()) {
                    writeln!(err, "All encountered attributes:")?;
                    for attr in attrs {
                        writeln!(err, "\t{attr}", attr = attr.as_ref())?;
                    }
                }
            }
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
        pub entries_after_prune: usize,
        pub excluded: usize,
        pub with_attributes: usize,
        pub max_attributes_per_path: usize,
        pub cache: Option<gix::worktree::stack::Statistics>,
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

    fn to_human_simple(
        out: &mut impl std::io::Write,
        file: &gix::index::File,
        entry: &gix::index::Entry,
        attrs: Option<Attrs>,
    ) -> std::io::Result<()> {
        match attrs {
            Some(attrs) => {
                out.write_all(entry.path(file))?;
                out.write_all(print_attrs(Some(attrs)).as_bytes())
            }
            None => out.write_all(entry.path(file)),
        }?;
        out.write_all(b"\n")
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
            print_attrs(attrs)
        )
    }

    fn print_attrs(attrs: Option<Attrs>) -> Cow<'static, str> {
        attrs.map_or(Cow::Borrowed(""), |a| {
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
    }
}
