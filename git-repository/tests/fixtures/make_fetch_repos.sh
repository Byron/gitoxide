set -eu -o pipefail

git clone --bare "${1:?First argument is the complex base repo from make_remote_repos.sh/base}" base

git clone --shared base clone-as-base-with-changes
(cd clone-as-base-with-changes
  touch new-file
  git add new-file
  git commit -m "add new-file"
  git tag -m "new-file introduction" v1.0
)

git clone --shared base two-origins
(cd two-origins
  git remote add changes-on-top-of-origin "$PWD/../clone-as-base-with-changes"
  git branch "not-currently-checked-out"
)
