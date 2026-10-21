#!/usr/bin/env bash
set -eu -o pipefail

# Generate with Git 2.50 or newer, which disables frequency pruning for --minimal.
# Count frequent lines across the common prefix and postfix, not just the changed region.
# Each file has 26 lines, so Git's frequency limit is 8. Eight blank lines in the common
# prefix make the ninth blank frequent. It is surrounded by two unmatched runs of four
# lines and gets pruned by Myers (9 changes), but not by minimal Myers (8 changes).
mkdir whole-file-frequency
for tag in old new; do
  {
    for i in {1..8}; do
      printf 'paragraph %s\n\n' "$i"
    done
    for i in {1..4}; do
      printf '%s %s\n' "$tag" "$i"
    done
    printf '\n'
    for i in {5..8}; do
      printf '%s %s\n' "$tag" "$i"
    done
    printf 'end\n'
  } >"whole-file-frequency/$tag"
done

# Exceed Git's expensive-split limit of 256 rounds with 600 values from 0 to 19,
# replacing about 60% in the second file. This deterministic 64-bit LCG reproduces
# the regression where counting rounds from 0 instead of 1 chose a different split.
# Bash arithmetic wraps at 64 bits; masking after the signed shift emulates u64 >> 33.
state=1
next() {
  state=$((state * 6364136223846793005 + 1442695040888963407))
  value=$(((state >> 33) & 2147483647))
}

mkdir expensive-split
before=()
for ((i = 0; i < 600; i++)); do
  next
  before[i]=$((value % 20))
  printf '%s\n' "${before[i]}"
done >expensive-split/old
for ((i = 0; i < 600; i++)); do
  next
  if ((value % 100 < 60)); then
    next
    printf '%s\n' "$((value % 20))"
  else
    printf '%s\n' "${before[i]}"
  fi
done >expensive-split/new

# Minimal Myers must search past the 256-round cutoff on these largely unrelated inputs.
mkdir expensive-minimal-split
state=1
for tag in old new; do
  for ((i = 0; i < 350; i++)); do
    next
    printf '%s\n' "$((value % 50))"
  done >"expensive-minimal-split/$tag"
done

# These inputs exhaust one side after stripping common ends. Git still classifies and
# prunes records, while our shortcut marks the remainder changed without preprocessing.
# Cover empty/identical files, each insertion position, overlapping repeated ends, and
# an unchanged final line without a newline. Reverse every pair below to cover deletions.
cases=(trivial-empty trivial-identical trivial-empty-file trivial-prepend trivial-append
  trivial-middle trivial-repeated trivial-frequent trivial-no-final-newline
  near-miss-newline near-miss-kept-anchor near-miss-pruned-anchor)
mkdir "${cases[@]}"
: >trivial-empty/old
: >trivial-empty/new
printf 'begin\n\n}\nend\n' >trivial-identical/old
cp trivial-identical/old trivial-identical/new
: >trivial-empty-file/old
printf 'new 1\nnew 2\n' >trivial-empty-file/new
printf 'anchor\nend\n' >trivial-prepend/old
printf 'inserted\nanchor\nend\n' >trivial-prepend/new
printf 'begin\nanchor\n' >trivial-append/old
printf 'begin\nanchor\ninserted\n' >trivial-append/new
printf 'begin\nend\n' >trivial-middle/old
printf 'begin\ninserted 1\ninserted 2\nend\n' >trivial-middle/new
for tag in old new; do
  {
    printf 'begin\n'
    printf 'repeat\n%.0s' {1..8}
    if test "$tag" = new; then
      printf 'repeat\n'
    fi
    printf 'end\n'
  } >"trivial-repeated/$tag"
done
printf 'begin\nend' >trivial-no-final-newline/old
printf 'begin\ninserted\nend' >trivial-no-final-newline/new

# Adding a final newline changes a token: this is NOT a trivial append after trimming.
cp trivial-no-final-newline/old near-miss-newline/old
printf 'begin\nend\ninserted\n' >near-miss-newline/new

# Eight common blank lines make the inserted/interior blank frequent. The trivial case
# mixes unmatched lines, a prunable blank, and a retained match from the common suffix.
# The near misses leave a blank on BOTH trimmed sides: with 3+3 unmatched neighbors it
# is kept; with 3+4 it is pruned by Myers but not --minimal, despite being insertion-only.
for case in trivial-frequent near-miss-kept-anchor near-miss-pruned-anchor; do
  for tag in old new; do
    {
      for i in {1..8}; do
        printf 'paragraph %s\n\n' "$i"
      done
      if test "$tag" = new; then
        printf 'inserted %s\n' {1..3}
        printf '\n'
        printf 'inserted %s\n' {4..6}
        if test "$case" != near-miss-kept-anchor; then
          printf 'inserted 7\n'
        fi
        if test "$case" = trivial-frequent; then
          printf 'end\n'
        fi
      elif test "$case" != trivial-frequent; then
        printf '\n'
      fi
      printf 'end\n'
    } >"$case/$tag"
  done
done

for case in "${cases[@]}"; do
  mkdir "$case-reversed"
  cp "$case/new" "$case-reversed/old"
  cp "$case/old" "$case-reversed/new"
done

baseline() {
  local name=$1
  shift
  # --no-index returns 1 for differences. Do not hide actual Git failures.
  git diff --no-index --no-ext-diff --no-textconv --no-indent-heuristic \
    --numstat "$@" -- old new >"$name.numstat" || test "$?" -eq 1
  # Counts alone cannot detect misplaced edits among repeated lines. Zero context
  # exposes each compacted hunk's ranges; ignore file headers and function context.
  git diff --no-index --no-ext-diff --no-textconv --no-indent-heuristic \
    --no-color --unified=0 "$@" -- old new >"$name.patch" || test "$?" -eq 1
  awk '/^@@ / {print $2, $3}' "$name.patch" >"$name.hunks"
}

for case in */; do
  (
    cd "$case"
    baseline Myers --diff-algorithm=myers
    if test "$case" != expensive-split/; then
      baseline MyersMinimal --diff-algorithm=myers --minimal
    fi
  )
done
