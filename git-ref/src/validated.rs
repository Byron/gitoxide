use bstr::BStr;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum NameError {
        StartsWithDot {
            description("A ref must not start with a '.'")
        }
        Empty {
            description("A ref must not be empty")
        }
    }
}

pub fn name(name: &BStr) -> Result<&BStr, NameError> {
    if name.is_empty() {
        return Err(NameError::Empty);
    }
    if name[0] == b'.' {
        return Err(NameError::StartsWithDot);
    }
    Ok(name)
}

#[cfg(test)]
mod tests;
