fn submodule(bytes: &str) -> gix_submodule::File {
    gix_submodule::File::from_bytes(bytes.as_bytes(), None).expect("valid module")
}

mod path {
    use crate::file::submodule;
    use gix_submodule::config::path::Error;

    fn submodule_path(value: &str) -> Error {
        let module = submodule(&format!("[submodule.a]\npath = {value}"));
        module.path("a".into()).unwrap_err()
    }

    #[test]
    fn valid() -> crate::Result {
        let module = submodule("[submodule.a]\n path = relative/path/submodule");
        assert_eq!(module.path("a".into())?.as_ref(), "relative/path/submodule");
        Ok(())
    }

    #[test]
    fn validate_upon_retrieval() {
        assert!(matches!(
            submodule_path(if cfg!(windows) {
                "c:\\\\hello"
            } else {
                "/definitely/absolute\\\\"
            }),
            Error::Absolute { .. }
        ));
        assert!(matches!(submodule_path(""), Error::Missing { .. }));
        assert!(matches!(submodule_path("../attack"), Error::OutsideOfWorktree { .. }));

        {
            let module = submodule("[submodule.a]\n path");
            assert!(matches!(module.path("a".into()).unwrap_err(), Error::Missing { .. }));
        }

        {
            let module = submodule("[submodule.a]\n");
            assert!(matches!(module.path("a".into()).unwrap_err(), Error::Missing { .. }));
        }
    }
}

mod url {
    use crate::file::submodule;
    use gix_submodule::config::url::Error;

    fn submodule_url(value: &str) -> Error {
        let module = submodule(&format!("[submodule.a]\nurl = {value}"));
        module.url("a".into()).unwrap_err()
    }

    #[test]
    fn valid() -> crate::Result {
        let module = submodule("[submodule.a]\n url = path-to-repo");
        assert_eq!(module.url("a".into())?.to_bstring(), "path-to-repo");
        Ok(())
    }

    #[test]
    fn validate_upon_retrieval() {
        assert!(matches!(submodule_url(""), Error::Missing { .. }));
        {
            let module = submodule("[submodule.a]\n url");
            assert!(matches!(module.url("a".into()).unwrap_err(), Error::Missing { .. }));
        }

        {
            let module = submodule("[submodule.a]\n");
            assert!(matches!(module.url("a".into()).unwrap_err(), Error::Missing { .. }));
        }

        assert!(matches!(submodule_url("file://"), Error::Parse { .. }));
    }
}

mod update {
    use crate::file::submodule;
    use gix_submodule::config::update::Error;
    use gix_submodule::config::Update;
    use std::str::FromStr;

    fn submodule_update(value: &str) -> Error {
        let module = submodule(&format!("[submodule.a]\nupdate = {value}"));
        module.update("a".into()).unwrap_err()
    }

    #[test]
    fn default() {
        assert_eq!(Update::default(), Update::Checkout, "as defined in the docs");
    }

    #[test]
    fn valid() -> crate::Result {
        for (valid, expected) in [
            ("checkout", Update::Checkout),
            ("rebase", Update::Rebase),
            ("merge", Update::Merge),
            ("none", Update::None),
        ] {
            let module = submodule(&format!("[submodule.a]\n update = {valid}"));
            assert_eq!(module.update("a".into())?.expect("present"), expected);
        }
        Ok(())
    }

    #[test]
    fn valid_in_overrides() -> crate::Result {
        let mut module = submodule("[submodule.a]\n update = merge");
        let repo_config = gix_config::File::from_str("[submodule.a]\n update = !dangerous")?;
        module.append_submodule_overrides(&repo_config);

        assert_eq!(
            module.update("a".into())?.expect("present"),
            Update::Command("dangerous".into()),
            "overridden values are picked up and make commands possible - these are local"
        );
        Ok(())
    }

    #[test]
    fn validate_upon_retrieval() {
        assert!(matches!(submodule_update(""), Error::Invalid { .. }));
        assert!(matches!(submodule_update("bogus"), Error::Invalid { .. }));
        assert!(
            matches!(
                submodule_update("!dangerous"),
                Error::CommandForbiddenInModulesConfiguration { .. }
            ),
            "forbidden unless it's an override"
        );
    }
}

mod fetch_recurse {
    use crate::file::submodule;
    use gix_submodule::config::FetchRecurse;

    #[test]
    fn default() {
        assert_eq!(
            FetchRecurse::default(),
            FetchRecurse::OnDemand,
            "as defined in git codebase actually"
        );
    }

    #[test]
    fn valid() -> crate::Result {
        for (valid, expected) in [
            ("yes", FetchRecurse::Always),
            ("true", FetchRecurse::Always),
            ("", FetchRecurse::Never),
            ("no", FetchRecurse::Never),
            ("false", FetchRecurse::Never),
            ("on-demand", FetchRecurse::OnDemand),
        ] {
            let module = submodule(&format!("[submodule.a]\n fetchRecurseSubmodules = {valid}"));
            assert_eq!(module.fetch_recurse("a".into())?.expect("present"), expected);
        }
        let module = submodule("[submodule.a]\n fetchRecurseSubmodules");
        assert_eq!(
            module.fetch_recurse("a".into())?.expect("present"),
            FetchRecurse::Always,
            "no value means true, which means to always recurse"
        );
        Ok(())
    }

    #[test]
    fn validate_upon_retrieval() -> crate::Result {
        for invalid in ["foo", "ney", "On-demand"] {
            let module = submodule(&format!("[submodule.a]\n fetchRecurseSubmodules = \"{invalid}\""));
            assert!(module.fetch_recurse("a".into()).is_err());
        }
        Ok(())
    }
}

mod ignore {
    use crate::file::submodule;
    use gix_submodule::config::Ignore;

    #[test]
    fn default() {
        assert_eq!(Ignore::default(), Ignore::None, "as defined in the docs");
    }

    #[test]
    fn valid() -> crate::Result {
        for (valid, expected) in [
            ("all", Ignore::All),
            ("dirty", Ignore::Dirty),
            ("untracked", Ignore::Untracked),
            ("none", Ignore::None),
        ] {
            let module = submodule(&format!("[submodule.a]\n ignore = {valid}"));
            assert_eq!(module.ignore("a".into())?.expect("present"), expected);
        }
        let module = submodule("[submodule.a]\n ignore");
        assert!(
            module.ignore("a".into())?.is_none(),
            "no value is interpreted as non-existing string, hence the caller will see None"
        );
        Ok(())
    }

    #[test]
    fn validate_upon_retrieval() -> crate::Result {
        for invalid in ["All", ""] {
            let module = submodule(&format!("[submodule.a]\n ignore = \"{invalid}\""));
            assert!(module.ignore("a".into()).is_err());
        }
        Ok(())
    }
}

mod branch {
    use crate::file::submodule;
    use gix_submodule::config::Branch;

    #[test]
    fn valid() -> crate::Result {
        for (valid, expected) in [
            (".", Branch::CurrentInSuperproject),
            ("", Branch::Name("HEAD".into())),
            ("master", Branch::Name("master".into())),
            ("feature/a", Branch::Name("feature/a".into())),
        ] {
            let module = submodule(&format!("[submodule.a]\n branch = {valid}"));
            assert_eq!(module.branch("a".into())?.expect("present"), expected);
        }
        let module = submodule("[submodule.a]\n branch");
        assert!(
            module.branch("a".into())?.is_none(),
            "no value implies it's not set, but the caller will then default"
        );
        Ok(())
    }

    #[test]
    fn validate_upon_retrieval() -> crate::Result {
        let module = submodule("[submodule.a]\n branch = /invalid");
        assert!(module.branch("a".into()).is_err());
        Ok(())
    }
}

#[test]
fn shallow() -> crate::Result {
    let module = submodule("[submodule.a]\n shallow");
    assert_eq!(
        module.shallow("a".into())?,
        Some(true),
        "shallow is a simple boolean without anything special (yet)"
    );
    Ok(())
}

mod append_submodule_overrides {
    use crate::file::submodule;
    use std::str::FromStr;

    #[test]
    fn last_of_multiple_values_wins() -> crate::Result {
        let mut module = submodule("[submodule.a] url = from-module");
        let repo_config =
            gix_config::File::from_str("[submodule.a]\n url = a\n url = b\n ignore = x\n [submodule.a]\n url = c\n[submodule.b] url = not-relevant")?;
        module.append_submodule_overrides(&repo_config);
        Ok(())
    }
}

mod baseline;
