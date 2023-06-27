use once_cell::sync::Lazy;
use std::path::PathBuf;

static DRIVER: Lazy<PathBuf> = Lazy::new(|| {
    let mut cargo = std::process::Command::new(env!("CARGO"));
    let res = cargo
        .args(["build", "--example", "ident"])
        .status()
        .expect("cargo should run fine");
    assert!(res.success(), "cargo invocation should be successfull");

    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .ancestors()
        .nth(1)
        .expect("first parent in target dir")
        .join("debug")
        .join("examples")
        .join(if cfg!(windows) { "ident.exe" } else { "ident" });
    assert!(path.is_file(), "Expecting driver to be located at {path:?}");
    path
});

mod apply {
    mod no_process {
        fn driver_no_process() -> Driver {
            let mut exe = DRIVER.to_string_lossy().into_owned();
            if cfg!(windows) {
                exe = exe.replace('\\', "/");
            }
            Driver {
                name: "ident".into(),
                clean: Some((exe.clone() + " clean %f").into()),
                smudge: Some((exe + " smudge %f").into()),
                process: None,
                required: true,
            }
        }

        use crate::driver::DRIVER;
        use bstr::ByteSlice;
        use gix_filter::{driver, Driver};
        use std::io::Read;

        #[test]
        fn smudge_and_clean_failure_is_translated_to_observable_error_for_required_drivers() -> crate::Result {
            let mut state = gix_filter::driver::State::default();
            let driver = driver_no_process();
            assert!(driver.required);

            let mut filtered = state.apply(
                &driver,
                &b"hello\nthere\n"[..],
                driver::Operation::Smudge,
                "do/fail".into(),
            )?;
            let mut buf = Vec::new();
            let err = filtered.read_to_end(&mut buf).unwrap_err();
            assert!(err.to_string().ends_with(" failed"));

            Ok(())
        }

        #[test]
        fn smudge_and_clean_failure_means_nothing_if_required_is_false() -> crate::Result {
            let mut state = gix_filter::driver::State::default();
            let mut driver = driver_no_process();
            driver.required = false;

            let mut filtered = state.apply(
                &driver,
                &b"hello\nthere\n"[..],
                driver::Operation::Clean,
                "do/fail".into(),
            )?;
            let mut buf = Vec::new();
            let num_read = filtered.read_to_end(&mut buf)?;
            assert_eq!(
                num_read, 0,
                "the example fails right away so no output is produced to stdout"
            );

            Ok(())
        }

        #[test]
        fn smudge_and_clean() -> crate::Result {
            let mut state = gix_filter::driver::State::default();
            let driver = driver_no_process();
            assert!(
                driver.required,
                "we want errors to definitely show, and don't expect them"
            );

            let input = "hello\nthere\n";
            let mut filtered = state.apply(
                &driver,
                input.as_bytes(),
                driver::Operation::Smudge,
                "some/path.txt".into(),
            )?;
            let mut buf = Vec::new();
            filtered.read_to_end(&mut buf)?;
            assert_eq!(
                buf.as_bstr(),
                "\thello\n\tthere\n",
                "ident applies indentation in smudge mode"
            );

            let smudge_result = buf.clone();
            let mut filtered = state.apply(
                &driver,
                smudge_result.as_bytes(),
                driver::Operation::Clean,
                "some/path.txt".into(),
            )?;
            buf.clear();
            filtered.read_to_end(&mut buf)?;
            assert_eq!(
                buf.as_bstr(),
                input,
                "the clean filter reverses the smudge filter (and we call the right one)"
            );

            Ok(())
        }
    }
}
