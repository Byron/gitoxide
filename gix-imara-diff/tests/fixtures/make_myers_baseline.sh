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

baseline() {
  local name=$1
  shift
  # --no-index returns 1 for differences. Do not hide actual Git failures.
  git diff --no-index --no-ext-diff --no-textconv --no-indent-heuristic \
    --numstat "$@" -- old new >"$name.numstat" || test "$?" -eq 1
}

for case in whole-file-frequency expensive-split expensive-minimal-split; do
  (
    cd "$case"
    baseline Myers --diff-algorithm=myers
    if test "$case" != expensive-split; then
      baseline MyersMinimal --diff-algorithm=myers --minimal
    fi
  )
done
