use crate::{MagicSignature, Pattern};
use bstr::BString;
use quick_error::quick_error;
use std::iter::{FromIterator, Peekable};

quick_error! {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Error {
        InvalidSignature { found_signature: BString } {
            display("Found {}, which is not a valid signature", found_signature)
        }
        WhitespaceInSignature {
            display("Whitespace in magic keywords are not allowed")
        }
    }
}

impl Pattern {
    pub fn from_bytes(input: &[u8]) -> Result<Self, Error> {
        Parser::new(input).parse()
    }
}

type ByteIter<'a> = Peekable<core::slice::Iter<'a, u8>>;

struct Parser<'a> {
    input: ByteIter<'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input: input.iter().peekable(),
        }
    }

    fn parse(mut self) -> Result<Pattern, Error> {
        let signature = self.parse_magic_signature()?;
        let path = self.parse_path();

        Ok(Pattern { path, signature })
    }

    fn parse_magic_signature(&mut self) -> Result<Option<MagicSignature>, Error> {
        match self.input.peek() {
            Some(b':') => {
                self.input.next();
                let mut signature = MagicSignature::empty();

                while let Some(&b) = self.input.peek() {
                    match b {
                        b':' => {
                            self.input.next();
                            if !signature.is_empty() {
                                break;
                            }
                        }
                        b'/' => {
                            self.input.next();
                            signature |= MagicSignature::TOP
                        }
                        b'^' | b'!' => {
                            self.input.next();
                            signature |= MagicSignature::EXCLUDE;
                        }
                        b'(' => {
                            self.input.next();
                            signature |= self.parse_magic_keywords()?;
                        }
                        _ => break,
                    }
                }

                (!signature.is_empty()).then(|| signature).map(Result::Ok).transpose()
            }
            _ => Ok(None),
        }
    }

    fn parse_magic_keywords(&mut self) -> Result<MagicSignature, Error> {
        let mut buf = Vec::new();
        let mut keywords = Vec::new();

        while let Some(b) = self.input.next() {
            match b {
                b')' => {
                    if !buf.is_empty() {
                        keywords.push(buf);
                    }
                    break;
                }
                b',' => {
                    if !buf.is_empty() {
                        keywords.push(std::mem::take(&mut buf));
                    }
                }
                b' ' => {
                    return Err(Error::WhitespaceInSignature);
                }
                _ => {
                    buf.push(*b);
                }
            }
        }

        let mut signature = MagicSignature::empty();

        for keyword in keywords.into_iter() {
            signature |= match &keyword[..] {
                b"top" => MagicSignature::TOP,
                b"literal" => MagicSignature::LITERAL,
                b"icase" => MagicSignature::ICASE,
                b"glob" => MagicSignature::GLOB,
                b"attr" => MagicSignature::ATTR,
                b"exclude" => MagicSignature::EXCLUDE,
                s => {
                    return Err(Error::InvalidSignature {
                        found_signature: BString::from(s),
                    })
                }
            }
        }

        Ok(signature)
    }

    fn parse_path(self) -> BString {
        BString::from_iter(self.input.copied())
    }
}
