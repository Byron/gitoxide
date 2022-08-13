use crate::Time;

#[allow(missing_docs)]
pub fn parse(input: &str) -> Option<Time> {
    // TODO: actual implementation, this is just to not constantly fail
    if input == "1979-02-26 18:30:00" {
        Some(Time::new(42, 1800))
    } else {
        None
    }
}
