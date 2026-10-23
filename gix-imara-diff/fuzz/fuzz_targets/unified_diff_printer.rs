#![no_main]

use imara_diff::{Algorithm, BasicLineDiffPrinter, Diff, InternedInput, UnifiedDiffConfig};
use libfuzzer_sys::arbitrary;
use libfuzzer_sys::fuzz_target;

/// Valid prefixes for unified diff output lines
const VALID_DIFF_LINE_PREFIXES: [char; 4] = [' ', '+', '-', '@'];

#[derive(arbitrary::Arbitrary, Debug)]
struct Input<'a> {
    before: &'a str,
    after: &'a str,
    context_len: u32,
}

/// Tests unified diff printing with:
/// - Different context lengths (0-10)
/// - Various input combinations
/// - Validates output format (lines start with ' ', '+', '-', or '@')
fn do_fuzz(
    Input {
        before,
        after,
        context_len,
    }: Input<'_>,
) {
    let mut input = InternedInput::new(before, after);

    // Test with different algorithms
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
        let mut diff = Diff::compute(algorithm, &input);

        // Postprocess before printing
        diff.postprocess_lines(&input);

        // Create printer and config
        let printer = BasicLineDiffPrinter(&input.interner);
        let mut config = UnifiedDiffConfig::default();
        config.context_len(context_len);

        // Generate unified diff
        let unified = diff.unified_diff(&printer, config, &input);
        let output = unified.to_string();

        // Basic sanity checks on output
        // It should be valid UTF-8 (already guaranteed by to_string)
        // Lines should start with valid diff prefixes
        for line in output.lines() {
            if !line.is_empty() {
                let first_char = line.chars().next().unwrap();
                // Should be a valid diff line prefix
                assert!(
                    VALID_DIFF_LINE_PREFIXES.contains(&first_char),
                    "Invalid diff line prefix: '{}' in line: '{}'",
                    first_char,
                    line
                );
            }
        }
    }
}

fuzz_target!(|input: Input<'_>| {
    do_fuzz(input);
});
