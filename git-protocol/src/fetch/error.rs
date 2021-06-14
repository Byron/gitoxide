use crate::{
    credentials,
    fetch::{refs, response},
};
use git_transport::client;
use quick_error::quick_error;
use std::io;

quick_error! {
    /// The error used in [`fetch()`][super::fetch].
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        PackIo(err: io::Error) {
            display("Could not read streaming pack file")
            from()
            source(err)
        }
        Credentials(err: credentials::Error) {
            display("Failed to obtain, approve or reject credentials")
            from()
            source(err)
        }
        Transport(err: client::Error) {
            display("An error occurred on the transport layer while fetching data")
            from()
            source(err)
        }
        SymrefWithoutValue {
            display("A symref 'capability' is expected to have a value")
        }
        Ref(err: refs::Error) {
            display("A reference could not be parsed or invariants were not met")
            from()
            source(err)
        }
        Response(err: response::Error) {
            display("The server response could not be parsed")
            from()
            source(err)
        }
    }
}
