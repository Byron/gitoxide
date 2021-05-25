use std::{fs, io::Read, path::Path};

use anyhow::{anyhow, Result};
use quick_error::quick_error;

use git_features::progress::{self, Progress};
use git_odb::{store::loose, Write};

#[derive(PartialEq, Debug)]
pub enum SafetyCheck {
    SkipFileChecksumVerification,
    SkipFileAndObjectChecksumVerification,
    SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError,
    All,
}

impl Default for SafetyCheck {
    fn default() -> Self {
        SafetyCheck::All
    }
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

impl From<SafetyCheck> for git_pack::index::traverse::SafetyCheck {
    fn from(v: SafetyCheck) -> Self {
        use git_pack::index::traverse::SafetyCheck::*;
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

quick_error! {
    #[derive(Debug)]
    enum Error {
        Io(err: std::io::Error) {
            display("An IO error occurred while writing an object")
            source(err)
            from()
        }
        OdbWrite(err: loose::backend::write::Error) {
            display("An object could not be written to the database")
            source(err)
            from()
        }
        Write(err: Box<dyn std::error::Error + Send + Sync>, kind: git_object::Kind, id: git_hash::ObjectId) {
            display("Failed to write {} object {}", kind, id)
            source(&**err)
        }
        Verify(err: git_pack::data::object::verify::Error) {
            display("Object didn't verify after right after writing it")
            source(err)
            from()
        }
        ObjectEncodeMismatch(kind: git_object::Kind, actual: git_hash::ObjectId, expected: git_hash::ObjectId) {
            display("{} object {} wasn't re-encoded without change - new hash is {}", kind, expected, actual)
        }
        WrittenFileMissing(id: git_hash::ObjectId) {
            display("The recently written file for loose object {} could not be found", id)
        }
        WrittenFileCorrupt(err: loose::backend::find::Error, id: git_hash::ObjectId) {
            display("The recently written file for loose object {} cold not be read", id)
            source(err)
        }
    }
}

#[allow(clippy::large_enum_variant)]
enum OutputWriter {
    Loose(loose::Backend),
    Sink(git_odb::store::Sink),
}

impl git_odb::write::Write for OutputWriter {
    type Error = Error;

    fn write_buf(
        &self,
        kind: git_object::Kind,
        from: &[u8],
        hash: git_hash::Kind,
    ) -> Result<git_hash::ObjectId, Self::Error> {
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
        hash: git_hash::Kind,
    ) -> Result<git_hash::ObjectId, Self::Error> {
        match self {
            OutputWriter::Loose(db) => db.write_stream(kind, size, from, hash).map_err(Into::into),
            OutputWriter::Sink(db) => db.write_stream(kind, size, from, hash).map_err(Into::into),
        }
    }
}

impl OutputWriter {
    fn new(path: Option<impl AsRef<Path>>, compress: bool) -> Self {
        match path {
            Some(path) => OutputWriter::Loose(loose::Backend::at(path.as_ref())),
            None => OutputWriter::Sink(git_odb::store::sink().compress(compress)),
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

pub fn pack_or_pack_index(
    pack_path: impl AsRef<Path>,
    object_path: Option<impl AsRef<Path>>,
    check: SafetyCheck,
    progress: Option<impl Progress>,
    Context {
        thread_limit,
        delete_pack,
        sink_compress,
        verify,
    }: Context,
) -> Result<()> {
    use anyhow::Context;

    let path = pack_path.as_ref();
    let bundle = git_pack::Bundle::at(path).with_context(|| {
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
        .map(|_| git_pack::index::traverse::Algorithm::Lookup)
        .unwrap_or_else(|| {
            if sink_compress {
                git_pack::index::traverse::Algorithm::Lookup
            } else {
                git_pack::index::traverse::Algorithm::DeltaTreeLookup
            }
        });
    let mut progress = bundle.index.traverse(
        &bundle.pack,
        progress,
        {
            let object_path = object_path.map(|p| p.as_ref().to_owned());
            move || {
                let out = OutputWriter::new(object_path.clone(), sink_compress);
                let object_verifier = if verify {
                    object_path.as_ref().map(loose::Backend::at)
                } else {
                    None
                };
                let mut read_buf = Vec::new();
                move |object_kind, buf, index_entry, progress| {
                    let written_id = out
                        .write_buf(object_kind, buf, git_hash::Kind::Sha1)
                        .map_err(|err| Error::Write(Box::new(err) as Box<dyn std::error::Error + Send + Sync>, object_kind, index_entry.oid))?;
                    if written_id != index_entry.oid {
                        if let git_object::Kind::Tree = object_kind {
                            progress.info(format!("The tree in pack named {} was written as {} due to modes 100664 and 100640 rewritten as 100644.", index_entry.oid, written_id));
                        } else {
                            return Err(Error::ObjectEncodeMismatch(object_kind, index_entry.oid, written_id));
                        }
                    }
                    if let Some(verifier) = object_verifier.as_ref() {
                        let obj = verifier.find(written_id, &mut read_buf)
                            .map_err(|err| Error::WrittenFileCorrupt(err, written_id))?
                            .ok_or(Error::WrittenFileMissing(written_id))?;
                        obj.verify_checksum(written_id)?;
                    }
                    Ok(())
                }
            }
        },
        git_pack::cache::lru::StaticLinkedList::<64>::default,
        git_pack::index::traverse::Options {
            algorithm,
            thread_limit,
            check: check.into(),
        },
    ).map(|(_, _, c)| progress::DoOrDiscard::from(c)).with_context(|| "Failed to explode the entire pack - some loose objects may have been created nonetheless")?;

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
