pub mod name {
    use quick_error::quick_error;

    quick_error! {
        #[derive(Debug)]
        pub enum Error {
            Tag(err: crate::tag::name::Error) {
                display("A reference must be a valid tag name as well")
                from()
                source(err)
            }
            SomeLowercase {
                display("Standalone references must be all uppercased, like 'HEAD'")
            }
            StartsWithSlash {
                display("A reference name must not start with a slash '/'")
            }
            RepeatedSlash {
                display("Multiple slashes in a row are not allowed as they may change the reference's meaning")
            }
            SingleDot {
                display("Names must not be a single '.', but may contain it.")
            }

        }
    }
}

use bstr::BStr;

pub fn name(path: &BStr) -> Result<&BStr, name::Error> {
    crate::tagname(path)?;
    if path[0] == b'/' {
        return Err(name::Error::StartsWithSlash);
    }
    let mut previous = 0;
    let mut one_before_previous = 0;
    let mut saw_slash = false;
    for byte in path.iter() {
        match *byte {
            b'/' if previous == b'.' && one_before_previous == b'/' => return Err(name::Error::SingleDot),
            b'/' if previous == b'/' => return Err(name::Error::RepeatedSlash),
            _ => {}
        }

        if *byte == b'/' {
            saw_slash = true;
        }
        one_before_previous = previous;
        previous = *byte;
    }

    if !saw_slash && path.iter().any(|c| !c.is_ascii_uppercase()) {
        return Err(name::Error::SomeLowercase);
    }
    Ok(path)
}
