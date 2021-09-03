/// A function for use in [`loose::Reference::peel_to_id_in_place()`] to indicate no peeling should happen.
pub fn none(
    _id: git_hash::ObjectId,
    _buf: &mut Vec<u8>,
) -> Result<Option<(git_object::Kind, &[u8])>, std::convert::Infallible> {
    Ok(Some((git_object::Kind::Commit, &[])))
}

///
pub mod to_id {
    use std::path::PathBuf;

    use bstr::BString;
    use quick_error::quick_error;

    use crate::file;

    quick_error! {
        /// The error returned by [`crate::Reference::peel_to_id_in_place()`].
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            Follow(err: file::find::existing::Error) {
                display("Could not follow a single level of a symbolic reference")
                from()
                source(err)
            }
            Cycle(start_absolute: PathBuf){
                display("Aborting due to reference cycle with first seen path being '{}'", start_absolute.display())
            }
            DepthLimitExceeded { max_depth: usize } {
                display("Refusing to follow more than {} levels of indirection", max_depth)
            }
            Find(err: Box<dyn std::error::Error + Send + Sync + 'static>) {
                display("An error occurred when trying to resolve an object a refererence points to")
                from()
                source(&**err)
            }
            NotFound{oid: git_hash::ObjectId, name: BString} {
                display("Object {} as referred to by '{}' could not be found", oid, name)
            }
        }
    }
}
