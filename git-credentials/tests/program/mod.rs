use git_credentials::helper::{invoke, Kind};
use git_credentials::Program;

#[test]
fn git_credentials() {
    assert!(
        matches!(
            git_credentials::helper::invoke(
                Program::from_kind(Kind::GitCredential),
                invoke::Action::get_for_url("/path/without/scheme/fails/with/error"),
            )
            .unwrap_err(),
            invoke::Error::CredentialsHelperFailed { .. }
        ),
        "this failure indicates we could launch the helper, even though it wasn't happy which is fine"
    );
}
