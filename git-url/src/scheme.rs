use std::convert::TryFrom;

/// A scheme for use in a [`Url`]
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Scheme {
    File,
    Git,
    Ssh,
    Http,
    Https,
    Ext(&'static str),
}

impl<'a> TryFrom<&'a str> for Scheme {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value {
            "ssh" => Scheme::Ssh,
            "file" => Scheme::File,
            "git" => Scheme::Git,
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            "rad" => Scheme::Ext("rad"),
            unknown => return Err(unknown),
        })
    }
}

impl Scheme {
    /// Return ourselves parseable name.
    pub fn as_str(&self) -> &'static str {
        use Scheme::*;
        match self {
            File => "file",
            Git => "git",
            Ssh => "ssh",
            Http => "http",
            Https => "https",
            Ext(name) => name,
        }
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
