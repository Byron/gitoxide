#!/usr/bin/env bash
set -eu -o pipefail

# Git clone, rather than ref-reading commands, is the oracle. Every source owns
# its objects; clones use transport (also for depth), never alternates/hardlinks.
# Inputs and exit diagnostics stay beside each Git-produced clone. Relative URLs
# keep copied fixtures independent of the directory in which they were generated.
git init -q seed
echo one >seed/file
git -C seed add file
git -C seed commit -qm one
git -C seed tag lightweight
git -C seed tag -am annotated annotated
git -C seed branch other
echo two >seed/file
git -C seed commit -qam two
git -C seed tag -am latest latest
echo three >seed/file
git -C seed commit -qam three
git -C seed tag other main
git clone -q --bare --no-local seed symbolic
git -C symbolic config --unset remote.origin.url
git -C symbolic config --unset-all remote.origin.fetch || :

for source in detached-none detached-one detached-many detached-master collision-equal collision-different missing; do
  cp -R symbolic "$source"
done
git -C detached-none update-ref --no-deref HEAD "$(git -C seed rev-parse main~1)"
git -C detached-one update-ref --no-deref HEAD "$(git -C seed rev-parse main)"
git -C detached-many update-ref --no-deref HEAD "$(git -C seed rev-parse main)"
git -C detached-many branch alpha main
git -C detached-many branch preferred main
git -C detached-master update-ref --no-deref HEAD "$(git -C seed rev-parse main)"
git -C detached-master branch alpha main
git -C detached-master branch master main
git -C collision-equal update-ref refs/heads/HEAD "$(git -C seed rev-parse main)"
git -C collision-different update-ref refs/heads/HEAD "$(git -C seed rev-parse other)"
git -C missing symbolic-ref HEAD refs/heads/absent
git init -q --bare unborn
git -C unborn symbolic-ref HEAD refs/heads/unborn

mkdir cases
# `clone --revision` arrived in Git 2.49; older hosts still cover the other clone modes.
clone_supports_revision=false
clone_help=$(git clone -h 2>&1 || :)
case "$clone_help" in *'revision <rev>'*) clone_supports_revision=true ;; esac
function baseline() {
  local name=$1 source=$2 branch=$3 depth=$4 default_branch=$5 remote=$6 revision=$7
  shift 7
  if test "$revision" != '-' && ! "$clone_supports_revision"; then return; fi
  local mode case_dir
  for mode in normal no-checkout bare; do
    case_dir="cases/$name-$mode"
    mkdir "$case_dir"
    printf '%s\n' "$source" "$branch" "$depth" "$default_branch" "$remote" "$revision" >"$case_dir/inputs"
    printf '%s\n' "$@" >"$case_dir/extra-refspecs"
    local args=(--no-local --origin "$remote")
    case "$mode" in no-checkout) args+=(--no-checkout) ;; bare) args+=(--bare) ;; esac
    if test "$branch" != '-'; then args+=(--branch "$branch"); fi
    if test "$depth" != '-'; then args+=(--depth "$depth"); fi
    if test "$revision" != '-'; then args+=(--revision "$revision"); fi
    local spec
    for spec in "$@"; do args+=(--config "remote.$remote.fetch=$spec"); done
    local status=0
    git -c init.defaultBranch="$default_branch" -c protocol.version=2 clone "${args[@]}" "$source" "$case_dir/git" >"$case_dir/stdout" 2>"$case_dir/stderr" || status=$?
    printf '%s\n' "$status" >"$case_dir/status"
    if test "$status" = 0; then
      git -C "$case_dir/git" remote set-url "$remote" "../../../$source"
    fi
  done
}

#        name                source              branch      depth default_branch remote      revision
baseline symbolic            symbolic            -           -     main           origin      -
baseline detached-none       detached-none       -           -     main           origin      -
baseline detached-one        detached-one        -           -     main           origin      -
baseline detached-preferred  detached-many       -           -     preferred      origin      -
baseline detached-first      detached-many       -           -     absent         origin      -
baseline detached-master     detached-master     -           -     absent         origin      -
baseline collision-equal     collision-equal     -           -     main           origin      -
baseline collision-different collision-different -           -     main           origin      -
baseline missing             missing             -           -     main           origin      -
baseline unborn              unborn              -           -     main           origin      -
baseline shallow-default     symbolic            -           1     main           origin      -
baseline shallow-other       symbolic            other       1     main           origin      -
baseline shallow-detached    detached-one        -           1     main           origin      -
baseline shallow-unmatched   detached-none       -           1     main           origin      -
baseline shallow-unborn      unborn              -           1     main           origin      -
baseline branch              symbolic            other       -     main           origin      -
baseline lightweight         symbolic            lightweight -     main           origin      -
baseline annotated           symbolic            annotated   -     main           origin      -
baseline shallow-tag         symbolic            latest      1     main           origin      -
baseline revision            symbolic            -           -     main           origin      refs/heads/other
baseline revision-tag        symbolic            -           -     main           origin      refs/tags/annotated
baseline missing-branch      symbolic            absent      -     main           origin      -
baseline missing-revision    symbolic            -           -     main           origin      refs/heads/absent
baseline remote-name         symbolic            -           -     main           team/origin -

# Git's configured refspecs supplement its clone defaults. Use the same ordering
# in gix; replacement-only refspecs are a gix API extension, tested separately.
#        name                source              branch      depth default_branch remote      revision extra-refspecs
baseline custom              symbolic            -           -     main           origin      -        '+refs/heads/main:refs/tracking/default'
baseline multiple            symbolic            -           -     main           origin      -        '+refs/heads/main:refs/tracking/first' '+refs/heads/main:refs/tracking/second'
baseline collision-local     symbolic            other       -     main           origin      -        '+refs/heads/main:refs/heads/other'
