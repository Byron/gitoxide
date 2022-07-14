use crate::{Assignment, AssignmentRef, NameRef, StateRef};

impl<'a> AssignmentRef<'a> {
    pub(crate) fn new(name: NameRef<'a>, state: StateRef<'a>) -> AssignmentRef<'a> {
        AssignmentRef { name, state }
    }

    pub fn to_owned(self) -> Assignment {
        self.into()
    }
}

impl<'a> From<AssignmentRef<'a>> for Assignment {
    fn from(a: AssignmentRef<'a>) -> Self {
        Assignment {
            name: a.name.to_owned(),
            state: a.state.to_owned(),
        }
    }
}

impl<'a> Assignment {
    pub fn as_ref(&'a self) -> AssignmentRef<'a> {
        AssignmentRef::new(self.name.as_ref(), self.state.as_ref())
    }
}
