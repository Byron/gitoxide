use crate::{driver, eol, Driver, Pipeline};
use bstr::BString;

/// Define how to perform CRLF round-trip checking.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CrlfRoundTripCheck {
    /// Fail with an error if CRLF conversion isn't round-trip safe.
    Fail,
    /// Emit a warning using `gix_trace::warn!`, but don't fail.
    ///
    /// Note that the parent application has to setup tracing to make these events visible, along with a parent `span!`.
    #[default]
    Warn,
    /// Do nothing, do not perform round-trip check at all.
    Skip,
}

/// Context that typically doesn't change throughout the lifetime of a pipeline, for use with `process` filters.
///
/// Note that this is quite specific to third-party filters that actually make use of this additional context.
#[derive(Default, Debug, Clone)]
pub struct Context {
    /// The name of the reference that `HEAD` is pointing to. It's passed to `process` filters if present.
    pub ref_name: Option<BString>,
    /// The root-level tree that contains the current entry directly or indirectly, or the commit owning the tree (if available).
    ///
    /// This is passed to `process` filters if present.
    pub treeish: Option<gix_hash::ObjectId>,
    /// The actual blob-hash of the data we are processing. It's passed to `process` filters if present.
    ///
    /// Note that this hash might be different from the `$Id$` of the respective `ident` filter, as the latter generates the hash itself.
    pub blob: Option<gix_hash::ObjectId>,
}

const ATTRS: [&str; 6] = ["crlf", "ident", "filter", "eol", "text", "working-tree-encoding"];

/// Lifecycle
impl Pipeline {
    /// Create a new pipeline with configured `drivers` (which should be considered safe to invoke) as well as a way to initialize
    /// our attributes with `collection`.
    /// `eol_config` serves as fallback to understand how to convert line endings if no line-ending attributes are present.
    /// `crlf_roundtrip_check` corresponds to the git-configuration of `core.safecrlf`.
    /// `object_hash` is relevant for the `ident` filter.
    pub fn new(
        drivers: Vec<Driver>,
        collection: &gix_attributes::search::MetadataCollection,
        eol_config: eol::Configuration,
        encodings_with_roundtrip_check: Vec<&'static encoding_rs::Encoding>,
        crlf_roundtrip_check: CrlfRoundTripCheck,
        object_hash: gix_hash::Kind,
    ) -> Self {
        let mut attrs = gix_attributes::search::Outcome::default();
        attrs.initialize_with_selection(collection, ATTRS);
        Pipeline {
            drivers,
            attrs,
            eol_config,
            encodings_with_roundtrip_check,
            crlf_roundtrip_check,
            context: Context::default(),
            processes: driver::State::default(),
            object_hash,
            bufs: Default::default(),
        }
    }

    /// Turn ourselves into state managing possibly running driver processes.
    ///
    /// This can be used to control how these are terminated via [driver::State::shutdown()].
    pub fn into_driver_state(self) -> driver::State {
        self.processes
    }
}

///
pub mod convert;

pub(crate) mod util;

#[cfg(test)]
mod tests;
