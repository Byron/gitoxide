use crate::file::cow_str;
use crate::file::from_paths::escape_backslashes;
use bstr::BString;
use git_config::file::from_paths;
use git_ref::FullName;
use std::convert::TryFrom;
use std::fs;
use tempfile::tempdir;

#[test]
fn mixed() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("a");
    let hierarchy_branch_path = dir.path().join("hierarchy_branch");
    let branch_path = dir.path().join("branch");
    let another_branch_path = dir.path().join("another branch");
    fs::write(
        branch_path.as_path(),
        "
[core]
x = branch-override",
    )
    .unwrap();

    fs::write(
        another_branch_path.as_path(),
        "
[core]
z = another-branch-override",
    )
    .unwrap();

    fs::write(
        hierarchy_branch_path.as_path(),
        "
[core]
y = hierarchy-branch-override",
    )
    .unwrap();

    fs::write(
        config_path.as_path(),
        format!(
            r#"
[core]
x = 1
y = 1
z = 1
[includeIf "onbranch:foo*"]
path = {}
[includeIf "onbranch:br/"]
path = {}
[includeIf "onbranch:foo*"]
path = {}"#,
            escape_backslashes(&branch_path),
            escape_backslashes(&hierarchy_branch_path),
            escape_backslashes(&another_branch_path),
        ),
    )
    .unwrap();

    {
        let branch_name = FullName::try_from(BString::from("refs/heads/foobar")).unwrap();
        let branch_name = branch_name.as_ref();
        let options = from_paths::Options {
            branch_name: Some(branch_name),
            ..Default::default()
        };

        let config = git_config::File::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "x"),
            Some(cow_str("branch-override")),
            "branch name match"
        );
    }

    {
        let branch_name = FullName::try_from(BString::from("refs/heads/foo/bar")).unwrap();
        let branch_name = branch_name.as_ref();
        let options = from_paths::Options {
            branch_name: Some(branch_name),
            ..Default::default()
        };

        let config = git_config::File::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "z"),
            Some(cow_str("1")),
            "branch name match"
        );
    }

    {
        let branch_name = FullName::try_from(BString::from("refs/heads/br/one")).unwrap();
        let branch_name = branch_name.as_ref();
        let options = from_paths::Options {
            branch_name: Some(branch_name),
            ..Default::default()
        };

        let config = git_config::File::from_paths(Some(&config_path), options).unwrap();
        assert_eq!(
            config.string("core", None, "y"),
            Some(cow_str("hierarchy-branch-override")),
            "hierarchy branch name match"
        );
    }
}
