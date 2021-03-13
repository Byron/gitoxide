use bstr::ByteSlice;
use git_config::file::GitConfig;
use git_features::progress::Progress;
use std::convert::TryFrom;
use std::io::Read;
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

fn find_git_repository_workdirs<P: Progress>(root: impl AsRef<Path>, mut progress: P) -> impl Iterator<Item = PathBuf>
where
    <P as Progress>::SubProgress: Sync,
{
    progress.init(None, git_features::progress::count("filesystem items"));
    fn is_repository(path: &PathBuf) -> bool {
        if !(path.is_dir() && path.ends_with(".git")) {
            return false;
        }
        path.join("HEAD").is_file() && path.join("config").is_file()
    }
    fn into_workdir(path: PathBuf) -> PathBuf {
        fn is_bare(path: &Path) -> bool {
            !path.join("index").exists()
        }
        if is_bare(&path) {
            path
        } else {
            path.parent().expect("git is never in the root").to_owned()
        }
    }

    #[derive(Debug)]
    struct State {
        is_repo: bool,
    }

    impl Default for State {
        fn default() -> Self {
            State { is_repo: false }
        }
    }

    let walk = jwalk::WalkDirGeneric::<((), State)>::new(root)
        .follow_links(false)
        .sort(true)
        .skip_hidden(false);

    // On macos with apple silicon, the IO subsystem is entirely different and one thread can mostly max it out.
    // Thus using more threads just burns energy unnecessarily.
    // It's notable that `du` is very fast even on a single core and more power efficient than dua with a single core.
    // The default of '4' seems related to the amount of performance cores present in the system.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let walk = walk.parallelism(jwalk::Parallelism::RayonNewPool(4));

    walk.process_read_dir(move |_depth, _path, _read_dir_state, children| {
        let mut found_repo = false;
        for entry in children.iter_mut() {
            if let Ok(e) = entry {
                if is_repository(&e.path()) {
                    e.client_state = State { is_repo: true };
                    e.read_children_path = None;
                    found_repo = true;
                    break;
                }
            }
        }
        if found_repo {
            children.retain(|e| e.as_ref().map(|e| e.client_state.is_repo).unwrap_or(false));
        }
    })
    .into_iter()
    .inspect(move |_| progress.inc())
    .filter_map(Result::ok)
    .filter(|e| e.client_state.is_repo)
    .map(|e| into_workdir(e.path()))
}

fn find_origin_remote(repo: &Path) -> anyhow::Result<Option<git_url::Url>> {
    let mut config_bytes = vec![];
    let config = {
        let mut file = std::fs::File::open(repo.join("./config"))?;
        file.read_to_end(&mut config_bytes)?;
        GitConfig::try_from(&config_bytes).map_err(|e| e.to_owned())?
    };
    Ok(config.value::<git_url::Url>("remote", Some("origin"), "url").ok())
}

fn handle(
    mode: Mode,
    git_workdir: &Path,
    canonicalized_destination: &Path,
    progress: &mut impl Progress,
) -> anyhow::Result<()> {
    fn to_relative(path: PathBuf) -> PathBuf {
        path.components()
            .skip_while(|c| c == &std::path::Component::RootDir)
            .collect()
    }

    fn find_parent_repo(mut git_workdir: &Path) -> Option<PathBuf> {
        while let Some(parent) = git_workdir.parent() {
            let has_contained_git_folder_or_file = std::fs::read_dir(parent).ok()?.any(|e| {
                e.ok()
                    .and_then(|e| {
                        e.file_name()
                            .to_str()
                            .map(|name| name == ".git" && e.path() != git_workdir)
                    })
                    .unwrap_or(false)
            });
            if has_contained_git_folder_or_file {
                return Some(parent.to_owned());
            }
            git_workdir = parent;
        }
        None
    }

    if let Some(parent_repo_path) = find_parent_repo(git_workdir) {
        progress.fail(format!(
            "Skipping repository at {:?} as it is nested within repository {:?}",
            git_workdir.display(),
            parent_repo_path
        ));
        return Ok(());
    }

    let url = match find_origin_remote(git_workdir)? {
        None => {
            progress.info(format!(
                "Skipping repository {:?} without 'origin' remote",
                git_workdir.display()
            ));
            return Ok(());
        }
        Some(url) => url,
    };
    if url.path.is_empty() {
        progress.info(format!(
            "Skipping repository at {:?} whose remote does not have a path: {:?}",
            git_workdir.display(),
            url.to_string()
        ));
        return Ok(());
    }

    let destination = canonicalized_destination
        .join(
            url.host
                .as_ref()
                .ok_or_else(|| anyhow::Error::msg(format!("Remote URLs must have host names: {}", url)))?,
        )
        .join(to_relative(git_url::expand_path(None, url.path.as_bstr())?));

    if let Ok(destination) = destination.canonicalize() {
        if git_workdir.canonicalize()? == destination {
            progress.info(format!(
                "Skipping {:?} as it is in the correct spot",
                git_workdir.display()
            ));
            return Ok(());
        }
    }
    match mode {
        Mode::Simulate => progress.info(format!(
            "WOULD move {} to {}",
            git_workdir.display(),
            destination.display()
        )),
        Mode::Execute => {
            std::fs::create_dir_all(destination.parent().expect("repo destination is not the root"))?;
            progress.done(format!("Moving {} to {}", git_workdir.display(), destination.display()));
            std::fs::rename(git_workdir, &destination)?;
        }
    }
    Ok(())
}

/// Find all working directories in the given `source_dir` and print them to `out` while providing `progress`.
pub fn discover<P: Progress>(
    source_dir: impl AsRef<Path>,
    mut out: impl std::io::Write,
    mut progress: P,
) -> anyhow::Result<()>
where
    <<P as Progress>::SubProgress as Progress>::SubProgress: Sync,
{
    for git_workdir in find_git_repository_workdirs(source_dir, progress.add_child("Searching repositories")) {
        writeln!(&mut out, "{}", git_workdir.display())?;
    }
    Ok(())
}

pub fn run<P: Progress>(
    mode: Mode,
    source_dir: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    mut progress: P,
) -> anyhow::Result<()>
where
    <<P as Progress>::SubProgress as Progress>::SubProgress: Sync,
{
    let mut num_errors = 0usize;
    let destination = destination.as_ref().canonicalize()?;
    for path_to_move in find_git_repository_workdirs(source_dir, progress.add_child("Searching repositories")) {
        if let Err(err) = handle(mode, &path_to_move, &destination, &mut progress) {
            progress.fail(format!(
                "Error when handling directory {:?}: {}",
                path_to_move.display(),
                err.to_string()
            ));
            num_errors += 1;
        }
    }

    if num_errors > 0 {
        anyhow::bail!("Failed to handle {} repositories", num_errors)
    } else {
        Ok(())
    }
}
