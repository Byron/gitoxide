use crate::{
    mutable::Target,
    store::{file, packed},
};
use bstr::BString;
use quick_error::quick_error;

quick_error! {
    /// The error returned by various [`Transaction`][super::Transaction] methods.
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub enum Error {
        Packed(err: packed::buffer::open::Error) {
            display("The packed ref buffer could not be loaded")
            from()
            source(err)
        }
        PackedTransactionAcquire(err: git_lock::acquire::Error) {
            display("The lock for the packed-ref file could not be obtained")
            source(err)
        }
        PackedTransactionCommit(err: packed::transaction::commit::Error) {
            display("The packed-ref transaction could not be committed")
            source(err)
        }
        PackedTransactionPrepare(err: packed::transaction::prepare::Error) {
            display("The packed transaction could not be prepared")
            from()
            source(err)
        }
        PackedFind(err: packed::find::Error) {
            display("The packed ref file could not be parsed")
            source(err)
            from()
        }
        PreprocessingFailed(err: std::io::Error) {
            display("Edit preprocessing failed with error: {}", err.to_string())
            source(err)
        }
        LockAcquire{err: git_lock::acquire::Error, full_name: BString} {
            display("A lock could not be obtained for reference {}", full_name)
            source(err)
        }
        LockCommit{err: std::io::Error, full_name: BString} {
            display("THe change for reference {} could not be committed", full_name)
            source(err)
        }
        Io(err: std::io::Error) {
            display("An IO error occurred while applying an edit")
            from()
            source(err)
        }
        DeleteReferenceMustExist { full_name: BString } {
            display("The reference '{}' for deletion did not exist or could not be parsed", full_name)
        }
        DeleteReference{ full_name: BString, err: std::io::Error } {
            display("The reference '{}' could not be deleted", full_name)
            source(err)
        }
        DeleteReflog{ full_name: BString, err: std::io::Error } {
            display("The reflog of reference '{}' could not be deleted", full_name)
            source(err)
        }
        CreateOrUpdateRefLog(err: file::log::create_or_update::Error) {
            display("The reflog could not be created or updated")
            from()
            source(err)
        }
        MustNotExist { full_name: BString, actual: Target, new: Target } {
            display("Reference '{}' was not supposed to exist when writing it with value {}, but actual content was {}", full_name, new, actual)
        }
        MustExist { full_name: BString, expected: Target } {
            display("Reference '{}' was supposed to exist with value {}, but didn't.", full_name, expected)
        }
        ReferenceOutOfDate { full_name: BString, expected: Target, actual: Target } {
            display("The reference '{}' should have content {}, actual content was {}", full_name, expected, actual)
        }
        ReferenceDecode(err: file::loose::reference::decode::Error) {
            display("Could not read reference")
            from()
            source(err)
        }
    }
}
