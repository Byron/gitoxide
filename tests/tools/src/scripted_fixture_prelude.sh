# Sourced into the Bash instance executing a fixture, not installed in its environment.
__gix_testtools_fixture_root=$PWD

# Keep diagnostics out of stderr redirected into fixture baselines.
exec 9>&2
if (( BASH_VERSINFO[0] > 4 || (BASH_VERSINFO[0] == 4 && BASH_VERSINFO[1] >= 1) )); then
  BASH_XTRACEFD=9
  set -x
else
  # Bash 3.2 cannot redirect xtrace independently of stderr.
  set -E
  trap 'printf "fixture command failed (%s): %s:%s: %s\n" "$?" "$0" "$LINENO" "$BASH_COMMAND" >&9' ERR
fi

gix_testtools_require_symlinks() {
  local probe status
  probe=$(mktemp -d "$__gix_testtools_fixture_root/.gix-symlink-preflight.XXXXXX") || return
  if ! printf "probe\n" >"$probe/target" || ! command -v ln >/dev/null; then
    rm -rf "$probe"
    return 1
  fi
  if ln -s target "$probe/link" 2>"$probe/error"; then
    if ! test -L "$probe/link"; then
      echo "symlink preflight: ln succeeded without creating a symbolic link" >&2
      rm -rf "$probe"
      return 1
    fi
    printf "supported\n" >"$__gix_testtools_fixture_root/__gix_testtools_symlinks__" || {
      rm -rf "$probe"
      return 1
    }
    rm -rf "$probe" || return
  else
    status=$?
    case "$status" in
      1) ;;
      *)
        cat "$probe/error" >&2
        rm -rf "$probe"
        return 1
        ;;
    esac
    { printf "unsupported\n"; cat "$probe/error"; } >"$__gix_testtools_fixture_root/__gix_testtools_symlinks__" || {
      rm -rf "$probe"
      return 1
    }
    rm -rf "$probe" || return
    exit 0
  fi
}
