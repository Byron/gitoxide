use std::path::{Path, PathBuf};

use crate::DOT_GIT_DIR;

pub(crate) fn shorten_path_with_cwd(cursor: PathBuf, cwd: &Path) -> PathBuf {
    fn comp_len(c: std::path::Component<'_>) -> usize {
        use std::path::Component;
        match c {
            Component::Prefix(p) => p.as_os_str().len(),
            Component::CurDir => 1,
            Component::ParentDir => 2,
            Component::Normal(p) => p.len(),
            Component::RootDir => 1,
        }
    }

    debug_assert_eq!(cursor.file_name().and_then(std::ffi::OsStr::to_str), Some(DOT_GIT_DIR));
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
}

/// Find the number of components parenting the `search_dir` before the first directory in `ceiling_dirs`.
/// `search_dir` needs to be normalized, and we normalize every ceiling as well.
pub(crate) fn find_ceiling_height(search_dir: &Path, ceiling_dirs: &[PathBuf], cwd: &Path) -> Option<usize> {
    if ceiling_dirs.is_empty() {
        return None;
    }

    let search_realpath;
    let search_dir = if search_dir.is_absolute() {
        search_dir
    } else {
        search_realpath = gix_path::realpath_opts(search_dir, cwd, gix_path::realpath::MAX_SYMLINKS).ok()?;
        search_realpath.as_path()
    };
    ceiling_dirs
        .iter()
        .filter_map(|ceiling_dir| {
            #[cfg(windows)]
            let ceiling_dir = dunce::simplified(ceiling_dir);
            let mut ceiling_dir = gix_path::normalize(ceiling_dir, cwd)?;
            if !ceiling_dir.is_absolute() {
                ceiling_dir = gix_path::normalize(cwd.join(ceiling_dir.as_ref()), cwd)?;
            }
            search_dir
                .strip_prefix(ceiling_dir.as_ref())
                .ok()
                .map(|path_relative_to_ceiling| path_relative_to_ceiling.components().count())
                .filter(|height| *height > 0)
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
