mod cascade;
mod context;
mod invoke;

mod invoke_outcome_to_helper_result {
    use gix_credentials::{helper, protocol, protocol::helper_outcome_to_result};

    #[test]
    fn missing_username_or_password_causes_failure_with_get_action() {
        let action = helper::Action::get_for_url("does/not/matter");
        let err = helper_outcome_to_result(
            Some(helper::Outcome {
                username: None,
                password: None,
                quit: false,
                next: protocol::Context::default().into(),
            }),
            action,
        )
        .unwrap_err();
        assert!(matches!(err, protocol::Error::IdentityMissing { .. }));
    }

    #[test]
    fn quit_message_in_context_causes_special_error_ignoring_missing_identity() {
        let action = helper::Action::get_for_url("does/not/matter");
        let err = helper_outcome_to_result(
            Some(helper::Outcome {
                username: None,
                password: None,
                quit: true,
                next: protocol::Context::default().into(),
            }),
            action,
        )
        .unwrap_err();
        assert!(matches!(err, protocol::Error::Quit));
    }
}
