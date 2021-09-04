use git_ref::transaction::PreviousValue;
use git_repository as git;
use git_repository::prelude::ReferenceAccessExt;
use git_testtools::hex_to_id;

#[test]
fn symbolic() -> crate::Result {
    let repo = crate::basic_repo()?;
    let head = repo.head()?;
    match &head.kind {
        git::easy::head::Kind::Symbolic(r) => {
            assert_eq!(
                r.target.as_id().map(ToOwned::to_owned),
                Some(hex_to_id("3189cd3cb0af8586c39a838aa3e54fd72a872a41"))
            );
        }
        _ => panic!("unexpected head kind"),
    }
    assert_eq!(head.name().expect("born").as_bstr(), "refs/heads/main");
    assert!(!head.is_detached());
    Ok(())
}

#[test]
fn detached() -> crate::Result {
    let (repo, _keep) = crate::basic_rw_repo()?;
    repo.reference(
        "HEAD",
        hex_to_id("3189cd3cb0af8586c39a838aa3e54fd72a872a41"),
        PreviousValue::Any,
        "",
    )?;

    let head = repo.head()?;
    assert!(head.is_detached(), "head is detached");
    assert!(head.name().is_none());
    Ok(())
}
