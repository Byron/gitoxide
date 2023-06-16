use gix_trace::{coarse, detail, span};
#[test]
fn span() {
    let _x = span!(gix_trace::Level::Coarse, "hello");
    span!(gix_trace::Level::Coarse, "hello", x = "value", y = 42);
    span!(target: "other", gix_trace::Level::Coarse, "hello", x = "value", y = 42);
}

#[test]
fn coarse() {
    let _x = coarse!("hello");
    coarse!("hello", x = "value", y = 42);
    coarse!(target: "other", "hello", x = "value", y = 42);
}

#[test]
fn detail() {
    let _y = detail!("hello");
    detail!("hello", x = "value", y = 42);
    detail!(target: "other", "hello", x = "value", y = 42);
}
