mod name {
    use std::str::FromStr;

    use git_config::color::Name;

    #[test]
    fn non_bright() {
        assert_eq!(Name::from_str("normal"), Ok(Name::Normal));
        assert_eq!(Name::from_str("-1"), Ok(Name::Normal));
        assert_eq!(Name::from_str("default"), Ok(Name::Default));
        assert_eq!(Name::from_str("black"), Ok(Name::Black));
        assert_eq!(Name::from_str("red"), Ok(Name::Red));
        assert_eq!(Name::from_str("green"), Ok(Name::Green));
        assert_eq!(Name::from_str("yellow"), Ok(Name::Yellow));
        assert_eq!(Name::from_str("blue"), Ok(Name::Blue));
        assert_eq!(Name::from_str("magenta"), Ok(Name::Magenta));
        assert_eq!(Name::from_str("cyan"), Ok(Name::Cyan));
        assert_eq!(Name::from_str("white"), Ok(Name::White));
    }

    #[test]
    fn bright() {
        assert_eq!(Name::from_str("brightblack"), Ok(Name::BrightBlack));
        assert_eq!(Name::from_str("brightred"), Ok(Name::BrightRed));
        assert_eq!(Name::from_str("brightgreen"), Ok(Name::BrightGreen));
        assert_eq!(Name::from_str("brightyellow"), Ok(Name::BrightYellow));
        assert_eq!(Name::from_str("brightblue"), Ok(Name::BrightBlue));
        assert_eq!(Name::from_str("brightmagenta"), Ok(Name::BrightMagenta));
        assert_eq!(Name::from_str("brightcyan"), Ok(Name::BrightCyan));
        assert_eq!(Name::from_str("brightwhite"), Ok(Name::BrightWhite));
    }

    #[test]
    fn ansi() {
        assert_eq!(Name::from_str("255"), Ok(Name::Ansi(255)));
        assert_eq!(Name::from_str("0"), Ok(Name::Ansi(0)));
    }

    #[test]
    fn hex() {
        assert_eq!(Name::from_str("#ff0010"), Ok(Name::Rgb(255, 0, 16)));
        assert_eq!(Name::from_str("#ffffff"), Ok(Name::Rgb(255, 255, 255)));
        assert_eq!(Name::from_str("#000000"), Ok(Name::Rgb(0, 0, 0)));
    }

    #[test]
    fn invalid() {
        assert!(Name::from_str("-2").is_err());
        assert!(Name::from_str("brightnormal").is_err());
        assert!(Name::from_str("brightdefault").is_err());
        assert!(Name::from_str("").is_err());
        assert!(Name::from_str("bright").is_err());
        assert!(Name::from_str("256").is_err());
        assert!(Name::from_str("#").is_err());
        assert!(Name::from_str("#fff").is_err());
        assert!(Name::from_str("#gggggg").is_err());
    }
}

mod attribute {
    use std::str::FromStr;

    use git_config::color::Attribute;

    #[test]
    fn non_inverted() {
        assert_eq!(Attribute::from_str("reset"), Ok(Attribute::Reset));
        assert_eq!(Attribute::from_str("bold"), Ok(Attribute::Bold));
        assert_eq!(Attribute::from_str("dim"), Ok(Attribute::Dim));
        assert_eq!(Attribute::from_str("ul"), Ok(Attribute::Ul));
        assert_eq!(Attribute::from_str("blink"), Ok(Attribute::Blink));
        assert_eq!(Attribute::from_str("reverse"), Ok(Attribute::Reverse));
        assert_eq!(Attribute::from_str("italic"), Ok(Attribute::Italic));
        assert_eq!(Attribute::from_str("strike"), Ok(Attribute::Strike));
    }

    #[test]
    fn inverted_no_dash() {
        assert_eq!(Attribute::from_str("nobold"), Ok(Attribute::NoBold));
        assert_eq!(Attribute::from_str("nodim"), Ok(Attribute::NoDim));
        assert_eq!(Attribute::from_str("noul"), Ok(Attribute::NoUl));
        assert_eq!(Attribute::from_str("noblink"), Ok(Attribute::NoBlink));
        assert_eq!(Attribute::from_str("noreverse"), Ok(Attribute::NoReverse));
        assert_eq!(Attribute::from_str("noitalic"), Ok(Attribute::NoItalic));
        assert_eq!(Attribute::from_str("nostrike"), Ok(Attribute::NoStrike));
    }

    #[test]
    fn inverted_dashed() {
        assert_eq!(Attribute::from_str("no-bold"), Ok(Attribute::NoBold));
        assert_eq!(Attribute::from_str("no-dim"), Ok(Attribute::NoDim));
        assert_eq!(Attribute::from_str("no-ul"), Ok(Attribute::NoUl));
        assert_eq!(Attribute::from_str("no-blink"), Ok(Attribute::NoBlink));
        assert_eq!(Attribute::from_str("no-reverse"), Ok(Attribute::NoReverse));
        assert_eq!(Attribute::from_str("no-italic"), Ok(Attribute::NoItalic));
        assert_eq!(Attribute::from_str("no-strike"), Ok(Attribute::NoStrike));
    }

    #[test]
    fn invalid() {
        assert!(Attribute::from_str("no-reset").is_err());
        assert!(Attribute::from_str("noreset").is_err());
        assert!(Attribute::from_str("a").is_err());
        assert!(Attribute::from_str("no bold").is_err());
        assert!(Attribute::from_str("").is_err());
        assert!(Attribute::from_str("no").is_err());
        assert!(Attribute::from_str("no-").is_err());
    }
}

mod from_git {
    use bstr::BStr;
    use git_config::Color;
    use std::convert::TryFrom;

    #[test]
    #[ignore]
    fn reset() {
        assert_eq!(color("reset"), "reset");
    }

    #[test]
    fn empty() {
        assert_eq!(color(""), "");
    }

    #[test]
    fn attribute_before_color_name() {
        assert_eq!(color("bold red"), "red bold");
    }

    #[test]
    fn color_name_before_attribute() {
        assert_eq!(color("red bold"), "red bold");
    }

    #[test]
    fn attribute_fg_bg() {
        assert_eq!(color("ul blue red"), "blue red ul");
    }

    #[test]
    fn fg_bg_attribute() {
        assert_eq!(color("blue red ul"), "blue red ul");
    }

    #[test]
    fn multiple_attributes() {
        assert_eq!(
            color("blue bold dim ul blink reverse"),
            "blue bold dim ul blink reverse"
        );
    }

    #[test]
    fn reset_then_multiple_attributes() {
        assert_eq!(
            color("reset blue bold dim ul blink reverse"),
            "blue reset bold dim ul blink reverse"
        );
    }

    #[test]
    fn long_color_spec() {
        assert_eq!(
            color("254 255 bold dim ul blink reverse"),
            "254 255 bold dim ul blink reverse"
        );
        let input = "#ffffff #ffffff bold nobold dim nodim italic noitalic ul noul blink noblink reverse noreverse strike nostrike";
        let expected = input;
        assert_eq!(color(input), expected);
    }

    #[test]
    fn normal_default_can_clear_backgrounds() {
        assert_eq!(color("normal default"), "normal default");
    }

    #[test]
    fn default_can_combine_with_attributes() {
        assert_eq!(
            color("default default no-reverse bold"),
            "default default noreverse bold"
        );
    }

    fn color<'a>(name: impl Into<&'a BStr>) -> String {
        try_color(name).expect("input color is expected to be valid")
    }

    fn try_color<'a>(name: impl Into<&'a BStr>) -> crate::Result<String> {
        Ok(Color::try_from(name.into())?.to_string())
    }
}
