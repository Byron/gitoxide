use crate::helper::invoke::util::MockHelper;
use git_credentials::helper::Action;

#[test]
#[ignore]
fn get() {
    let mut helper = MockHelper::default();
    let outcome =
        git_credentials::helper::invoke(&mut helper, Action::get_for_url("https://github.com/byron/gitoxide"))
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
        "url=https://github.com/byron/gitoxide\nusername=user\npass=pass\n"
    );
}

mod util {
    use git_credentials::helper::Action;

    #[derive(Default)]
    pub struct MockHelper {
        handle: Option<std::thread::JoinHandle<()>>,
    }

    impl git_credentials::Helper for &mut MockHelper {
        type Send = git_features::io::pipe::Writer;
        type Receive = git_features::io::pipe::Reader;

        fn start(&mut self, action: &Action) -> std::io::Result<(Self::Send, Option<Self::Receive>)> {
            let ((them_send, us_receive), output) = match action {
                Action::Get(_) => (git_features::io::pipe::unidirectional(128), None),
                Action::Erase(_) | Action::Store(_) => (
                    git_features::io::pipe::unidirectional(128),
                    git_features::io::pipe::unidirectional(128).into(),
                ),
            };
            let (us_send, them_receive) = output.map(|(tx, rx)| (Some(tx), Some(rx))).unwrap_or_default();
            self.handle = std::thread::spawn(move || todo!("thread main")).into();
            Ok((them_send, them_receive))
        }

        fn finish(self) -> std::io::Result<()> {
            Ok(self.handle.take().expect("started").join().unwrap())
        }
    }
}
