use gix_index::entry::{Mode, mode::Change};

#[test]
fn is_dir_for_pattern_matching() {
    for (mode, expected) in [
        (Mode::DIR, true),
        (Mode::COMMIT, true),
        (Mode::FILE, false),
        (Mode::FILE_EXECUTABLE, false),
        (Mode::SYMLINK, false),
        (Mode::empty(), false),
    ] {
        assert_eq!(
            mode.is_dir_for_pattern_matching(),
            expected,
            "{mode:?}: decoded modes have the same classification on every host"
        );
    }
}

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

#[test]
fn debug() {
    assert_eq!(
        format!("{:?}", Mode::FILE),
        "Mode(FILE)",
        "Assure the debug output is easy to understand"
    );

    assert_eq!(
        format!("{:?}", Mode::from_bits(0o120744)),
        "Some(Mode(FILE | SYMLINK | 0x40))",
        "strange modes are also mostly legible"
    );
}
