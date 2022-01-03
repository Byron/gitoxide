use cargo_metadata::{
    camino::{Utf8Path, Utf8PathBuf},
    Package,
};
use git_repository as git;

use crate::{
    changelog::{section::segment, Section},
    commit, ChangeLog,
};

#[derive(Clone, Copy)]
pub enum State {
    Created,
    Modified,
    Unchanged,
}

impl State {
    pub fn is_modified(&self) -> bool {
        !matches!(self, State::Unchanged)
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Created => "created",
            State::Modified => "modified",
            State::Unchanged => "unchanged",
        }
    }
}

pub struct Outcome {
    pub log: ChangeLog,
    pub state: State,
    pub lock: git::lock::File,
    pub previous_content: Option<String>,
}

impl ChangeLog {
    pub fn for_package_with_write_lock<'a>(
        package: &'a Package,
        history: &commit::History,
        ctx: &'a crate::Context,
        selection: segment::Selection,
    ) -> anyhow::Result<Outcome> {
        let mut generated = ChangeLog::from_history_segments(
            package,
            &crate::git::history::crate_ref_segments(
                package,
                ctx,
                history,
                crate::git::history::SegmentScope::EntireHistory,
            )?,
            &ctx.repo,
            selection,
        );
        generated.sections.insert(
            0,
            Section::Verbatim {
                text: include_str!("header.md").to_owned(),
                generated: true,
            },
        );
        let changelog_path = path_from_manifest(&package.manifest_path);
        let lock =
            git::lock::File::acquire_to_update_resource(&changelog_path, git::lock::acquire::Fail::Immediately, None)?;
        let (log, state, previous_content) = if let Ok(markdown) = std::fs::read_to_string(changelog_path) {
            let existing_log = ChangeLog::from_markdown(&markdown);
            let copy_of_existing = existing_log.clone();
            let merged = existing_log.merge_generated(generated);
            let changed = merged != copy_of_existing;
            (
                merged,
                if changed { State::Modified } else { State::Unchanged },
                Some(markdown),
            )
        } else {
            (generated, State::Created, None)
        };
        Ok(Outcome {
            log,
            state,
            lock,
            previous_content,
        })
    }

    pub fn for_crate_by_name_with_write_lock<'a>(
        package: &'a Package,
        history: &commit::History,
        ctx: &'a crate::Context,
        selection: segment::Selection,
    ) -> anyhow::Result<(Outcome, &'a Package)> {
        let out = Self::for_package_with_write_lock(package, history, ctx, selection)?;
        Ok((out, package))
    }

    pub fn from_history_segments(
        package: &Package,
        segments: &[commit::history::Segment<'_>],
        repo: &git::easy::Handle,
        selection: segment::Selection,
    ) -> Self {
        let mut prev_segment = None;
        ChangeLog {
            sections: segments.iter().fold(Vec::new(), |mut acc, segment| {
                acc.push(Section::from_history_segment(
                    package,
                    segment,
                    repo,
                    selection,
                    prev_segment,
                ));
                prev_segment = segment.into();
                acc
            }),
        }
    }
}

fn path_from_manifest(path: &Utf8Path) -> Utf8PathBuf {
    path.parent().expect("parent for Cargo.toml").join("CHANGELOG.md")
}
