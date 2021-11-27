use crate::{store, Namespace};
use git_features::threading::OwnShared;

#[derive(Clone)]
pub(crate) enum State {
    Loose {
        store: crate::file::Store,
        packed_buffer: OwnShared<store::packed::ModifiableBuffer>,
    },
}

impl From<(&store::State, Option<Namespace>)> for State {
    fn from(s: (&store::State, Option<Namespace>)) -> Self {
        match s.0 {
            store::State::Loose {
                path,
                reflog_mode,
                packed_buffer,
            } => store::handle::State::Loose {
                store: {
                    let mut store = crate::file::Store::at(path, *reflog_mode);
                    store.namespace = s.1;
                    store
                },
                packed_buffer: packed_buffer.clone(),
            },
        }
    }
}

impl crate::Store {
    /// Return a new handle which sees all references if `namespace` is `None` or all read and write operations are limited
    /// to the given `namespace` if `Some`.
    pub fn to_handle(&self) -> store::Handle {
        Self::new_handle_inner(&self.state, None)
    }

    /// As above, but supports a namespace to be set
    pub fn to_handle_namespaced(&self, namespace: Option<Namespace>) -> store::Handle {
        Self::new_handle_inner(&self.state, namespace)
    }

    fn new_handle_inner(state: &store::State, namespace: Option<Namespace>) -> store::Handle {
        store::Handle {
            state: match state {
                store::State::Loose {
                    path,
                    reflog_mode,
                    packed_buffer,
                } => store::handle::State::Loose {
                    store: {
                        let mut store = crate::file::Store::at(path, *reflog_mode);
                        store.namespace = namespace;
                        store
                    },
                    packed_buffer: packed_buffer.clone(),
                },
            },
        }
    }
}

mod find {
    use crate::{store, PartialNameRef, Reference};
    use std::convert::TryInto;

    mod error {
        use quick_error::quick_error;
        use std::convert::Infallible;

        quick_error! {
            /// The error returned by [crate::Store::find()].
            #[derive(Debug)]
            #[allow(missing_docs)]
            pub enum Error {
                Loose(err: crate::file::find::Error) {
                    display("An error occurred while finding a reference in the loose file database")
                    from()
                    source(err)
                }
            }
        }

        impl From<Infallible> for Error {
            fn from(_: Infallible) -> Self {
                unreachable!("this impl is needed to allow passing a known valid partial path as parameter")
            }
        }
    }
    pub use error::Error;

    impl store::Handle {
        /// TODO: actually implement this with handling of the packed buffer.
        pub fn try_find<'a, Name, E>(&self, _partial: Name) -> Result<Option<Reference>, Error>
        where
            Name: TryInto<PartialNameRef<'a>, Error = E>,
            Error: From<E>,
        {
            todo!()
        }
    }

    mod existing {
        mod error {
            use std::path::PathBuf;

            use quick_error::quick_error;

            quick_error! {
                /// The error returned by [file::Store::find_existing()][crate::file::Store::find_existing()].
                #[derive(Debug)]
                #[allow(missing_docs)]
                pub enum Error {
                    LooseFind(err: crate::file::find::Error) {
                        display("An error occurred while finding a reference in the loose file database")
                        from()
                        source(err)
                    }
                    NotFound(name: PathBuf) {
                        display("The ref partially named '{}' could not be found", name.display())
                    }
                }
            }
        }
        pub use error::Error;
    }
}

mod iter {
    // impl store::Handle {
    //     pub fn iter<'p, 's>(&'s self, packed: Option<&'p packed::Buffer>) -> std::io::Result<LooseThenPacked<'p, 's>> {
    // }
}
