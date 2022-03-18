use git_object::bstr::BStr;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome<'a> {
    pub name: Cow<'a, BStr>,
    pub id: git_hash::ObjectId,
    pub hex_len: usize,
    pub depth: usize,
    pub long: bool,
    pub dirty_suffix: Option<String>,
}

impl<'a> Outcome<'a> {
    pub fn is_exact_match(&self) -> bool {
        self.depth == 0
    }
    pub fn long(&mut self) -> &mut Self {
        self.long = true;
        self
    }
    pub fn short(&mut self) -> &mut Self {
        self.long = false;
        self
    }
}

impl<'a> Display for Outcome<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if !self.long && self.is_exact_match() {
            self.name.fmt(f)?;
        } else {
            write!(
                f,
                "{}-{}-g{}",
                self.name,
                self.depth,
                self.id.to_hex_with_len(self.hex_len)
            )?;
        }
        if let Some(suffix) = &self.dirty_suffix {
            write!(f, "-{}", suffix)?;
        }
        Ok(())
    }
}

pub type Error = ();

pub(crate) mod function {
    use super::{Error, Outcome};
    use git_hash::{oid, ObjectId};
    use git_object::bstr::BStr;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[allow(clippy::result_unit_err)]
    pub fn describe<'a>(
        commit: &oid,
        hex_len: usize,
        name_set: &HashMap<ObjectId, Cow<'a, BStr>>,
    ) -> Result<Option<Outcome<'a>>, Error> {
        if let Some(name) = name_set.get(commit) {
            return Ok(Some(Outcome {
                name: name.to_owned(),
                id: commit.to_owned(),
                hex_len,
                depth: 0,
                long: false,
                dirty_suffix: None,
            }));
        }
        todo!("actually search for it")
    }
}
