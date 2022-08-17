use crate::DOT_GIT_DIR;
use std::path::{Path, PathBuf};

pub(crate) fn shorten_path_with_cwd(cursor: PathBuf, cwd: Option<&Path>) -> PathBuf {
    fn comp_len(c: std::path::Component<'_>) -> usize {
        use std::path::Component::*;
        match c {
            Prefix(p) => p.as_os_str().len(),
            CurDir => 1,
            ParentDir => 2,
            Normal(p) => p.len(),
            RootDir => 1,
        }
    }

    if let Some(cwd) = cwd {
        debug_assert_eq!(cursor.file_name().and_then(|f| f.to_str()), Some(DOT_GIT_DIR));
        let parent = cursor.parent().expect(".git appended");
        cwd.strip_prefix(parent)
            .ok()
            .and_then(|path_relative_to_cwd| {
                let relative_path_components = path_relative_to_cwd.components().count();
                let current_component_len = cursor.components().map(comp_len).sum::<usize>();
                (relative_path_components * "..".len() < current_component_len).then(|| {
                    std::iter::repeat("..")
                        .take(relative_path_components)
                        .chain(Some(DOT_GIT_DIR))
                        .collect()
                })
            })
            .unwrap_or(cursor)
    } else {
        cursor
    }
}

/// Find the number of components parenting the `search_dir` before the first directory in `ceiling_dirs`.
/// `search_dir` needs to be absolutized, and we absolutize every ceiling as well.
pub(crate) fn find_ceiling_height(search_dir: &Path, ceiling_dirs: &[PathBuf], cwd: Option<&Path>) -> Option<usize> {
    ceiling_dirs
        .iter()
        .filter_map(|ceiling_dir| {
            let mut ceiling_dir = git_path::absolutize(ceiling_dir, cwd);
            match (search_dir.is_absolute(), ceiling_dir.is_absolute()) {
                (true, false) => ceiling_dir = cwd?.join(ceiling_dir.as_ref()).into(),
                (false, true) => {
                    let stripped = ceiling_dir.as_ref().strip_prefix(cwd?).ok()?.to_owned();
                    ceiling_dir = stripped.into();
                }
                (false, false) | (true, true) => {}
            };
            search_dir
                .strip_prefix(ceiling_dir.as_ref())
                .ok()
                .map(|path_relative_to_ceiling| path_relative_to_ceiling.components().count())
        })
        .min()
}

/// Returns the device ID of the directory.
#[cfg(target_os = "linux")]
pub(crate) fn device_id(m: &std::fs::Metadata) -> u64 {
    use std::os::linux::fs::MetadataExt;
    m.st_dev()
}

/// Returns the device ID of the directory.
#[cfg(all(unix, not(target_os = "linux")))]
pub(crate) fn device_id(m: &std::fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    m.dev()
}
