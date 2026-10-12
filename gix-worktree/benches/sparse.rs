//! Benchmark public `gix_worktree::sparse::apply_patterns()` on deterministic, sorted, synthetic
//! full indices. This is not a parity or performance comparison against Git.
//!
//! # Layouts
//!
//! | Scenario | Layout | Files | Included / excluded | Patterns |
//! |---|---|---:|---:|---:|
//! | `typical_20k_cone` | About 100 packages; select two, retaining root/parent files | 20,000 | 420 / 19,580 | 6 |
//! | `mostly_excluded_200k_cone` | About 1,000 packages; select the same two | 200,000 | 420 / 199,580 | 6 |
//! | `deep_shared_prefixes_60k_cone` | Products/platform/regions/teams/components/source; select two components and retain ancestor files | 60,000 | 90 / 59,910 | 16 |
//! | `mixed_case_20k_folded_cone` | Typical layout with mixed-case paths and patterns, using `Case::Fold` | 20,000 | 420 / 19,580 | 6 |
//! | `full_200k_cone` | Large package layout, selecting everything with `/*` | 200,000 | 200,000 / 0 | 1 |
//! | `selections_exclusions_20k_non_cone` | Select 20 packages, excluding each package's generated directory | 20,000 | 2,680 / 17,320 | 40 |
//!
//! Complete packages contain a README, 67 source files, 66 tests, and 66 generated files;
//! the final package is shortened to reach the exact index size. Membership, flags, and
//! inclusion counts are independently checked before timing.
//!
//! # Timing boundaries and caveats
//!
//! - Parsing, matching, and exclusion-flag updates are included in each timed call.
//! - Index construction, sorting, cloning/setup, verification, and disk I/O are excluded.
//!   `iter_batched_ref` clones the pristine index outside timing, so every iteration starts with
//!   fresh, empty flags, never pre-existing `SKIP_WORKTREE`.
//! - Untimed cloning can influence cache warmth; these are in-memory measurements, not
//!   disk-backed checkout timings.
//! - Full-cone selection skips index traversal. Criterion still reports throughput using 200,000
//!   entries, so that throughput does not represent scanning or matching 200,000 paths.

use std::{fmt::Write, hint::black_box, path::Path, time::Duration};

use criterion::{BatchSize, Criterion, SamplingMode, Throughput, criterion_group, criterion_main};
use gix_ignore::glob::pattern::Case;
use gix_index::{State, entry::Flags, entry::Mode};

fn sparse(c: &mut Criterion) {
    // Build and independently check every full-index fixture before any timing begins.
    let scenarios = scenarios();
    let source = Path::new("info/sparse-checkout");
    let mut group = c.benchmark_group("sparse_apply_patterns");
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(3));
    group.sample_size(30);
    group.sampling_mode(SamplingMode::Flat);

    for scenario in &scenarios {
        group.throughput(Throughput::Elements(scenario.index.entries().len() as u64));
        group.bench_function(scenario.name, |b| {
            b.iter_batched_ref(
                || scenario.index.clone(),
                |index| {
                    gix_worktree::sparse::apply_patterns(
                        black_box(&mut *index),
                        black_box(scenario.patterns.as_bytes()),
                        black_box(source),
                        black_box(scenario.cone),
                        black_box(scenario.case),
                    )
                    .expect("verified sparse patterns can be applied to a fresh full index");
                    black_box(index);
                },
                BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, sparse);
criterion_main!(benches);

struct Scenario {
    name: &'static str,
    index: State,
    patterns: String,
    cone: bool,
    case: Case,
}

impl Scenario {
    fn new(
        name: &'static str,
        mut paths: Vec<(String, bool)>,
        patterns: &str,
        cone: bool,
        case: Case,
        expected_included: usize,
    ) -> Self {
        paths.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        assert!(
            paths.windows(2).all(|pair| pair[0].0 < pair[1].0),
            "{name}: fixture paths must be sorted and unique"
        );
        assert_eq!(
            paths.iter().filter(|(_, included)| *included).count(),
            expected_included,
            "{name}: independently generated membership must agree with the expected count"
        );

        let hash = gix_hash::Kind::Sha1;
        let mut index = State::new(hash);
        for (path, _) in &paths {
            index.dangerously_push_entry(
                Default::default(),
                hash.null(),
                Flags::empty(),
                Mode::FILE,
                path.as_str().into(),
            );
        }
        index.sort_entries();
        let mut checked = index.clone();
        gix_worktree::sparse::apply_patterns(
            &mut checked,
            patterns.as_bytes(),
            Path::new("info/sparse-checkout"),
            cone,
            case,
        )
        .expect("fixture sparse patterns can be applied");
        for ((entry, path), (expected_path, included)) in checked.entries_mut_with_paths().zip(&paths) {
            assert_eq!(
                path,
                expected_path.as_bytes(),
                "{name}: sorted index entries must retain fixture path order"
            );
            assert_eq!(
                entry.flags,
                if *included {
                    Flags::empty()
                } else {
                    Flags::SKIP_WORKTREE | Flags::EXTENDED
                },
                "{name}: unexpected membership or flags for {expected_path}"
            );
        }
        assert_eq!(
            checked
                .entries()
                .iter()
                .filter(|entry| !entry.flags.contains(Flags::SKIP_WORKTREE))
                .count(),
            expected_included,
            "{name}: actual inclusion count must match the independent expectation"
        );
        eprintln!(
            "Verified {name}: {} files, {expected_included} included, {} excluded, {} patterns",
            paths.len(),
            paths.len() - expected_included,
            patterns.lines().count()
        );
        Self {
            name,
            index,
            patterns: patterns.into(),
            cone,
            case,
        }
    }
}

fn scenarios() -> Vec<Scenario> {
    let cone_patterns = "/*
!/*/
/packages/
!/packages/*/
/packages/pkg-007/
/packages/pkg-042/
";
    let selected = |package: Option<usize>, _: &str| package.is_none_or(|package| matches!(package, 7 | 42));
    let typical = Scenario::new(
        "typical_20k_cone",
        package_paths(20_000, false, selected),
        cone_patterns,
        true,
        Case::Sensitive,
        420,
    );
    let monorepo = Scenario::new(
        "mostly_excluded_200k_cone",
        package_paths(200_000, false, selected),
        cone_patterns,
        true,
        Case::Sensitive,
        420,
    );
    let deep = Scenario::new(
        "deep_shared_prefixes_60k_cone",
        deep_paths(),
        "/*
!/*/
/products/
!/products/*/
/products/platform/
!/products/platform/*/
/products/platform/region-00/
!/products/platform/region-00/*/
/products/platform/region-00/teams/
!/products/platform/region-00/teams/*/
/products/platform/region-00/teams/team-00/
!/products/platform/region-00/teams/team-00/*/
/products/platform/region-00/teams/team-00/components/
!/products/platform/region-00/teams/team-00/components/*/
/products/platform/region-00/teams/team-00/components/component-00/
/products/platform/region-00/teams/team-00/components/component-01/
",
        true,
        Case::Sensitive,
        90,
    );
    let folded = Scenario::new(
        "mixed_case_20k_folded_cone",
        package_paths(20_000, true, selected),
        "/*
!/*/
/PACKAGES/
!/packages/*/
/packages/PKG-007/
/PACKAGES/pkg-042/
",
        true,
        Case::Fold,
        420,
    );
    let full = Scenario::new(
        "full_200k_cone",
        package_paths(200_000, false, |_, _| true),
        "/*\n",
        true,
        Case::Sensitive,
        200_000,
    );

    // Select twenty packages, then exclude their generated bindings (40 meaningful patterns).
    let mut non_cone_patterns = String::new();
    for package in 0..20 {
        write!(
            non_cone_patterns,
            "/packages/pkg-{package:03}/
!/packages/pkg-{package:03}/generated/
"
        )
        .expect("writing sparse patterns to a String cannot fail");
    }
    let non_cone = Scenario::new(
        "selections_exclusions_20k_non_cone",
        package_paths(20_000, false, |package, directory| {
            package.is_some_and(|package| package < 20) && directory != "generated"
        }),
        &non_cone_patterns,
        false,
        Case::Sensitive,
        2_680,
    );
    vec![typical, monorepo, deep, folded, full, non_cone]
}

fn package_paths(
    file_count: usize,
    mixed_case: bool,
    included: impl Fn(Option<usize>, &str) -> bool,
) -> Vec<(String, bool)> {
    let mut paths = Vec::with_capacity(file_count);
    let packages = if mixed_case { "Packages" } else { "packages" };
    let package_prefix = if mixed_case { "Pkg" } else { "pkg" };
    for file in 0..10 {
        paths.push((format!("workspace-{file:02}.toml"), included(None, "")));
        paths.push((format!("{packages}/manifest-{file:02}.toml"), included(None, "")));
    }
    // Each complete package has a README, 67 source files, 66 tests and 66 generated files.
    // The final package is smaller so the requested index size is exact.
    for ordinal in 0..file_count - 20 {
        let package = ordinal / 200;
        let file = ordinal % 200;
        let prefix = format!("{packages}/{package_prefix}-{package:03}");
        let directory = match file {
            0 => "",
            1..=67 => "src",
            68..=133 => "tests",
            _ => "generated",
        };
        let path = if file == 0 {
            format!("{prefix}/README.md")
        } else {
            let path_directory = if mixed_case {
                match directory {
                    "src" => "Src",
                    "tests" => "Tests",
                    _ => "Generated",
                }
            } else {
                directory
            };
            format!("{prefix}/{path_directory}/module-{file:03}.rs")
        };
        paths.push((path, included(Some(package), directory)));
    }
    paths
}

fn deep_paths() -> Vec<(String, bool)> {
    let mut paths = Vec::with_capacity(60_000);
    // Root and selected-ancestor files are included, but sibling subtrees are pruned.
    for parent in [
        "",
        "products/",
        "products/platform/",
        "products/platform/region-00/",
        "products/platform/region-00/teams/",
        "products/platform/region-00/teams/team-00/",
        "products/platform/region-00/teams/team-00/components/",
    ] {
        for file in 0..10 {
            paths.push((format!("{parent}manifest-{file:02}.toml"), true));
        }
    }
    for ordinal in 0..59_930 {
        let region = ordinal / 1_000;
        let team = ordinal / 100 % 10;
        let component = ordinal / 10 % 10;
        let file = ordinal % 10;
        paths.push((
            format!(
                "products/platform/region-{region:02}/teams/team-{team:02}/components/component-{component:02}/src/module-{file:02}.rs"
            ),
            region == 0 && team == 0 && component < 2,
        ));
    }
    paths
}
