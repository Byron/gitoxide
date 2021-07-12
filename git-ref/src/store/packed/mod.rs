#![allow(missing_docs, dead_code)]

pub mod iter {}

#[derive(Debug, PartialEq, Eq)]
enum Peeled {
    Unspecified,
    Partial,
    Fully,
}

/// Information parsed from the header of a packed ref file
#[derive(Debug, PartialEq, Eq)]
struct Header {
    peeled: Peeled,
    sorted: bool,
}

mod decode;
