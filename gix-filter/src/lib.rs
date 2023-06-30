//! A library for implementing everything needed to deal with git filter pipelines.
//!
//! Generally, multiple filters are applied in a row forming a pipeline, with each filter being a stage in that pipeline.
//! This pipeline is pre-determined with each stage being configurable.
//!
//! The transformation on an input buffer goes in two ways: either a filter is applied, or its effects are undone. Differentiating
//! between these states is important to avoid comparing unfiltered buffers with filtered ones, for example.
//!
//! This crate implements the building blocks in terms of applying and undoing filters, along with logic to decide whether
//! or not to apply such a filter.
#![deny(rust_2018_idioms, missing_docs, unsafe_code)]

use bstr::BString;

///
pub mod ident;

/// utilities related to handling line endings in buffers
pub mod eol;

/// Utilities for handling worktree encodings.
pub mod worktree;

/// Utilities around driver programs.
pub mod driver;

/// A declaration of a driver program.
///
/// It consists of up to three program declarations.
#[derive(Debug, Clone)]
pub struct Driver {
    /// The name of the driver as stored in the configuration.
    pub name: BString,

    /// The program invocation that cleans a worktree file for storage in `git`.
    ///
    /// Note that the command invocation may need its `%f` argument substituted with the name of the file to process. It will be quoted.
    pub clean: Option<BString>,
    /// The program invocation that readies a file stored in `git` for the worktree.
    ///
    /// Note that the command invocation may need its `%f` argument substituted with the name of the file to process. It will be quoted.
    pub smudge: Option<BString>,
    /// the long-running program that can typically handle both smudge and clean, and possibly delay processing as well.
    pub process: Option<BString>,
    /// If `true`, the `clean` or `smudge` programs need to succeed in order to make their content usable. Otherwise their
    /// exit code is ignored.
    /// Note that this is more of a suggestion as we will always report errors as they happen as the driver API is streaming in nature,
    /// which makes soft-failures impossible unless the caller takes precautions.
    pub required: bool,
}

fn clear_and_set_capacity(buf: &mut Vec<u8>, cap: usize) {
    buf.clear();
    if buf.capacity() < cap {
        buf.reserve(cap - buf.capacity());
    }
}
