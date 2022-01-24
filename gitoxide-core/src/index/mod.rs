use std::convert::TryFrom;
use std::path::Path;

use git_repository as git;

pub mod entries;

mod information {
    use git_repository as git;
    use std::convert::TryFrom;

    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    pub(crate) struct EntryKind {
        dir: usize,
        file: usize,
        executable: usize,
        symlink: usize,
        submodule: usize,
        other: usize,
    }

    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    pub(crate) struct EntryFlag {
        intent_to_add: usize,
        skip_worktree: usize,
    }

    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    pub(crate) struct Entries {
        stage_0: usize,
        stage_1: usize,
        stage_2: usize,
        kind: EntryKind,
        flags: EntryFlag,
    }

    #[cfg_attr(feature = "serde1", derive(serde::Serialize))]
    pub(crate) struct Collection {
        version: u8,
        entries: Entries,
    }

    impl TryFrom<git::index::File> for Collection {
        type Error = anyhow::Error;

        fn try_from(f: git::index::File) -> Result<Self, Self::Error> {
            Ok(Collection {
                version: f.version() as u8,
                entries: {
                    let (mut stage_0, mut stage_1, mut stage_2) = (0, 0, 0);
                    let (mut dir, mut file, mut executable, mut symlink, mut submodule, mut other) = (0, 0, 0, 0, 0, 0);
                    let (mut intent_to_add, mut skip_worktree) = (0, 0);
                    for entry in f.entries() {
                        match entry.flags.stage() {
                            0 => stage_0 += 1,
                            1 => stage_1 += 1,
                            2 => stage_2 += 1,
                            invalid => anyhow::bail!("Invalid stage {} encountered", invalid),
                        }
                        match entry.mode {
                            git::index::entry::Mode::DIR => dir += 1,
                            git::index::entry::Mode::FILE => file += 1,
                            git::index::entry::Mode::FILE_EXECUTABLE => executable += 1,
                            git::index::entry::Mode::SYMLINK => symlink += 1,
                            git::index::entry::Mode::COMMIT => submodule += 1,
                            _ => other += 1,
                        }
                        if entry.flags.contains(git::index::entry::Flags::INTENT_TO_ADD) {
                            intent_to_add += 1;
                        }
                        if entry.flags.contains(git::index::entry::Flags::SKIP_WORKTREE) {
                            skip_worktree += 1;
                        }
                    }
                    Entries {
                        stage_0,
                        stage_1,
                        stage_2,
                        kind: EntryKind {
                            dir,
                            file,
                            executable,
                            symlink,
                            submodule,
                            other,
                        },
                        flags: EntryFlag {
                            intent_to_add,
                            skip_worktree,
                        },
                    }
                },
            })
        }
    }
}

pub fn information(
    index_path: impl AsRef<Path>,
    out: impl std::io::Write,
    entries::Options { object_hash, format }: entries::Options,
) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    let info = information::Collection::try_from(parse_file(index_path, object_hash)?)?;
    match format {
        Human => {
            anyhow::bail!("Only JSON output is implemented");
        }
        #[cfg(feature = "serde1")]
        Json => serde_json::to_writer_pretty(out, &info)?,
    }
    Ok(())
}

pub fn entries(
    index_path: impl AsRef<Path>,
    mut out: impl std::io::Write,
    entries::Options { object_hash, format }: entries::Options,
) -> anyhow::Result<()> {
    use crate::OutputFormat::*;
    let file = parse_file(index_path, object_hash)?;

    #[cfg(feature = "serde1")]
    if let Json = format {
        out.write_all(b"[\n")?;
    }

    let mut entries = file.entries().iter().peekable();
    while let Some(entry) = entries.next() {
        match format {
            Human => entries::to_human(&mut out, &file, entry)?,
            #[cfg(feature = "serde1")]
            Json => entries::to_json(&mut out, &file, entry, entries.peek().is_none())?,
        }
    }

    #[cfg(feature = "serde1")]
    if let Json = format {
        out.write_all(b"]\n")?;
    }
    Ok(())
}

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
