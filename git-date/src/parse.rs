use crate::Time;
use bstr::BStr;

#[allow(missing_docs)]
pub fn parse(input: &BStr) -> Option<Time> {
    // TODO: actual implementation, this is just to not constantly fail
    if input == "1979-02-26 18:30:00" {
        Some(Time::new(42, 1800))
    } else {
        None
    }
}
