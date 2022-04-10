use git_config::values::ColorValue;
use std::str::FromStr;

#[test]
fn non_bright() {
    assert_eq!(ColorValue::from_str("normal"), Ok(ColorValue::Normal));
    assert_eq!(ColorValue::from_str("black"), Ok(ColorValue::Black));
    assert_eq!(ColorValue::from_str("red"), Ok(ColorValue::Red));
    assert_eq!(ColorValue::from_str("green"), Ok(ColorValue::Green));
    assert_eq!(ColorValue::from_str("yellow"), Ok(ColorValue::Yellow));
    assert_eq!(ColorValue::from_str("blue"), Ok(ColorValue::Blue));
    assert_eq!(ColorValue::from_str("magenta"), Ok(ColorValue::Magenta));
    assert_eq!(ColorValue::from_str("cyan"), Ok(ColorValue::Cyan));
    assert_eq!(ColorValue::from_str("white"), Ok(ColorValue::White));
}

#[test]
fn bright() {
    assert_eq!(ColorValue::from_str("brightblack"), Ok(ColorValue::BrightBlack));
    assert_eq!(ColorValue::from_str("brightred"), Ok(ColorValue::BrightRed));
    assert_eq!(ColorValue::from_str("brightgreen"), Ok(ColorValue::BrightGreen));
    assert_eq!(ColorValue::from_str("brightyellow"), Ok(ColorValue::BrightYellow));
    assert_eq!(ColorValue::from_str("brightblue"), Ok(ColorValue::BrightBlue));
    assert_eq!(ColorValue::from_str("brightmagenta"), Ok(ColorValue::BrightMagenta));
    assert_eq!(ColorValue::from_str("brightcyan"), Ok(ColorValue::BrightCyan));
    assert_eq!(ColorValue::from_str("brightwhite"), Ok(ColorValue::BrightWhite));
}

#[test]
fn ansi() {
    assert_eq!(ColorValue::from_str("255"), Ok(ColorValue::Ansi(255)));
    assert_eq!(ColorValue::from_str("0"), Ok(ColorValue::Ansi(0)));
}

#[test]
fn hex() {
    assert_eq!(ColorValue::from_str("#ff0010"), Ok(ColorValue::Rgb(255, 0, 16)));
    assert_eq!(ColorValue::from_str("#ffffff"), Ok(ColorValue::Rgb(255, 255, 255)));
    assert_eq!(ColorValue::from_str("#000000"), Ok(ColorValue::Rgb(0, 0, 0)));
}

#[test]
fn invalid() {
    assert!(ColorValue::from_str("brightnormal").is_err());
    assert!(ColorValue::from_str("").is_err());
    assert!(ColorValue::from_str("bright").is_err());
    assert!(ColorValue::from_str("256").is_err());
    assert!(ColorValue::from_str("#").is_err());
    assert!(ColorValue::from_str("#fff").is_err());
    assert!(ColorValue::from_str("#gggggg").is_err());
}
