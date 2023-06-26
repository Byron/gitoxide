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

///
pub mod ident;

/// utilities related to handling line endings in buffers
pub mod eol;

/// Utilities for handling worktree encodings.
pub mod worktree;

fn clear_and_set_capacity(buf: &mut Vec<u8>, cap: usize) {
    buf.clear();
    if buf.capacity() < cap {
        buf.reserve(cap - buf.capacity());
    }
}
