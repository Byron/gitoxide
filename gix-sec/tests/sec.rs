pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod trust {
    use gix_sec::Trust;

    #[test]
    fn ordering() {
        assert!(Trust::Reduced < Trust::Full);
    }
}

mod permission {
    use gix_sec::Permission;

    #[test]
    fn check() {
        assert_eq!(Permission::Allow.check("hi").unwrap(), Some("hi"));
        assert_eq!(Permission::Deny.check("hi").unwrap(), None);
        assert!(Permission::Forbid.check("hi").is_err());
    }

    #[test]
    fn check_opt() {
        assert_eq!(Permission::Allow.check_opt("hi"), Some("hi"));
        assert_eq!(Permission::Deny.check_opt("hi"), None);
        assert_eq!(Permission::Forbid.check_opt("hi"), None);
    }

    #[test]
    fn is_allowed() {
        assert!(Permission::Allow.is_allowed());
        assert!(!Permission::Deny.is_allowed());
        assert!(!Permission::Forbid.is_allowed());
    }
}

mod identity;
