use git::{odb::FindExt, prelude::ObjectIdExt};
use git_repository as git;
use std::path::{Path, PathBuf};

pub struct Options {
    pub object_hash: git::hash::Kind,
    pub format: crate::OutputFormat,
}

mod entries;
pub use entries::entries;

pub mod information;

fn parse_file(index_path: impl AsRef<Path>, object_hash: git::hash::Kind) -> anyhow::Result<git::index::File> {
    git::index::File::at(
        index_path.as_ref(),
        git::index::decode::Options {
            object_hash,
            ..Default::default()
        },
    )
    .map_err(Into::into)
}

pub mod checkout_exclusive {
    pub struct Options {
        pub index: super::Options,
        /// If true, all files will be written with zero bytes despite having made an ODB lookup.
        pub empty_files: bool,
        pub keep_going: bool,
        /// If set, don't use more than this amount of threads.
        /// Otherwise, usually use as many threads as there are logical cores.
        /// A value of 0 is interpreted as no-limit
        pub thread_limit: Option<usize>,
    }
}

mod checkout;
pub use checkout::checkout_exclusive;

pub fn verify(
    index_path: impl AsRef<Path>,
    mut out: impl std::io::Write,
    Options { object_hash, format }: Options,
) -> anyhow::Result<()> {
    let file = parse_file(index_path, object_hash)?;
    file.verify_integrity()?;
    file.verify_entries()?;
    file.verify_extensions(false, git::index::verify::extensions::no_find)?;
    #[cfg_attr(not(feature = "serde1"), allow(irrefutable_let_patterns))]
    if let crate::OutputFormat::Human = format {
        writeln!(out, "OK").ok();
    }
    Ok(())
}

#[cfg_attr(not(feature = "serde1"), allow(unused_variables, unused_mut))]
pub fn information(
    index_path: impl AsRef<Path>,
    out: impl std::io::Write,
    mut err: impl std::io::Write,
    information::Options {
        index: Options {
            object_hash,
            mut format,
        },
        extension_details,
    }: information::Options,
) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    #[cfg(feature = "serde1")]
    if let Human = format {
        writeln!(err, "Defaulting to JSON printing as nothing else will be implemented.").ok();
        format = Json;
    }
    match format {
        Human => {
            anyhow::bail!("Cannot print information using 'human' format.")
        }
        #[cfg(feature = "serde1")]
        Json => {
            let info = information::Collection::try_from_file(parse_file(index_path, object_hash)?, extension_details)?;
            serde_json::to_writer_pretty(out, &info)?;
            Ok(())
        }
    }
}

pub fn from_tree(
    id: String,
    index: Option<PathBuf>,
    force: bool,
    repo: git::Repository,
    mut out: impl std::io::Write,
    mut err: impl std::io::Write,
) -> anyhow::Result<()> {
    // TODO: consider HashKind
    let id = match id.len() {
        40 => git::hash::ObjectId::from_hex(id.as_bytes())?,
        _ => {
            let prefix = git::hash::Prefix::from_hex(&id)?;
            match repo.objects.lookup_prefix(prefix, None) {
                Ok(Some(Ok(id))) => id,
                Ok(Some(Err(_))) => anyhow::bail!("multiple objects found while trying to disambiguate id: {:?}", id),
                Ok(None) => anyhow::bail!("no objects found while trying to disambiguate id: {:?}", id),
                Err(e) => anyhow::bail!(e),
            }
        }
    };

    let tree = id.attach(&repo).object()?.peel_to_kind(git::objs::Kind::Tree)?.id();
    let state = git::index::State::from_tree(&tree, |oid, buf| repo.objects.find_tree_iter(oid, buf).ok())?;

    match index {
        Some(index) => {
            if index.is_file() {
                writeln!(err, "File {:?} already exists", index).ok();
                if force {
                    writeln!(err, "overwriting").ok();
                } else {
                    anyhow::bail!("exiting, to overwrite use the '-f' flag");
                }
            }
            let mut file = std::fs::File::create(&index)?;
            state.write_to(&mut file, git::index::write::Options::default())?;
            writeln!(err, "Successfully wrote file {:?}", index).ok();
        }
        None => {
            state.write_to(&mut out, git::index::write::Options::default())?;
        }
    }

    Ok(())
}
