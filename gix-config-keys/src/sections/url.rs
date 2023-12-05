use crate::{keys, Key, Section, SubSectionRequirement, Url, Tree};

const BASE_PARAMETER: Option<SubSectionRequirement> = Some(SubSectionRequirement::Parameter("base"));

impl Url {
    /// The `url.<base>.insteadOf` key
    pub const INSTEAD_OF: keys::Any =
        keys::Any::new("insteadOf", &Tree::URL).with_subsection_requirement(BASE_PARAMETER);
    /// The `url.<base>.pushInsteadOf` key
    pub const PUSH_INSTEAD_OF: keys::Any =
        keys::Any::new("pushInsteadOf", &Tree::URL).with_subsection_requirement(BASE_PARAMETER);
}

impl Section for Url {
    fn name(&self) -> &str {
        "url"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::INSTEAD_OF, &Self::PUSH_INSTEAD_OF]
    }
}
