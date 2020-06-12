quick_error! {
    #[derive(Debug)]
    pub enum Error {
        WalkDir(err: walkdir::Error) {
            cause(err)
        }
    }
}
