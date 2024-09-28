use crate::blob::platform::{merge, DriverChoice, ResourceRef};
use crate::blob::{BuiltinDriver, Platform, PlatformRef, ResourceKind};
use bstr::{BStr, BString, ByteSlice};
use gix_filter::attributes;

/// The error returned by [Platform::prepare_merge_state()](Platform::prepare_merge()).
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

/// Preparation
impl Platform {
    /// Prepare all state needed for performing a merge, using all [previously set](Self::set_resource()) resources.
    /// `objects` is used to possibly lookup attribute files when obtaining merge-related attributes.
    ///
    /// `options` are to be used when merging later, and they may be altered to implement correct binary merges
    /// in the present of [virtual merge bases](merge::Options::is_virtual_ancestor).
    ///
    /// Note that no additional validation is performed here to facilitate inspection, which means that
    /// resource buffers might still be too large to be merged, preventing a successful merge at a later time.
    pub fn prepare_merge(
        &mut self,
        objects: &impl gix_object::Find,
        mut options: merge::Options,
    ) -> Result<PlatformRef<'_>, Error> {
        let current = self.current.as_ref().ok_or(Error::UnsetResource)?;
        let ancestor = self.ancestor.as_ref().ok_or(Error::UnsetResource)?;
        let other = self.other.as_ref().ok_or(Error::UnsetResource)?;

        let entry = self
            .attr_stack
            .at_entry(current.rela_path.as_bstr(), None, objects)
            .map_err(|err| Error::Attributes {
                source: err,
                kind: ResourceKind::CurrentOrOurs,
                rela_path: current.rela_path.clone(),
            })?;
        entry.matching_attributes(&mut self.attrs);
        let attr = self.attrs.iter_selected().next().expect("pre-initialized with 'diff'");
        let mut driver = match attr.assignment.state {
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
                self.find_driver_by_name(name)
            }
        };
        if let Some(recursive_driver_name) = match driver {
            DriverChoice::Index(idx) => self.drivers.get(idx),
            _ => None,
        }
        .and_then(|driver| driver.recursive.as_deref())
        .filter(|_| options.is_virtual_ancestor)
        {
            driver = self.find_driver_by_name(Some(recursive_driver_name.as_bstr()));
            options.resolve_binary_with = Some(crate::blob::builtin_driver::binary::ResolveWith::Ours);
        }

        let out = PlatformRef {
            parent: self,
            driver,
            current: ResourceRef::new(current),
            ancestor: ResourceRef::new(ancestor),
            other: ResourceRef::new(other),
            options,
        };
        Ok(out)
    }

    fn find_driver_by_name(&self, name: Option<&BStr>) -> DriverChoice {
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
}
