mod registration {
    use std::path::Path;

    fn filecount_in(path: impl AsRef<Path>) -> usize {
        std::fs::read_dir(path).expect("valid dir").count()
    }

    #[test]
    fn it_can_be_kept() -> crate::Result {
        let dir = tempfile::tempdir()?;
        drop(git_tempfile::new(dir.path())?.take().expect("not taken yet").keep()?);
        assert_eq!(filecount_in(&dir), 1, "a temp file and persisted");
        Ok(())
    }

    #[test]
    fn it_is_removed_if_it_goes_out_of_scope() -> crate::Result {
        let dir = tempfile::tempdir()?;
        {
            let _keep = git_tempfile::new(dir.path());
            assert_eq!(filecount_in(&dir), 1, "a temp file was created");
        }
        assert_eq!(filecount_in(&dir), 0, "tempfile was automatically removed");
        Ok(())
    }
}

mod slab_assumptions {
    use sharded_slab::Slab;

    #[test]
    fn the_capacity_is_limited_or_this_hangs_or_runs_out_of_memory() {
        let s = Slab::new();
        let mut item = 0;
        const MAX: usize = 10_000;
        loop {
            if s.insert(item).is_none() || item == MAX {
                break;
            }
            item += 1;
        }
        eprintln!(
            "Could store {} items in slab with default configuration, gave up after {}",
            item, MAX
        );
    }
}
