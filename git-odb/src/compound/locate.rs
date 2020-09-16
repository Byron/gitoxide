use crate::{compound, loose, pack};
use git_object::borrowed;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Loose(err: loose::db::locate::Error) {
            display("An error occurred while obtaining an object from the loose object store")
            source(err)
            from()
        }
        Pack(err: pack::bundle::locate::Error) {
            display("An error occurred while obtaining an object from the packed object store")
            source(err)
            from()
        }
    }
}

impl compound::Db {
    pub fn locate<'a>(
        &self,
        id: borrowed::Id<'_>,
        buffer: &'a mut Vec<u8>,
    ) -> Option<Result<compound::Object<'a>, Error>> {
        for pack in &self.packs {
            // See 8c5bd095539042d7db0e611460803cdbf172beb0 for a commit that adds polonius and makes the proper version compile.
            // See https://stackoverflow.com/questions/63906425/nll-limitation-how-to-work-around-cannot-borrow-buf-as-mutable-more-than?noredirect=1#comment113007288_63906425
            // The underlying issue is described here https://github.com/rust-lang/rust/issues/45402,
            // Once Polonius becomes a thing AND is not too slow, we must remove this double-lookup to become something like this:
            // if let Some(object) = if pack.locate(id, buffer, &mut pack::cache::DecodeEntryNoop) {…}
            if pack.locate(id, buffer, &mut pack::cache::DecodeEntryNoop).is_some() {
                let object = pack.locate(id, buffer, &mut pack::cache::DecodeEntryNoop).unwrap();
                return Some(object.map(compound::Object::Borrowed).map_err(Into::into));
            }
        }
        self.loose
            .locate(id)
            .map(|object| object.map(compound::Object::Loose).map_err(Into::into))
    }
}
