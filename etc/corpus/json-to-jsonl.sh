#!/usr/bin/env bash

jq -c '.[]' "${1:?One argument that is the path to the JSON file to convert}" > "${1}l"
