use git_features::progress::Progress;
use std::path::PathBuf;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Execute,
    Simulate,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Simulate
    }
}

pub fn run(_mode: Mode, _source_dir: PathBuf, _destination: PathBuf, _progress: impl Progress) -> anyhow::Result<()> {
    Ok(())
}

mod parse {
    use anyhow::{bail, Context};
    use bstr::{BStr, ByteSlice};

    #[allow(unused)]
    fn verbose_remotes(input: &[u8]) -> anyhow::Result<Vec<(&BStr, git_url::Url)>> {
        fn parse_line(line: &BStr) -> anyhow::Result<(&BStr, git_url::Url)> {
            let mut tokens = line.splitn(2, |b| *b == b'\t');
            Ok(match (tokens.next(), tokens.next(), tokens.next()) {
                (Some(remote), Some(url_and_type), None) => {
                    let mut tokens = url_and_type.splitn(2, |b| *b == b' ');
                    match (tokens.next(), tokens.next(), tokens.next()) {
                        (Some(url), Some(_type), None) => (remote.as_bstr(), git_url::parse(url)?),
                        _ => bail!("None or more than one 'space' as separator"),
                    }
                }
                _ => bail!("None or more than one tab as separator"),
            })
        }

        let mut out = Vec::new();
        for line in input.split(|b| *b == b'\n') {
            let line = line.as_bstr();
            if line.trim().is_empty() {
                continue;
            }
            out.push(
                parse_line(line).with_context(|| format!("Line {:?} should be <origin>TAB<URL>SPACE<TYPE>", line))?,
            );
        }

        Ok(out)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        static GITOXIDE_REMOTES: &[u8] = br#"commitgraph	https://github.com/avoidscorn/gitoxide (fetch)
commitgraph	https://github.com/avoidscorn/gitoxide (push)
origin	https://github.com/Byron/gitoxide (fetch)
origin	https://github.com/Byron/gitoxide (push)
rad	rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git (fetch)
rad	rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git (push)
"#;
        #[test]
        fn valid_verbose_remotes() -> anyhow::Result<()> {
            assert_eq!(verbose_remotes(GITOXIDE_REMOTES)?, vec![]);
            Ok(())
        }
    }
}
