pub mod history;

mod message {

    pub struct MessageRef<'a> {
        category
    }

    mod decode {
        use nom::error::{ContextError, ParseError};
        use nom::IResult;

        /// Parse a signature from the bytes input `i` using `nom`.
        pub fn decode<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
            i: &'a [u8],
        ) -> IResult<&'a [u8], (), E> {
            todo!("probably not to be done")
        }
    }
}

pub struct History {
    pub head: git_repository::refs::Reference,
    pub items: Vec<history::Item>,
}
