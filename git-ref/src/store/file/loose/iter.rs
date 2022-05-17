use std::path::{Path, PathBuf};

use git_features::fs::walkdir::DirEntryIter;
use git_object::bstr::ByteSlice;

use crate::file::iter::LooseThenPacked;
use crate::{store_impl::file, BString, FullName};

/// An iterator over all valid loose reference paths as seen from a particular base directory.
pub(in crate::store_impl::file) struct SortedLoosePaths {
    pub(crate) base: PathBuf,
    filename_prefix: Option<BString>,
    file_walk: DirEntryIter,
}

impl SortedLoosePaths {
    pub fn at_root_with_filename_prefix(
        path: impl AsRef<Path>,
        base: impl Into<PathBuf>,
        filename_prefix: Option<BString>,
    ) -> Self {
        let file_walk = git_features::fs::walkdir_sorted_new(path).into_iter();
        SortedLoosePaths {
            base: base.into(),
            filename_prefix,
            file_walk,
        }
    }
}

impl Iterator for SortedLoosePaths {
    type Item = std::io::Result<(PathBuf, FullName)>;

    fn next(&mut self) -> Option<Self::Item> {
        for entry in self.file_walk.by_ref() {
            match entry {
                Ok(entry) => {
                    if !entry.file_type().is_file() {
                        continue;
                    }
                    let full_path = entry.path().to_owned();
                    if let Some((prefix, name)) = self
                        .filename_prefix
                        .as_deref()
                        .and_then(|prefix| full_path.file_name().map(|name| (prefix, name)))
                    {
                        match git_path::os_str_into_bstr(name) {
                            Ok(name) => {
                                if !name.starts_with(prefix) {
                                    continue;
                                }
                            }
                            Err(_) => continue, // TODO: silently skipping ill-formed UTF-8 on windows - maybe this can be better?
                        }
                    }
                    let full_name = full_path
                        .strip_prefix(&self.base)
                        .expect("prefix-stripping cannot fail as prefix is our root");
                    let full_name = match git_path::try_into_bstr(full_name) {
                        Ok(name) => {
                            let name = git_path::to_unix_separators_on_windows(name);
                            name.into_owned()
                        }
                        Err(_) => continue, // TODO: silently skipping ill-formed UTF-8 on windows here, maybe there are better ways?
                    };

                    if git_validate::reference::name_partial(full_name.as_bstr()).is_ok() {
                        let name = FullName(full_name);
                        return Some(Ok((full_path, name)));
                    } else {
                        continue;
                    }
                }
                Err(err) => return Some(Err(err.into_io_error().expect("no symlink related errors"))),
            }
        }
        None
    }
}

impl file::Store {
    /// Return an iterator over all loose references, notably not including any packed ones, in lexical order.
    /// Each of the references may fail to parse and the iterator will not stop if parsing fails, allowing the caller
    /// to see all files that look like references whether valid or not.
    ///
    /// Reference files that do not constitute valid names will be silently ignored.
    pub fn loose_iter(&self) -> std::io::Result<LooseThenPacked<'_, '_>> {
        self.iter_packed(None)
    }

    /// Return an iterator over all loose references that start with the given `prefix`.
    ///
    /// Otherwise it's similar to [`loose_iter()`][file::Store::loose_iter()].
    pub fn loose_iter_prefixed(&self, prefix: impl AsRef<Path>) -> std::io::Result<LooseThenPacked<'_, '_>> {
        self.iter_prefixed_packed(prefix, None)
    }
}

impl file::Store {
    pub(in crate::store_impl::file) fn common_and_refs_dir(&self) -> (&Path, PathBuf) {
        let commondir = self.common_dir_resolved();
        let refs = commondir.join("refs");
        (commondir, refs)
    }

    pub(in crate::store_impl::file) fn validate_prefix(
        &self,
        base: &Path,
        prefix: &Path,
    ) -> std::io::Result<(PathBuf, Option<BString>)> {
        if prefix.is_absolute() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "prefix must be a relative path, like 'refs/heads'",
            ));
        }
        use std::path::Component::*;
        if prefix.components().any(|c| matches!(c, CurDir | ParentDir)) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Refusing to handle prefixes with relative path components",
            ));
        }
        let base = base.join(prefix);
        if base.is_dir() {
            Ok((base, None))
        } else {
            Ok((
                base.parent().expect("a parent is always there unless empty").to_owned(),
                base.file_name()
                    .map(ToOwned::to_owned)
                    .map(|p| {
                        git_path::try_into_bstr(PathBuf::from(p))
                            .map(|p| p.into_owned())
                            .map_err(|_| {
                                std::io::Error::new(
                                    std::io::ErrorKind::InvalidInput,
                                    "prefix contains ill-formed UTF-8",
                                )
                            })
                    })
                    .transpose()?,
            ))
        }
    }
}
