use crate::{MagicSignature, Pattern};
use bstr::BString;
use quick_error::quick_error;
use std::iter::{FromIterator, Peekable};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidSignature { found_signature: BString } {
            display("Found \"{}\", which is not a valid signature", found_signature)
        }
        // TODO: Fix error messages
        InvalidAttribute(err: git_attributes::parse::Error) {
            display("{}", err)
            from()
            source(err)
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
        let (signature, attributes) = self.parse_magic_signature()?;
        let path = self.parse_path();

        Ok(Pattern {
            path,
            signature,
            attributes,
        })
    }

    fn parse_magic_signature(
        &mut self,
    ) -> Result<(Option<MagicSignature>, Vec<(BString, git_attributes::State)>), Error> {
        match self.input.peek() {
            Some(b':') => {
                self.input.next();
                let mut signature = MagicSignature::empty();
                let mut attributes = Vec::new();

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
                            let (signatures, attrs) = self.parse_magic_keywords()?;
                            signature |= signatures;
                            attributes = attrs;
                        }
                        _ => break,
                    }
                }

                (!signature.is_empty())
                    .then(|| signature)
                    .map(Result::Ok)
                    .transpose()
                    .map(|x| (x, attributes))
            }
            _ => Ok((None, Vec::new())),
        }
    }

    fn parse_magic_keywords(&mut self) -> Result<(MagicSignature, Vec<(BString, git_attributes::State)>), Error> {
        let mut buf = Vec::new();
        let mut keywords = Vec::new();
        let mut attributes = Vec::new();

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
                s if s.starts_with(b"attr:") => git_attributes::parse::Iter::new(s[5..].into(), 0)
                    .map(|res| res.map(|(attr, state)| (BString::from(attr), git_attributes::State::from(state))))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|v| {
                        attributes = v;
                        MagicSignature::ATTR
                    })?,
                s => {
                    return Err(Error::InvalidSignature {
                        found_signature: BString::from(s),
                    });
                }
            }
        }

        Ok((signature, attributes))
    }

    fn parse_path(self) -> BString {
        BString::from_iter(self.input.copied())
    }
}
