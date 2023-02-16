use bstr::ByteSlice;

use crate::{State, StateRef};

impl<'a> StateRef<'a> {
    /// Turn ourselves into our owned counterpart.
    pub fn to_owned(self) -> State {
        self.into()
    }
}

impl<'a> State {
    /// Turn ourselves into our ref-type.
    pub fn as_ref(&'a self) -> StateRef<'a> {
        match self {
            State::Value(v) => StateRef::Value(v.as_bytes().as_bstr()),
            State::Set => StateRef::Set,
            State::Unset => StateRef::Unset,
            State::Unspecified => StateRef::Unspecified,
        }
    }
}

impl<'a> From<StateRef<'a>> for State {
    fn from(s: StateRef<'a>) -> Self {
        match s {
            StateRef::Value(v) => State::Value(v.to_str().expect("no illformed unicode").into()),
            StateRef::Set => State::Set,
            StateRef::Unset => State::Unset,
            StateRef::Unspecified => State::Unspecified,
        }
    }
}
