use std::{fs, io, path::PathBuf, str::FromStr, sync::atomic::AtomicBool};

use gix::{odb::pack, Progress};

use crate::OutputFormat;

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub enum IterationMode {
    AsIs,
    #[default]
    Verify,
    Restore,
}

impl IterationMode {
    pub fn variants() -> &'static [&'static str] {
        &["as-is", "verify", "restore"]
    }
}

impl FromStr for IterationMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slc = s.to_ascii_lowercase();
        Ok(match slc.as_str() {
            "as-is" => IterationMode::AsIs,
            "verify" => IterationMode::Verify,
            "restore" => IterationMode::Restore,
            _ => return Err("invalid value".into()),
        })
    }
}

impl From<IterationMode> for pack::data::input::Mode {
    fn from(v: IterationMode) -> Self {
        match v {
            IterationMode::AsIs => Self::AsIs,
            IterationMode::Verify => Self::Verify,
            IterationMode::Restore => Self::Restore,
        }
    }
}

pub struct Context<'a, W: io::Write> {
    pub thread_limit: Option<usize>,
    pub iteration_mode: IterationMode,
    pub format: OutputFormat,
    pub should_interrupt: &'a AtomicBool,
    pub out: W,
    pub object_hash: gix::hash::Kind,
}

pub fn stream_len(mut s: impl io::Seek) -> io::Result<u64> {
    use io::SeekFrom;
    let old_pos = s.stream_position()?;
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

pub fn from_pack<P>(
    pack: PathOrRead,
    directory: Option<PathBuf>,
    progress: P,
    ctx: Context<'static, impl io::Write>,
) -> anyhow::Result<()>
where
    P: Progress,
    P::SubProgress: 'static,
{
    use anyhow::Context;
    let options = pack::bundle::write::Options {
        thread_limit: ctx.thread_limit,
        iteration_mode: ctx.iteration_mode.into(),
        index_version: pack::index::Version::default(),
        object_hash: ctx.object_hash,
    };
    let out = ctx.out;
    let format = ctx.format;
    let res = match pack {
        PathOrRead::Path(pack) => {
            let pack_len = pack.metadata()?.len();
            let pack_file = fs::File::open(pack)?;
            pack::Bundle::write_to_directory_eagerly(
                pack_file,
                Some(pack_len),
                directory,
                progress,
                ctx.should_interrupt,
                None,
                options,
            )
        }
        PathOrRead::Read(input) => pack::Bundle::write_to_directory_eagerly(
            input,
            None,
            directory,
            progress,
            ctx.should_interrupt,
            None,
            options,
        ),
    }
    .with_context(|| "Failed to write pack and index")?;
    match format {
        OutputFormat::Human => drop(human_output(out, res)),
        #[cfg(feature = "serde")]
        OutputFormat::Json => serde_json::to_writer_pretty(out, &res)?,
    };
    Ok(())
}

fn human_output(mut out: impl io::Write, res: pack::bundle::write::Outcome) -> io::Result<()> {
    writeln!(&mut out, "index: {}", res.index.index_hash)?;
    writeln!(&mut out, "pack: {}", res.index.data_hash)
}
