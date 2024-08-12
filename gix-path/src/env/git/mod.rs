use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use bstr::{BStr, BString, ByteSlice};
use std::sync::LazyLock;

/// Other places to find Git in.
#[cfg(windows)]
pub(super) static ALTERNATIVE_LOCATIONS: LazyLock<Vec<PathBuf>> =
    LazyLock::new(|| locations_under_program_files(|key| std::env::var_os(key)));
#[cfg(not(windows))]
pub(super) static ALTERNATIVE_LOCATIONS: LazyLock<Vec<PathBuf>> = LazyLock::new(Vec::new);

#[cfg(windows)]
fn locations_under_program_files<F>(var_os_func: F) -> Vec<PathBuf>
where
    F: Fn(&str) -> Option<std::ffi::OsString>,
{
    // Should give a 64-bit program files path from a 32-bit or 64-bit process on a 64-bit system.
    let varname_64bit = "ProgramW6432";

    // Should give a 32-bit program files path from a 32-bit or 64-bit process on a 64-bit system.
    // This variable is x86-specific, but neither Git nor Rust target 32-bit ARM on Windows.
    let varname_x86 = "ProgramFiles(x86)";

    // Should give a 32-bit program files path on a 32-bit system. We also check this on a 64-bit
    // system, even though it *should* equal the process's architecture specific variable, so that
    // we cover the case of a parent process that passes down an overly sanitized environment that
    // lacks the architecture-specific variable. On a 64-bit system, because parent and child
    // processes' architectures can be different, Windows sets the child's ProgramFiles variable
    // from the ProgramW6432 or ProgramFiles(x86) variable applicable to the child's architecture.
    // Only if the parent does not pass that down is the passed-down ProgramFiles variable even
    // used. But this behavior is not well known, so that situation does sometimes happen.
    let varname_current = "ProgramFiles";

    // 64-bit relative bin dir. So far, this is always mingw64, not ucrt64, clang64, or clangarm64.
    let suffix_64 = Path::new(r"Git\mingw64\bin");

    // 32-bit relative bin dir. So far, this is always mingw32, not clang32.
    let suffix_32 = Path::new(r"Git\mingw32\bin");

    // Whichever of the 64-bit or 32-bit relative bin better matches this process's architecture.
    // Unlike the system architecture, the process architecture is always known at compile time.
    #[cfg(target_pointer_width = "64")]
    let suffix_current = suffix_64;
    #[cfg(target_pointer_width = "32")]
    let suffix_current = suffix_32;

    let rules = [
        (varname_64bit, suffix_64),
        (varname_x86, suffix_32),
        (varname_current, suffix_current),
    ];

    let mut locations = vec![];

    for (name, suffix) in rules {
        let Some(pf) = var_os_func(name) else { continue };
        let pf = Path::new(&pf);
        if pf.is_relative() {
            // This shouldn't happen, but if it does then don't use the path. This is mainly in
            // case we are accidentally invoked with the environment variable set but empty.
            continue;
        }
        let location = pf.join(suffix);
        if !locations.contains(&location) {
            locations.push(location);
        }
    }

    locations
}

#[cfg(windows)]
pub(super) static EXE_NAME: &str = "git.exe";
#[cfg(not(windows))]
pub(super) static EXE_NAME: &str = "git";

/// Invoke the git executable to obtain the origin configuration, which is cached and returned.
///
/// The git executable is the one found in PATH or an alternative location.
pub(super) static EXE_INFO: LazyLock<Option<BString>> = LazyLock::new(|| {
    let git_cmd = |executable: PathBuf| {
        let mut cmd = Command::new(executable);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd.args(["config", "-l", "--show-origin"])
            .stdin(Stdio::null())
            .stderr(Stdio::null());
        cmd
    };
    let mut cmd = git_cmd(EXE_NAME.into());
    gix_trace::debug!(cmd = ?cmd, "invoking git for installation config path");
    let cmd_output = match cmd.output() {
        Ok(out) => out.stdout,
        #[cfg(windows)]
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            let executable = ALTERNATIVE_LOCATIONS.iter().find_map(|prefix| {
                let candidate = prefix.join(EXE_NAME);
                candidate.is_file().then_some(candidate)
            })?;
            gix_trace::debug!(cmd = ?cmd, "invoking git for installation config path in alternate location");
            git_cmd(executable).output().ok()?.stdout
        }
        Err(_) => return None,
    };

    first_file_from_config_with_origin(cmd_output.as_slice().into()).map(ToOwned::to_owned)
});

/// Try to find the file that contains git configuration coming with the git installation.
///
/// This returns the configuration associated with the `git` executable found in the current `PATH`
/// or an alternative location, or `None` if no `git` executable was found or there were other
/// errors during execution.
pub(super) fn install_config_path() -> Option<&'static BStr> {
    let _span = gix_trace::detail!("gix_path::git::install_config_path()");
    static PATH: LazyLock<Option<BString>> = LazyLock::new(|| {
        // Shortcut: Specifically in Git for Windows 'Git Bash' shells, this variable is set. It
        // may let us deduce the installation directory, so we can save the `git` invocation.
        #[cfg(windows)]
        if let Some(mut exec_path) = std::env::var_os("EXEPATH").map(PathBuf::from) {
            exec_path.push("etc");
            exec_path.push("gitconfig");
            return crate::os_string_into_bstring(exec_path.into()).ok();
        }
        EXE_INFO.clone()
    });
    PATH.as_ref().map(AsRef::as_ref)
}

fn first_file_from_config_with_origin(source: &BStr) -> Option<&BStr> {
    let file = source.strip_prefix(b"file:")?;
    let end_pos = file.find_byte(b'\t')?;
    file[..end_pos].trim_with(|c| c == '"').as_bstr().into()
}

/// Given `config_path` as obtained from `install_config_path()`, return the path of the git installation base.
pub(super) fn config_to_base_path(config_path: &Path) -> &Path {
    config_path
        .parent()
        .expect("config file paths always have a file name to pop")
}

#[cfg(test)]
mod tests;
