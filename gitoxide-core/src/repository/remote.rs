#[cfg(any(feature = "blocking-client", feature = "async-client"))]
mod net {
    use crate::OutputFormat;
    use anyhow::bail;
    use git_repository as git;
    use git_repository::protocol::fetch;

    pub mod refs {
        use crate::OutputFormat;

        pub const PROGRESS_RANGE: std::ops::RangeInclusive<u8> = 1..=2;

        pub struct Context {
            pub format: OutputFormat,
            pub name: Option<String>,
            pub url: Option<git_repository::Url>,
        }

        pub(crate) use super::print;
    }

    #[git::protocol::maybe_async::maybe_async]
    pub async fn refs_fn(
        repo: git::Repository,
        mut progress: impl git::Progress,
        out: impl std::io::Write,
        refs::Context { format, name, url }: refs::Context,
    ) -> anyhow::Result<()> {
        use anyhow::Context;
        let remote = match (name, url) {
            (Some(name), None) => repo.find_remote(&name)?,
            (None, None) => repo
                .head()?
                .into_remote(git::remote::Direction::Fetch)
                .context("Cannot find a remote for unborn branch")??,
            (None, Some(url)) => repo.remote_at(url)?,
            (Some(_), Some(_)) => bail!("Must not set both the remote name and the url - they are mutually exclusive"),
        };
        progress.info(format!(
            "Connecting to {:?}",
            remote
                .url(git::remote::Direction::Fetch)
                .context("Remote didn't have a URL to connect to")?
                .to_bstring()
        ));
        let refs = remote
            .connect(git::remote::Direction::Fetch, progress)
            .await?
            .list_refs()
            .await?;

        match format {
            OutputFormat::Human => drop(print(out, &refs)),
            #[cfg(feature = "serde1")]
            OutputFormat::Json => {
                serde_json::to_writer_pretty(out, &refs.into_iter().map(JsonRef::from).collect::<Vec<_>>())?
            }
        };
        Ok(())
    }

    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub enum JsonRef {
        Peeled {
            path: String,
            tag: String,
            object: String,
        },
        Direct {
            path: String,
            object: String,
        },
        Symbolic {
            path: String,
            target: String,
            object: String,
        },
    }

    impl From<fetch::Ref> for JsonRef {
        fn from(value: fetch::Ref) -> Self {
            match value {
                fetch::Ref::Direct { path, object } => JsonRef::Direct {
                    path: path.to_string(),
                    object: object.to_string(),
                },
                fetch::Ref::Symbolic { path, target, object } => JsonRef::Symbolic {
                    path: path.to_string(),
                    target: target.to_string(),
                    object: object.to_string(),
                },
                fetch::Ref::Peeled { path, tag, object } => JsonRef::Peeled {
                    path: path.to_string(),
                    tag: tag.to_string(),
                    object: object.to_string(),
                },
            }
        }
    }

    pub(crate) fn print(mut out: impl std::io::Write, refs: &[fetch::Ref]) -> std::io::Result<()> {
        for r in refs {
            match r {
                fetch::Ref::Direct { path, object } => writeln!(&mut out, "{} {}", object.to_hex(), path),
                fetch::Ref::Peeled { path, object, tag } => {
                    writeln!(&mut out, "{} {} tag:{}", object.to_hex(), path, tag)
                }
                fetch::Ref::Symbolic { path, target, object } => {
                    writeln!(&mut out, "{} {} symref-target:{}", object.to_hex(), path, target)
                }
            }?;
        }
        Ok(())
    }
}
#[cfg(any(feature = "blocking-client", feature = "async-client"))]
pub use net::{refs, refs_fn as refs, JsonRef};
