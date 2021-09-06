use git_ref::{FullName, Target};
use std::convert::TryInto;

#[test]
fn strip_namespace() {
    let ns = git_ref::namespace::expand("ns").unwrap();
    let mut r = git_ref::Reference {
        name: {
            let mut n: FullName = "refs/heads/main".try_into().unwrap();
            n.prefix_namespace(&ns);
            n
        },
        target: Target::Symbolic({
            let mut n: FullName = "refs/tags/foo".try_into().unwrap();
            n.prefix_namespace(&ns);
            n
        }),
        peeled: None,
    };
    r.strip_namespace(&ns);
    assert_eq!(r.name.as_bstr(), "refs/heads/main");
    assert!(matches!(r.target, Target::Symbolic(n) if n.as_bstr() == "refs/namespaces/ns/refs/tags/foo"));
}
