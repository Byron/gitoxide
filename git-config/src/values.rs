use std::convert::{Infallible, TryFrom};

#[derive(PartialEq, Debug)]
pub enum Value<'a> {
    Boolean(Boolean),
    Integer(Integer),
    Color(Color),
    Other(&'a str),
}

impl<'a> Value<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self::Other(s)
    }
}

#[derive(PartialEq, Debug)]
pub enum Boolean {
    True(TrueVariant),
    False(FalseVariant),
}

#[derive(PartialEq, Debug)]
pub enum TrueVariant {
    Yes,
    On,
    True,
    One,
    /// For variables defined without a `= <value>`.
    Implicit,
}

impl TryFrom<&str> for TrueVariant {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "yes" => Ok(Self::Yes),
            "on" => Ok(Self::On),
            "true" => Ok(Self::True),
            "one" => Ok(Self::One),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum FalseVariant {
    No,
    Off,
    False,
    Zero,
    EmptyString,
}

impl TryFrom<&str> for FalseVariant {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "no" => Ok(Self::No),
            "off" => Ok(Self::Off),
            "false" => Ok(Self::False),
            "zero" => Ok(Self::Zero),
            "" => Ok(Self::EmptyString),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Boolean {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        let value = value.as_str();
        if let Ok(v) = TrueVariant::try_from(value) {
            return Ok(Self::True(v));
        }

        if let Ok(v) = FalseVariant::try_from(value) {
            return Ok(Self::False(v));
        }

        Err(())
    }
}

// todo!()
#[derive(PartialEq, Debug)]
pub struct Integer {}

#[derive(PartialEq, Debug)]
pub struct Color {
    foreground: ColorValue,
    background: Option<ColorValue>,
    attributes: Vec<ColorAttribute>,
}

#[derive(PartialEq, Debug)]
enum ColorValue {
    Normal,
    Black,
    BrightBlack,
    Red,
    BrightRed,
    Green,
    BrightGreen,
    Yellow,
    BrightYellow,
    Blue,
    BrightBlue,
    Magenta,
    BrightMagenta,
    Cyan,
    BrightCyan,
    White,
    BrightWhite,
    Ansi(u8),
    Rgb(u8, u8, u8),
}

#[derive(PartialEq, Debug)]
enum ColorAttribute {
    Bold,
    NoBold,
    Dim,
    NoDim,
    Ul,
    NoUl,
    Blink,
    NoBlink,
    Reverse,
    NoReverse,
    Italic,
    NoItalic,
    Strike,
    NoStrike,
}

#[derive(PartialEq, Debug)]
struct Pathname<'a>(&'a str);
