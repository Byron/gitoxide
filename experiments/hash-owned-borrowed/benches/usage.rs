use criterion::*;
use std::borrow::Borrow;

fn use_owned_by_ref(id: &hash::Owned) {
    black_box(id);
}

fn use_by_ref_impl_borrow(id: impl Borrow<hash::Owned>) {
    black_box(id.borrow());
}

fn usage(c: &mut Criterion) {
    c.benchmark_group("Owned::cloned")
        .throughput(Throughput::Elements(1))
        .bench_function("Sha1", |b| {
            let source = hash::Owned::sha1();
            b.iter(|| {
                black_box(source.clone());
            });
        })
        .bench_function("Sha256", |b| {
            let source = hash::Owned::sha256();
            b.iter(|| {
                black_box(source.clone());
            });
        });
    c.benchmark_group("Owned::by_ref")
        .throughput(Throughput::Elements(1))
        .bench_function("Sha1", |b| {
            let source = hash::Owned::sha1();
            b.iter(|| {
                use_owned_by_ref(&source);
            });
        })
        .bench_function("Sha256", |b| {
            let source = hash::Owned::sha256();
            b.iter(|| {
                use_owned_by_ref(&source);
            });
        });
    c.benchmark_group("Owned::by_ref_via_impl_borrow")
        .throughput(Throughput::Elements(1))
        .bench_function("Sha1", |b| {
            let source = hash::Owned::sha1();
            b.iter(|| {
                use_by_ref_impl_borrow(&source);
            });
        })
        .bench_function("Sha256", |b| {
            let source = hash::Owned::sha256();
            b.iter(|| {
                use_by_ref_impl_borrow(&source);
            });
        });
}

criterion_group!(benches, usage);
criterion_main!(benches);
