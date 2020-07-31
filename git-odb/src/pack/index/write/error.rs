use crate::pack;
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred when reading the pack or creating a temporary file")
            from()
            source(err)
        }
        PackEntryDecode(err: pack::data::iter::Error) {
            display("A pack entry could not be extracted")
            from()
            source(err)
        }
        Unsupported(kind: pack::index::Kind) {
            display("Indices of type {} cannot be written, only {} are supported", *kind as usize, pack::index::Kind::default() as usize)
        }
        RefDelta {
            display("Ref delta objects are not supported as there is no way to look them up. Resolve them beforehand.")
        }
        IteratorInvariantTrailer {
            display("The iterator failed to set a trailing hash over all prior pack entries in the last provided entry")
        }
        IteratorInvariantNonEmpty {
            display("Is there ever a need to create empty indices? If so, please post a PR.")
        }
        IteratorInvariantBasesPresent {
            display("Did not find a single base")
        }
        IteratorInvariantBasesBeforeDeltasNeedThem(delta_pack_offset: u64, base_pack_offset: u64) {
            display("The delta at pack offset {} could not find its base at {} - it should have been seen already", delta_pack_offset, base_pack_offset)
        }
        IteratorInvariantTooManyObjects(num_objects: usize) {
            display("Only u32::MAX objects can be stored in a pack, found {}", num_objects)
        }
        IteratorInvariantIncreasingPackOffset(last_pack_offset: u64, pack_offset: u64) {
            display("Pack offsets must only increment. The previous pack offset was {}, the current one is {}", last_pack_offset, pack_offset)
        }
        ConsumeZlibInflate(err: crate::zlib::Error, msg: &'static str) {
            display("{}", msg)
            source(err)
        }
    }
}
