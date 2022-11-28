use crate::named_repo;

#[test]
fn commit_auto_rollback() -> crate::Result {
    let mut repo: git_repository::Repository = named_repo("make_basic_repo.sh")?;
    assert_eq!(repo.head_id()?.shorten()?.to_string(), "3189cd3");

    {
        let mut config = repo.config_snapshot_mut();
        config.set_raw_value("core", None, "abbrev", "4")?;
        let repo = config.commit_auto_rollback()?;
        assert_eq!(repo.head_id()?.shorten()?.to_string(), "3189");
    }

    assert_eq!(repo.head_id()?.shorten()?.to_string(), "3189cd3");

    let repo = {
        let mut config = repo.config_snapshot_mut();
        config.set_raw_value("core", None, "abbrev", "4")?;
        let mut repo = config.commit_auto_rollback()?;
        assert_eq!(repo.head_id()?.shorten()?.to_string(), "3189");
        // access to the mutable repo underneath
        repo.object_cache_size_if_unset(16 * 1024);
        repo.rollback()?
    };
    assert_eq!(repo.head_id()?.shorten()?.to_string(), "3189cd3");

    Ok(())
}

#[test]
fn snapshot_mut_commit_and_forget() -> crate::Result {
    let mut repo: git_repository::Repository = named_repo("make_basic_repo.sh")?;
    let repo = {
        let mut repo = repo.config_snapshot_mut();
        repo.set_raw_value("core", None, "abbrev", "4")?;
        repo.commit()?
    };
    assert_eq!(repo.config_snapshot().integer("core.abbrev").expect("set"), 4);
    {
        let mut repo = repo.config_snapshot_mut();
        repo.set_raw_value("core", None, "abbrev", "8")?;
        repo.forget();
    }
    assert_eq!(repo.config_snapshot().integer("core.abbrev"), Some(4));
    Ok(())
}

#[test]
fn values_are_set_in_memory_only() {
    let mut repo = named_repo("make_config_repo.sh").unwrap();
    let repo_clone = repo.clone();
    let key = "hallo.welt";
    let key_subsection = "hallo.unter.welt";
    assert_eq!(repo.config_snapshot().boolean(key), None, "no value there just yet");
    assert_eq!(repo.config_snapshot().string(key_subsection), None);

    {
        let mut config = repo.config_snapshot_mut();
        config.set_raw_value("hallo", None, "welt", "true").unwrap();
        config
            .set_raw_value("hallo", Some("unter".into()), "welt", "value")
            .unwrap();
    }

    assert_eq!(
        repo.config_snapshot().boolean(key),
        Some(true),
        "value was set and applied"
    );
    assert_eq!(
        repo.config_snapshot().string(key_subsection).as_deref(),
        Some("value".into())
    );

    assert_eq!(
        repo_clone.config_snapshot().boolean(key),
        None,
        "values are not written back automatically nor are they shared between clones"
    );
    assert_eq!(repo_clone.config_snapshot().string(key_subsection), None);
}

#[test]
fn apply_cli_overrides() -> crate::Result {
    let mut repo = named_repo("make_config_repo.sh").unwrap();
    repo.config_snapshot_mut().append_config(
        [
            "a.b=c",
            "remote.origin.url = url",
            "implicit.bool-true",
            "implicit.bool-false = ",
        ],
        git_config::Source::Cli,
    )?;

    let config = repo.config_snapshot();
    assert_eq!(config.string("a.b").expect("present").as_ref(), "c");
    assert_eq!(config.string("remote.origin.url").expect("present").as_ref(), "url");
    assert_eq!(
        config.string("implicit.bool-true"),
        None,
        "no keysep is interpreted as 'not present' as we don't make up values"
    );
    assert_eq!(
        config.string("implicit.bool-false").expect("present").as_ref(),
        "",
        "empty values are fine"
    );
    assert_eq!(
        config.boolean("implicit.bool-false"),
        Some(false),
        "empty values are boolean true"
    );
    assert_eq!(
        config.boolean("implicit.bool-true"),
        Some(true),
        "values without key-sep are true"
    );

    Ok(())
}

mod credential_helpers;
