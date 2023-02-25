use std::borrow::Cow;

use gix_object::bstr::BStr;

fn bcow(input: &str) -> Cow<'_, BStr> {
    Cow::Borrowed(input.into())
}

mod keys {
    use std::borrow::Cow;

    use gix::config::tree::{Key, Section};
    use gix_object::bstr::ByteSlice;

    use crate::config::tree::bcow;

    #[test]
    fn string() -> crate::Result {
        assert_eq!(
            gix::config::tree::Http::USER_AGENT.try_into_string(bcow("agent"))?,
            "agent"
        );
        assert!(gix::config::tree::Http::USER_AGENT.validate("agent".into()).is_ok());

        let invalid = b"\xF0\x80\x80".as_bstr();
        assert_eq!(
            gix::config::tree::Http::USER_AGENT
                .try_into_string(Cow::Borrowed(invalid))
                .unwrap_err()
                .to_string(),
            "The utf-8 string at \"http.userAgent=���\" could not be decoded"
        );
        assert!(gix::config::tree::Http::USER_AGENT.validate(invalid).is_err());

        Ok(())
    }

    #[test]
    fn any() {
        assert!(
            !gix::config::Tree.sections().is_empty(),
            "the root has at least one section"
        );
        assert_eq!(gix::config::Tree::AUTHOR.name(), "author");
        assert_eq!(gix::config::tree::Author.keys().len(), 2);
        assert_eq!(gix::config::tree::Author::NAME.name(), "name");
        assert_eq!(gix::config::tree::Author::EMAIL.name(), "email");
        assert_eq!(
            gix::config::tree::Author::NAME
                .validated_assignment("user".into())
                .unwrap(),
            "author.name=user"
        );
        assert_eq!(
            gix::config::tree::Author::NAME
                .validated_assignment("user".into())
                .unwrap(),
            "author.name=user"
        );
    }

    #[test]
    fn remote_name() {
        assert!(gix::config::tree::Remote::PUSH_DEFAULT
            .validate("origin".into())
            .is_ok());
        assert!(gix::config::tree::Remote::PUSH_DEFAULT
            .validate("https://github.com/byron/gitoxide".into())
            .is_ok());
    }
}

mod branch {
    use gix::config::tree::{branch, Branch, Key};

    use crate::config::tree::bcow;

    #[test]
    fn merge() {
        assert!(branch::Merge::try_into_fullrefname(bcow("refs/heads/main")).is_ok());
        assert!(branch::Merge::try_into_fullrefname(bcow("main")).is_err());

        assert!(Branch::MERGE.full_name(None).is_err());
        assert_eq!(
            Branch::MERGE.full_name(Some("name".into())).expect("valid"),
            "branch.name.merge"
        );
    }
}

mod ssh {

    #[test]
    #[cfg(feature = "blocking-network-client")]
    fn variant() -> crate::Result {
        use gix::config::tree::Ssh;
        use gix_protocol::transport::client::ssh::ProgramKind;

        use crate::config::tree::bcow;
        for (actual, expected) in [
            ("auto", None),
            ("ssh", Some(ProgramKind::Ssh)),
            ("simple", Some(ProgramKind::Simple)),
            ("plink", Some(ProgramKind::Plink)),
            ("putty", Some(ProgramKind::Putty)),
            ("tortoiseplink", Some(ProgramKind::TortoisePlink)),
        ] {
            assert_eq!(Ssh::VARIANT.try_into_variant(bcow(actual))?, expected);
        }

        assert_eq!(
            Ssh::VARIANT.try_into_variant(bcow("SSH")).unwrap_err().to_string(),
            "The key \"ssh.variant=SSH\" (possibly from GIT_SSH_VARIANT) was invalid",
            "case-sensitive comparisons"
        );
        Ok(())
    }
}

mod diff {
    use gix::{
        config::tree::{Diff, Key},
        diff::rename::Tracking,
    };
    use gix_diff::blob::Algorithm;

    use crate::config::tree::bcow;

    #[test]
    fn renames() -> crate::Result {
        assert_eq!(
            Diff::RENAMES.try_into_renames(Ok(true), || unreachable!())?,
            Tracking::Renames
        );
        assert!(Diff::RENAMES.validate("1".into()).is_ok());
        assert_eq!(
            Diff::RENAMES.try_into_renames(Ok(false), || unreachable!())?,
            Tracking::Disabled
        );
        assert!(Diff::RENAMES.validate("0".into()).is_ok());
        assert_eq!(
            Diff::RENAMES.try_into_renames(Err(gix_config::value::Error::new("err", "err")), || Some(bcow("copy")))?,
            Tracking::RenamesAndCopies
        );
        assert!(Diff::RENAMES.validate("copy".into()).is_ok());
        assert_eq!(
            Diff::RENAMES.try_into_renames(Err(gix_config::value::Error::new("err", "err")), || Some(bcow(
                "copies"
            )))?,
            Tracking::RenamesAndCopies
        );
        assert!(Diff::RENAMES.validate("copies".into()).is_ok());

        assert_eq!(
            Diff::RENAMES
                .try_into_renames(Err(gix_config::value::Error::new("err", "err")), || Some(bcow("foo")))
                .unwrap_err()
                .to_string(),
            "The value of key \"diff.renames=foo\" was invalid"
        );
        Ok(())
    }

    #[test]
    fn algorithm() -> crate::Result {
        for (actual, expected) in [
            ("myers", Algorithm::Myers),
            ("Myers", Algorithm::Myers),
            ("default", Algorithm::Myers),
            ("Default", Algorithm::Myers),
            ("minimal", Algorithm::MyersMinimal),
            ("histogram", Algorithm::Histogram),
        ] {
            assert_eq!(Diff::ALGORITHM.try_into_algorithm(bcow(actual))?, expected);
            assert!(Diff::ALGORITHM.validate(actual.into()).is_ok());
        }
        assert_eq!(
            Diff::ALGORITHM
                .try_into_algorithm(bcow("patience"))
                .unwrap_err()
                .to_string(),
            "The 'patience' algorithm is not yet implemented"
        );
        assert_eq!(
            Diff::ALGORITHM.try_into_algorithm(bcow("foo")).unwrap_err().to_string(),
            "Unknown diff algorithm named 'foo'"
        );
        Ok(())
    }
}

mod core {
    use std::time::Duration;

    use gix::{
        config::tree::{Core, Key},
        revision::spec::parse::ObjectKindHint,
    };
    use gix_lock::acquire::Fail;

    use crate::config::tree::bcow;

    fn signed(value: i64) -> Result<i64, gix_config::value::Error> {
        Ok(value)
    }

    #[test]
    fn timeouts() -> crate::Result {
        assert_eq!(
            Core::FILES_REF_LOCK_TIMEOUT.try_into_lock_timeout(Ok(0))?,
            Fail::Immediately
        );
        assert!(Core::FILES_REF_LOCK_TIMEOUT.validate("0".into()).is_ok());
        assert_eq!(
            Core::FILES_REF_LOCK_TIMEOUT.try_into_lock_timeout(Ok(-5))?,
            Fail::AfterDurationWithBackoff(Duration::from_secs(u64::MAX))
        );
        assert!(Core::FILES_REF_LOCK_TIMEOUT.validate("-1".into()).is_ok());

        assert_eq!(
            Core::FILES_REF_LOCK_TIMEOUT.try_into_lock_timeout(Ok(2500))?,
            Fail::AfterDurationWithBackoff(Duration::from_millis(2500))
        );
        assert!(Core::FILES_REF_LOCK_TIMEOUT.validate("2500".into()).is_ok());
        assert_eq!(
            Core::FILES_REF_LOCK_TIMEOUT
                .try_into_lock_timeout(Err(gix_config::value::Error::new("err", "bogus")))
                .unwrap_err()
                .to_string(),
            "The timeout at key \"core.filesRefLockTimeout\" was invalid"
        );
        Ok(())
    }

    #[test]
    fn disambiguate() -> crate::Result {
        for (value, expected) in [
            ("none", None),
            ("commit", Some(ObjectKindHint::Commit)),
            ("committish", Some(ObjectKindHint::Committish)),
            ("tree", Some(ObjectKindHint::Tree)),
            ("treeish", Some(ObjectKindHint::Treeish)),
            ("blob", Some(ObjectKindHint::Blob)),
        ] {
            assert_eq!(
                Core::DISAMBIGUATE.try_into_object_kind_hint(bcow(value)).unwrap(),
                expected
            );
            assert!(Core::DISAMBIGUATE.validate(value.into()).is_ok());
        }
        assert_eq!(
            Core::DISAMBIGUATE
                .try_into_object_kind_hint(bcow("CommiT"))
                .unwrap_err()
                .to_string(),
            "The key \"core.disambiguate=CommiT\" was invalid"
        );
        Ok(())
    }

    #[test]
    fn log_all_ref_updates() -> crate::Result {
        assert_eq!(
            Core::LOG_ALL_REF_UPDATES.try_into_ref_updates(Some(Ok(true)), || None)?,
            Some(gix_ref::store::WriteReflog::Normal)
        );
        assert!(Core::LOG_ALL_REF_UPDATES.validate("true".into()).is_ok());
        assert_eq!(
            Core::LOG_ALL_REF_UPDATES.try_into_ref_updates(Some(Ok(false)), || None)?,
            Some(gix_ref::store::WriteReflog::Disable)
        );
        assert!(Core::LOG_ALL_REF_UPDATES.validate("0".into()).is_ok());
        assert_eq!(
            Core::LOG_ALL_REF_UPDATES.try_into_ref_updates(None, || Some(bcow("always")))?,
            Some(gix_ref::store::WriteReflog::Always)
        );
        assert!(Core::LOG_ALL_REF_UPDATES.validate("always".into()).is_ok());
        assert_eq!(
            Core::LOG_ALL_REF_UPDATES
                .try_into_ref_updates(None, || Some(bcow("invalid")))
                .unwrap_err()
                .to_string(),
            "The key \"core.logAllRefUpdates=invalid\" was invalid"
        );
        assert!(Core::LOG_ALL_REF_UPDATES.validate("invalid".into()).is_err());
        Ok(())
    }

    #[test]
    fn abbrev() -> crate::Result {
        let object_hash = gix_hash::Kind::Sha1;
        assert_eq!(Core::ABBREV.try_into_abbreviation(bcow("4"), object_hash)?, Some(4));
        assert_eq!(Core::ABBREV.try_into_abbreviation(bcow("auto"), object_hash)?, None);
        assert_eq!(
            Core::ABBREV.try_into_abbreviation(bcow("AUto"), object_hash)?,
            None,
            "case-insensitive"
        );
        assert_eq!(
            Core::ABBREV.try_into_abbreviation(bcow("false"), object_hash)?,
            Some(object_hash.len_in_hex()),
            "turns abbreviations off entirely"
        );

        assert_eq!(
            Core::ABBREV
                .try_into_abbreviation(bcow("   "), object_hash)
                .unwrap_err()
                .to_string(),
            "Invalid value for 'core.abbrev' = '   '. It must be between 4 and 40"
        );
        for invalid in ["foo", "3", "41"] {
            assert!(Core::ABBREV.try_into_abbreviation(bcow(invalid), object_hash).is_err());
        }
        Ok(())
    }

    #[test]
    fn delta_base_cache_limit() -> crate::Result {
        assert_eq!(Core::DELTA_BASE_CACHE_LIMIT.try_into_usize(signed(1))?, 1);
        assert_eq!(Core::DELTA_BASE_CACHE_LIMIT.try_into_usize(signed(0))?, 0);
        assert!(Core::DELTA_BASE_CACHE_LIMIT.validate("0".into()).is_ok());
        assert!(Core::DELTA_BASE_CACHE_LIMIT.validate("1".into()).is_ok());
        assert_eq!(
            Core::DELTA_BASE_CACHE_LIMIT
                .try_into_usize(signed(-1))
                .unwrap_err()
                .to_string(),
            "The value of key \"core.deltaBaseCacheLimit\" (possibly from GITOXIDE_PACK_CACHE_MEMORY) could not be parsed as unsigned integer"
        );
        assert!(Core::DELTA_BASE_CACHE_LIMIT.validate("-1".into()).is_err());
        Ok(())
    }

    #[test]
    fn check_stat() -> crate::Result {
        assert!(Core::CHECK_STAT.try_into_checkstat(bcow("default"))?);
        assert!(!Core::CHECK_STAT.try_into_checkstat(bcow("minimal"))?);
        assert_eq!(
            Core::CHECK_STAT
                .try_into_checkstat(bcow("normal"))
                .unwrap_err()
                .to_string(),
            "The key \"core.checkStat=normal\" was invalid"
        );

        assert!(Core::CHECK_STAT.validate("default".into()).is_ok());
        assert!(Core::CHECK_STAT.validate("minimal".into()).is_ok());
        assert!(Core::CHECK_STAT.validate("foo".into()).is_err());
        Ok(())
    }
}

mod extensions {
    use gix::config::tree::{Extensions, Key};

    use crate::config::tree::bcow;

    #[test]
    fn object_format() -> crate::Result {
        assert_eq!(
            Extensions::OBJECT_FORMAT.try_into_object_format(bcow("sha1"))?,
            gix_hash::Kind::Sha1
        );
        assert_eq!(
            Extensions::OBJECT_FORMAT.try_into_object_format(bcow("SHA1"))?,
            gix_hash::Kind::Sha1,
            "case-insensitive"
        );
        assert_eq!(
            Extensions::OBJECT_FORMAT
                .try_into_object_format(bcow("invalid"))
                .unwrap_err()
                .to_string(),
            "The key \"extensions.objectFormat=invalid\" was invalid"
        );
        assert!(Extensions::OBJECT_FORMAT.validate("sha1".into()).is_ok());
        assert!(Extensions::OBJECT_FORMAT.validate("invalid".into()).is_err());
        Ok(())
    }
}

mod checkout {
    use gix::config::tree::{Checkout, Key};

    fn int(value: i64) -> Result<i64, gix_config::value::Error> {
        Ok(value)
    }

    #[test]
    fn workers() -> crate::Result {
        assert!(Checkout::WORKERS.validate("0".into()).is_ok());
        assert_eq!(Checkout::WORKERS.try_from_workers(int(0))?, 0);
        assert!(Checkout::WORKERS.validate("-1".into()).is_ok());
        assert_eq!(Checkout::WORKERS.try_from_workers(int(-1))?, 0);
        assert!(Checkout::WORKERS.validate("-2".into()).is_ok());
        assert!(Checkout::WORKERS.validate("3".into()).is_ok());
        assert_eq!(Checkout::WORKERS.try_from_workers(int(2))?, 2);
        Ok(())
    }
}

mod pack {
    use gix::config::tree::{Key, Pack};

    #[test]
    fn index_version() -> crate::Result {
        assert_eq!(
            Pack::INDEX_VERSION.try_into_index_version(Ok(1))?,
            gix_pack::index::Version::V1
        );
        assert!(Pack::INDEX_VERSION.validate("1".into()).is_ok());
        assert_eq!(
            Pack::INDEX_VERSION.try_into_index_version(Ok(2))?,
            gix_pack::index::Version::V2
        );
        assert!(Pack::INDEX_VERSION.validate("2".into()).is_ok());
        assert_eq!(
            Pack::INDEX_VERSION
                .try_into_index_version(Ok(3))
                .unwrap_err()
                .to_string(),
            "The value of key \"pack.indexVersion\" was invalid"
        );
        assert!(Pack::INDEX_VERSION.validate("3".into()).is_err());
        assert!(Pack::INDEX_VERSION.validate("-1".into()).is_err());
        Ok(())
    }
}

#[cfg(any(feature = "blocking-network-client", feature = "async-network-client"))]
mod protocol {
    use gix::{
        config::tree::{protocol, Key, Protocol},
        remote::url::scheme_permission::Allow,
    };

    use crate::config::tree::bcow;

    #[test]
    fn allow() -> crate::Result {
        for (key, protocol_name_parameter) in [
            (&Protocol::ALLOW, None),
            (&protocol::NameParameter::ALLOW, Some("http")),
        ] {
            for (input, expected) in [
                ("always", Allow::Always),
                ("never", Allow::Never),
                ("user", Allow::User),
            ] {
                assert_eq!(key.try_into_allow(bcow(input), protocol_name_parameter)?, expected);
                assert!(key.validate(input.into()).is_ok());
            }
            assert_eq!(
                key.try_into_allow(bcow("User"), protocol_name_parameter)
                    .unwrap_err()
                    .to_string(),
                format!(
                    "The value \"User\" must be allow|deny|user in configuration key {}",
                    protocol_name_parameter
                        .map(|key| format!("protocol.{key}.allow"))
                        .unwrap_or_else(|| "protocol.allow".into())
                )
            );
        }
        Ok(())
    }
}

mod gitoxide {
    mod http {
        use std::time::Duration;

        use gix::config::tree::{gitoxide, Key};

        #[test]
        fn connect_timeout() -> crate::Result {
            assert_eq!(
                gitoxide::Http::CONNECT_TIMEOUT.validated_assignment_fmt(&Duration::from_millis(1000).as_millis())?,
                "gitoxide.http.connectTimeout=1000"
            );
            Ok(())
        }
    }
    mod allow {
        use gix::config::tree::{gitoxide, Key};

        #[test]
        fn protocol_from_user() {
            assert!(
                gitoxide::Allow::PROTOCOL_FROM_USER.validate("1".into()).is_ok(),
                "this really is the only valid value"
            );
            assert!(gitoxide::Allow::PROTOCOL_FROM_USER.validate("true".into()).is_err());
            assert!(gitoxide::Allow::PROTOCOL_FROM_USER.validate("0".into()).is_err());
        }
    }
    mod commit {
        use gix::config::tree::{gitoxide, Key};

        #[test]
        fn author_and_committer_date() {
            assert_eq!(
                gitoxide::Commit::AUTHOR_DATE
                    .validated_assignment("Thu, 1 Aug 2022 12:45:06 +0800".into())
                    .expect("valid"),
                "gitoxide.commit.authorDate=Thu, 1 Aug 2022 12:45:06 +0800"
            );
            assert_eq!(
                gitoxide::Commit::COMMITTER_DATE
                    .validated_assignment("Thu, 1 Aug 2022 12:45:06 +0800".into())
                    .expect("valid"),
                "gitoxide.commit.committerDate=Thu, 1 Aug 2022 12:45:06 +0800"
            );
        }
    }
    mod author {
        use gix::config::tree::{gitoxide, Key};

        #[test]
        fn name_and_email_fallback() {
            assert_eq!(
                gitoxide::Author::NAME_FALLBACK
                    .validated_assignment("name".into())
                    .expect("valid"),
                "gitoxide.author.nameFallback=name"
            );
            assert_eq!(
                gitoxide::Author::EMAIL_FALLBACK
                    .validated_assignment("email".into())
                    .expect("valid"),
                "gitoxide.author.emailFallback=email"
            );
        }
    }
    mod committer {
        use gix::config::tree::{gitoxide, Key};

        #[test]
        fn name_and_email_fallback() {
            assert_eq!(
                gitoxide::Committer::NAME_FALLBACK
                    .validated_assignment("name".into())
                    .expect("valid"),
                "gitoxide.committer.nameFallback=name"
            );
            assert_eq!(
                gitoxide::Committer::EMAIL_FALLBACK
                    .validated_assignment("email".into())
                    .expect("valid"),
                "gitoxide.committer.emailFallback=email"
            );
        }
    }
}

#[cfg(any(
    feature = "blocking-http-transport-reqwest",
    feature = "blocking-http-transport-curl"
))]
mod http {
    use std::borrow::Cow;

    use gix::config::tree::{Http, Key};
    use gix_object::bstr::ByteSlice;

    use crate::config::tree::bcow;

    #[test]
    fn follow_redirects() -> crate::Result {
        use gix_transport::client::http::options::FollowRedirects;
        assert_eq!(
            Http::FOLLOW_REDIRECTS.try_into_follow_redirects(bcow("initial"), || unreachable!("no call"))?,
            FollowRedirects::Initial
        );
        for (actual, cb_val, expected) in [
            ("true", Ok(Some(true)), FollowRedirects::All),
            ("false", Ok(Some(false)), FollowRedirects::None),
            // even though this is uncommon, with leniency it's possible to force it to internally default
            ("true", Ok(None), FollowRedirects::Initial),
        ] {
            assert_eq!(
                Http::FOLLOW_REDIRECTS.try_into_follow_redirects(bcow(actual), || cb_val)?,
                expected
            );
            assert!(Http::FOLLOW_REDIRECTS.validate(actual.into()).is_ok());
        }

        assert_eq!(
            Http::FOLLOW_REDIRECTS
                .try_into_follow_redirects(bcow("something"), || Err(gix_config::value::Error::new(
                    "invalid", "value"
                )))
                .unwrap_err()
                .to_string(),
            "The key \"http.followRedirects=something\" was invalid",
        );
        assert!(Http::FOLLOW_REDIRECTS.validate("foo".into()).is_err());
        Ok(())
    }

    #[test]
    fn extra_header() -> crate::Result {
        assert_eq!(
            Http::EXTRA_HEADER.try_into_extra_header(vec![bcow("a"), bcow("b")])?,
            ["a", "b"]
        );
        assert_eq!(
            Http::EXTRA_HEADER.try_into_extra_header(vec![bcow("a"), bcow("b"), bcow(""), bcow("c"), bcow("d")])?,
            ["c", "d"]
        );

        assert!(Http::EXTRA_HEADER.validate("a".into()).is_ok());

        let invalid = b"\xF0\x80\x80";
        assert!(Http::EXTRA_HEADER.validate(invalid.as_bstr()).is_err());
        assert_eq!(
            Http::EXTRA_HEADER
                .try_into_extra_header(vec![Cow::Borrowed(invalid.as_bstr())])
                .unwrap_err()
                .to_string(),
            "The utf-8 string at \"http.extraHeader=���\" could not be decoded"
        );
        Ok(())
    }

    #[test]
    fn http_version() -> crate::Result {
        use gix_transport::client::http::options::HttpVersion;

        for (actual, expected) in [("HTTP/1.1", HttpVersion::V1_1), ("HTTP/2", HttpVersion::V2)] {
            assert_eq!(Http::VERSION.try_into_http_version(bcow(actual))?, expected);
            assert!(Http::VERSION.validate(actual.into()).is_ok());
        }

        assert_eq!(
            Http::VERSION
                .try_into_http_version(bcow("invalid"))
                .unwrap_err()
                .to_string(),
            "The key \"http.version=invalid\" was invalid"
        );
        assert!(Http::VERSION.validate("invalid".into()).is_err());
        Ok(())
    }

    #[test]
    fn ssl_version() -> crate::Result {
        use gix_transport::client::http::options::SslVersion::*;

        for (actual, expected) in [
            ("default", Default),
            ("", Default),
            ("tlsv1", TlsV1),
            ("sslv2", SslV2),
            ("sslv3", SslV3),
            ("tlsv1.0", TlsV1_0),
            ("tlsv1.1", TlsV1_1),
            ("tlsv1.2", TlsV1_2),
            ("tlsv1.3", TlsV1_3),
        ] {
            assert_eq!(Http::SSL_VERSION.try_into_ssl_version(bcow(actual))?, expected);
            assert!(Http::SSL_VERSION.validate(actual.into()).is_ok());
        }

        assert_eq!(
            Http::SSL_VERSION
                .try_into_ssl_version(bcow("invalid"))
                .unwrap_err()
                .to_string(),
            "The ssl version at \"http.sslVersion=invalid\" (possibly from GIT_SSL_VERSION) was invalid"
        );
        assert!(Http::SSL_VERSION.validate("invalid".into()).is_err());
        Ok(())
    }

    #[test]
    fn proxy_auth_method() -> crate::Result {
        use gix_transport::client::http::options::ProxyAuthMethod::*;
        for (actual, expected) in [
            ("anyauth", AnyAuth),
            ("basic", Basic),
            ("digest", Digest),
            ("negotiate", Negotiate),
            ("ntlm", Ntlm),
        ] {
            assert_eq!(
                Http::PROXY_AUTH_METHOD.try_into_proxy_auth_method(bcow(actual))?,
                expected
            );
            assert!(Http::PROXY_AUTH_METHOD.validate(actual.into()).is_ok());
        }

        assert_eq!(
            Http::PROXY_AUTH_METHOD
                .try_into_proxy_auth_method(bcow("invalid"))
                .unwrap_err()
                .to_string(),
            "The key \"http.proxyAuthMethod=invalid\" was invalid"
        );
        assert!(Http::PROXY_AUTH_METHOD.validate("invalid".into()).is_err());
        Ok(())
    }
}

mod remote {
    use gix::{
        config::tree::{Key, Remote},
        remote,
    };

    use crate::config::tree::bcow;

    #[test]
    fn tag_opt() -> crate::Result {
        assert_eq!(
            Remote::TAG_OPT.try_into_tag_opt(bcow("--tags"))?,
            remote::fetch::Tags::All
        );
        assert!(Remote::TAG_OPT.validate("--tags".into()).is_ok());
        assert_eq!(
            Remote::TAG_OPT.try_into_tag_opt(bcow("--no-tags"))?,
            remote::fetch::Tags::None
        );
        assert!(Remote::TAG_OPT.validate("--no-tags".into()).is_ok());

        assert_eq!(
            Remote::TAG_OPT
                .try_into_tag_opt(bcow("--unknown"))
                .unwrap_err()
                .to_string(),
            "The key \"remote.<name>.tagOpt=--unknown\" was invalid"
        );
        Ok(())
    }

    #[test]
    fn url_and_push_url() {
        assert!(Remote::URL.try_into_url(bcow("http://example.org")).is_ok());
        assert!(Remote::URL.validate("http://example.org".into()).is_ok());

        assert_eq!(
            Remote::URL.try_into_url(bcow("https://")).unwrap_err().to_string(),
            "The url at \"remote.<name>.url=https://\" could not be parsed"
        );
        assert!(Remote::URL.validate("http://".into()).is_err());
    }

    #[test]
    fn refspecs() {
        let fetch_spec = "+refs/heads/*:refs/remotes/origin/*";
        assert!(Remote::FETCH
            .try_into_refspec(bcow(fetch_spec), gix_refspec::parse::Operation::Fetch)
            .is_ok());
        assert!(Remote::FETCH.validate(fetch_spec.into()).is_ok());

        let push_spec = "HEAD:refs/heads/name";
        assert!(Remote::PUSH
            .try_into_refspec(bcow(push_spec), gix_refspec::parse::Operation::Push)
            .is_ok());
        assert!(Remote::PUSH.validate(push_spec.into()).is_ok());

        assert_eq!(
            Remote::FETCH
                .try_into_refspec(bcow("*/*/*"), gix_refspec::parse::Operation::Fetch)
                .unwrap_err()
                .to_string(),
            "The refspec at \"remote.<name>.fetch=*/*/*\" could not be parsed"
        );
        assert_eq!(
            Remote::PUSH
                .try_into_refspec(bcow("*/*/*"), gix_refspec::parse::Operation::Push)
                .unwrap_err()
                .to_string(),
            "The refspec at \"remote.<name>.push=*/*/*\" could not be parsed"
        );
    }
}
