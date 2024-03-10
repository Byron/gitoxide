use std::borrow::Cow;

use gix_ref::{Category, FullNameRef, PartialNameRef};

#[test]
fn cow() {
    fn _fn(_x: Cow<'_, FullNameRef>) {}
    fn _pn(_x: Cow<'_, PartialNameRef>) {}
}

#[test]
fn file_name() {
    let name: gix_ref::FullName = "refs/heads/main".try_into().unwrap();
    assert_eq!(name.as_ref().file_name(), "main");
}
#[test]
fn shorten_and_category() {
    for (input, expected, category, is_worktree_private) in [
        ("refs/tags/tag-name", "tag-name", Category::Tag, false),
        ("refs/heads/main", "main", Category::LocalBranch, false),
        ("refs/remotes/origin/main", "origin/main", Category::RemoteBranch, false),
        ("refs/notes/note-name", "notes/note-name", Category::Note, false),
        ("HEAD", "HEAD", Category::PseudoRef, true),
        ("FETCH_HEAD", "FETCH_HEAD", Category::PseudoRef, true),
        ("main-worktree/HEAD", "HEAD", Category::MainPseudoRef, true),
        ("main-worktree/FETCH_HEAD", "FETCH_HEAD", Category::MainPseudoRef, true),
        (
            "main-worktree/refs/heads/main",
            "refs/heads/main",
            Category::MainRef,
            false,
        ),
        (
            "main-worktree/refs/notes/note",
            "refs/notes/note",
            Category::MainRef,
            false,
        ),
        (
            "worktrees/name/HEAD",
            "HEAD",
            Category::LinkedPseudoRef { name: "name".into() },
            true,
        ),
        (
            "worktrees/name/FETCH_HEAD",
            "FETCH_HEAD",
            Category::LinkedPseudoRef { name: "name".into() },
            true,
        ),
        (
            "worktrees/name/refs/heads/main",
            "refs/heads/main",
            Category::LinkedRef { name: "name".into() },
            false,
        ),
        (
            "worktrees/name/refs/notes/note",
            "refs/notes/note",
            Category::LinkedRef { name: "name".into() },
            false,
        ),
        (
            "worktrees/name/refs/heads/main",
            "refs/heads/main",
            Category::LinkedRef { name: "name".into() },
            false,
        ),
        ("refs/bisect/good", "bisect/good", Category::Bisect, true),
        ("refs/rewritten/123456", "rewritten/123456", Category::Rewritten, true),
        (
            "refs/worktree/private",
            "worktree/private",
            Category::WorktreePrivate,
            true,
        ),
    ] {
        let name: gix_ref::FullName = input.try_into().unwrap();
        assert_eq!(category.is_worktree_private(), is_worktree_private);
        let category = Some(category);
        assert_eq!(name.as_ref().shorten(), expected);
        assert_eq!(name.shorten(), expected);
        assert_eq!(name.category(), category);
        assert_eq!(
            name.category_and_short_name(),
            category.map(|cat| (cat, expected.into()))
        );
        assert_eq!(name.as_ref().category(), category);
    }

    for special in ["hello/world", "main-worktree/head"] {
        let name: gix_ref::FullName = special.try_into().unwrap();
        assert_eq!(
            name.shorten(),
            special,
            "the whole name is returned if there is no prefix"
        );
        assert_eq!(name.category(), None);
    }
}

#[test]
fn prefix_with_namespace_and_stripping() {
    let ns = gix_ref::namespace::expand("foo").unwrap();
    let mut name: gix_ref::FullName = "refs/heads/main".try_into().unwrap();
    assert_eq!(
        name.prefix_namespace(&ns).as_bstr(),
        "refs/namespaces/foo/refs/heads/main"
    );
    assert_eq!(
        name.prefix_namespace(&ns).as_bstr(),
        "refs/namespaces/foo/refs/heads/main",
        "idempotent prefixing"
    );
    assert_eq!(name.strip_namespace(&ns).as_bstr(), "refs/heads/main");
    assert_eq!(
        name.strip_namespace(&ns).as_bstr(),
        "refs/heads/main",
        "idempotent stripping"
    );
}
