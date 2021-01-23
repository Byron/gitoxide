use git_features::{fs, progress::Progress};
use std::path::{Path, PathBuf};

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

// TODO: handle nested repos, skip everything inside a parent directory.
fn find_git_repositories(root: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
    fn is_repository(path: &PathBuf) -> bool {
        path.is_dir() && path.ends_with(".git")
    }

    let walk = fs::sorted(fs::WalkDir::new(root).follow_links(false));
    walk.into_iter()
        .filter_map(Result::ok)
        .map(|entry: fs::DirEntry| fs::direntry_path(&entry))
        .filter(is_repository)
}

pub fn run(_mode: Mode, source_dir: PathBuf, _destination: PathBuf, _progress: impl Progress) -> anyhow::Result<()> {
    let _repo_paths = find_git_repositories(source_dir);
    Ok(())
}

mod parse {
    use anyhow::{bail, Context};
    use bstr::{BStr, ByteSlice};

    #[allow(unused)]
    fn remotes_from_git_remote_verbose(input: &[u8]) -> anyhow::Result<Vec<(&BStr, git_url::Url)>> {
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
        use bstr::ByteSlice;

        static GITOXIDE_REMOTES: &[u8] = br#"commitgraph	https://github.com/avoidscorn/gitoxide (fetch)
commitgraph	https://github.com/avoidscorn/gitoxide (push)
origin	https://github.com/Byron/gitoxide (fetch)
origin	https://github.com/Byron/gitoxide (push)
rad	rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git (fetch)
rad	rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git (push)
"#;
        fn url(input: &str) -> git_url::Url {
            git_url::Url::from_bytes(input.as_bytes()).expect("valid url")
        }

        #[test]
        fn valid_verbose_remotes() -> anyhow::Result<()> {
            assert_eq!(
                remotes_from_git_remote_verbose(GITOXIDE_REMOTES)?,
                vec![
                    (b"commitgraph".as_bstr(), url("https://github.com/avoidscorn/gitoxide")),
                    (b"commitgraph".as_bstr(), url("https://github.com/avoidscorn/gitoxide")),
                    (b"origin".as_bstr(), url("https://github.com/Byron/gitoxide")),
                    (b"origin".as_bstr(), url("https://github.com/Byron/gitoxide")),
                    (
                        b"rad".as_bstr(),
                        url("rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git")
                    ),
                    (
                        b"rad".as_bstr(),
                        url("rad://hynkuwzskprmswzeo4qdtku7grdrs4ffj3g9tjdxomgmjzhtzpqf81@hwd1yregyf1dudqwkx85x5ps3qsrqw3ihxpx3ieopq6ukuuq597p6m8161c.git")
                    )
                ]
            );
            Ok(())
        }
    }
}
