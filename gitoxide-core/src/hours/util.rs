use std::sync::atomic::{AtomicUsize, Ordering};

use gix::bstr::{BStr, ByteSlice};
use itertools::Itertools;

use crate::hours::core::HOURS_PER_WORKDAY;

#[derive(Debug)]
pub struct WorkByPerson {
    pub name: Vec<&'static BStr>,
    pub email: Vec<&'static BStr>,
    pub hours: f32,
    pub num_commits: u32,
    pub files: FileStats,
    pub lines: LineStats,
}

impl<'a> WorkByPerson {
    pub fn merge(&mut self, other: &'a WorkByEmail) {
        if !self.name.contains(&other.name) {
            self.name.push(other.name);
        }
        if !self.email.contains(&other.email) {
            self.email.push(other.email);
        }
        self.num_commits += other.num_commits;
        self.hours += other.hours;
        self.files.add(&other.files);
        self.lines.add(&other.lines);
    }
}

impl<'a> From<&'a WorkByEmail> for WorkByPerson {
    fn from(w: &'a WorkByEmail) -> Self {
        WorkByPerson {
            name: vec![w.name],
            email: vec![w.email],
            hours: w.hours,
            num_commits: w.num_commits,
            files: w.files,
            lines: w.lines,
        }
    }
}

impl WorkByPerson {
    pub fn write_to(
        &self,
        total_hours: f32,
        total_files: Option<FileStats>,
        total_lines: Option<LineStats>,
        mut out: impl std::io::Write,
    ) -> std::io::Result<()> {
        writeln!(
            out,
            "{} <{}>",
            self.name.iter().join(", "),
            self.email.iter().join(", ")
        )?;
        writeln!(out, "{} commits found", self.num_commits)?;
        writeln!(
            out,
            "total time spent: {:.02}h ({:.02} 8h days, {:.02}%)",
            self.hours,
            self.hours / HOURS_PER_WORKDAY,
            (self.hours / total_hours) * 100.0
        )?;
        if let Some(total) = total_files {
            writeln!(
                out,
                "total files added/removed/modified: {}/{}/{} ({:.02}%)",
                self.files.added,
                self.files.removed,
                self.files.modified,
                (self.files.sum() / total.sum()) * 100.0
            )?;
        }
        if let Some(total) = total_lines {
            writeln!(
                out,
                "total lines added/removed: {}/{} ({:.02}%)",
                self.lines.added,
                self.lines.removed,
                (self.lines.sum() / total.sum()) * 100.0
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct WorkByEmail {
    pub name: &'static BStr,
    pub email: &'static BStr,
    pub hours: f32,
    pub num_commits: u32,
    pub files: FileStats,
    pub lines: LineStats,
}

/// File statistics for a particular commit.
#[derive(Debug, Default, Copy, Clone)]
pub struct FileStats {
    /// amount of added files
    pub added: usize,
    /// amount of removed files
    pub removed: usize,
    /// amount of modified files
    pub modified: usize,
}

/// Line statistics for a particular commit.
#[derive(Debug, Default, Copy, Clone)]
pub struct LineStats {
    /// amount of added lines
    pub added: usize,
    /// amount of removed lines
    pub removed: usize,
}

impl FileStats {
    pub fn add(&mut self, other: &FileStats) -> &mut Self {
        self.added += other.added;
        self.removed += other.removed;
        self.modified += other.modified;
        self
    }

    pub fn added(&self, other: &FileStats) -> Self {
        let mut a = *self;
        a.add(other);
        a
    }

    pub fn sum(&self) -> f32 {
        (self.added + self.removed + self.modified) as f32
    }
}

impl LineStats {
    pub fn add(&mut self, other: &LineStats) -> &mut Self {
        self.added += other.added;
        self.removed += other.removed;
        self
    }

    pub fn added(&self, other: &LineStats) -> Self {
        let mut a = *self;
        a.add(other);
        a
    }

    pub fn sum(&self) -> f32 {
        (self.added + self.removed) as f32
    }
}

/// An index able to address any commit
pub type CommitIdx = u32;

pub fn add_lines(line_stats: bool, lines_counter: &AtomicUsize, lines: &mut LineStats, id: gix::Id<'_>) {
    if let Some(Ok(blob)) = line_stats.then(|| id.object()) {
        let nl = blob.data.lines_with_terminator().count();
        lines.added += nl;
        lines_counter.fetch_add(nl, Ordering::Relaxed);
    }
}

pub fn remove_lines(line_stats: bool, lines_counter: &AtomicUsize, lines: &mut LineStats, id: gix::Id<'_>) {
    if let Some(Ok(blob)) = line_stats.then(|| id.object()) {
        let nl = blob.data.lines_with_terminator().count();
        lines.removed += nl;
        lines_counter.fetch_add(nl, Ordering::Relaxed);
    }
}
