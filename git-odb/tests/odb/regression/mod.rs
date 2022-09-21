mod repo_with_small_packs {
    use git_odb::Find;

    use crate::odb::{fixture_path, hex_to_id};

    #[test]
    fn all_packed_objects_can_be_found() -> crate::Result {
        let store = git_odb::at(fixture_path("repos/small-packs.git/objects"))?;
        let mut buf = Vec::new();
        assert!(
            store
                .try_find(hex_to_id("ecc68100297fff843a7eef8df0d0fb80c1c8bac5"), &mut buf)?
                .is_some(),
            "object is present and available"
        );
        Ok(())
    }

    #[test]
    #[cfg(feature = "internal-testing-git-features-parallel")]
    fn multi_threaded_access_will_not_panic() {
        for arg in ["no", "without-multi-index"] {
            let base = git_testtools::scripted_fixture_repo_read_only_with_args("make_repo_multi_index.sh", Some(arg))
                .unwrap()
                .join(".git")
                .join("objects");
            let store = git_odb::at(base).unwrap();
            let (tx, barrier) = crossbeam_channel::unbounded::<()>();
            let handles = (0..num_cpus::get()).map(|tid| {
                std::thread::spawn({
                    let store = store.clone();
                    let barrier = barrier.clone();
                    move || {
                        barrier.recv().ok();
                        let mut buf = Vec::new();
                        let mut count = 0;
                        for id in store.iter().unwrap() {
                            let id = id.unwrap();
                            assert!(
                                store.try_find(id, &mut buf).is_ok(),
                                "Thread {} could not find {}",
                                tid,
                                id
                            );
                            count += 1;
                        }
                        count
                    }
                })
            });

            std::thread::sleep(std::time::Duration::from_millis(50));
            drop(tx);
            let expected = store.iter().unwrap().count();
            for handle in handles {
                let actual = handle.join().expect("no panic");
                assert_eq!(actual, expected);
            }
        }
    }
}
