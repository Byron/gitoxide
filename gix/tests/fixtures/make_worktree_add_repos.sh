#!/usr/bin/env bash
set -eu -o pipefail

# Prerequisites and Git baselines for worktree-add tests. Platform-specific
# configuration and assertions remain in Rust.
case "$1" in
  sparse-gitlink)
    # A gitlink references a real commit and should match directory-only patterns.
    # Pre-create its directory only for Git to work around ce_to_dtype()'s byte-swap
    # bug; gix must classify it from the index when its destination is empty.
    mkdir main
    cd main
    git init
    printf 'root\n' >file
    git add file
    git commit -m source
    git update-index --add --cacheinfo "160000,$(git rev-parse HEAD),module"
    git commit -m gitlink
    git sparse-checkout init --no-cone
    printf '/*/\n' >.git/info/sparse-checkout
    git worktree add --detach --no-checkout ../git-created HEAD
    mkdir ../git-created/module
    git -C ../git-created read-tree -mu HEAD
    git -C ../git-created ls-files -t >../git-created.ls-files.baseline
    test -d ../git-created/module
    printf 'S file
H module
' >../expected-listing
    cmp ../expected-listing ../git-created.ls-files.baseline
    ;;
  sparse)
    mode="${2:-}"
    state="${3:-}"
    case "$mode" in
      cone) sparse_flag=--cone ;;
      non-cone) sparse_flag=--no-cone ;;
      *)
        echo "unknown sparse worktree-add fixture mode: $mode (expected cone or non-cone)" >&2
        exit 1
        ;;
    esac
    case "$state" in
      inherit|missing|empty|disabled|directory|invalid-cone) ;;
      deferred)
        if [[ "$mode" != cone ]]; then
          echo "sparse worktree-add fixture state deferred requires cone mode" >&2
          exit 1
        fi
        ;;
      unreadable)
        if [[ "$mode" != non-cone ]]; then
          echo "sparse worktree-add fixture state unreadable requires non-cone mode" >&2
          exit 1
        fi
        ;;
      *)
        echo "unknown sparse worktree-add fixture state: $state" >&2
        exit 1
        ;;
    esac

    # Use Creation::Execute: linked worktrees contain absolute links into root/main.
    # The target adds files absent from the source index, so additions must match
    # patterns against the target tree, not just copy source skip-worktree bits.
    root="$PWD"
    mkdir main
    cd main
    git init
    for directory in selected other; do
      mkdir -p "$directory/nested" "$directory/deeper"
      printf '%s\n' "$directory" >"$directory/file.txt"
      printf 'keep\n' >"$directory/nested/keep.txt"
      printf 'drop\n' >"$directory/nested/drop.txt"
      printf 'deep\n' >"$directory/deeper/deep.txt"
    done
    printf 'root\n' >root.txt
    git add .
    git commit -m source
    printf 'new root\n' >target-root.txt
    for directory in selected other; do
      printf 'new nested\n' >"$directory/nested/new.txt"
    done
    git add .
    git commit -m target
    git branch target
    git reset --hard HEAD~1

    sparse_select() {
      local worktree="$1" directory="$2" sparse_file
      if [[ "$mode" == cone ]]; then
        git -C "$worktree" sparse-checkout set --cone --sparse-index "$directory/nested"
      else
        git -C "$worktree" sparse-checkout init --no-cone
        sparse_file="$(git -C "$worktree" rev-parse --path-format=absolute --git-path info/sparse-checkout)"
        printf '/%s/
!/%s/nested/
/%s/nested/keep.txt
/%s/nested/new.txt
!**/drop.txt
/root.txt
' \
          "$directory" "$directory" "$directory" "$directory" >"$sparse_file"
        git -C "$worktree" read-tree -mu HEAD
      fi
    }

    # Leave root/git-* intact for content/existence comparisons. Index listings
    # live beside each baseline worktree as root/git-*.ls-files.baseline.
    sparse_baseline() {
      git -C "$1" ls-files -t >"$1.ls-files.baseline"
    }

    case "$state" in
      inherit)
        sparse_select "$root/main" selected
        git worktree add --detach "$root/linked" HEAD
        sparse_select "$root/linked" other
        git worktree add --detach "$root/git-main" target
        sparse_baseline "$root/git-main"
        git -C "$root/linked" worktree add --detach "$root/git-linked" target
        sparse_baseline "$root/git-linked"
        ;;
      deferred)
        git sparse-checkout set --cone selected/nested
        git worktree add --detach "$root/git-deferred" target
        git -C "$root/git-deferred" sparse-checkout set --cone other/nested
        sparse_baseline "$root/git-deferred"
        ;;
      unreadable)
        git sparse-checkout init --no-cone
        rm .git/info/sparse-checkout
        mkdir .git/info/sparse-checkout
        # No linked worktrees: Rust verifies failed addition rolls back registration.
        ;;
      *)
        git sparse-checkout init "$sparse_flag"
        case "$state" in
          missing) rm .git/info/sparse-checkout ;;
          empty) : >.git/info/sparse-checkout ;;
          disabled)
            printf '/root.txt\n' >.git/info/sparse-checkout
            git config --worktree core.sparseCheckout false
            ;;
          directory) printf '/selected/\n' >.git/info/sparse-checkout ;;
          invalid-cone) printf '/selected/nested/new.txt\n' >.git/info/sparse-checkout ;;
        esac
        git worktree add --detach "$root/git-created" target
        sparse_baseline "$root/git-created"
        ;;
    esac
    ;;
  config-inheritance)
    # Preserve the basic history and the this file used for line-ending checks.
    # Rust installs the platform-specific filters after these attributes are committed.
    source "$(dirname "${BASH_SOURCE[0]}")/make_basic_repo.sh"
    printf "filtered filter=inherit\n" >.gitattributes
    printf "hello\n" >filtered
    git add .gitattributes filtered
    git commit -m "record filter attributes"
    ;;
  relative-config-lock)
    # Honor the fixture runner-selected hash; Rust holds the config lock during addition.
    git init
    git commit --allow-empty -m initial
    ;;
  relative-config-unknown-extension)
    # Only SHA-1 starts at format v0. The unknown extension must prevent upgrading
    # to v1 when enabling relative worktree links, even in SHA-256 fixture runs.
    git init --object-format=sha1
    git commit --allow-empty -m initial
    git config extensions.futureExtension true
    ;;
  separate-git-dir)
    # Use Creation::Execute: main/.git contains an absolute path to separate.git.
    git init --initial-branch=main --separate-git-dir separate.git main
    git -C main commit --allow-empty -m initial
    ;;
  *)
    echo "unknown worktree-add fixture scenario: $1" >&2
    exit 1
    ;;
esac
