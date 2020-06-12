quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidObjectKind(kind: Vec<u8>) {
            display("Unknown object kind: {:?}", std::str::from_utf8(&kind))
        }
        ParseError(msg: &'static str, kind: Vec<u8>) {
            display("{}: {:?}", msg, std::str::from_utf8(&kind))
        }
    }
}
