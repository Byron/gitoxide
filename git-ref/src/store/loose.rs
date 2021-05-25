use crate::Kind;
use bstr::BString;
use git_hash::ObjectId;

enum State {
    Id(ObjectId),
    Path(BString),
}

pub struct Reference {
    state: State,
}

impl Reference {
    pub fn kind(&self) -> Kind {
        match self.state {
            State::Path(_) => Kind::Symbolic,
            State::Id(_) => Kind::Peeled,
        }
    }
}

pub mod decode {
    use crate::loose::Reference;
    use nom::IResult;

    fn file(bytes: &[u8]) -> IResult<&[u8], Reference> {
        todo!("parse loose ref bytes into reference")
    }
}
