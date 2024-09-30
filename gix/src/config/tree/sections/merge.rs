use crate::config;
use crate::config::tree::SubSectionRequirement;
use crate::config::{
    tree::{keys, Key, Merge, Section},
    Tree,
};

impl Merge {
    /// The `merge.renormalize` key
    pub const RENORMALIZE: keys::Boolean = keys::Boolean::new_boolean("renormalize", &Tree::MERGE);
    /// The `merge.default` key
    pub const DEFAULT: keys::String = keys::String::new_string("default", &Tree::MERGE);
    /// The `merge.<driver>.name` key.
    pub const DRIVER_NAME: keys::String = keys::String::new_string("name", &config::Tree::MERGE)
        .with_subsection_requirement(Some(SubSectionRequirement::Parameter("driver")));
    /// The `merge.<driver>.driver` key.
    pub const DRIVER_COMMAND: keys::Program = keys::Program::new_program("driver", &config::Tree::MERGE)
        .with_subsection_requirement(Some(SubSectionRequirement::Parameter("driver")));
    /// The `merge.<driver>.recursive` key.
    pub const DRIVER_RECURSIVE: keys::String = keys::String::new_string("recursive", &config::Tree::MERGE)
        .with_subsection_requirement(Some(SubSectionRequirement::Parameter("driver")));
}

impl Section for Merge {
    fn name(&self) -> &str {
        "merge"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[
            &Self::RENORMALIZE,
            &Self::DEFAULT,
            &Self::DRIVER_NAME,
            &Self::DRIVER_COMMAND,
            &Self::DRIVER_RECURSIVE,
        ]
    }
}
