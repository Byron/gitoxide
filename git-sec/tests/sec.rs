pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

mod trust {
    use git_sec::Trust;

    #[test]
    fn ordering() {
        assert!(Trust::Reduced < Trust::Full);
    }
}

mod identity;
