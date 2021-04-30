///
pub mod ancestors {
    use git_hash::{oid, ObjectId};
    use git_object::immutable;
    use quick_error::quick_error;
    use std::collections::{BTreeSet, VecDeque};

    quick_error! {
        /// The error is part of the item returned by the [Ancestors] iterator.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            NotFound{oid: ObjectId} {
                display("The object {} could not be found", oid)
            }
            ObjectDecode(err: immutable::object::decode::Error) {
                display("An object could not be decoded")
                source(err)
                from()
            }
        }
    }

    /// An iterator over the ancestors one or more starting commits
    pub struct Ancestors<Find> {
        find: Find,
        next: VecDeque<ObjectId>,
        buf: Vec<u8>,
        seen: BTreeSet<ObjectId>,
    }

    impl<Find> Ancestors<Find>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
    {
        /// Create a new instance.
        ///
        /// * `find` - a way to lookup new object data during traversal by their ObjectId, writing their data into buffer and returning
        ///    an iterator over commit tokens if the object is present and is a commit. Caching should be implemented within this function
        ///    as needed.
        /// * `tips`
        ///   * the starting points of the iteration, usually commits
        ///   * each commit they lead to will only be returned once, including the tip that started it
        pub fn new(find: Find, tips: impl IntoIterator<Item = impl Into<ObjectId>>) -> Self {
            let next: VecDeque<_> = tips.into_iter().map(Into::into).collect();
            let seen = next.iter().cloned().collect();
            Ancestors {
                find,
                next,
                buf: Vec::with_capacity(4096),
                seen,
            }
        }
    }

    impl<Find> Iterator for Ancestors<Find>
    where
        Find: for<'a> FnMut(&oid, &'a mut Vec<u8>) -> Option<immutable::CommitIter<'a>>,
    {
        type Item = Result<ObjectId, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            let res = self.next.pop_front();
            if let Some(oid) = res {
                match (self.find)(&oid, &mut self.buf) {
                    Some(mut commit_iter) => {
                        if let Some(Err(decode_tree_err)) = commit_iter.next() {
                            return Some(Err(decode_tree_err.into()));
                        }
                        for token in commit_iter {
                            match token {
                                Ok(immutable::commit::iter::Token::Parent { id }) => {
                                    let was_inserted = self.seen.insert(id);
                                    if was_inserted {
                                        self.next.push_back(id);
                                    }
                                }
                                Ok(_a_token_past_the_parents) => break,
                                Err(err) => return Some(Err(err.into())),
                            }
                        }
                    }
                    None => return Some(Err(Error::NotFound { oid })),
                }
            }
            res.map(Ok)
        }
    }
}
#[doc(inline)]
pub use ancestors::Ancestors;
