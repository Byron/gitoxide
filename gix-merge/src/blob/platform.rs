use crate::blob::{pipeline, BuiltinDriver, Pipeline, Platform, ResourceKind};
use bstr::{BStr, BString, ByteSlice};
use gix_filter::attributes;

/// A stored value representing a resource that participates in a merge.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub(super) struct Resource {
    /// The `id` of the value, or `null` if it's only living in a worktree.
    id: gix_hash::ObjectId,
    /// The repository-relative path where the resource lives in the tree.
    rela_path: BString,
    /// The outcome of converting a resource into a mergable format using [Pipeline::convert_to_mergeable()].
    data: Option<pipeline::Data>,
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
    /// The id of the content as it would be stored in `git`, or `null` if the content doesn't exist anymore at
    /// `rela_path` or if it was never computed. This can happen with content read from the worktree, which
    /// after its 'to-git' conversion never had its hash computed.
    pub id: &'a gix_hash::oid,
}

/// Options for use in a [`Platform`].
#[derive(Default, Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub struct Options {
    /// Define which driver to use by name if the `merge` attribute for a resource is unspecified.
    ///
    /// This is the value of the `merge.default` git configuration.
    pub default_driver: Option<BString>,
}

/// The selection of the driver to use by a resource obtained with [`Pipeline::convert_to_mergeable()`].
///
/// If available, an index into the `drivers` field to access more diff-related information of the driver for items
/// at the given path, as previously determined by git-attributes.
///
/// * `merge` is set
///     - Use the [`BuiltinDriver::Text`]
/// * `-merge` is unset
///     - Use the [`BuiltinDriver::Binary`]
/// * `!merge` is unspecified
///     - Use [`Options::default_driver`] or [`BuiltinDriver::Text`].
/// * `merge=name`
///     - Search for a user-configured or built-in driver called `name`.
///     - If not found, silently default to [`BuiltinDriver::Text`]
///
/// Note that drivers are queried even if there is no object available.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub enum DriverChoice {
    /// Use the given built-in driver to perform the merge.
    BuiltIn(BuiltinDriver),
    /// Use the user-provided driver program using the index into [the pipelines driver array](Pipeline::drivers().
    Index(usize),
}

impl Default for DriverChoice {
    fn default() -> Self {
        DriverChoice::BuiltIn(Default::default())
    }
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
                data: cache.data.map_or(Data::Missing, |data| match data {
                    pipeline::Data::Buffer => Data::Buffer(&cache.buffer),
                    pipeline::Data::TooLarge { size } => Data::Binary { size },
                }),
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
    use crate::blob::platform::DriverChoice;
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
        /// Which driver to use according to the resource's configuration,
        /// using the path of `current` to read git-attributes.
        pub driver_choice: DriverChoice,
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
            match self.driver_choice {
                DriverChoice::BuiltIn(builtin) => Err(builtin),
                DriverChoice::Index(idx) => self.parent.drivers.get(idx).ok_or(BuiltinDriver::default()),
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
    use crate::blob::ResourceKind;
    use bstr::BString;

    /// The error returned by [Platform::prepare_merge()](super::Platform::prepare_merge_state()).
    #[derive(Debug, thiserror::Error)]
    #[allow(missing_docs)]
    pub enum Error {
        #[error("The 'current', 'ancestor' or 'other' resource for the merge operation were not set")]
        UnsetResource,
        #[error("Failed to obtain attributes for {kind:?} resource at '{rela_path}'")]
        Attributes {
            rela_path: BString,
            kind: ResourceKind,
            source: std::io::Error,
        },
    }
}

/// Lifecycle
impl Platform {
    /// Create a new instance with a way to `filter` data from the object database and turn it into something that is merge-able.
    /// `filter_mode` decides how to do that specifically.
    /// Use `attr_stack` to access attributes pertaining worktree filters and merge settings.
    /// `drivers` are the list of available merge drivers that individual paths can refer to by means of git attributes.
    /// `options` further configure the operation.
    pub fn new(
        filter: Pipeline,
        filter_mode: pipeline::Mode,
        attr_stack: gix_worktree::Stack,
        mut drivers: Vec<super::Driver>,
        options: Options,
    ) -> Self {
        drivers.sort_by(|a, b| a.name.cmp(&b.name));
        Platform {
            drivers,
            current: None,
            ancestor: None,
            other: None,
            filter,
            filter_mode,
            attr_stack,
            attrs: {
                let mut out = attributes::search::Outcome::default();
                out.initialize_with_selection(&Default::default(), Some("merge"));
                out
            },
            options,
        }
    }
}

/// Access
impl Platform {
    /// Return all drivers that this instance was initialized with.
    ///
    /// They are sorted by [`name`](super::Driver::name) to support binary searches.
    pub fn drivers(&self) -> &[super::Driver] {
        &self.drivers
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

    /// Prepare all state needed for performing a merge, using all [previously set](Self::set_resource()) resources.
    /// Note that no additional validation is performed here to facilitate inspection.
    pub fn prepare_merge_state(
        &mut self,
        objects: &impl gix_object::Find,
    ) -> Result<merge::State<'_>, prepare_merge::Error> {
        let current = self.current.as_ref().ok_or(prepare_merge::Error::UnsetResource)?;
        let ancestor = self.ancestor.as_ref().ok_or(prepare_merge::Error::UnsetResource)?;
        let other = self.other.as_ref().ok_or(prepare_merge::Error::UnsetResource)?;

        let entry = self
            .attr_stack
            .at_entry(current.rela_path.as_bstr(), None, objects)
            .map_err(|err| prepare_merge::Error::Attributes {
                source: err,
                kind: ResourceKind::CurrentOrOurs,
                rela_path: current.rela_path.clone(),
            })?;
        entry.matching_attributes(&mut self.attrs);
        let attr = self.attrs.iter_selected().next().expect("pre-initialized with 'diff'");
        let driver = match attr.assignment.state {
            attributes::StateRef::Set => DriverChoice::BuiltIn(BuiltinDriver::Text),
            attributes::StateRef::Unset => DriverChoice::BuiltIn(BuiltinDriver::Binary),
            attributes::StateRef::Value(_) | attributes::StateRef::Unspecified => {
                let name = match attr.assignment.state {
                    attributes::StateRef::Value(name) => Some(name.as_bstr()),
                    attributes::StateRef::Unspecified => {
                        self.options.default_driver.as_ref().map(|name| name.as_bstr())
                    }
                    _ => unreachable!("only value and unspecified are possible here"),
                };
                name.and_then(|name| {
                    self.drivers
                        .binary_search_by(|d| d.name.as_bstr().cmp(name))
                        .ok()
                        .map(DriverChoice::Index)
                        .or_else(|| {
                            name.to_str()
                                .ok()
                                .and_then(BuiltinDriver::by_name)
                                .map(DriverChoice::BuiltIn)
                        })
                })
                .unwrap_or_default()
            }
        };

        let out = merge::State {
            parent: self,
            driver_choice: driver,
            current: ResourceRef::new(current),
            ancestor: ResourceRef::new(ancestor),
            other: ResourceRef::new(other),
        };

        Ok(out)
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
                    data: out,
                    mode,
                    buffer: buf_storage,
                });
            }
            Some(storage) => {
                storage.id = id;
                storage.rela_path = rela_path.to_owned();
                storage.data = out;
                storage.mode = mode;
            }
        };
        Ok(())
    }
}
