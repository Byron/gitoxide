use bstr::ByteSlice;
use std::path::Path;
use std::time::Duration;

#[test]
fn fuzzed() {
    for name in [
        "very-long6",
        "very-long5",
        "very-long4",
        "very-long3",
        "very-long2",
        "very-long",
    ] {
        let base = Path::new("tests").join("fixtures").join("fuzzed");
        let location = base.join(Path::new(name).with_extension("url"));
        let url = std::fs::read(&location).unwrap();
        let start = std::time::Instant::now();
        gix_url::parse(url.as_bstr()).ok();
        assert!(
            start.elapsed() < Duration::from_millis(100),
            "URL at '{}' parsed too slowly, took {:.00}s",
            location.display(),
            start.elapsed().as_secs_f32()
        );
    }
}
