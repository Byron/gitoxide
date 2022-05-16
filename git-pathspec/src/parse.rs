use crate::{MagicSignature, Pattern};
use bstr::BString;
use std::iter::{FromIterator, Peekable};

impl Pattern {
    pub fn from_bytes(input: &[u8]) -> Self {
        Parser::new(input).parse()
    }
}

struct Parser<'a> {
    input: Peekable<core::slice::Iter<'a, u8>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input: input.iter().peekable(),
        }
    }

    fn parse(mut self) -> Pattern {
        let signature = self.parse_magic_signature();
        let path = self.parse_path();

        Pattern { path, signature }
    }

    fn parse_magic_signature(&mut self) -> Option<MagicSignature> {
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
                            signature |= self.parse_magic_keywords();
                        }
                        _ => break,
                    }
                }

                (!signature.is_empty()).then(|| signature)
            }
            _ => None,
        }
    }

    fn parse_magic_keywords(&mut self) -> MagicSignature {
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
                    // TODO: make this non panicking
                    panic!("space in magic keywords not allowed");
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
                // TODO: make this an error
                _ => panic!("Invalid signature"),
            }
        }

        signature
    }

    fn parse_path(self) -> BString {
        BString::from_iter(self.input.copied())
    }
}
