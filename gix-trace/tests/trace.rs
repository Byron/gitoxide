use gix_trace::{coarse, detail, span};
#[test]
fn span() {
    let _x = span!(gix_trace::Level::Coarse, "hello");
    let fourty_two = span!(gix_trace::Level::Coarse, "hello", x = "value", y = 42).into_scope(|| 42);
    assert_eq!(fourty_two, 42);
    let span = span!(target: "other", gix_trace::Level::Coarse, "hello", x = "value", y = 42);
    span.record("y", "hello").record("x", 36);
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
