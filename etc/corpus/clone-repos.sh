#!/bin/bash

# Check if there is input on stdin
if [ -t 0 ]; then
  echo "No input from stdin. Pipe a JSONL file with records form https://www.kaggle.com/datasets/pelmers/github-repository-metadata-with-5-stars"
  exit 1
fi

prefix=${1:-.}

# Define maximum repository size: 5GB in kilobytes
max_repo_size_kb=$((5 * 1024 * 1024))

# Iterate over each line in the file
while IFS= read -r line
do
    # Parse the JSON and extract the repository name and size
    repo=$(echo $line | jq -r '.nameWithOwner')
    repo_dir=${prefix}/github.com/${repo}.git
    repo_size_kb=$(echo $line | jq -r '.diskUsageKb')

    if [ ! -z "$repo" -a "$repo" != " " ]; then
        if [ ! -d "$repo_dir" ]; then
            # Only clone if the repository size is less than the max size
            if [ $repo_size_kb -lt $max_repo_size_kb ]; then
                echo "cloning $repo to $repo_dir"
                git clone --bare https://github.com/$repo "$repo_dir"
                git -C $repo_dir read-tree @
                git -C $repo_dir commit-graph write --no-progress --reachable
            else
                echo "Skipping repository $repo due to size larger than 5GB."
            fi
        else
            echo "Skipping existing: $repo_dir"
        fi
    fi
done
