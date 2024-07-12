use bytesize::ByteSize;
use cap::Cap;
use std::alloc;
use std::time::Instant;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::MAX);

#[test]
fn usage() {
    let before = ALLOCATOR.allocated();
    let start = Instant::now();
    let config = gix_config::File::from_bytes_no_includes(
        include_bytes!("fixtures/fuzzed/mem-amplification.config"),
        gix_config::file::Metadata::from(gix_config::Source::User),
        Default::default(),
    )
    .unwrap();
    let after = ALLOCATOR.allocated();
    let used = after - before;
    let elapsed = start.elapsed().as_secs_f32();
    eprintln!(
        "used mem: {}B for {} sections, took {elapsed:.02}s [total-mem: {total}, peak-mem: {peak}]",
        ByteSize(used as u64),
        config.sections().count(),
        total = ByteSize(ALLOCATOR.total_allocated() as u64),
        peak = ByteSize(ALLOCATOR.max_allocated() as u64),
    );
    assert!(
        used < 200 * 1024 * 1024,
        "we should now start using more memory than anticipated, to keep mem-amplification low"
    );
}
