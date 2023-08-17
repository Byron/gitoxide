#!/bin/bash
set -eu -o pipefail

git init;
git init sub
(cd sub
  : >empty
  git add empty
  git commit -m init-for-submodule
)

git init parent
(cd parent
  function baseline() {
    local args=""
    local specs=""

    for arg in "$@"; do
      if [[ $arg == *"+"* ]]; then
        echo "BUG: Argument '$arg' contains a space - we use that for separating pathspecs right now" >&2
        exit 1
      fi
      args="$args -c submodule.active=$arg"
      specs="${specs}+${arg}"
    done

    {
        echo "$specs"
        git $args submodule
        echo -n ';'
    } >> baseline.git
  }

  cat <<EOF >.gitattributes
bb bb-file
bb/ bb-dir
/bb/ bb-dir-from-top
EOF

  for p in a bb dir/b dir/bb dir/nested/c cc; do
    git submodule add ../sub $p
    git config --unset submodule.$p.active
  done
  git commit -m "init"

  git submodule > paths

  baseline ':'
  baseline ':!'
  baseline 'a'
  baseline ':(attr:bb-file)'
  # :(attr:bb-dir) - ["bb", "dir/bb"] == []
  baseline ':(attr:bb-dir)' git-inconsistency # bb/ matches recursively, git doesn't get it
  # :(attr:bb-dir-from-top) - ["bb"] == []
  baseline ':(attr:bb-dir-from-top)' git-inconsistency # probably git doesn't really care about correctness here
  baseline ':(icase)A'
  baseline ':(icase,exclude)A'
  baseline ':(icase,exclude)*/B*'
  baseline ':(icase,exclude)*/B?'
  baseline 'di'
  baseline 'di?'
  baseline 'di?/'
  baseline 'dir*'
  baseline 'dir/*'
  baseline ':(glob)dir*'
  baseline ':(glob,icase,exclude)dir*'
  baseline ':(glob)dir/*'
  baseline 'dir'
  baseline 'dir/'
  baseline ':(literal)dir'
  baseline ':(literal)dir/'
  baseline 'dir/nested'
  baseline 'dir/nested/'
  baseline ':(exclude)dir/'
  baseline ':(icase)DIR'
  baseline ':(icase)DIR/'
  baseline ':!a'
  baseline ':' ':!bb'
  baseline ':!bb'
  baseline 'a/'
  baseline 'bb'
  baseline 'dir/b'
  baseline 'dir/b/'
  # ["dir/b"] == []
  baseline '*/b/' git-inconsistency
  baseline '*b'
  baseline '*b*'
  baseline '*b?'
  baseline '*/b'
  baseline '*/b?'
  baseline '*/b*'
  baseline '*c'
  baseline '*/c'
  baseline ':(glob)**/c'
  baseline ':(glob)**/c?'
  baseline ':(glob)**/c*'
)
