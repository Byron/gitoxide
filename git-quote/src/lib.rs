#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

use bstr::{BStr, BString, ByteSlice};

///
pub mod ansi_c;

/// Transforms the given value to be suitable for use as an argument for Bourne shells by wrapping in single quotes
pub fn to_single_quoted(mut value: &BStr) -> BString {
    let mut quoted = BString::new(b"'".to_vec());

    while let Some(pos) = value.find_byteset(b"!'") {
        quoted.extend_from_slice(&value[..pos]);
        quoted.extend_from_slice(b"'\\");
        quoted.push(value[pos]);
        quoted.push(b'\'');

        value = &value[pos + 1..];
    }

    quoted.extend_from_slice(value);
    quoted.push(b'\'');
    quoted
}

#[cfg(test)]
mod tests {
    use crate::to_single_quoted;
    use bstr::BStr;

    #[test]
    fn quoted_strings() {
        assert_eq!(to_single_quoted("my cool string".into()), "'my cool string'");
        assert_eq!(to_single_quoted(r"'\''".into()), BStr::new(r"''\''\'\'''\'''"));
        assert_eq!(
            to_single_quoted("my 'quoted' string".into()),
            BStr::new(r"'my '\''quoted'\'' string'")
        );
        assert_eq!(to_single_quoted(r"'\!'".into()), BStr::new(r"''\''\'\!''\'''"));
        assert_eq!(
            to_single_quoted("my excited string!!!".into()),
            BStr::new(r"'my excited string'\!''\!''\!''")
        );
        assert_eq!(
            to_single_quoted("\0my `even` ~cooler~ $t\\'ring\\// with \"quotes!\"".into()),
            BStr::new("'\0my `even` ~cooler~ $t\\'\\''ring\\// with \"quotes'\\!'\"'")
        );
    }
}
