use anyhow::{Context, Result};
use git_features::progress::Progress;
use git_object::{owned, HashKind};
use git_odb::{loose, pack};
use std::io::Read;
use std::path::Path;

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
            source(err)
            from()
        }
        Odb(err: loose::db::write::Error) {
            source(err)
            from()
        }
    }
}

struct OutputWriter(Option<loose::Db>);

impl git_odb::Write for OutputWriter {
    type Error = Error;

    fn write_stream(
        &self,
        kind: git_object::Kind,
        size: u64,
        from: impl Read,
        hash: HashKind,
    ) -> Result<owned::Id, Self::Error> {
        match self.0.as_ref() {
            Some(db) => db.write_stream(kind, size, from, hash).map_err(Into::into),
            None => git_odb::sink().write_stream(kind, size, from, hash).map_err(Into::into),
        }
    }
    fn write_buf(&self, kind: git_object::Kind, from: &[u8], hash: HashKind) -> Result<owned::Id, Self::Error> {
        match self.0.as_ref() {
            Some(db) => db.write_buf(kind, from, hash).map_err(Into::into),
            None => git_odb::sink().write_buf(kind, from, hash).map_err(Into::into),
        }
    }
}

pub fn pack_or_pack_index<P>(
    pack_path: impl AsRef<Path>,
    object_path: Option<impl AsRef<Path>>,
    _check: SafetyCheck,
    _progress: Option<P>,
    _delete_pack: bool,
) -> Result<()>
where
    P: Progress,
{
    let path = pack_path.as_ref();
    let _bundle = pack::Bundle::at(path).with_context(|| {
        format!(
            "Could not find .idx or .pack file from given file at '{}'",
            path.display()
        )
    })?;

    let _out = OutputWriter(object_path.map(|path| loose::Db::at(path.as_ref())));
    Ok(())
}
