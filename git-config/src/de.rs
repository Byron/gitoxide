use std::ops::{AddAssign, MulAssign, Neg};

use crate::values::Boolean;
use crate::{
    error::{Error, Result},
    values::PeekParse,
};
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};

use serde::Deserialize;

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        todo!()
    }
}

impl<'de> Deserializer<'de> {
    fn peek(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    fn next(&mut self) -> Result<char> {
        let ch = self.peek()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn parse_bool(&mut self) -> Result<bool> {
        let (value, size) = Boolean::peek_parse(self.input)?;
        self.input = &self.input[size..];
        Ok(value)
    }

    fn parse_unsigned<T>(&mut self) -> Result<T> {
        self.parse_int(true)
    }

    fn parse_signed<T>(&mut self) -> Result<T> {
        self.parse_int(false)
    }

    fn parse_int<T>(&mut self, positive_only: bool) -> Result<T> {
        self.consume_whitespace()?;

        match self.next()? {
            c @ '0'..='9' => {
                let mut significand = (c as u8 - b'0') as u64;

                loop {
                    match self.peek()? {
                        c @ '0'..='9' => {
                            let digit = (c as u8 - b'0') as u64;

                            if significand.wrapping_mul(10).wrapping_add(digit) < u64::MAX {}

                            let _ = self.next();
                            significand = significand * 10 + digit;
                        }
                        _ => {
                            // return self.parse_number(positive, significand);
                            todo!()
                        }
                    }
                }
            }
            _ => Err(Error::InvalidInteger),
        }
    }

    fn consume_whitespace(&mut self) -> Result<()> {
        loop {
            match self.peek()? {
                ' ' | '\n' | '\t' | '\r' => {
                    let _ = self.next();
                }
                _ => {
                    return Ok(());
                }
            }
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek()? {
            _ => todo!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("Cannot deserialize into a float value! Use a integer variant instead.")
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.consume_whitespace()?;
        Ok(visitor.visit_map(self)?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

impl<'de, 'a> MapAccess<'de> for Deserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // A map section is ended when another section begins or we hit EOL.
        // Therefore, we only check if a next section begins or in the case of
        // EOL indicate that we're done.
        if self.peek().unwrap_or('[') == '[' {
            return Ok(None);
        }

        seed.deserialize(self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}

#[cfg(test)]
mod deserialize {
    use crate::from_str;
    use serde_derive::Deserialize;

    #[test]
    fn unit() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Test;
        assert_eq!(Test, from_str("").unwrap());
        assert_eq!((), from_str("").unwrap());
    }

    #[test]
    #[should_panic]
    fn float() {
        from_str::<f64>("").unwrap();
    }

    #[test]
    fn basic() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Config {
            user: User,
        }

        #[derive(Deserialize, PartialEq, Debug)]
        struct User {
            email: String,
            name: String,
        }

        let expected = Config {
            user: User {
                email: "code@eddie.sh".to_string(),
                name: "Edward Shen".to_string(),
            },
        };

        assert_eq!(
            expected,
            from_str("[user]\nemail=code@eddie.sh\nname=Edward Shen\n").unwrap()
        );
    }
}
