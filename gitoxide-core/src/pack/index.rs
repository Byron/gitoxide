use crate::OutputFormat;
use git_repository::{odb::pack, Progress};
use std::{fs, io, path::PathBuf, str::FromStr};

#[derive(PartialEq, Debug)]
pub enum IterationMode {
    AsIs,
    Verify,
    Restore,
}

impl IterationMode {
    pub fn variants() -> &'static [&'static str] {
        &["as-is", "verify", "restore"]
    }
}

impl Default for IterationMode {
    fn default() -> Self {
        IterationMode::Verify
    }
}

impl FromStr for IterationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IterationMode::*;
        let slc = s.to_ascii_lowercase();
        Ok(match slc.as_str() {
            "as-is" => AsIs,
            "verify" => Verify,
            "restore" => Restore,
            _ => return Err("invalid value".into()),
        })
    }
}

impl From<IterationMode> for pack::data::input::Mode {
    fn from(v: IterationMode) -> Self {
        use git_pack::data::input::Mode::*;
        match v {
            IterationMode::AsIs => AsIs,
            IterationMode::Verify => Verify,
            IterationMode::Restore => Restore,
        }
    }
}

pub struct Context<W: io::Write> {
    pub thread_limit: Option<usize>,
    pub iteration_mode: IterationMode,
    pub format: OutputFormat,
    pub out: W,
}

pub fn stream_len(mut s: impl io::Seek) -> io::Result<u64> {
    use io::SeekFrom;
    let old_pos = s.seek(SeekFrom::Current(0))?;
    let len = s.seek(SeekFrom::End(0))?;
    if old_pos != len {
        s.seek(SeekFrom::Start(old_pos))?;
    }
    Ok(len)
}

pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 2..=3;

pub enum PathOrRead {
    Path(PathBuf),
    Read(Box<dyn std::io::Read + Send + 'static>),
}

pub fn from_pack(
    pack: PathOrRead,
    directory: Option<PathBuf>,
    progress: impl Progress,
    ctx: Context<impl io::Write>,
) -> anyhow::Result<()> {
    use anyhow::Context;
    let options = pack::bundle::write::Options {
        thread_limit: ctx.thread_limit,
        iteration_mode: ctx.iteration_mode.into(),
        index_kind: pack::index::Version::default(),
    };
    let out = ctx.out;
    let format = ctx.format;
    let res = match pack {
        PathOrRead::Path(pack) => {
            let pack_len = pack.metadata()?.len();
            let pack_file = fs::File::open(pack)?;
            pack::Bundle::write_to_directory_eagerly(pack_file, Some(pack_len), directory, progress, options)
        }
        PathOrRead::Read(input) => pack::Bundle::write_to_directory_eagerly(input, None, directory, progress, options),
    }
    .with_context(|| "Failed to write pack and index")?;
    match format {
        OutputFormat::Human => drop(human_output(out, res)),
        #[cfg(feature = "serde1")]
        OutputFormat::Json => serde_json::to_writer_pretty(out, &res)?,
    };
    Ok(())
}

fn human_output(mut out: impl io::Write, res: pack::bundle::write::Outcome) -> io::Result<()> {
    writeln!(&mut out, "index: {}", res.index.index_hash)?;
    writeln!(&mut out, "pack: {}", res.index.data_hash)
}
