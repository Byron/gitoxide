use bstr::{BStr, ByteSlice};

///
#[allow(clippy::empty_docs)]
pub mod component {
    /// The error returned by [`component()`](super::component()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("A path component must not be empty")]
        Empty,
        #[error("Path separators like / or \\ are not allowed")]
        PathSeparator,
        #[error("Window path prefixes are not allowed")]
        WindowsPathPrefix,
        #[error("Windows device-names may have side-effects and are not allowed")]
        WindowsReservedName,
        #[error("Trailing spaces or dots and the following characters are forbidden in Windows paths, along with non-printable ones: <>:\"|?*")]
        WindowsIllegalCharacter,
        #[error("The .git name may never be used")]
        DotGitDir,
        #[error("The .gitmodules file must not be a symlink")]
        SymlinkedGitModules,
    }

    /// Further specify what to check for in [`component()`](super::component())
    ///
    /// Note that the `Default` implementation maximizes safety by enabling all protections.
    #[derive(Debug, Copy, Clone)]
    pub struct Options {
        /// This flag should be turned on when on Windows, but can be turned on when on other platforms
        /// as well to prevent path components that can cause trouble on Windows.
        pub protect_windows: bool,
        /// If `true`, protections for the MacOS HFS+ filesystem will be active, checking for
        /// special directories that we should never write while ignoring codepoints just like HFS+ would.
        ///
        /// This field is equivalent to `core.protectHFS`.
        pub protect_hfs: bool,
        /// If `true`, protections for Windows NTFS specific features will be active. This adds special handling
        /// for `8.3` filenames and alternate data streams, both of which could be used to mask th etrue name of
        /// what would be created on disk.
        ///
        /// This field is equivalent to `core.protectNTFS`.
        pub protect_ntfs: bool,
    }

    impl Default for Options {
        fn default() -> Self {
            Options {
                protect_windows: true,
                protect_hfs: true,
                protect_ntfs: true,
            }
        }
    }

    /// The mode of the component, if it's the leaf of a path.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Mode {
        /// The item is a symbolic link.
        Symlink,
    }
}

/// Assure the given `input` resembles a valid name for a tree or blob, and in that sense, a path component.
/// `mode` indicates the kind of `input` and it should be `Some` if `input` is the last component in the underlying
/// path. Currently, this is only used to determine if `.gitmodules` is a symlink.
///
/// `input` must not make it possible to exit the repository, or to specify absolute paths.
pub fn component(
    input: &BStr,
    mode: Option<component::Mode>,
    component::Options {
        protect_windows,
        protect_hfs,
        protect_ntfs,
    }: component::Options,
) -> Result<&BStr, component::Error> {
    if input.is_empty() {
        return Err(component::Error::Empty);
    }
    if protect_windows {
        if input.find_byteset(b"/\\").is_some() {
            return Err(component::Error::PathSeparator);
        }
        if input.chars().skip(1).next() == Some(':') {
            return Err(component::Error::WindowsPathPrefix);
        }
    } else if input.find_byte(b'/').is_some() {
        return Err(component::Error::PathSeparator);
    }
    if protect_hfs {
        if is_dot_hfs(input, "git") {
            return Err(component::Error::DotGitDir);
        }
        if is_symlink(mode) && is_dot_hfs(input, "gitmodules") {
            return Err(component::Error::SymlinkedGitModules);
        }
    }

    if protect_ntfs {
        if is_dot_git_ntfs(input) {
            return Err(component::Error::DotGitDir);
        }
        if is_symlink(mode) && is_dot_ntfs(input, "gitmodules", "gi7eba") {
            return Err(component::Error::SymlinkedGitModules);
        }

        if protect_windows {
            if let Some(err) = check_win_devices_and_illegal_characters(input) {
                return Err(err);
            }
        }
    }

    if !(protect_hfs | protect_ntfs) {
        if input.eq_ignore_ascii_case(b".git") {
            return Err(component::Error::DotGitDir);
        }
        if is_symlink(mode) && input.eq_ignore_ascii_case(b".gitmodules") {
            return Err(component::Error::SymlinkedGitModules);
        }
    }
    Ok(input)
}

fn check_win_devices_and_illegal_characters(input: &BStr) -> Option<component::Error> {
    let in3 = input.get(..3)?;
    if in3.eq_ignore_ascii_case(b"aux") && is_done_windows(input.get(3..)) {
        return Some(component::Error::WindowsReservedName);
    }
    if in3.eq_ignore_ascii_case(b"nul") && is_done_windows(input.get(3..)) {
        return Some(component::Error::WindowsReservedName);
    }
    if in3.eq_ignore_ascii_case(b"prn") && is_done_windows(input.get(3..)) {
        return Some(component::Error::WindowsReservedName);
    }
    if in3.eq_ignore_ascii_case(b"com")
        && input.get(3).map_or(false, |n| *n >= b'1' && *n <= b'9')
        && is_done_windows(input.get(4..))
    {
        return Some(component::Error::WindowsReservedName);
    }
    if in3.eq_ignore_ascii_case(b"lpt")
        && input.get(3).map_or(false, |n| n.is_ascii_digit())
        && is_done_windows(input.get(4..))
    {
        return Some(component::Error::WindowsReservedName);
    }
    if in3.eq_ignore_ascii_case(b"con")
        && ((input.get(3..6).map_or(false, |n| n.eq_ignore_ascii_case(b"in$")) && is_done_windows(input.get(6..)))
            || (input.get(3..7).map_or(false, |n| n.eq_ignore_ascii_case(b"out$")) && is_done_windows(input.get(7..))))
    {
        return Some(component::Error::WindowsReservedName);
    }
    if input.iter().find(|b| **b < 0x20 || b":<>\"|?*".contains(b)).is_some() {
        return Some(component::Error::WindowsIllegalCharacter);
    }
    if input.ends_with(b".") || input.ends_with(b" ") {
        return Some(component::Error::WindowsIllegalCharacter);
    }
    None
}

fn is_symlink(mode: Option<component::Mode>) -> bool {
    mode.map_or(false, |m| m == component::Mode::Symlink)
}

fn is_dot_hfs(input: &BStr, search_case_insensitive: &str) -> bool {
    let mut input = input.chars().filter(|c| match *c as u32 {
            0x200c | /* ZERO WIDTH NON-JOINER */
            0x200d | /* ZERO WIDTH JOINER */
            0x200e | /* LEFT-TO-RIGHT MARK */
            0x200f | /* RIGHT-TO-LEFT MARK */
            0x202a | /* LEFT-TO-RIGHT EMBEDDING */
            0x202b | /* RIGHT-TO-LEFT EMBEDDING */
            0x202c | /* POP DIRECTIONAL FORMATTING */
            0x202d | /* LEFT-TO-RIGHT OVERRIDE */
            0x202e | /* RIGHT-TO-LEFT OVERRIDE */
            0x206a | /* INHIBIT SYMMETRIC SWAPPING */
            0x206b | /* ACTIVATE SYMMETRIC SWAPPING */
            0x206c | /* INHIBIT ARABIC FORM SHAPING */
            0x206d | /* ACTIVATE ARABIC FORM SHAPING */
            0x206e | /* NATIONAL DIGIT SHAPES */
            0x206f | /* NOMINAL DIGIT SHAPES */
            0xfeff => false, /* ZERO WIDTH NO-BREAK SPACE */
            _ => true
        });
    if input.next() != Some('.') {
        return false;
    }

    let mut comp = search_case_insensitive.chars();
    loop {
        match (comp.next(), input.next()) {
            (Some(a), Some(b)) => {
                if !a.eq_ignore_ascii_case(&b) {
                    return false;
                }
            }
            (None, None) => return true,
            _ => return false,
        }
    }
}

fn is_dot_git_ntfs(input: &BStr) -> bool {
    if input
        .get(..4)
        .map_or(false, |input| input.eq_ignore_ascii_case(b".git"))
    {
        return is_done_ntfs(input.get(4..));
    }
    if input
        .get(..5)
        .map_or(false, |input| input.eq_ignore_ascii_case(b"git~1"))
    {
        return is_done_ntfs(input.get(5..));
    }
    false
}

fn is_dot_ntfs(input: &BStr, search_case_insensitive: &str, ntfs_shortname_prefix: &str) -> bool {
    if input.get(0) == Some(&b'.') {
        let end_pos = 1 + search_case_insensitive.len();
        if input.get(1..end_pos).map_or(false, |input| {
            input.eq_ignore_ascii_case(search_case_insensitive.as_bytes())
        }) {
            is_done_ntfs(input.get(end_pos..))
        } else {
            false
        }
    } else {
        let search_case_insensitive: &[u8] = search_case_insensitive.as_bytes();
        if search_case_insensitive
            .get(..6)
            .zip(input.get(..6))
            .map_or(false, |(ntfs_prefix, first_6_of_input)| {
                first_6_of_input.eq_ignore_ascii_case(ntfs_prefix)
                    && input.get(6) == Some(&b'~')
                    && input.get(7).map_or(false, |num| num >= &b'1' && num <= &b'4')
            })
        {
            return is_done_ntfs(input.get(8..));
        }

        let ntfs_shortname_prefix: &[u8] = ntfs_shortname_prefix.as_bytes();
        let mut saw_tilde = false;
        let mut pos = 0;
        while pos < 8 {
            let Some(b) = input.get(pos).copied() else {
                return false;
            };
            if saw_tilde {
                if b < b'0' || b > b'9' {
                    return false;
                }
            } else if b == b'~' {
                saw_tilde = true;
                pos += 1;
                let Some(b) = input.get(pos).copied() else {
                    return false;
                };
                if b < b'1' || b > b'9' {
                    return false;
                }
            } else if pos >= 6 {
                return false;
            } else if b & 0x80 == 0x80 {
                return false;
            } else if ntfs_shortname_prefix
                .get(pos)
                .map_or(true, |ob| !b.eq_ignore_ascii_case(ob))
            {
                return false;
            }
            pos += 1;
        }
        is_done_ntfs(input.get(pos..))
    }
}

fn is_done_ntfs(input: Option<&[u8]>) -> bool {
    let Some(input) = input else { return true };
    for b in input.bytes() {
        if b == b':' {
            return true;
        }
        if b != b' ' && b != b'.' {
            return false;
        }
    }
    true
}

fn is_done_windows(input: Option<&[u8]>) -> bool {
    let Some(input) = input else { return true };
    let skip = input.bytes().take_while(|b| *b == b' ').count();
    let Some(next) = input.get(skip) else { return true };
    !(*next != b'.' && *next != b':')
}
