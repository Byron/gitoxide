use crate::helper::invoke::util::MockHelper;
use bstr::BString;
use git_credentials::helper::{invoke, Context};

#[test]
fn get() {
    let mut helper = MockHelper::default();
    let outcome = git_credentials::helper::invoke(
        &mut helper,
        invoke::Action::get_for_url("https://github.com/byron/gitoxide"),
    )
    .unwrap()
    .expect("mock provides credentials");
    assert_eq!(
        outcome.identity,
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
    for action in [invoke::Action::Store(ctxbuf()), invoke::Action::Erase(ctxbuf())] {
        let outcome = git_credentials::helper::invoke(&mut helper, action).unwrap();
        assert!(
            outcome.is_none(),
            "store and erase have no outcome, they just shouln't fail"
        );
    }
}

mod util {
    use git_credentials::helper::invoke::Action;
    use git_credentials::helper::{main, Context};

    #[derive(Default)]
    pub struct MockHelper {
        handle: Option<std::thread::JoinHandle<()>>,
    }

    impl git_credentials::Helper for &mut MockHelper {
        type Send = git_features::io::pipe::Writer;
        type Receive = git_features::io::pipe::Reader;

        fn start(&mut self, action: &Action) -> std::io::Result<(Self::Send, Option<Self::Receive>)> {
            let ((them_send, us_receive), (us_send, them_receive)) = (
                git_features::io::pipe::unidirectional(128),
                git_features::io::pipe::unidirectional(128),
            );
            let action_name = action.as_helper_arg(true).into();
            self.handle = std::thread::spawn(move || {
                git_credentials::helper::main(
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
