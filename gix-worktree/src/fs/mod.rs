use std::path::PathBuf;

use bstr::BString;

/// A stack of path components with the delegation of side-effects as the currently set path changes, component by component.
#[derive(Clone)]
pub struct Stack {
    /// The prefix/root for all paths we handle.
    root: PathBuf,
    /// the most recent known cached that we know is valid.
    current: PathBuf,
    /// The relative portion of `valid` that was added previously.
    current_relative: PathBuf,
    /// The amount of path components of 'current' beyond the roots components.
    valid_components: usize,
    /// If set, we assume the `current` element is a directory to affect calls to `(push|pop)_directory()`.
    current_is_directory: bool,
}

/// A cache for efficiently executing operations on directories and files which are encountered in sorted order.
/// That way, these operations can be re-used for subsequent invocations in the same directory.
///
/// This cache can be configured to create directories efficiently, read git-ignore files and git-attribute files,
/// in any combination.
///
/// A cache for directory creation to reduce the amount of stat calls when creating
/// directories safely, that is without following symlinks that might be on the way.
///
/// As a special case, it offers a 'prefix' which (by itself) is assumed to exist and may contain symlinks.
/// Everything past that prefix boundary must not contain a symlink. We do this by allowing any input path.
///
/// Another added benefit is its ability to store the path of full path of the entry to which leading directories
/// are to be created to avoid allocating memory.
///
/// For this to work, it remembers the last 'good' path to a directory and assumes that all components of it
/// are still valid, too.
/// As directories are created, the cache will be adjusted to reflect the latest seen directory.
///
/// The caching is only useful if consecutive calls to create a directory are using a sorted list of entries.
#[derive(Clone)]
pub struct Cache {
    stack: Stack,
    /// tells us what to do as we change paths.
    state: cache::State,
    /// A buffer used when reading attribute or ignore files or their respective objects from the object database.
    buf: Vec<u8>,
    /// If case folding should happen when looking up attributes or exclusions.
    case: gix_glob::pattern::Case,
    /// A lookup table for object ids to read from in some situations when looking up attributes or exclusions.
    attribute_files_in_index: Vec<PathOidMapping>,
}

pub(crate) type PathOidMapping = (BString, gix_hash::ObjectId);

///
pub mod cache;
///
pub mod stack;
