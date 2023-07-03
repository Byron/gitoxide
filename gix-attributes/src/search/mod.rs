use std::collections::HashMap;

use kstring::KString;
use smallvec::SmallVec;

use crate::{Assignment, AssignmentRef};

mod attributes;
mod outcome;
mod refmap;
pub(crate) use refmap::RefMap;

/// A typically sized list of attributes.
pub type Assignments = SmallVec<[TrackedAssignment; AVERAGE_NUM_ATTRS]>;

/// A value of a [pattern mapping][gix_glob::search::pattern::Mapping],
/// which is either a macro definition or a set of attributes.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub enum Value {
    /// A macro, whose name resolves to the contained assignments. Note that the name is the pattern of the mapping itself.
    MacroAssignments {
        /// The id of the macro itself, which is both an attribute as well as a set of additional attributes into which the macro
        /// resolves
        id: AttributeId,
        /// The attributes or assignments that the macro resolves to.
        assignments: Assignments,
    },
    /// A set of assignments which are the attributes themselves.
    Assignments(Assignments),
}

/// A way to have an assignment (`attr=value`) but also associated it with an id that allows perfect mapping
/// to tracking information.
/// Note that the order is produced after the files are parsed as global ordering is needed that goes beyond the scope of a
/// single `Search` instance.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct TrackedAssignment {
    /// The order of the assignment.
    pub id: AttributeId,
    /// The actual assignment information.
    pub inner: Assignment,
}

/// An implementation of the [`Pattern`][gix_glob::search::Pattern] trait for attributes.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Default)]
pub struct Attributes;

/// Describes a matching pattern with
#[derive(Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub struct Match<'a> {
    /// The glob pattern itself, like `/target/*`.
    pub pattern: &'a gix_glob::Pattern,
    /// The key=value pair of the attribute that matched at the pattern. There can be multiple matches per pattern.
    pub assignment: AssignmentRef<'a>,
    /// Additional information about the kind of match.
    pub kind: MatchKind,
    /// Information about the location of the match.
    pub location: MatchLocation<'a>,
}

/// Describes in which what file and line the match was found.
#[derive(Clone, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub struct MatchLocation<'a> {
    /// The path to the source from which the pattern was loaded, or `None` if it was specified by other means.
    pub source: Option<&'a std::path::Path>,
    /// The line at which the pattern was found in its `source` file, or the occurrence in which it was provided.
    pub sequence_number: usize,
}

/// The kind of attribute within the context of a [match][Match].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Ord, PartialOrd)]
pub enum MatchKind {
    /// A attribute.
    Attribute {
        /// The location of the macro which referred to it the list with all in-order attributes and macros, or `None` if
        /// this is attribute wasn't resolved.
        ///
        /// Use [`Outcome::match_by_id()`] to retrieve the macro.
        macro_id: Option<AttributeId>,
    },
    /// The attribute is a macro, which will resolve into one or more attributes or macros.
    Macro {
        /// The location of the parent macro which referred to this one in the list with all in-order attributes and macros,
        /// or `None` if this is macro wasn't resolved by another one.
        ///
        /// Use [`Outcome::match_by_id()`] to retrieve the parent.
        parent_macro_id: Option<AttributeId>,
    },
}

/// The result of a search, containing all matching attributes.
#[derive(Default, Clone)]
pub struct Outcome {
    /// The list of all available attributes, by ascending order. Each slots index corresponds to an attribute with that order, i.e.
    /// `arr[attr.id] = <attr info>`.
    ///
    /// This list needs to be up-to-date with the search group so all possible attribute names are known.
    matches_by_id: Vec<Slot>,
    /// A stack of attributes to use for processing attributes of matched patterns and for resolving their macros.
    attrs_stack: SmallVec<[(AttributeId, Assignment, Option<AttributeId>); 8]>,
    /// A set of attributes we should limit ourselves to, or empty if we should fill in all attributes, made of
    selected: SmallVec<[(KString, Option<AttributeId>); AVERAGE_NUM_ATTRS]>,
    /// storage for all patterns we have matched so far (in order to avoid referencing them, we copy them, but only once).
    patterns: RefMap<gix_glob::Pattern>,
    /// storage for all assignments we have matched so far (in order to avoid referencing them, we copy them, but only once).
    assignments: RefMap<Assignment>,
    /// storage for all source paths we have matched so far (in order to avoid referencing them, we copy them, but only once).
    source_paths: RefMap<std::path::PathBuf>,
    /// The amount of attributes that still need to be set, or `None` if this outcome is consumed which means it
    /// needs to be re-initialized.
    remaining: Option<usize>,
}

#[derive(Default, Clone)]
struct Slot {
    r#match: Option<outcome::Match>,
    /// A list of all assignments, being an empty list for non-macro attributes, or all assignments (with order) for macros.
    /// It's used to resolve macros.
    macro_attributes: Assignments,
}

/// A type to denote an id of an attribute assignment for uniquely identifying each attribute or assignment.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub struct AttributeId(pub usize);

impl Default for AttributeId {
    fn default() -> Self {
        AttributeId(usize::MAX)
    }
}

/// A utility type to collect metadata for each attribute, unified by its name.
#[derive(Clone, Debug, Default)]
pub struct MetadataCollection {
    /// A mapping of an attribute or macro name to its order, that is the time when it was *first* seen.
    ///
    /// This is the inverse of the order attributes are searched.
    name_to_meta: HashMap<KString, Metadata>,
}

/// Metadata associated with an attribute or macro name.
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
pub struct Metadata {
    /// The id to uniquely identify an attribute in the [MetadataCollection].
    pub id: AttributeId,
    /// If non-zero in length, this entry belongs to a macro which resolves to these attribute names.
    pub macro_attributes: Assignments,
}

const AVERAGE_NUM_ATTRS: usize = 3;
