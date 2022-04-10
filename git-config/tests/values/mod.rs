/// Converts string to byte slice
fn b(s: &str) -> &[u8] {
    s.as_bytes()
}

mod normalize;

mod boolean;

mod integer;

mod color_value;

mod color_attribute;

mod path;
