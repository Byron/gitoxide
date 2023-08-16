use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse_commit(c: &mut Criterion) {
    c.bench_function("CommitRef(sig)", |b| {
        b.iter(|| black_box(gix_object::CommitRef::from_bytes(COMMIT_WITH_MULTI_LINE_HEADERS)).unwrap())
    });
    c.bench_function("CommitRefIter(sig)", |b| {
        b.iter(|| black_box(gix_object::CommitRefIter::from_bytes(COMMIT_WITH_MULTI_LINE_HEADERS).count()))
    });
}

fn parse_tag(c: &mut Criterion) {
    c.bench_function("TagRef(sig)", |b| {
        b.iter(|| black_box(gix_object::TagRef::from_bytes(TAG_WITH_SIGNATURE)).unwrap())
    });
    c.bench_function("TagRefIter(sig)", |b| {
        b.iter(|| black_box(gix_object::TagRefIter::from_bytes(TAG_WITH_SIGNATURE).count()))
    });
}

fn parse_tree(c: &mut Criterion) {
    c.bench_function("TreeRef(sig)", |b| {
        b.iter(|| black_box(gix_object::TreeRef::from_bytes(TREE)).unwrap())
    });
    c.bench_function("TreeRefIter(sig)", |b| {
        b.iter(|| black_box(gix_object::TreeRefIter::from_bytes(TREE).count()))
    });
}

criterion_group!(benches, parse_commit, parse_tag, parse_tree);
criterion_main!(benches);

const COMMIT_WITH_MULTI_LINE_HEADERS: &[u8] = include_bytes!("../tests/fixtures/commit/two-multiline-headers.txt");
const TAG_WITH_SIGNATURE: &[u8] = include_bytes!("../tests/fixtures/tag/signed.txt");
const TREE: &[u8] = include_bytes!("../tests/fixtures/tree/everything.tree");
