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
        Io(err: io::Error) {
            display("Could not access repository or failed to read streaming pack file")
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
        TransportProtocolPolicyViolation{actual_version: git_transport::Protocol} {
            display("The transport didn't accept the advertised server version {:?} and closed the connection client side", actual_version)
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
