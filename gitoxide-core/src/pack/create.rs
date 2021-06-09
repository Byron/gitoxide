use std::time::Instant;
use std::{ffi::OsStr, io, path::Path, str::FromStr, sync::Arc};

use anyhow::bail;

use git_repository::{
    hash,
    hash::ObjectId,
    interrupt,
    object::bstr::ByteVec,
    odb::{linked, pack},
    prelude::FindExt,
    progress, traverse, Progress,
};

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

#[derive(PartialEq, Debug)]
pub enum ObjectExpansion {
    None,
    TreeTraversal,
    TreeDiff,
}

impl ObjectExpansion {
    pub fn variants() -> &'static [&'static str] {
        &["none", "tree-traversal", "tree-diff"]
    }
}

impl Default for ObjectExpansion {
    fn default() -> Self {
        ObjectExpansion::None
    }
}

impl FromStr for ObjectExpansion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ObjectExpansion::*;
        let slc = s.to_ascii_lowercase();
        Ok(match slc.as_str() {
            "none" => None,
            "tree-traversal" => TreeTraversal,
            "tree-diff" => TreeDiff,
            _ => return Err("invalid value".into()),
        })
    }
}

impl From<ObjectExpansion> for pack::data::output::count_objects::ObjectExpansion {
    fn from(v: ObjectExpansion) -> Self {
        use pack::data::output::count_objects::ObjectExpansion::*;
        match v {
            ObjectExpansion::None => AsIs,
            ObjectExpansion::TreeTraversal => TreeContents,
            ObjectExpansion::TreeDiff => TreeAdditionsComparedToAncestor,
        }
    }
}

/// A general purpose context for many operations provided here
pub struct Context {
    /// The way input objects should be handled
    pub expansion: ObjectExpansion,
    /// If set, don't use more than this amount of threads.
    /// Otherwise, usually use as many threads as there are logical cores.
    /// A value of 0 is interpreted as no-limit
    pub thread_limit: Option<usize>,
}

pub fn create<W: io::Write>(
    repository: impl AsRef<Path>,
    tips: impl IntoIterator<Item = impl AsRef<OsStr>>,
    input: Option<impl io::BufRead + Send + 'static>,
    out: W,
    mut progress: impl Progress,
    ctx: Context,
) -> anyhow::Result<W> {
    let db = Arc::new(find_db(repository)?);
    let tips = tips.into_iter();
    let input: Box<dyn Iterator<Item = ObjectId> + Send + 'static> = match input {
        None => Box::new(
            traverse::commit::Ancestors::new(
                tips.map(|t| ObjectId::from_hex(&Vec::from_os_str_lossy(t.as_ref())))
                    .collect::<Result<Vec<_>, _>>()?,
                traverse::commit::ancestors::State::default(),
                {
                    let db = Arc::clone(&db);
                    move |oid, buf| db.find_existing_commit_iter(oid, buf, &mut pack::cache::Never).ok()
                },
            )
            .filter_map(Result::ok),
        ),
        Some(input) => Box::new(input.lines().filter_map(|hex_id| {
            hex_id
                .ok()
                .and_then(|hex_id| ObjectId::from_hex(hex_id.as_bytes()).ok())
        })),
    };

    let chunk_size = 200;
    progress.init(Some(3), progress::steps());
    let start = Instant::now();
    let counts = {
        let mut progress = progress.add_child("counting");
        progress.init(None, progress::count("objects"));
        let counts_iter = pack::data::output::count_objects_iter(
            Arc::clone(&db),
            pack::cache::lru::StaticLinkedList::<64>::default,
            input,
            progress.add_child("threads"),
            pack::data::output::count_objects::Options {
                thread_limit: ctx.thread_limit,
                chunk_size,
                input_object_expansion: ctx.expansion.into(),
            },
        );
        let mut counts = Vec::new();
        for c in counts_iter {
            if interrupt::is_triggered() {
                bail!("Cancelled by user")
            }
            let c = c?;
            progress.inc_by(c.len());
            counts.extend(c.into_iter());
        }
        progress.show_throughput(start);
        counts.shrink_to_fit();
        counts
    };

    progress.inc();
    let num_objects = counts.len();
    let entries = {
        let progress = progress.add_child("creating entries");
        pack::data::output::objects_to_entries_iter(
            counts,
            Arc::clone(&db),
            pack::cache::lru::StaticLinkedList::<64>::default,
            progress,
            pack::data::output::objects_to_entries::Options {
                thread_limit: ctx.thread_limit,
                chunk_size,
                version: Default::default(),
            },
        )
    };

    progress.inc();
    let mut entries_progress = progress.add_child("entries written");
    let mut write_progress = progress.add_child("writing");
    write_progress.init(None, progress::bytes());
    let start = Instant::now();

    let mut output_iter = pack::data::output::EntriesToBytesIter::new(
        entries.inspect(|e| {
            if let Ok(entries) = e {
                entries_progress.inc_by(entries.len())
            }
        }),
        out,
        num_objects as u32,
        pack::data::Version::default(),
        hash::Kind::default(),
    );
    while let Some(io_res) = output_iter.next() {
        if interrupt::is_triggered() {
            bail!("Cancelled by user")
        }
        let written = io_res?;
        write_progress.inc_by(written as usize);
    }
    let mut out = output_iter.into_write();
    out.flush()?;

    write_progress.show_throughput(start);
    entries_progress.show_throughput(start);
    Ok(out)
}

fn find_db(repository: impl AsRef<Path>) -> anyhow::Result<linked::Store> {
    let path = repository.as_ref();
    Ok(linked::Store::at(path.join(".git").join("objects"))?)
}
