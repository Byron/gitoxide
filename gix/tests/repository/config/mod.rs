mod config_snapshot;
mod identity;
mod remote;

#[cfg(feature = "blocking-network-client")]
mod ssh_options {
    use std::ffi::OsStr;

    use crate::repository::config::repo;

    #[test]
    fn with_command_and_variant() -> crate::Result {
        let repo = repo("ssh-all-options");
        let opts = repo.ssh_connect_options()?;
        assert_eq!(opts.command.as_deref(), Some(OsStr::new("ssh -VVV")));
        assert_eq!(opts.kind, Some(gix::protocol::transport::client::ssh::ProgramKind::Ssh));
        assert!(!opts.disallow_shell, "we can use the shell by default");
        Ok(())
    }

    #[test]
    fn with_command_fallback_which_disallows_shell() -> crate::Result {
        let repo = repo("ssh-command-fallback");
        let opts = repo.ssh_connect_options()?;
        assert_eq!(opts.command.as_deref(), Some(OsStr::new("ssh --fallback")));
        assert_eq!(
            opts.kind,
            Some(gix::protocol::transport::client::ssh::ProgramKind::Putty)
        );
        assert!(
            opts.disallow_shell,
            "fallbacks won't allow shells, so must be a program or program name"
        );
        Ok(())
    }
}

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
mod transport_options;

#[cfg(feature = "blocking-network-client")]
#[cfg(feature = "blocking-network-client")]
pub fn repo(name: &str) -> gix::Repository {
    repo_opts(name, |opts| opts.strict_config(true))
}

#[cfg(feature = "blocking-network-client")]
pub fn repo_opts(name: &str, modify: impl FnOnce(gix::open::Options) -> gix::open::Options) -> gix::Repository {
    let dir = gix_testtools::scripted_fixture_read_only("make_config_repos.sh").unwrap();
    gix::open_opts(dir.join(name), modify(gix::open::Options::isolated())).unwrap()
}
