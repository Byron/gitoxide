use anyhow::bail;
use git_features::progress::Progress;
use git_hash::ObjectId;
use git_object::bstr::ByteVec;
use git_odb::{linked, pack, FindExt};
use std::time::Instant;
use std::{ffi::OsStr, io, path::Path, str::FromStr, sync::Arc};

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

#[derive(PartialEq, Debug)]
pub enum ObjectExpansion {
    None,
    TreeTraversal,
}

impl ObjectExpansion {
    pub fn variants() -> &'static [&'static str] {
        &["none", "tree-traversal"]
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

pub fn create(
    repository: impl AsRef<Path>,
    tips: impl IntoIterator<Item = impl AsRef<OsStr>>,
    input: Option<impl io::BufRead + Send + 'static>,
    out: impl io::Write,
    mut progress: impl Progress,
    ctx: Context,
) -> anyhow::Result<()> {
    let db = Arc::new(find_db(repository)?);
    let tips = tips.into_iter();
    let input: Box<dyn Iterator<Item = ObjectId> + Send + 'static> = match input {
        None => Box::new(
            git_traverse::commit::Ancestors::new(
                tips.map(|t| git_hash::ObjectId::from_hex(&Vec::from_os_str_lossy(t.as_ref())))
                    .collect::<Result<Vec<_>, _>>()?,
                git_traverse::commit::ancestors::State::default(),
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
                .and_then(|hex_id| git_hash::ObjectId::from_hex(hex_id.as_bytes()).ok())
        })),
    };

    let chunk_size = 200;
    progress.init(Some(3), git_features::progress::steps());
    let start = Instant::now();
    let counts = {
        let mut progress = progress.add_child("counting");
        progress.init(None, git_features::progress::count("objects"));
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
            if git_features::interrupt::is_triggered() {
                bail!("Cancelled by user")
            }
            let c = c?;
            progress.inc_by(c.len());
            counts.extend(c.into_iter());
        }
        progress.show_throughput(start);
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
    write_progress.init(None, git_features::progress::bytes());
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
        git_hash::Kind::default(),
    );
    while let Some(io_res) = output_iter.next() {
        if git_features::interrupt::is_triggered() {
            bail!("Cancelled by user")
        }
        let written = io_res?;
        write_progress.inc_by(written as usize);
    }
    output_iter.into_write().flush()?;

    write_progress.show_throughput(start);
    entries_progress.show_throughput(start);
    Ok(())
}

fn find_db(repository: impl AsRef<Path>) -> anyhow::Result<linked::Db> {
    let path = repository.as_ref();
    Ok(linked::Db::at(path.join(".git").join("objects"))?)
}
