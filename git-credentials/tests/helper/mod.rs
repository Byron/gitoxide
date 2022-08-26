mod context;
mod invoke;

mod invoke_outcome_to_helper_result {
    use git_credentials::helper;
    use git_credentials::helper::{invoke, invoke_outcome_to_helper_result};

    #[test]
    fn missing_username_or_password_causes_failure_with_get_action() {
        let action = invoke::Action::get_for_url("does/not/matter");
        let err = invoke_outcome_to_helper_result(
            Some(invoke::Outcome {
                username: None,
                password: None,
                quit: false,
                next: helper::Context::default().into(),
            }),
            action,
        )
        .unwrap_err();
        assert!(matches!(err, helper::Error::IdentityMissing { .. }));
    }

    #[test]
    fn quit_message_in_context_causes_special_error_ignoring_missing_identity() {
        let action = invoke::Action::get_for_url("does/not/matter");
        let err = invoke_outcome_to_helper_result(
            Some(invoke::Outcome {
                username: None,
                password: None,
                quit: true,
                next: helper::Context::default().into(),
            }),
            action,
        )
        .unwrap_err();
        assert!(matches!(err, helper::Error::Quit));
    }
}
