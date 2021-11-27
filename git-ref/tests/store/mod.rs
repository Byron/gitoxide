use std::convert::TryFrom;

pub fn store_at(name: &str) -> crate::Result<git_ref::Store> {
    let path = git_testtools::scripted_fixture_repo_read_only(name)?;
    Ok(git_ref::Store::try_from(path.join(".git"))?)
}

#[test]
#[cfg(feature = "internal-testing-git-features-parallel")]
fn is_send_and_sync() {
    pub fn store_with_packed_refs() -> crate::Result<git_ref::Store> {
        store_at("make_packed_ref_repository.sh")
    }
    fn assert_type<T: Send + Sync>(_t: T) {}
    let store = store_with_packed_refs().unwrap();
    assert_type(&store);
    assert_type(store);
}

mod loose {
    // mod find {
    //     use crate::store::store_at;
    //     use git_testtools::hex_to_id;
    //
    //     #[test]
    //     fn with_packed_refs() -> crate::Result {
    //         let store = store_at("make_packed_ref_repository_for_overlay.sh")?.to_handle();
    //         let c1 = hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03");
    //         let r = store.find("main")?;
    //         assert_eq!(r.target.into_id(), c1);
    //         assert_eq!(r.name.as_bstr(), "refs/heads/main");
    //         Ok(())
    //     }
    // }

    // mod iter {
    //     use crate::store::store_at;
    //     use git_ref::bstr::ByteSlice;
    //     use git_testtools::hex_to_id;
    //     use std::convert::TryInto;
    //
    //     #[test]
    //     fn without_prefix() -> crate::Result {
    //         use git_ref::Target::*;
    //
    //         let store = store_at("make_packed_ref_repository_for_overlay.sh")?.to_handle();
    //         let ref_names = store
    //             .iter()?
    //             .map(|r| r.map(|r| (r.name.as_bstr().to_owned(), r.target)))
    //             .collect::<Result<Vec<_>, _>>()?;
    //         let c1 = hex_to_id("134385f6d781b7e97062102c6a483440bfda2a03");
    //         let c2 = hex_to_id("9902e3c3e8f0c569b4ab295ddf473e6de763e1e7");
    //         assert_eq!(
    //             ref_names,
    //             vec![
    //                 (b"refs/heads/main".as_bstr().to_owned(), Peeled(c1)),
    //                 ("refs/heads/newer-as-loose".into(), Peeled(c2)),
    //                 (
    //                     "refs/remotes/origin/HEAD".into(),
    //                     Symbolic("refs/remotes/origin/main".try_into()?),
    //                 ),
    //                 ("refs/remotes/origin/main".into(), Peeled(c1)),
    //                 (
    //                     "refs/tags/tag-object".into(),
    //                     Peeled(hex_to_id("b3109a7e51fc593f85b145a76c70ddd1d133fafd")),
    //                 )
    //             ]
    //         );
    //         Ok(())
    //     }
    // }
}
