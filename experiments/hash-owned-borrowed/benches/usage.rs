use criterion::*;

fn use_owned_by_ref(id: &hash::Owned) {
    black_box(id);
}

fn usage(c: &mut Criterion) {
    c.benchmark_group("Owned::cloned")
        .throughput(Throughput::Bytes(std::mem::size_of::<hash::Owned>() as u64))
        .bench_function("Sha1", |b| {
            let source = hash::Owned::Sha1([0; 20]);
            b.iter(|| {
                black_box(source.clone());
            });
        })
        .bench_function("Sha256", |b| {
            let source = hash::Owned::Sha256([0; 32]);
            b.iter(|| {
                black_box(source.clone());
            });
        });
    c.benchmark_group("Owned::by_ref")
        .throughput(Throughput::Bytes(std::mem::size_of::<hash::Owned>() as u64))
        .bench_function("Sha1", |b| {
            let source = hash::Owned::Sha1([0; 20]);
            b.iter(|| {
                use_owned_by_ref(&source);
            });
        })
        .bench_function("Sha256", |b| {
            let source = hash::Owned::Sha256([0; 32]);
            b.iter(|| {
                use_owned_by_ref(&source);
            });
        });
}

criterion_group!(benches, usage);
criterion_main!(benches);
