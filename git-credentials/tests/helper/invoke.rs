use crate::helper::invoke::util::MockHelper;
use bstr::BString;
use git_credentials::helper;
use git_credentials::protocol::Context;

#[test]
fn get() {
    let mut helper = MockHelper::default();
    let mut outcome = git_credentials::helper::invoke(
        &mut helper,
        &helper::Action::get_for_url("https://github.com/byron/gitoxide"),
    )
    .unwrap()
    .expect("mock provides credentials");
    assert_eq!(
        outcome.consume_identity().expect("complete"),
        git_sec::identity::Account {
            username: "user".into(),
            password: "pass".into()
        }
    );
    assert_eq!(
        outcome.next.store().payload().unwrap(),
        "url=https://github.com/byron/gitoxide\nusername=user\npassword=pass\n"
    );
}

#[test]
fn store_and_reject() {
    let mut helper = MockHelper::default();
    let ctx = Context {
        url: Some("https://github.com/byron/gitoxide".into()),
        ..Default::default()
    };
    let ctxbuf = || -> BString {
        let mut buf = Vec::<u8>::new();
        ctx.write_to(&mut buf).expect("cannot fail");
        buf.into()
    };
    for action in [helper::Action::Store(ctxbuf()), helper::Action::Erase(ctxbuf())] {
        let outcome = git_credentials::helper::invoke(&mut helper, &action).unwrap();
        assert!(
            outcome.is_none(),
            "store and erase have no outcome, they just shouln't fail"
        );
    }
}

mod program {
    use bstr::ByteVec;
    use git_credentials::{helper, program::Kind, Program};

    #[test]
    fn builtin() {
        assert!(
            matches!(
                git_credentials::helper::invoke(
                    Program::from_kind(Kind::Builtin).suppress_stderr(),
                    &helper::Action::get_for_url("/path/without/scheme/fails/with/error"),
                )
                .unwrap_err(),
                helper::Error::CredentialsHelperFailed { .. }
            ),
            "this failure indicates we could launch the helper, even though it wasn't happy which is fine. It doesn't like the URL"
        );
    }

    #[test]
    fn script() {
        assert_eq!(
            git_credentials::helper::invoke(
                &mut Program::from_custom_definition(
                    "!f() { test \"$1\" = get && echo \"password=pass\" && echo \"username=user\"; }; f"
                ),
                &helper::Action::get_for_url("/does/not/matter"),
            )
            .unwrap()
            .expect("present")
            .consume_identity()
            .expect("complete"),
            git_sec::identity::Account {
                username: "user".into(),
                password: "pass".into()
            }
        );
    }

    #[cfg(unix)] // needs executable bits to work
    #[test]
    fn path_to_helper_script() -> crate::Result {
        assert_eq!(
            git_credentials::helper::invoke(
                &mut Program::from_custom_definition(
                    git_path::into_bstr(git_path::realpath(git_testtools::fixture_path("custom-helper.sh"))?)
                        .into_owned()
                ),
                &helper::Action::get_for_url("/does/not/matter"),
            )?
            .expect("present")
            .consume_identity()
            .expect("complete"),
            git_sec::identity::Account {
                username: "user-script".into(),
                password: "pass-script".into()
            }
        );
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn path_to_helper_as_script_to_workaround_executable_bits() -> crate::Result {
        let mut helper = git_path::to_unix_separators_on_windows(git_path::into_bstr(git_testtools::fixture_path(
            "custom-helper.sh",
        )))
        .into_owned();
        helper.insert_str(0, "sh ");
        assert_eq!(
            git_credentials::helper::invoke(
                Program::from_kind(Kind::ExternalShellScript(helper)),
                &helper::Action::get_for_url("/does/not/matter"),
            )?
            .expect("present")
            .consume_identity()
            .expect("complete"),
            git_sec::identity::Account {
                username: "user-script".into(),
                password: "pass-script".into()
            }
        );
        Ok(())
    }
}

mod util {
    use git_credentials::helper;
    use git_credentials::program::main;
    use git_credentials::protocol::Context;

    #[derive(Default)]
    pub struct MockHelper {
        handle: Option<std::thread::JoinHandle<()>>,
    }

    impl git_credentials::Helper for &mut MockHelper {
        type Send = git_features::io::pipe::Writer;
        type Receive = git_features::io::pipe::Reader;

        fn start(&mut self, action: &helper::Action) -> std::io::Result<(Self::Send, Option<Self::Receive>)> {
            let ((them_send, us_receive), (us_send, them_receive)) = (
                git_features::io::pipe::unidirectional(None),
                git_features::io::pipe::unidirectional(None),
            );
            let action_name = action.as_arg(true).into();
            self.handle = std::thread::spawn(move || {
                git_credentials::program::main(
                    Some(action_name),
                    us_receive,
                    us_send,
                    |action, context| -> std::io::Result<_> {
                        match action {
                            main::Action::Get => Ok(Some(Context {
                                username: Some("user".into()),
                                password: Some("pass".into()),
                                ..context
                            })),
                            main::Action::Store | main::Action::Erase => Ok(None),
                        }
                    },
                )
                .expect("cannot fail")
            })
            .into();
            Ok((them_send, them_receive.into()))
        }

        fn finish(self) -> std::io::Result<()> {
            self.handle.take().expect("started").join().unwrap();
            Ok(())
        }
    }
}
