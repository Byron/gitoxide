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
