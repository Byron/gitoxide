use gix_index::entry::{mode::Change, Mode};

#[test]
fn apply() {
    assert_eq!(Change::ExecutableBit.apply(Mode::FILE), Mode::FILE_EXECUTABLE);
    assert_eq!(Change::ExecutableBit.apply(Mode::FILE_EXECUTABLE), Mode::FILE);
    assert_eq!(
        Change::Type {
            new_mode: Mode::SYMLINK
        }
        .apply(Mode::FILE),
        Mode::SYMLINK
    );
}
