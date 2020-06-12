use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    path::PathBuf,
};

quick_error! {
    #[derive(Debug)]
    pub enum InitError {
        IoOpen(err: std::io::Error, path: PathBuf) {
            display("Could not open file at '{}'", path.display())
            cause(err)
        }
        IoWrite(err: std::io::Error, path: PathBuf) {
            display("Could not write file at '{}'", path.display())
            cause(err)
        }
        DirectoryExists(path: PathBuf) {
            display("Refusing to initialize the existing '{}' directory", path.display())
        }
        CreateDirectory(err: std::io::Error, path: PathBuf) {
            display("Cold not create directory at '{}'", path.display())
            cause(err)
        }
    }
}

const GIT_DIR_NAME: &'static str = ".git";

const TPL_INFO_EXCLUDE: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/info/exclude");
const TPL_HOOKS_APPLYPATCH_MSG: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/applypatch-msg.sample");
const TPL_HOOKS_COMMIT_MSG: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/commit-msg.sample");
const TPL_HOOKS_FSMONITOR_WATCHMAN: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/fsmonitor-watchman.sample");
const TPL_HOOKS_POST_UPDATE: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/post-update.sample");
const TPL_HOOKS_PRE_APPLYPATCH: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/pre-applypatch.sample");
const TPL_HOOKS_PRE_COMMIT: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/pre-commit.sample");
const TPL_HOOKS_PRE_MERGE_COMMIT: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/pre-merge-commit.sample");
const TPL_HOOKS_PRE_PUSH: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/pre-push.sample");
const TPL_HOOKS_PRE_REBASE: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/pre-rebase.sample");
const TPL_HOOKS_PRE_RECEIVE: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/pre-receive.sample");
const TPL_HOOKS_PREPARE_COMMIT_MSG: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/prepare-commit-msg.sample");
const TPL_HOOKS_UPDATE: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/hooks/update.sample");
const TPL_CONFIG: &'static [u8] = include_bytes!("../../tests/snapshots/cli/baseline-init/config");
const TPL_DESCRIPTION: &'static [u8] =
    include_bytes!("../../tests/snapshots/cli/baseline-init/description");
const TPL_HEAD: &'static [u8] = include_bytes!("../../tests/snapshots/cli/baseline-init/HEAD");

struct PathCursor<'a>(&'a mut PathBuf);

struct NewDir<'a>(&'a mut PathBuf);

impl<'a> PathCursor<'a> {
    fn at(&mut self, component: &str) -> &Path {
        self.0.push(component);
        self.0.as_path()
    }
}

impl<'a> NewDir<'a> {
    fn at(self, component: &str) -> Result<Self, InitError> {
        self.0.push(component);
        create_dir(&self.0)?;
        Ok(self)
    }
    fn as_mut(&mut self) -> &mut PathBuf {
        self.0
    }
}

impl<'a> Drop for NewDir<'a> {
    fn drop(&mut self) {
        self.0.pop();
    }
}

impl<'a> Drop for PathCursor<'a> {
    fn drop(&mut self) {
        self.0.pop();
    }
}

fn write_file(data: &[u8], path: &Path) -> Result<(), InitError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(path)
        .map_err(|e| InitError::IoOpen(e, path.to_owned()))?;
    file.write_all(data)
        .map_err(|e| InitError::IoWrite(e, path.to_owned()))
}

fn create_dir(p: &Path) -> Result<(), InitError> {
    fs::create_dir(p).map_err(|e| InitError::CreateDirectory(e, p.to_owned()))
}

pub fn init() -> Result<(), InitError> {
    let mut cursor = PathBuf::from(GIT_DIR_NAME);
    if cursor.is_dir() {
        return Err(InitError::DirectoryExists(cursor));
    }
    create_dir(&cursor)?;

    {
        let mut cursor = NewDir(&mut cursor).at("info")?;
        write_file(TPL_INFO_EXCLUDE, PathCursor(cursor.as_mut()).at("exclude"))?;
    }

    {
        let mut cursor = NewDir(&mut cursor).at("hooks")?;
        for (tpl, filename) in &[
            (TPL_HOOKS_UPDATE, "update.sample"),
            (TPL_HOOKS_PREPARE_COMMIT_MSG, "prepare-commit-msg.sample"),
            (TPL_HOOKS_PRE_RECEIVE, "pre-receive.sample"),
            (TPL_HOOKS_PRE_REBASE, "pre-rebase.sample"),
            (TPL_HOOKS_PRE_PUSH, "pre-push.sample"),
            (TPL_HOOKS_PRE_COMMIT, "pre-commit.sample"),
            (TPL_HOOKS_PRE_MERGE_COMMIT, "pre-merge-commit.sample"),
            (TPL_HOOKS_PRE_APPLYPATCH, "pre-applypatch.sample"),
            (TPL_HOOKS_POST_UPDATE, "post-update.sample"),
            (TPL_HOOKS_FSMONITOR_WATCHMAN, "fsmonitor-watchman.sample"),
            (TPL_HOOKS_COMMIT_MSG, "commit-msg.sample"),
            (TPL_HOOKS_APPLYPATCH_MSG, "applypatch-msg.sample"),
        ] {
            write_file(tpl, PathCursor(cursor.as_mut()).at(filename))?;
        }
    }

    {
        let mut cursor = NewDir(&mut cursor).at("objects")?;
        create_dir(PathCursor(cursor.as_mut()).at("info"))?;
        create_dir(PathCursor(cursor.as_mut()).at("pack"))?;
    }

    {
        let mut cursor = NewDir(&mut cursor).at("refs")?;
        create_dir(PathCursor(cursor.as_mut()).at("heads"))?;
        create_dir(PathCursor(cursor.as_mut()).at("tags"))?;
    }

    for (tpl, filename) in &[
        (TPL_HEAD, "HEAD"),
        (TPL_DESCRIPTION, "description"),
        (TPL_CONFIG, "config"),
    ] {
        write_file(tpl, PathCursor(&mut cursor).at(filename))?;
    }

    Ok(())
}
