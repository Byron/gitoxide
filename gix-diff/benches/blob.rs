//! In-memory blob diffs: `cargo bench -p gix-diff --bench blob`.
//! Filter, for example, with `-- myers-minimal` or `-- slider-only`.
//!
//! Sizes name the old file's line count; throughput counts both input files' bytes.
//! Nontrivial fixtures have 16 replacements and 16 insertions, keeping Myers' edit distance bounded.
//! Two sizes keep the matrix bounded while retaining all three algorithms.
//! Fixture generation and correctness checks are untimed. Each stage is measured in isolation.
//! Interning and diff computation reuse their storage; private algorithm scratch space can still
//! allocate. Postprocessing gets a freshly computed diff as untimed setup. Interning hashes every line.
//!
//! `-- myers-trivial` isolates identical inputs and single insertions/deletions with common ends.
//! Compare with `-- myers-trivial --save-baseline upstream`, then `-- myers-trivial --baseline upstream`
//! using the same target directory and benchmark code on both revisions.

use std::{fmt::Write, hint::black_box};

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use gix_diff::blob::{Algorithm, Diff, InternedInput, platform::resource::ByteLinesWithoutTerminator};

const FUNCTION_COUNTS: &[usize] = &[64, 4_096];
const EDITED_FUNCTIONS: usize = 16;

fn interning(c: &mut Criterion) {
    let mut group = c.benchmark_group("blob/intern");
    for &functions in FUNCTION_COUNTS {
        let (before, after) = fixture(functions);
        group.throughput(Throughput::Bytes((before.len() + after.len()) as u64));
        let before = ByteLinesWithoutTerminator::new(&before);
        let after = ByteLinesWithoutTerminator::new(&after);
        let mut input = InternedInput::new(before, after);
        let lines = input.before.len();
        let tokens = input.interner.num_tokens();
        // Clearing must retain capacity without accumulating stale tokens across iterations.
        for _ in 0..2 {
            input.clear();
            input.update_before(before);
            input.update_after(after);
            assert_eq!(input.interner.num_tokens(), tokens, "reuse must not accumulate tokens");
            assert_eq!(input.before.len(), lines, "reuse must replace the token sequences");
        }
        group.bench_function(BenchmarkId::new("reuse", lines), |b| {
            b.iter(|| {
                input.clear();
                input.update_before(black_box(before));
                input.update_after(black_box(after));
                black_box(&input);
            });
        });
    }
    group.finish();
}

fn blob(c: &mut Criterion) {
    for (name, algorithm) in [
        ("histogram", Algorithm::Histogram),
        ("myers", Algorithm::Myers),
        ("myers-minimal", Algorithm::MyersMinimal),
    ] {
        let mut group = c.benchmark_group(format!("blob/{name}"));
        for &functions in FUNCTION_COUNTS {
            let (before, after) = fixture(functions);
            group.throughput(Throughput::Bytes((before.len() + after.len()) as u64));
            let before = ByteLinesWithoutTerminator::new(&before);
            let after = ByteLinesWithoutTerminator::new(&after);
            let input = InternedInput::new(before, after);
            let lines = input.before.len();
            let mut diff = Diff::compute(algorithm, &input);
            check_diff(&diff, &input);

            group.bench_function(BenchmarkId::new("diff-only/reuse", lines), |b| {
                b.iter(|| {
                    diff.compute_with(
                        algorithm,
                        black_box(&input.before),
                        black_box(&input.after),
                        input.interner.num_tokens(),
                    );
                    black_box(&diff);
                });
            });
            check_diff(&diff, &input);

            for (postprocess, with_slider) in [("postprocess-no-heuristic", false), ("slider", true)] {
                let process = |diff: &mut Diff| {
                    if with_slider {
                        diff.postprocess_lines(&input);
                    } else {
                        diff.postprocess_no_heuristic(&input);
                    }
                };
                let mut processed = Diff::compute(algorithm, &input);
                process(&mut processed);
                check_diff(&processed, &input);
                group.bench_function(BenchmarkId::new(format!("{postprocess}-only"), lines), |b| {
                    b.iter_batched_ref(
                        || Diff::compute(algorithm, &input),
                        |diff| {
                            process(black_box(&mut *diff));
                            black_box(&*diff);
                        },
                        BatchSize::SmallInput,
                    );
                });
            }
        }
        group.finish();
    }
}

fn myers_trivial(c: &mut Criterion) {
    let mut group = c.benchmark_group("blob/myers-trivial");
    for &functions in FUNCTION_COUNTS {
        let mut before = String::new();
        let mut after = String::new();
        for idx in 0..functions {
            if idx == functions / 2 {
                // A unique inserted line leaves both a common prefix and a common suffix.
                after.push_str("// inserted line\n");
            }
            push_function(&mut before, idx, false);
            push_function(&mut after, idx, false);
        }
        for (name, before, after, expected) in [
            ("identical", before.as_bytes(), before.as_bytes(), (0, 0)),
            ("insertion", before.as_bytes(), after.as_bytes(), (0, 1)),
            ("deletion", after.as_bytes(), before.as_bytes(), (1, 0)),
        ] {
            group.throughput(Throughput::Bytes((before.len() + after.len()) as u64));
            let input = InternedInput::new(
                ByteLinesWithoutTerminator::new(before),
                ByteLinesWithoutTerminator::new(after),
            );
            let mut diff = Diff::compute(Algorithm::Myers, &input);
            let check = |diff: &Diff| {
                assert_eq!(
                    (diff.count_removals(), diff.count_additions()),
                    expected,
                    "{name}: only the single inserted/deleted line may be changed"
                );
                check_reconstruction(diff, &input);
            };
            check(&diff);
            group.bench_function(BenchmarkId::new(format!("{name}/reuse"), input.before.len()), |b| {
                b.iter(|| {
                    diff.compute_with(
                        Algorithm::Myers,
                        black_box(&input.before),
                        black_box(&input.after),
                        input.interner.num_tokens(),
                    );
                    black_box(&diff);
                });
            });
            check(&diff);
        }
    }
    group.finish();
}

fn check_diff(diff: &Diff, input: &InternedInput<&[u8]>) {
    assert_eq!(
        diff.count_removals() as usize,
        EDITED_FUNCTIONS,
        "each edit replaces one line"
    );
    assert_eq!(
        diff.count_additions() as usize,
        EDITED_FUNCTIONS * 2,
        "each edit replaces and inserts a line"
    );
    check_reconstruction(diff, input);
}

fn check_reconstruction(diff: &Diff, input: &InternedInput<&[u8]>) {
    let mut reconstructed = Vec::new();
    let mut end = 0;
    for hunk in diff.hunks() {
        reconstructed.extend_from_slice(&input.before[end..hunk.before.start as usize]);
        reconstructed.extend_from_slice(&input.after[hunk.after.start as usize..hunk.after.end as usize]);
        end = hunk.before.end as usize;
    }
    reconstructed.extend_from_slice(&input.before[end..]);
    assert_eq!(
        reconstructed, input.after,
        "applying the diff must reconstruct the destination"
    );
}

fn fixture(functions: usize) -> (Vec<u8>, Vec<u8>) {
    let mut before = String::new();
    let mut after = String::new();
    // Fixed edit distance and unique function names keep MyersMinimal practical as file size grows.
    let stride = functions / EDITED_FUNCTIONS;
    for idx in 0..functions {
        push_function(&mut before, idx, false);
        push_function(&mut after, idx, idx % stride == stride / 2);
    }
    (before.into_bytes(), after.into_bytes())
}

fn push_function(out: &mut String, idx: usize, edited: bool) {
    writeln!(out, "fn section_{idx}() {{").expect("writing to a string cannot fail");
    writeln!(out, "    let value = {idx}{};", if edited { " + 1" } else { "" })
        .expect("writing to a string cannot fail");
    out.push_str("    if value % 2 == 0 {\n        println!(\"even\");\n");
    if edited {
        out.push_str("        println!(\"changed\");\n");
    }
    out.push_str("    }\n}\n\n");
}

criterion_group!(benches, interning, blob, myers_trivial);
criterion_main!(benches);
