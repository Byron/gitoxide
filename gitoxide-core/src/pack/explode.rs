use anyhow::{anyhow, Result};
use git_features::progress::{self, Progress};
use git_object::{owned, HashKind};
use git_odb::{loose, pack, Write};
use std::{fs, io::Read, path::Path};

#[derive(PartialEq, Debug)]
pub enum SafetyCheck {
    SkipFileChecksumVerification,
    SkipFileAndObjectChecksumVerification,
    SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError,
    All,
}

impl SafetyCheck {
    pub fn variants() -> &'static [&'static str] {
        &[
            "all",
            "skip-file-checksum",
            "skip-file-and-object-checksum",
            "skip-file-and-object-checksum-and-no-abort-on-decode",
        ]
    }
}

impl std::str::FromStr for SafetyCheck {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "skip-file-checksum" => SafetyCheck::SkipFileChecksumVerification,
            "skip-file-and-object-checksum" => SafetyCheck::SkipFileAndObjectChecksumVerification,
            "skip-file-and-object-checksum-and-no-abort-on-decode" => {
                SafetyCheck::SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError
            }
            "all" => SafetyCheck::All,
            _ => return Err(format!("Unknown value for safety check: '{}'", s)),
        })
    }
}

impl From<SafetyCheck> for pack::index::traverse::SafetyCheck {
    fn from(v: SafetyCheck) -> Self {
        use pack::index::traverse::SafetyCheck::*;
        match v {
            SafetyCheck::All => All,
            SafetyCheck::SkipFileChecksumVerification => SkipFileChecksumVerification,
            SafetyCheck::SkipFileAndObjectChecksumVerification => SkipFileAndObjectChecksumVerification,
            SafetyCheck::SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError => {
                SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError
            }
        }
    }
}

use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    enum Error {
        Io(err: std::io::Error) {
            display("An IO error occurred while writing an object")
            source(err)
            from()
        }
        OdbWrite(err: loose::db::write::Error) {
            display("An object could not be written to the database")
            source(err)
            from()
        }
        Write(err: Box<dyn std::error::Error + Send + Sync>, kind: git_object::Kind, id: owned::Id) {
            display("Failed to write {} object {}", kind, id)
            source(&**err)
        }
        ObjectEncodeMismatch(kind: git_object::Kind, actual: owned::Id, expected: owned::Id) {
            display("{} object {} wasn't re-encoded without change - new hash is {}", kind, expected, actual)
        }
        WrittenFileMissing(id: owned::Id) {
            display("The recently written file for loose object {} could not be found", id)
        }
        WrittenFileCorrupt(err: loose::db::locate::Error, id: owned::Id) {
            display("The recently written file for loose object {} cold not be read", id)
            source(err)
        }
    }
}

#[allow(clippy::large_enum_variant)]
enum OutputWriter {
    Loose(loose::Db),
    Sink(git_odb::Sink),
}

impl git_odb::Write for OutputWriter {
    type Error = Error;

    fn write_buf(&self, kind: git_object::Kind, from: &[u8], hash: HashKind) -> Result<owned::Id, Self::Error> {
        match self {
            OutputWriter::Loose(db) => db.write_buf(kind, from, hash).map_err(Into::into),
            OutputWriter::Sink(db) => db.write_buf(kind, from, hash).map_err(Into::into),
        }
    }

    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        from: impl Read,
        hash: HashKind,
    ) -> Result<owned::Id, Self::Error> {
        match self {
            OutputWriter::Loose(db) => db.write_stream(kind, size, from, hash).map_err(Into::into),
            OutputWriter::Sink(db) => db.write_stream(kind, size, from, hash).map_err(Into::into),
        }
    }
}

impl OutputWriter {
    fn new(path: Option<impl AsRef<Path>>, compress: bool) -> Self {
        match path {
            Some(path) => OutputWriter::Loose(loose::Db::at(path.as_ref())),
            None => OutputWriter::Sink(git_odb::sink().compress(compress)),
        }
    }
}

#[derive(Default)]
pub struct Context {
    pub thread_limit: Option<usize>,
    pub delete_pack: bool,
    pub sink_compress: bool,
    pub verify: bool,
}

pub fn pack_or_pack_index<P>(
    pack_path: impl AsRef<Path>,
    object_path: Option<impl AsRef<Path>>,
    check: SafetyCheck,
    progress: Option<P>,
    Context {
        thread_limit,
        delete_pack,
        sink_compress,
        verify,
    }: Context,
) -> Result<()>
where
    P: Progress,
    <P as Progress>::SubProgress: Send,
{
    use anyhow::Context;

    let path = pack_path.as_ref();
    let bundle = pack::Bundle::at(path).with_context(|| {
        format!(
            "Could not find .idx or .pack file from given file at '{}'",
            path.display()
        )
    })?;

    if !object_path.as_ref().map(|p| p.as_ref().is_dir()).unwrap_or(true) {
        return Err(anyhow!(
            "The object directory at '{}' is inaccessible",
            object_path
                .expect("path present if no directory on disk")
                .as_ref()
                .display()
        ));
    }

    let algorithm = object_path
        .as_ref()
        .map(|_| pack::index::traverse::Algorithm::Lookup)
        .unwrap_or_else(|| {
            if sink_compress {
                pack::index::traverse::Algorithm::Lookup
            } else {
                pack::index::traverse::Algorithm::DeltaTreeLookup
            }
        });
    let mut progress = bundle.index.traverse(
        &bundle.pack,
        pack::index::traverse::Options {
            algorithm,
            thread_limit,
            check: check.into(),
        },
        progress,
        {
            let object_path = object_path.map(|p| p.as_ref().to_owned());
            move || {
                let out = OutputWriter::new(object_path.clone(), sink_compress);
                let object_verifier = if verify {
                    object_path.as_ref().map(loose::Db::at)
                } else {
                    None
                };
                move |object_kind, buf, index_entry, _entry_stats, progress| {
                    let written_id = out
                        .write_buf(object_kind, buf, HashKind::Sha1)
                        .map_err(|err| Error::Write(Box::new(err) as Box<dyn std::error::Error + Send + Sync>, object_kind, index_entry.oid))
                        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)?;
                    if written_id != index_entry.oid {
                       if let git_object::Kind::Tree = object_kind {
                           progress.info(format!("The tree in pack named {} was written as {} due to modes 100664 and 100640 rewritten as 100644.", index_entry.oid, written_id));
                       } else {
                           return Err(Box::new(Error::ObjectEncodeMismatch(object_kind, index_entry.oid, written_id)))
                       }
                    }
                    if let Some(verifier) = object_verifier.as_ref() {
                        let mut obj = verifier.locate(written_id.to_borrowed())
                                            .ok_or_else(|| Error::WrittenFileMissing(written_id))?
                                            .map_err(|err| Error::WrittenFileCorrupt(err, written_id))?;
                        obj.verify_checksum(written_id.to_borrowed())?;
                    }
                    Ok(())
                }
            }
        },
        pack::cache::DecodeEntryLRU::default,
    ).map(|(_,_,c)|progress::DoOrDiscard::from(c)).with_context(|| "Failed to explode the entire pack - some loose objects may have been created nonetheless")?;

    let (index_path, data_path) = (bundle.index.path().to_owned(), bundle.pack.path().to_owned());
    drop(bundle);

    if delete_pack {
        fs::remove_file(&index_path)
            .and_then(|_| fs::remove_file(&data_path))
            .with_context(|| {
                format!(
                    "Failed to delete pack index file at '{} or data file at '{}'",
                    index_path.display(),
                    data_path.display()
                )
            })?;
        progress.info(format!(
            "Removed '{}' and '{}'",
            index_path.display(),
            data_path.display()
        ));
    }
    Ok(())
}
