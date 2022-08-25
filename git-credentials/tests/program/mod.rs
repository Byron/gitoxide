use git_credentials::helper::invoke;
use git_credentials::{program::Kind, Program};

#[test]
fn builtin() {
    assert!(
        matches!(
            git_credentials::helper::invoke(
                Program::from_kind(Kind::Builtin),
                invoke::Action::get_for_url("/path/without/scheme/fails/with/error"),
            )
            .unwrap_err(),
            invoke::Error::CredentialsHelperFailed { .. }
        ),
        "this failure indicates we could launch the helper, even though it wasn't happy which is fine"
    );
}
