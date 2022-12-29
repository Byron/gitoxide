mod config_snapshot;
mod identity;
mod remote;

#[cfg(feature = "blocking-network-client")]
mod ssh_options {
    use std::ffi::OsStr;

    use git_repository as git;

    use crate::repository::config::repo;

    #[test]
    fn with_command_and_variant() -> crate::Result {
        let repo = repo("ssh-all-options");
        let opts = repo.ssh_connect_options()?;
        assert_eq!(opts.command.as_deref(), Some(OsStr::new("ssh -VVV")));
        assert_eq!(opts.kind, Some(git::protocol::transport::client::ssh::ProgramKind::Ssh));
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
            Some(git::protocol::transport::client::ssh::ProgramKind::Putty)
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
use git_repository as git;
#[cfg(feature = "blocking-network-client")]
pub fn repo(name: &str) -> git::Repository {
    repo_opts(name, |opts| opts.strict_config(true))
}

#[cfg(feature = "blocking-network-client")]
pub fn repo_opts(name: &str, modify: impl FnOnce(git::open::Options) -> git::open::Options) -> git::Repository {
    let dir = git_testtools::scripted_fixture_read_only("make_config_repos.sh").unwrap();
    git::open_opts(dir.join(name), modify(git::open::Options::isolated())).unwrap()
}
