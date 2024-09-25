use bstr::{BStr, BString};

use crate::blob::pipeline::DriverChoice;
use crate::blob::{pipeline, Pipeline, Platform, ResourceKind};

/// A stored value representing a resource that participates in a merge.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub(super) struct Resource {
    /// The `id` of the value, or `null` if it's only living in a worktree.
    id: gix_hash::ObjectId,
    /// The repository-relative path where the resource lives in the tree.
    rela_path: BString,
    /// The outcome of converting a resource into a diffable format using [Pipeline::convert_to_mergeable()].
    conversion: pipeline::Outcome,
    /// The kind of the resource we are looking at. Only possible values are `Blob` and `BlobExecutable`.
    mode: gix_object::tree::EntryKind,
    /// A possibly empty buffer, depending on `conversion.data` which may indicate the data is considered binary
    /// or the resource doesn't exist.
    buffer: Vec<u8>,
}

/// A blob or executable ready to be merged in one way or another.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResourceRef<'a> {
    /// The data itself, suitable for merging, and if the object or worktree item is present at all.
    pub data: resource::Data<'a>,
    /// The location of the resource, relative to the working tree.
    pub rela_path: &'a BStr,
    /// Which driver to use according to the resource's configuration.
    pub driver_choice: DriverChoice,
    /// The id of the content as it would be stored in `git`, or `null` if the content doesn't exist anymore at
    /// `rela_path` or if it was never computed. This can happen with content read from the worktree, which
    /// after its 'to-git' conversion never had its hash computed.
    pub id: &'a gix_hash::oid,
}

///
pub mod resource {
    use crate::blob::{
        pipeline,
        platform::{Resource, ResourceRef},
    };

    impl<'a> ResourceRef<'a> {
        pub(super) fn new(cache: &'a Resource) -> Self {
            ResourceRef {
                data: cache.conversion.data.map_or(Data::Missing, |data| match data {
                    pipeline::Data::Buffer => Data::Buffer(&cache.buffer),
                    pipeline::Data::Binary { size } => Data::Binary { size },
                }),
                driver_choice: cache.conversion.driver,
                rela_path: cache.rela_path.as_ref(),
                id: &cache.id,
            }
        }
    }

    /// The data of a mergeable resource, as it could be determined and computed previously.
    #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub enum Data<'a> {
        /// The object is missing, either because it didn't exist in the working tree or because its `id` was null.
        Missing,
        /// The textual data as processed and ready for merging, i.e. suitable for storage in Git.
        Buffer(&'a [u8]),
        /// The size that the binary blob had at the given revision, without having applied filters, as it's either
        /// considered binary or above the big-file threshold.
        ///
        /// In this state, the binary file cannot be merged.
        Binary {
            /// The size of the object prior to performing any filtering or as it was found on disk.
            ///
            /// Note that technically, the size isn't always representative of the same 'state' of the
            /// content, as once it can be the size of the blob in Git, and once it's the size of file
            /// in the worktree.
            size: u64,
        },
    }

    impl<'a> Data<'a> {
        /// Return ourselves as slice of bytes if this instance stores data.
        pub fn as_slice(&self) -> Option<&'a [u8]> {
            match self {
                Data::Buffer(d) => Some(d),
                Data::Binary { .. } | Data::Missing => None,
            }
        }
    }
}

///
pub mod set_resource {
    use bstr::BString;

    use crate::blob::{pipeline, ResourceKind};

    /// The error returned by [Platform::set_resource](super::Platform::set_resource).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("Can only diff blobs, not {mode:?}")]
        InvalidMode { mode: gix_object::tree::EntryKind },
        #[error("Failed to read {kind:?} worktree data from '{rela_path}'")]
        Io {
            rela_path: BString,
            kind: ResourceKind,
            source: std::io::Error,
        },
        #[error("Failed to obtain attributes for {kind:?} resource at '{rela_path}'")]
        Attributes {
            rela_path: BString,
            kind: ResourceKind,
            source: std::io::Error,
        },
        #[error(transparent)]
        ConvertToMergeable(#[from] pipeline::convert_to_mergeable::Error),
    }
}

///
pub mod merge {
    use crate::blob::pipeline::DriverChoice;
    use crate::blob::platform::ResourceRef;
    use crate::blob::{builtin_driver, BuiltinDriver, Driver, Resolution};
    use bstr::BString;

    /// The product of a [`prepare_merge()`](crate::blob::Platform::prepare_merge_state()) call to finally
    /// perform the merge and retrieve the merge results.
    #[derive(Copy, Clone)]
    pub struct State<'parent> {
        /// The platform that hosts the resources, used to access drivers.
        pub(super) parent: &'parent super::Platform,
        /// The current or our side of the merge operation.
        pub current: ResourceRef<'parent>,
        /// The ancestor or base of the merge operation.
        pub ancestor: ResourceRef<'parent>,
        /// The other or their side of the merge operation.
        pub other: ResourceRef<'parent>,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct Options {
        /// If `true`, the resources being merged are contained in a virtual ancestor,
        /// which is the case when merge bases are merged into one.
        pub is_virtual_ancestor: bool,
        /// Determine how to resolve conflicts. If `None`, no conflict resolution is possible and it picks a side.
        pub resolve_binary_with: Option<builtin_driver::binary::ResolveWith>,
        /// Options for the builtin [text driver](BuiltinDriver::Text).
        pub text: builtin_driver::text::Options,
    }

    ///
    pub mod prepare_external_driver {
        use std::ops::{Deref, DerefMut};

        use crate::blob::ResourceKind;
        use bstr::BString;

        /// The error returned by [State::prepare_merge_command()](super::State::prepare_external_driver()).
        #[derive(Debug, thiserror::Error)]
        #[allow(missing_docs)]
        pub enum Error {
            #[error("Binary resources can't be diffed with an external command (as we don't have the data anymore)")]
            SourceOrDestinationAreBinary,
            #[error(
                "Tempfile to store content of '{rela_path}' ({kind:?}) for passing to external merge command could not be created"
            )]
            CreateTempfile {
                rela_path: BString,
                kind: ResourceKind,
                source: std::io::Error,
            },
            #[error(
                "Could not write content of '{rela_path}' ({kind:?}) to tempfile for passing to external merge command"
            )]
            WriteTempfile {
                rela_path: BString,
                kind: ResourceKind,
                source: std::io::Error,
            },
        }

        /// The product of a [`prepare_external_driver`](super::State::prepare_external_driver()) operation.
        ///
        /// This type acts like [`std::process::Command`], ready to run, with `stderr` set to *inherit*,
        /// but `stdin` closed and `stdout` setup to be captured.
        // TODO: remove this
        #[allow(dead_code)]
        pub struct Command {
            /// The pre-configured command
            cmd: std::process::Command,
            /// A tempfile holding the *current* (ours) state of the resource.
            current: gix_tempfile::Handle<gix_tempfile::handle::Closed>,
            /// A tempfile holding the *ancestor* (base) state of the resource.
            ancestor: gix_tempfile::Handle<gix_tempfile::handle::Closed>,
            /// A tempfile holding the *other* (their) state of the resource.
            other: gix_tempfile::Handle<gix_tempfile::handle::Closed>,
        }

        impl Deref for Command {
            type Target = std::process::Command;

            fn deref(&self) -> &Self::Target {
                &self.cmd
            }
        }

        impl DerefMut for Command {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.cmd
            }
        }
    }

    ///
    pub mod builtin_merge {
        /// An identifier to tell us how a merge conflict was resolved by [builtin_merge](super::State::builtin_merge).
        pub enum Pick {
            /// Chose the ancestor.
            Ancestor,
            /// Chose our side.
            Ours,
            /// Chose their side.
            Theirs,
            /// New data was produced with the result of the merge, to be found in the buffer that was passed to
            /// [builtin_merge()](super::State::builtin_merge).
            Buffer,
        }
    }

    /// The error returned by [State::merge()].
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error(transparent)]
        PrepareExternalDriver(#[from] prepare_external_driver::Error),
    }

    /// Plumbing
    impl<'parent> State<'parent> {
        /// Given `merge_command` and `context`, typically obtained from git-configuration, and the currently set merge-resources,
        /// prepare the invocation and temporary files needed to launch it according to protocol.
        ///
        /// Please note that this is an expensive operation this will always create three temporary files to hold all sides of the merge.
        ///
        /// ### Deviation
        ///
        /// We allow passing more context than Git would by taking a whole `context`, it's up to the caller to decide how much is filled.
        pub fn prepare_external_driver(
            &self,
            _merge_command: BString,
            _context: gix_command::Context,
        ) -> Result<prepare_external_driver::Command, prepare_external_driver::Error> {
            todo!("prepare command")
        }

        /// Perform the merge according to our resources and
        /// Note that if the *pick* wasn't [`Buffer`](builtin_merge::Pick::Buffer), then `out` will not have been cleared.
        pub fn builtin_merge(
            &self,
            _out: &mut Vec<u8>,
            _driver: BuiltinDriver,
            _opts: Options,
        ) -> (builtin_merge::Pick, Resolution) {
            todo!("do full merge")
        }

        /// Return the configured driver program for use with [`Self::prepare_external_driver()`], or `Err`
        /// with the built-in driver to use instead.
        pub fn configured_driver(&self) -> Result<&'parent Driver, BuiltinDriver> {
            match self.current.driver_choice {
                DriverChoice::BuiltIn(builtin) => Err(builtin),
                DriverChoice::Index(idx) => self.parent.filter.drivers.get(idx).ok_or(BuiltinDriver::default()),
            }
        }
    }

    /// Convenience
    impl<'parent> State<'parent> {
        /// Perform the merge, possibly invoking an external merge command, and store the result in `out`.
        /// The merge is configured by `opts` and possible merge driver command executions are affected by `context`.
        pub fn merge(
            &self,
            _out: &mut Vec<u8>,
            _opts: Options,
            _context: gix_command::Context,
        ) -> Result<Resolution, Error> {
            match self.configured_driver() {
                Ok(driver) => {
                    let _cmd = self.prepare_external_driver(driver.command.clone(), _context)?;
                    todo!("invoke command and copy result")
                }
                Err(_builtin) => {
                    todo!("call builtins and copy results")
                }
            }
        }
    }
}

///
pub mod prepare_merge {
    /// The error returned by [Platform::prepare_merge()](super::Platform::prepare_merge_state()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The 'current', 'ancestor' or 'other' resource for the merge operation were not set")]
        UnsetResource,
        #[error("Tried to merge 'current' and 'other' where at least one of them is removed")]
        CurrentOrOtherRemoved,
    }
}

/// Lifecycle
impl Platform {
    /// Create a new instance with a way to `filter` data from the object database and turn it into something that is merge-able.
    /// `filter_mode` decides how to do that specifically.
    /// Use `attr_stack` to access attributes pertaining worktree filters and merge settings.
    pub fn new(filter: Pipeline, filter_mode: pipeline::Mode, attr_stack: gix_worktree::Stack) -> Self {
        Platform {
            current: None,
            ancestor: None,
            other: None,
            filter,
            filter_mode,
            attr_stack,
        }
    }
}

/// Preparation
impl Platform {
    /// Store enough information about a resource to eventually use it in a merge, where…
    ///
    /// * `id` is the hash of the resource. If it [is null](gix_hash::ObjectId::is_null()), it should either
    ///   be a resource in the worktree, or it's considered a non-existing, deleted object.
    ///   If an `id` is known, as the hash of the object as (would) be stored in `git`, then it should be provided
    ///   for completeness. Note that it's not expected to be in `objects` if `rela_path` is set and a worktree-root
    ///   is available for `kind`.
    /// * `mode` is the kind of object (only blobs and links are allowed)
    /// * `rela_path` is the relative path as seen from the (work)tree root.
    /// * `kind` identifies the side of the merge this resource will be used for.
    /// * `objects` provides access to the object database in case the resource can't be read from a worktree.
    pub fn set_resource(
        &mut self,
        id: gix_hash::ObjectId,
        mode: gix_object::tree::EntryKind,
        rela_path: &BStr,
        kind: ResourceKind,
        objects: &impl gix_object::FindObjectOrHeader,
    ) -> Result<(), set_resource::Error> {
        self.set_resource_inner(id, mode, rela_path, kind, objects)
    }

    /// Returns the resource of the given kind if it was set.
    pub fn resource(&self, kind: ResourceKind) -> Option<ResourceRef<'_>> {
        let cache = match kind {
            ResourceKind::CurrentOrOurs => self.current.as_ref(),
            ResourceKind::CommonAncestorOrBase => self.ancestor.as_ref(),
            ResourceKind::OtherOrTheirs => self.other.as_ref(),
        }?;
        ResourceRef::new(cache).into()
    }

    /// Prepare all state needed for performing a merge, using all [previously set](Self::set_resource()) resources.
    pub fn prepare_merge_state(&self) -> Result<merge::State<'_>, prepare_merge::Error> {
        let current = self.current.as_ref().ok_or(prepare_merge::Error::UnsetResource)?;
        let ancestor = self.ancestor.as_ref().ok_or(prepare_merge::Error::UnsetResource)?;
        let other = self.other.as_ref().ok_or(prepare_merge::Error::UnsetResource)?;

        let out = merge::State {
            parent: self,
            current: ResourceRef::new(current),
            ancestor: ResourceRef::new(ancestor),
            other: ResourceRef::new(other),
        };

        match (current.conversion.data, other.conversion.data) {
            (None, None) => Err(prepare_merge::Error::CurrentOrOtherRemoved),
            (_, _) => Ok(out),
        }
    }
}

impl Platform {
    fn set_resource_inner(
        &mut self,
        id: gix_hash::ObjectId,
        mode: gix_object::tree::EntryKind,
        rela_path: &BStr,
        kind: ResourceKind,
        objects: &impl gix_object::FindObjectOrHeader,
    ) -> Result<(), set_resource::Error> {
        if !matches!(
            mode,
            gix_object::tree::EntryKind::Blob | gix_object::tree::EntryKind::BlobExecutable
        ) {
            return Err(set_resource::Error::InvalidMode { mode });
        }
        let entry =
            self.attr_stack
                .at_entry(rela_path, None, objects)
                .map_err(|err| set_resource::Error::Attributes {
                    source: err,
                    kind,
                    rela_path: rela_path.to_owned(),
                })?;

        let storage = match kind {
            ResourceKind::OtherOrTheirs => &mut self.other,
            ResourceKind::CommonAncestorOrBase => &mut self.ancestor,
            ResourceKind::CurrentOrOurs => &mut self.current,
        };

        let mut buf_storage = Vec::new();
        let out = self.filter.convert_to_mergeable(
            &id,
            mode,
            rela_path,
            kind,
            &mut |_, out| {
                let _ = entry.matching_attributes(out);
            },
            objects,
            self.filter_mode,
            storage.as_mut().map_or(&mut buf_storage, |s| &mut s.buffer),
        )?;

        match storage {
            None => {
                *storage = Some(Resource {
                    id,
                    rela_path: rela_path.to_owned(),
                    conversion: out,
                    mode,
                    buffer: buf_storage,
                });
            }
            Some(storage) => {
                storage.id = id;
                storage.rela_path = rela_path.to_owned();
                storage.conversion = out;
                storage.mode = mode;
            }
        };
        Ok(())
    }
}
