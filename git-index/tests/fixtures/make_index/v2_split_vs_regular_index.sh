#!/bin/bash

# Creates a 2 identical repositories, one using a split index, the other
# using a regular index

set -eu -o pipefail

export GIT_INDEX_VERSION=2
git init split -q
git init regular -q

(cd split
    echo "a" > a
    echo "b" > b
    echo "c" > c
    echo "x" > x
    echo "y" > y
    echo "z" > z

    git add .
    git commit -m initial

    # create shared index from current index
    git update-index --split-index

    # never write changes to shared index
    git config splitIndex.maxPercentChange 100

    rm a x
    echo "b changed" > b
    echo "d" > d

    git add .
    git commit -m second
)

(cd regular
    echo "a" > a
    echo "b" > b
    echo "c" > c
    echo "x" > x
    echo "y" > y
    echo "z" > z

    git add .
    git commit -m initial

    rm a x
    echo "b changed" > b
    echo "d" > d

    git add .
    git commit -m second
)
