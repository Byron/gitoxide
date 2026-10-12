#!/usr/bin/env bash
set -eu -o pipefail

# A single read-only fixture for sparse pattern matching, recorded by the host
# Git rather than by our implementation.
root="$PWD"
export LC_ALL=C
git --version >git-version
: >scenarios
cat >README <<'EOF'
Sparse checkout baselines recorded by the Git version in git-version.

scenarios lists directory names, one per line, in the former Rust test order.
Each directory contains:
  description       the original assertion's intent and essential caveats
  config            cone and ignorecase, each true or false on its own line
  patterns          exact sparse-checkout bytes, including empty files
  index.before      full Git index, initially without skip-worktree flags
  index.after       full Git index after Git applies the sparse patterns
  baseline.before   git ls-files -t -z before applying patterns (all H)
  baseline          git ls-files -t -z after applying patterns (H or S)
  status/stderr/stdout  exit status and diagnostics of read-tree -mu
  method            read-tree, or reapply after a checked all-excluded refusal
  tree              the tree object id used by read-tree
  repo/.git         real blobs, gitlink commit, tree, and resulting index

Entries are made with NUL-delimited update-index --index-info, not git add.
The full index already describes the target tree. read-tree -mu applies sparse
matching without materializing unchanged entries, even literal '*' or backslash
paths or mode-120000 symlinks. Gitlinks reference a real commit object.
index.sparse=false keeps both indexes full. Directory ancestors of nested files
match as directories. No tracked paths are pre-created except module/ in the
directory-only gitlink scenario: Git's ce_to_dtype() erroneously byte-swaps an
already host-order mode on little-endian hosts. Pre-creating that empty directory
lets filesystem fallback recover Git's intended directory classification. The
directory is removed after recording the baseline; Rust matches the index alone.

Only the empty_non_cone_patterns_exclude_everything scenario may recover from
Git's exact 'Sparse checkout leaves no entry on working directory' fatal error
(status 128). In that case Git sparse-checkout reapply is run on index.before,
with separate reapply.status/stderr/stdout files. It must succeed and its real
ls-files output must contain only S entries; no expected flags are fabricated.
All other command failures abort generation. Newer Git can accept this case
with read-tree directly, in which case status is 0 and method is read-tree.

apply_patterns_preserves_existing_entry_flags also has flags.before and
flags.after (git ls-files -v -z): h means assume-unchanged, s additionally means
skip-worktree. Both entries must retain assume-unchanged. UPTODATE is transient
in-memory state, not a disk index flag: Rust must inject it into index.before
and check it separately. The Git comparison does not claim to record UPTODATE.
EOF

# Read MODE PATH pairs without interpreting backslashes. Only these controlled
# paths are used; the index-info stream itself uses NUL to preserve path bytes.
scenario() {
  local name="$1" cone="$2" ignorecase="$3" description="$4"
  current="$root/$name"
  printf '%s\n' "$name" >>"$root/scenarios"
  mkdir -p "$current/repo"
  printf '%s\n' "$description" >"$current/description"
  printf '%s\n%s\n' "$cone" "$ignorecase" >"$current/config"
  (
    cd "$current/repo"
    git init -q
    git config core.longpaths true
    git config core.ignoreCase "$ignorecase"
    git config core.sparseCheckout true
    git config core.sparseCheckoutCone "$cone"
    git config index.sparse false
    git config core.symlinks false
    # Index-only paths must also be accepted by Git for Windows.
    git config core.protectNTFS false
    git config core.protectHFS false
    local blob_id empty_tree_id commit_id mode path object_id
    blob_id=$(printf 'sparse baseline blob\n' | git hash-object -w --stdin)
    empty_tree_id=$(git mktree </dev/null)
    commit_id=$(printf 'gitlink target\n' | git commit-tree "$empty_tree_id")
    while IFS=' ' read -r mode path; do
      case "$mode" in
        FILE) mode=100644; object_id="$blob_id" ;;
        SYMLINK) mode=120000; object_id="$blob_id" ;;
        COMMIT) mode=160000; object_id="$commit_id" ;;
        *) echo "unknown sparse fixture entry mode: $mode" >&2; exit 1 ;;
      esac
      printf '%s %s\t%s\0' "$mode" "$object_id" "$path"
    done | git update-index -z --index-info
    git write-tree >../tree
    if [[ "$name" == apply_patterns_preserves_existing_entry_flags ]]; then
      git update-index --assume-unchanged included excluded
      git ls-files -v -z >../flags.before
      printf 'h excluded\0h included\0' >../flags.expected
      cmp ../flags.expected ../flags.before
    fi
    cp .git/index ../index.before
    git ls-files -t -z >../baseline.before
    # Verify the initial index has no sparse flags, using Git's own listing.
    git ls-files -z | while IFS= read -r -d '' path; do
      printf 'H %s\0' "$path"
    done >../baseline.expected
    cmp ../baseline.expected ../baseline.before
    rm ../baseline.expected
  )
}

record() {
  cat >"$current/patterns"
  (
    cd "$current/repo"
    cp ../patterns .git/info/sparse-checkout
    local tree_id status path
    tree_id=$(cat ../tree)
    if [[ "$current" == "$root/non_cone_directory_patterns_include_gitlinks" ]]; then
      # Work around Git's ce_to_dtype() byte-swap bug without fabricating index flags.
      mkdir module
    fi
    if git read-tree -mu "$tree_id" >../stdout 2>../stderr; then
      status=0
    else
      status=$?
    fi
    printf '%s\n' "$status" >../status
    printf 'read-tree\n' >../method
    if [[ "$status" != 0 ]]; then
      # Do not mistake a failed checkout (which leaves the old index intact)
      # for a matching baseline. Recover only this historically refused case.
      if [[ "$current" != "$root/empty_non_cone_patterns_exclude_everything" || "$status" != 128 ]] ||
        ! grep -Fxq 'fatal: Sparse checkout leaves no entry on working directory' ../stderr; then
        cat ../stderr >&2
        echo "unexpected sparse checkout failure in $current (status $status)" >&2
        exit 1
      fi
      cp ../index.before .git/index
      if git sparse-checkout reapply >../reapply.stdout 2>../reapply.stderr; then
        printf '0\n' >../reapply.status
      else
        status=$?
        printf '%s\n' "$status" >../reapply.status
        cat ../reapply.stderr >&2
        echo "all-excluded sparse reapply failed (status $status)" >&2
        exit 1
      fi
      printf 'reapply\n' >../method
    fi
    cp .git/index ../index.after
    git ls-files -t -z >../baseline
    if [[ "$current" == "$root/non_cone_directory_patterns_include_gitlinks" ]]; then
      rmdir module
    fi
    if [[ "$current" == "$root/empty_non_cone_patterns_exclude_everything" ]]; then
      git ls-files -z | while IFS= read -r -d '' path; do
        printf 'S %s\0' "$path"
      done >../baseline.expected
      cmp ../baseline.expected ../baseline
      rm ../baseline.expected
    fi
    if [[ "$current" == "$root/apply_patterns_preserves_existing_entry_flags" ]]; then
      git ls-files -v -z >../flags.after
      printf 's excluded\0h included\0' >../flags.expected
      cmp ../flags.expected ../flags.after
      rm ../flags.expected
    fi
    if [[ $(find . -mindepth 1 -maxdepth 1 ! -name .git -print) != '' ]]; then
      echo "sparse baseline unexpectedly materialized tracked paths in $current" >&2
      exit 1
    fi
  )
}

scenario cone_patterns_match_escaped_directories true false \
  'escaped metacharacters select literal directories without including unrelated directories' <<'EOF'
FILE selected/[nested]/file
FILE selected/back\slash/file
FILE selected/literal*/file
FILE selected/unselected/file
FILE unselected/file
EOF
record <<'EOF'
/*
!/*/
/selected/
!/selected/*/
/selected/\[nested]/
/selected/back\\slash/
/selected/literal\*/
EOF

scenario cone_membership_is_independent_of_entry_type true false \
  'cone parent entries remain included even when their types are files, gitlinks, or symlinks' <<'EOF'
COMMIT root
FILE selected/file
COMMIT selected/module
SYMLINK selected/link
FILE selected/unselected/file
EOF
record <<'EOF'
/*
!/*/
/selected/
!/selected/*/
EOF

scenario handcrafted_cone_includes_root_entries_and_selected_path true false \
  'a cone without the usual root preamble includes root entries and the selected path, not its siblings' <<'EOF'
FILE root
COMMIT selected/other
SYMLINK selected/root
FILE unselected/file
EOF
record <<'EOF'
/selected/other/
EOF

scenario final_full_cone_pattern_overrides_directory_exclusions true false \
  'a final full-cone pattern includes everything despite earlier directory exclusions' <<'EOF'
FILE root
FILE selected/nested/file
FILE unselected/file
EOF
record <<'EOF'
/*
!/*/
/selected/
!/selected/*/
/*
EOF

scenario cone_validation_and_matching_fold_parent_exclusions true true \
  'case folding also applies when validating a parent-directory exclusion' <<'EOF'
FILE SELECTED/root
FILE selected/nested/file
EOF
record <<'EOF'
/selected/
!/SELECTED/*/
EOF

scenario unmatched_cone_exclusion_falls_back_to_ordinary_patterns true false \
  'an unmatched negative cone pattern falls back to ordinary case-sensitive patterns' <<'EOF'
FILE SELECTED/root
FILE selected/nested/file
EOF
record <<'EOF'
/selected/
!/SELECTED/*/
EOF

scenario cone_checkout_prunes_unmatched_ancestors true false \
  'matching descendants remain skipped when checkout prunes an intermediate directory' <<'EOF'
FILE root
FILE a/b/c/file
FILE a/b/other
FILE a/root
EOF
record <<'EOF'
/a/b/c/
EOF

scenario cone_parent_selection_allows_reaching_descendants true false \
  'including the parent permits checkout to reach the selected descendant' <<'EOF'
FILE a/b/c/file
FILE a/b/other
FILE a/root
EOF
record <<'EOF'
/a/
!/a/*/
/a/b/c/
EOF

scenario cone_root_directory_glob_is_a_literal_selection true false \
  "Git accepts a root directory glob as a cone selecting the literal '*' directory" <<'EOF'
FILE root
FILE */file
FILE other/file
EOF
record <<'EOF'
/*/
EOF

scenario duplicate_cone_selection_survives_one_exclusion true false \
  'one parent exclusion cancels only one of two recursive selections' <<'EOF'
FILE selected/root
FILE selected/nested/file
FILE unselected/file
EOF
record <<'EOF'
/*
!/*/
/selected/
/selected/
!/selected/*/
EOF

scenario duplicate_cone_selection_is_canceled_by_two_exclusions true false \
  'two parent exclusions cancel both recursive selections while retaining parent files' <<'EOF'
FILE selected/root
FILE selected/nested/file
EOF
record <<'EOF'
/*
!/*/
/selected/
/selected/
!/selected/*/
!/selected/*/
EOF

scenario empty_cone_patterns_retain_root_entries_and_prune_descendants true false \
  'an empty cone retains root entries but prunes unselected directory descendants' <<'EOF'
FILE root
FILE nested/file
EOF
record </dev/null

scenario empty_non_cone_patterns_exclude_everything false false \
  'empty ordinary patterns select no entries, including root files, gitlinks, and symlinks; a checked Git refusal requires a successful all-skipped reapply baseline' <<'EOF'
FILE file
COMMIT module
SYMLINK link
FILE nested/file
EOF
record </dev/null

scenario non_cone_last_matching_pattern_wins false false \
  'the last matching pattern can either include or exclude an entry' <<'EOF'
FILE included
FILE excluded
EOF
record <<'EOF'
!/included
/included
/excluded
!/excluded
EOF

scenario non_cone_ancestor_exclusion_applies_to_descendants false false \
  'excluding a nested directory excludes its descendants without excluding its siblings' <<'EOF'
FILE selected/root
FILE selected/nested/file
EOF
record <<'EOF'
/selected/
!/selected/nested/
EOF

scenario non_cone_descendant_can_override_ancestor_exclusion false false \
  'an explicit descendant inclusion overrides its excluded ancestor without reincluding siblings' <<'EOF'
FILE selected/nested/keep
FILE selected/nested/other
EOF
record <<'EOF'
/selected/
!/selected/nested/
/selected/nested/keep
EOF

scenario non_cone_directory_patterns_include_gitlinks false false \
  'directory-only patterns include gitlinks and ancestors of nested files, but exclude symlinks and root files; an empty module directory works around Git ce_to_dtype() before recording its intended classification' <<'EOF'
COMMIT module
SYMLINK link
FILE file
FILE nested/file
EOF
record <<'EOF'
/*/
EOF

scenario apply_patterns_preserves_existing_entry_flags false false \
  'included entries retain exactly their original flags; excluded entries retain unrelated flags and gain only skip-worktree and extended flags. Git records ASSUME_VALID via --assume-unchanged on both entries; Rust must inject transient UPTODATE into index.before separately.' <<'EOF'
FILE included
FILE excluded
EOF
record <<'EOF'
/included
EOF
