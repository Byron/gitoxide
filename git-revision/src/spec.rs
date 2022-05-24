pub mod parse {
    #![allow(missing_docs)]
    use git_object::bstr::BStr;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("The delegate didn't indicate success - check delegate for more information")]
        Delegate,
    }

    pub trait Delegate {
        fn resolve_ref(&mut self, input: &BStr) -> Option<()>;
        fn find_by_prefix(&mut self, input: &BStr) -> Option<()>;

        fn nth_ancestor(&mut self, n: usize) -> Option<()>;
        fn nth_parent(&mut self, n: usize) -> Option<()>;
    }

    pub(crate) mod function {
        use crate::spec::parse::{Delegate, Error};
        use git_object::bstr::BStr;

        pub fn parse(input: &BStr, delegate: &mut impl Delegate) -> Result<(), Error> {
            // TODO: don't hardcode these cases, see how git does it. This remains
            //       just an example.
            if input == "@" || input == "HEAD" {
                return delegate.resolve_ref("HEAD".into()).ok_or(Error::Delegate);
            }

            todo!("")
        }
    }
}
pub use parse::function::parse;
