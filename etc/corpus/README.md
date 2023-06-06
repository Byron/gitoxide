`gitoxide` uses a corpus of popular git repositories to validate its own algorithms against.
This file contains all information needed to build a corpus of varying sizes and to run `gitoxide` against it.

### Setup corpus

The `corpus` is a set of git repositories to run algorithms against. The set is a filtered list…

* …ordered by popularity on GitHub by stars and…
* …filtered by repos smaller than 5GB…
* …and limited until a certain size on disk is reached…

which is then cloned to disk into a directory structure that mirrors its URL.

#### Obtaining the set of repository metadata

The ~1.8GB file can be downloaded here https://www.kaggle.com/datasets/pelmers/github-repository-metadata-with-5-stars (*needs account*).
When downloaded it has to be converted to JSONL for consumption.

```shell
# convert the downloaded JSON into JSONL into the same directory
json-to-jsonl.sh repo_metadata.json
```

#### Figuring out how many repos fit the available space

Skipping all repositories larger than 5GB and filling a limit, one can use the Jupyter Notebook at https://www.kaggle.com/code/pelmers/explore-github-repository-metadata (*account required).

Add the following snippet to the notebook and adjust `limit` to your needs.

```python
five_gb_in_kb = 5 * 1024 * 1024
limit = 3500 * 1024 * 1024

# Order by 'stars' column and filter by 'diskUsageKb'
df = df[df['diskUsageKb'] < five_gb_in_kb]
df_sorted = df.sort_values(by='stars', ascending=False)

# Calculate how many entries would fit into 350GB
disk_usage_cumsum = df_sorted['diskUsageKb'].cumsum()
(disk_usage_cumsum <= limit).sum()
```

In the example above, one would manage to fit 68568 repositories into 3.5TB.

#### Cloning the repositories

Run `head -n 999 repo_metadata.sample.jsonl | ./clone-repos.sh <corpus>` to clone into the given `<corpus>` location, or any other invocation with 
your respective `repo_metadata.jsonl` and the computed amount of repos to include as in `head -n <your-count> <your.jsonl>`.

#### Add one large (100GB+) repository and one with a lot of commits repository by hand

Invoke `git clone --bare https://github.com/NagatoDEV/PlayStation-Home-Master-Archive  <corpus>/github.com/NagatoDEV/PlayStation-Home-Master-Archive.git` (after replacing `<curpus>` with your base path)
to obtain one sample of a huge repository with a lot of assets and other binary data whose tree spans more than 440k files. 

That way, we also get to see what happens when we have to handle huge binary files in massive trees.

Another massive tree and a more than 1.3m commits comes in with this invocation: 

`git clone --bare https://github.com/archlinux/svntogit-community <corpus>/github.com/archlinux/svntogit-community.git`.

This repo has 100MB+ files with a lot of append-only changes to it, giving it a very imbalanced delta-tree that triggers worst-case behaviour that needed
special mitigations:

`git clone --bare https://github.com/fz139/vigruzki <corpus>/github.com/fz139/vigruzki.git`.

All repos should be topped off with…

```shell
cd <corpus>
for d in github.com/archlinux/svntogit-community.git github.com/NagatoDEV/PlayStation-Home-Master-Archive.git github.com/fz139/vigruzki.git; do
    git -C $d read-tree @
    git -C $d commit-graph write --no-progress --reachable
done
```

### Run on-off `gix` commands by hand

Sometimes it's interesting to try a new command against all available repositories to see if it fails:

`ein t find <corpus> | xargs -P10 -I {} bash -c 'echo {}; gix -r {} <command>`

### Run `gix corpus`

The `corpus` sub-command runs specifically implemented commands against the corpus in a parallel fashion and stores these results in a local sqlite database for
later comparison.

**TBD**
