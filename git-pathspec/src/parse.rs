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

    fn parse(&mut self) -> Pattern {
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
        let mut buf: Vec<u8> = vec![];
        let mut keywords: Vec<Vec<u8>> = vec![];

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
            let keyword = keyword.iter().map(|&x| x.to_owned()).collect::<Vec<_>>();
            let keyword = String::from_utf8_lossy(&keyword[..]);

            match &keyword[..] {
                "top" => signature |= MagicSignature::TOP,
                "literal" => signature |= MagicSignature::LITERAL,
                "icase" => signature |= MagicSignature::ICASE,
                "glob" => signature |= MagicSignature::GLOB,
                "attr" => signature |= MagicSignature::ATTR,
                "exclude" => signature |= MagicSignature::EXCLUDE,
                _ => panic!("Invalid signature: \"{}\"", keyword),
            }
        }

        signature
    }

    fn parse_path(&mut self) -> BString {
        BString::from_iter(self.input.clone().copied())
    }
}
