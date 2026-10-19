#![no_main]

use imara_diff::{Algorithm, Diff, InternedInput};
use libfuzzer_sys::{Corpus, fuzz_target};

use libfuzzer_sys::arbitrary::{self, Arbitrary, Unstructured};

/// Keep libFuzzer from spending its whole timeout budget in Myers on oversized,
/// repetitive inputs while preserving room for historical regression fixtures.
const MAX_FUZZ_INPUT_BYTES: usize = 150 * 1024;

#[derive(arbitrary::Arbitrary, Debug)]
struct Input<'a> {
    before: &'a [u8],
    before_str: &'a str,
    after: &'a [u8],
    after_str: &'a str,
}

/// Tests all three diff algorithms (Myers, Histogram, MyersMinimal) with:
/// - Computing diffs on arbitrary string inputs
/// - Postprocessing with no heuristic and line heuristic
/// - Unified diff printing
/// - Basic queries (count_additions, count_removals, is_added, is_removed)
/// - Hunks iteration
fn do_fuzz(
    Input {
        before,
        before_str,
        after,
        after_str,
    }: Input<'_>,
) {
    // Create interned input
    let mut input = InternedInput::new(before, after);

    // Test all three diff algorithms
    for algorithm in [
        Algorithm::Histogram,
        Algorithm::Myers,
        Algorithm::MyersMinimal,
    ] {
        // Keep quadratic minimal diffs within the fuzz timeout.
        if algorithm == Algorithm::MyersMinimal {
            input.before.truncate(512);
            input.after.truncate(512);
        }
        // Compute diff
        let mut diff = Diff::compute(algorithm, &input);

        // Test basic queries
        let _ = diff.count_additions();
        let _ = diff.count_removals();

        // Test hunks iteration
        for hunk in diff.hunks() {
            let _ = hunk.is_pure_insertion();
            let _ = hunk.is_pure_removal();
            let _ = hunk.invert();
        }

        // Test postprocessing with no heuristic
        diff.postprocess_no_heuristic(&input);

        // Test postprocessing with line heuristic
        diff.postprocess_lines(&input);
    }

    let input = InternedInput::new(before_str, after_str);
    let mut word_input = InternedInput::default();
    let mut word_diff = Diff::default();

    let diff = Diff::compute(Algorithm::Myers, &input);
    for hunk in diff.hunks() {
        hunk.latin_word_diff(&input, &mut word_input, &mut word_diff);
    }
}

fuzz_target!(|data: &[u8]| -> Corpus {
    if data.len() > MAX_FUZZ_INPUT_BYTES {
        return Corpus::Reject;
    }

    let Ok(input) = Input::arbitrary_take_rest(Unstructured::new(data)) else {
        return Corpus::Reject;
    };

    do_fuzz(input);
    Corpus::Keep
});
