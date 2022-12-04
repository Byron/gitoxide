# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

This release also fixes compatibility issues that formerly prevented to fetch or clone form `https://googlesource.com`.

### Changed

 - <csr-id-a4ac9cf3e667a3059e33aac8188150529578622d/> represent `GIT_(COMMITTER|AUTHOR)_(NAME|EMAIL|DATE)` with git configuration.
   That way it becomes more obvious where values are coming from.

### New Features

 - <csr-id-98143699bb9481b010e21647f64dcb8a74bd80ad/> auto-enabled verbosity for `gix fetch/clone` and add `--no-verbose`.
   I found myself always adding (and having to remember to add) the `-v` flag
   for long-running operations so these should be able to default to a
   higher verbosity level.
   
   To counter that, there is a new `--no-verbose` flag to turn that off.
 - <csr-id-aeb4a1d5cb76316058c7d687e26f5c7db351c09c/> add `--strict` option to enforce strict checking of configuration.

### Changed (BREAKING)

 - <csr-id-49f39d6bb487c0254176a5082f2c7851b83952a1/> `open::ReplacementObjects` is removed in favor of two custom git-configuration flags.
   Now it's possible to map the environment variables `GIT_REPLACE_REF_BASE` and `GIT_NO_REPLACE_OBJECTS`
   to custom git configuration keys which can also be set, namely `gitoxide.odb.replaceObjectsRefBase`
   and `gitoxide.odb.noReplaceObjects`.
   
   Along with the possibility of disabling the usage of `GIT_` prefixed environment variables one
   reaches the previous level of control without making object replacement a special case.

### New Features (BREAKING)

 - <csr-id-becbd8d896a1663f1607be4e86e632773e926f1f/> represent object cache configuration like `GITOXIDE_PACK_CACHE_MEMORY` in git-configuration.
   That way there is a unified system for how to set values, which may be overridable by configuration
   variables or not.
   
   With this changes, the explicit application of environment variables for setting the cache
   isn't required anymore as everything happens using git-configuration, and automatically,
   while providing full control like before.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 12 calendar days.
 - 12 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - auto-enabled verbosity for `gix fetch/clone` and add `--no-verbose`. ([`9814369`](https://github.com/Byron/gitoxide/commit/98143699bb9481b010e21647f64dcb8a74bd80ad))
    - switch from `atty` to `is-terminal` ([`7304bc1`](https://github.com/Byron/gitoxide/commit/7304bc1c0efaad64a39520962072343ef02f6c25))
    - adapt to changes in `git-repository` ([`c4f68bf`](https://github.com/Byron/gitoxide/commit/c4f68bf775b854625d901fe0bfcbdd38f656d408))
    - represent object cache configuration like `GITOXIDE_PACK_CACHE_MEMORY` in git-configuration. ([`becbd8d`](https://github.com/Byron/gitoxide/commit/becbd8d896a1663f1607be4e86e632773e926f1f))
    - represent `GIT_(COMMITTER|AUTHOR)_(NAME|EMAIL|DATE)` with git configuration. ([`a4ac9cf`](https://github.com/Byron/gitoxide/commit/a4ac9cf3e667a3059e33aac8188150529578622d))
    - `open::ReplacementObjects` is removed in favor of two custom git-configuration flags. ([`49f39d6`](https://github.com/Byron/gitoxide/commit/49f39d6bb487c0254176a5082f2c7851b83952a1))
    - apply related environment variables as config overrides ([`9441c26`](https://github.com/Byron/gitoxide/commit/9441c261bcae61d1d1e674b5e783f38b0471be29))
    - adapt to changes in `git-repository` ([`f1a4c8b`](https://github.com/Byron/gitoxide/commit/f1a4c8b42ed8c94e7fe3a61eb222cf6b0886f4ee))
    - update progress of http.proxyAuthMethod ([`872dc1a`](https://github.com/Byron/gitoxide/commit/872dc1ab43ce626b4166dae3dc8bddf8e85c9409))
    - add `--strict` option to enforce strict checking of configuration. ([`aeb4a1d`](https://github.com/Byron/gitoxide/commit/aeb4a1d5cb76316058c7d687e26f5c7db351c09c))
    - don't lock stdout/stderr as it will deadlock on dbg-printing ([`62cae0e`](https://github.com/Byron/gitoxide/commit/62cae0e6bfe8113c0225152a896338017c8de474))
    - adapt to changes in `git-config` ([`1c2e755`](https://github.com/Byron/gitoxide/commit/1c2e755e517b0f9fe8671187f5c30076ce43a3c9))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.19.0 (2022-11-21)

### New Features

 - <csr-id-3ddbd2de369b521fa3f21935f10fe9c248840893/> Make `reqwest` TLS backend configuration easy.
   We provide the choice of `native-tls` or `rust-tls`. If none is
   provided, the user can configure on their on similar to how it's done
   in `git-repository`.
   
   Please note that a choice now has to be made or HTTPS will not be
   available, so use one of…
   
   * blocking-http-transport-reqwest-rust-tls
* blocking-http-transport-reqwest-native-tls

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make `reqwest` TLS backend configuration easy. ([`3ddbd2d`](https://github.com/Byron/gitoxide/commit/3ddbd2de369b521fa3f21935f10fe9c248840893))
</details>

## 0.18.0 (2022-11-17)

This releases fixes `gix fetch` so that it is able to clone or fetch `pytorch` or other repositories on case-insensitive file systems. 

It's also an attempt to trigger CI to build binary releases.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 9 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'http-config' ([`665b53e`](https://github.com/Byron/gitoxide/commit/665b53e1c2e1de65fafa28b669f58977868bbc81))
    - Document that histogram is now the default diff algorithm ([`c76572b`](https://github.com/Byron/gitoxide/commit/c76572b3662776b524a7e4a1fd96d2eaa22a560f))
    - Introduce new `gitoxide.http.connectTimeout` for more control for git clients ([`2ab80e4`](https://github.com/Byron/gitoxide/commit/2ab80e4c95a7bf3c7e56bb5a95ac78ac930fc9ee))
    - keep track of `no_proxy` environment variable support ([`f0625de`](https://github.com/Byron/gitoxide/commit/f0625de13073de4767881ed0398d0cd2791b0ad2))
    - update progress ([`3d9fb6c`](https://github.com/Byron/gitoxide/commit/3d9fb6c095a272d5ddf6c5b6ce96820bc9d59cbb))
    - don't forget to update 'progress' ([`0ec5220`](https://github.com/Byron/gitoxide/commit/0ec5220fea50f06eb61bafff525111ce2435c994))
    - update progress with gitoxide.userAgent ([`1c012f4`](https://github.com/Byron/gitoxide/commit/1c012f4c2e05e1f565fc51fffee2f7d278e5a7de))
    - plan for user agent string configuration ([`f5499a5`](https://github.com/Byron/gitoxide/commit/f5499a55ed0230e2852b41b54648003e3d6cb859))
    - keep track of http related configuration keys. ([`1afaebd`](https://github.com/Byron/gitoxide/commit/1afaebdcae977af8a9a0f0788ec754746d6d05bb))
</details>

## 0.17.0 (2022-11-06)

### New Features

 - <csr-id-e973dfeaf17ca63385496202e9fdcdd912e20f42/> `gix remote ref-map --show-unmapped-remote-refs`.
   That way it's more obvious to see what was filtered out by ref-specs.
   
   It's also great to validate that server-side filtering via ref-prefix
   will not send refs that are referred to by symbolic refs that are
   not filtered out. That should be fine as it's all about objects,
   it's just something to deal with as we may have to deal with symbolic
   refs that aren't in the set of refs the server sent to us.
 - <csr-id-b1edb9e3537df86669714f03666f4a88e0ac3709/> diff algorithm is controlled by git configuration `diff.algorithm`
 - <csr-id-20259da4ddf9fabfb2d2bd4e2274c0ed42bdb0e5/> `ein t hours` allows to specify the amount of worker threads.

### Bug Fixes

 - <csr-id-3a053284cfefe27873dcc5b4f74d63a560bb5d41/> collect `stderr` and print it afterwards to avoid intersection with line progress.
   Previously it would happen that stderr would be printed directly and mix
   with the line progress (as in `-v`) which also prints to stderr.
   
   Now errors are collected and output at the end once the line renderer
   was already shutdown.

### Changed (BREAKING)

 - <csr-id-3a0fb1b45c757add49677450836c0aaf6179a2b5/> remote `lock_mode` from all methods dealing with reference edits.
   It is now read from `core.filesRefLockTimeout` accordingly.

### New Features (BREAKING)

 - <csr-id-92bbe335688e4c8e96061663e71a599022f7b96f/> remove `gix remote --url` in favor of determining the intention similar to `git fetch`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 65 commits contributed to the release over the course of 47 calendar days.
 - 47 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#536](https://github.com/Byron/gitoxide/issues/536)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - keep track of several branch related keys ([`443a75a`](https://github.com/Byron/gitoxide/commit/443a75a6098968995bed644c84ed8ce9d8ed0c34))
    - `gix remote ref-map --show-unmapped-remote-refs`. ([`e973dfe`](https://github.com/Byron/gitoxide/commit/e973dfeaf17ca63385496202e9fdcdd912e20f42))
    - update progress as we now respect `diff.algorithm` ([`30d32e7`](https://github.com/Byron/gitoxide/commit/30d32e7255986e6e7ec92d55c86b747d7486e183))
    - apply configuration overrides to newborn repo during clone ([`c8ef759`](https://github.com/Byron/gitoxide/commit/c8ef759923a3c980b5a23c868f38804eccbc0fbc))
    - collect `stderr` and print it afterwards to avoid intersection with line progress. ([`3a05328`](https://github.com/Byron/gitoxide/commit/3a053284cfefe27873dcc5b4f74d63a560bb5d41))
    - reduce verbosity of `clone` and print once entire clone is done ([`9a476df`](https://github.com/Byron/gitoxide/commit/9a476df519bccfc6bcda8bd02aa4c1573a2691e7))
    - first rough sketch of `gix clone` ([`23a5e8b`](https://github.com/Byron/gitoxide/commit/23a5e8b658c5642c3f3060e013fd0eab06cbf027))
    - progress totals (planned, devitaion) ([`63947ae`](https://github.com/Byron/gitoxide/commit/63947ae77ecb5952d7b5da5c6695f002bb7a7c4c))
    - prepare attribute-group setup as far as possible. ([`f5e2eeb`](https://github.com/Byron/gitoxide/commit/f5e2eebe9560f664f044b82ffa0cd19fd0df311f))
    - update usage of `core.checkState` ([`8b2aba1`](https://github.com/Byron/gitoxide/commit/8b2aba1d2baf42a3578ee8c9ce89dbe29d4d3f4c))
    - update progress based on filesystem config usage ([`de13c66`](https://github.com/Byron/gitoxide/commit/de13c66ade29635ff585241fec2783cf972dd5c3))
    - inform about even more keys in `gix progress` ([`0e9bd41`](https://github.com/Byron/gitoxide/commit/0e9bd4133fe33382d6d8c42ca1280601582a1a17))
    - plan more core filesystem attributes ([`d10a82e`](https://github.com/Byron/gitoxide/commit/d10a82e86a461e72b9e5339a180f8adb34721e3e))
    - update `init.defaultBranch` - now used by custom initialization. ([`4d6d7bb`](https://github.com/Byron/gitoxide/commit/4d6d7bbf5c3d8159535e5756da082ca493e9a66a))
    - support for handshake information in `gix fetch` ([`c47dcc6`](https://github.com/Byron/gitoxide/commit/c47dcc69e54c635650b540c590131dbe7d32d05b))
    - fix build ([`d034882`](https://github.com/Byron/gitoxide/commit/d03488211ec5bd186f6d274c55cd96cfd9d119d5))
    - remove `gix remote --url` in favor of determining the intention similar to `git fetch` ([`92bbe33`](https://github.com/Byron/gitoxide/commit/92bbe335688e4c8e96061663e71a599022f7b96f))
    - support for `--url` for arbitrary urls when fetching ([`8c7351c`](https://github.com/Byron/gitoxide/commit/8c7351c4c50517b3ccc3479f2a7a020bc607bf24))
    - Frame for `gix fetch` ([`5b72d27`](https://github.com/Byron/gitoxide/commit/5b72d2708889dc388facd9cbc61e5bfa5403e003))
    - update progress information to include packedRefsTimeout ([`fd18320`](https://github.com/Byron/gitoxide/commit/fd18320561e05431796aa4044c0a2b0605c9ca9d))
    - remote `lock_mode` from all methods dealing with reference edits. ([`3a0fb1b`](https://github.com/Byron/gitoxide/commit/3a0fb1b45c757add49677450836c0aaf6179a2b5))
    - add information about planned lock timeout support (from configuration) ([`7076891`](https://github.com/Byron/gitoxide/commit/7076891fb1b44cd442928f7e56f53f4b085e7a11))
    - Add remotes.<group> as planned feature for remotes ([`3c188b2`](https://github.com/Byron/gitoxide/commit/3c188b2253bfb6d47394718425eef2d1a0547949))
    - slightly nicer styling of config keys ([`eade88f`](https://github.com/Byron/gitoxide/commit/eade88f8ebc638f504881e8bbbd60d42a5a3d9be))
    - complete listing of records based on current usage, probably ([`6abd5a4`](https://github.com/Byron/gitoxide/commit/6abd5a4e1daaf91fd109acb714057a82f67fa076))
    - add more records ([`5c0d0ab`](https://github.com/Byron/gitoxide/commit/5c0d0ab66d46a8d093ca0b5451996099a27ef1dd))
    - Add tabled for nicer printing ([`65e6496`](https://github.com/Byron/gitoxide/commit/65e64964c7cd151e53e5a7d4b9ba8fabda1c0e16))
    - refactor ([`b42b08a`](https://github.com/Byron/gitoxide/commit/b42b08afafd904ff2adb1f00688437357532193a))
    - add support for more types of configurations ([`317e02a`](https://github.com/Byron/gitoxide/commit/317e02a5900189ec7f9f3c2bb27d5696178d7869))
    - A very first version of `gix progress show` ([`92e082a`](https://github.com/Byron/gitoxide/commit/92e082a288cf5e48caa205a4c7bd1ced025fea46))
 * **[#536](https://github.com/Byron/gitoxide/issues/536)**
    - `ein t hours` allows to specify the amount of worker threads. ([`20259da`](https://github.com/Byron/gitoxide/commit/20259da4ddf9fabfb2d2bd4e2274c0ed42bdb0e5))
 * **Uncategorized**
    - Merge branch 'write-sparse-index' ([`ba17db0`](https://github.com/Byron/gitoxide/commit/ba17db03e4e832db724ab3e08e3df05eb61dd401))
    - plan `index.version` for when we can write V4 indices. ([`da96d34`](https://github.com/Byron/gitoxide/commit/da96d34bd609776f3c8590171ddbf894f2205ae5))
    - notes about the split-index extension. ([`ad44982`](https://github.com/Byron/gitoxide/commit/ad449822c952a7ed5e6124216e7d7668f0f5873f))
    - take note of additional options for promisor packs and partial clone filters ([`1ec27f8`](https://github.com/Byron/gitoxide/commit/1ec27f86113aa45a1540441a21152988b3911015))
    - make note of `extension.worktreeConfig` ([`fe1e646`](https://github.com/Byron/gitoxide/commit/fe1e6467296ff93c23fe191ed5a265348ce930dc))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - add `init.templateDir` to `gix progress` ([`9fab050`](https://github.com/Byron/gitoxide/commit/9fab0501f5017e43132138579238a46726a7348f))
    - make fmt ([`ea2136b`](https://github.com/Byron/gitoxide/commit/ea2136b065979cecb3a1fdbf7b20ed7514128d9a))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - diff algorithm is controlled by git configuration `diff.algorithm` ([`b1edb9e`](https://github.com/Byron/gitoxide/commit/b1edb9e3537df86669714f03666f4a88e0ac3709))
    - Merge branch 'main' into gix-clone ([`fa27570`](https://github.com/Byron/gitoxide/commit/fa27570f491388cce6137af44330d76870d07202))
    - Merge branch 'imra-diff' ([`f53f942`](https://github.com/Byron/gitoxide/commit/f53f9426f206686b30abd73a201a92b1405e782d))
    - adapt to changes in `git-diff` for a 2x speedup when calculating line changes ([`296f3b6`](https://github.com/Byron/gitoxide/commit/296f3b6ee29d5e628a19d56db80ba8736223e226))
    - update `gix progress` records ([`b05a2e7`](https://github.com/Byron/gitoxide/commit/b05a2e7939df0c25da8186395e7eceda9e1baa9b))
    - Merge branch 'main' into gix-clone ([`de4fe06`](https://github.com/Byron/gitoxide/commit/de4fe06202906ea5c62e667826b42cf7b57b1ff0))
    - Mark the upcoming usage of init.defaultBranch. ([`6225f35`](https://github.com/Byron/gitoxide/commit/6225f35398bc494ad74da342c4ebbe0487b106f8))
    - Merge branch 'fix-gix-index-from-tree' ([`da5f63c`](https://github.com/Byron/gitoxide/commit/da5f63cbc7506990f46d310f8064678decb86928))
    - write index without output path to memory only. ([`c8d0345`](https://github.com/Byron/gitoxide/commit/c8d03454f1b2c18876cc8935e0afcea611cb8647))
    - Merge branch 'gix-index-from-tree' ([`8c24386`](https://github.com/Byron/gitoxide/commit/8c24386f1874cd94f78fefbe434963f772878b1f))
    - refactor ([`67f2247`](https://github.com/Byron/gitoxide/commit/67f224785193a5269cf65963fd21b21b723d485e))
    - refactor ([`01ab5ca`](https://github.com/Byron/gitoxide/commit/01ab5cac23427a9c3b7f153201627eb8c8898e96))
    - update with various configuration variables relevant to checking out worktrees ([`09d767a`](https://github.com/Byron/gitoxide/commit/09d767abd371a268a9fa475956be09ddda6b42ea))
    - Merge branch 'clone' ([`507dc7e`](https://github.com/Byron/gitoxide/commit/507dc7e706cb3c9d89d048b3aff5df239a9b6788))
    - update usage of clone related configuration ([`1a1e862`](https://github.com/Byron/gitoxide/commit/1a1e862b2e8cd88f5f6fbb9d86f618761bb71ef1))
    - update progress with intended uses of `clone.` variables ([`8b804a3`](https://github.com/Byron/gitoxide/commit/8b804a31cb20a5264311f0b6ba02f857bea933ad))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - make fmt ([`53acf25`](https://github.com/Byron/gitoxide/commit/53acf2565743eff7cead7a42011107b2fc8d7e0e))
    - Merge branch 'fetch-pack' ([`f47c891`](https://github.com/Byron/gitoxide/commit/f47c89129732bcb06fe76a4696fe38ab1151fb0c))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - update docs ([`c5c0ac5`](https://github.com/Byron/gitoxide/commit/c5c0ac50616957b5b0dcaf530d294abe63e3b4c7))
    - input id can now be a commit or tree as prefix or full object id ([`8ef3fcb`](https://github.com/Byron/gitoxide/commit/8ef3fcbc416b2a53ead7d6ba36991a3d035f5f22))
    - thanks clippy ([`8dadd70`](https://github.com/Byron/gitoxide/commit/8dadd70f8b7db1794652805c6238763886a8570d))
    - Merge branch 'fix-odb-race' ([`b862fc5`](https://github.com/Byron/gitoxide/commit/b862fc52dd2409e912c892c7f428a571f565b64a))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.16.0 (2022-09-20)

### Changed

 - <csr-id-3c7c9a735f5771ef787cbc86b46cbafc9226f4d6/> `ein tool hours -s` was split into `-f|--file-stats` and `-l|line-stats`.
   That way more information is generated at increasingly high costs.

### New Features

 - <csr-id-28c4cae70aab2bd5b479961fcc6ee91ff80f651b/> `ein tool hours --stat` to collect additional statistics per author.
   Note that these are expensive and unconditionally use threads to speed
   up these computations.
 - <csr-id-5d0332f51c63c5456a28c8f3f466ad805b2e0626/> `ein tool hours -b` ignores bots.
   For now it only considers bots with names containing `[bot]`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 20 calendar days.
 - 27 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - refactor ([`11851f3`](https://github.com/Byron/gitoxide/commit/11851f334f642e7bd69bcbfc7ad4f1990fc326ba))
    - option to print server information about the connection ([`4720666`](https://github.com/Byron/gitoxide/commit/4720666c8bfdaa3acc5c832b44755d4b4f86e16e))
    - show fixes as well ([`2237495`](https://github.com/Byron/gitoxide/commit/2237495d82624b39bf75c6430549424a5e36b8bb))
    - Correct printing of tag information (even though it doesn't look great) ([`f4d8198`](https://github.com/Byron/gitoxide/commit/f4d8198992b4c45f64d81e20f40a1cad69883162))
    - wire up the `ref-map` sub-command. ([`94c2b78`](https://github.com/Byron/gitoxide/commit/94c2b785f892f85503b8927c7fa98ae99d677be7))
    - Select `gix` commands will now load the git installation configuration ([`23d2dec`](https://github.com/Byron/gitoxide/commit/23d2dec375305c39d472c4f8ff764274dd033f6b))
    - refactor ([`7abc0a3`](https://github.com/Byron/gitoxide/commit/7abc0a39205b9f374c90c4750fe6cc9b3749d7b9))
    - Add sketch of `gix credential` ([`642e21f`](https://github.com/Byron/gitoxide/commit/642e21fc58d8d4b68cba3067c88d44c019ec4ace))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - `ein tool hours -s` was split into `-f|--file-stats` and `-l|line-stats`. ([`3c7c9a7`](https://github.com/Byron/gitoxide/commit/3c7c9a735f5771ef787cbc86b46cbafc9226f4d6))
    - upgrade to prodash 20.1 for `Progress::counter()` feature ([`0ac4a2c`](https://github.com/Byron/gitoxide/commit/0ac4a2c514aeb94d8e90ce28ae7a0e0350c21ab2))
    - `ein tool hours --stat` to collect additional statistics per author. ([`28c4cae`](https://github.com/Byron/gitoxide/commit/28c4cae70aab2bd5b479961fcc6ee91ff80f651b))
 * **Uncategorized**
    - Merge branch 'hours-upgrade' ([`26489d1`](https://github.com/Byron/gitoxide/commit/26489d14472b840b36696435c22d9077f7ab323d))
    - use rev-specs instead of ref-names ([`cf7182e`](https://github.com/Byron/gitoxide/commit/cf7182e3390c03df97c10cd101440f7aa8874904))
    - `ein tool hours -b` ignores bots. ([`5d0332f`](https://github.com/Byron/gitoxide/commit/5d0332f51c63c5456a28c8f3f466ad805b2e0626))
    - Merge branch 'filter-refs' ([`3773b92`](https://github.com/Byron/gitoxide/commit/3773b92b8372c9a40a74d281149ca65b057a7da9))
    - implement `gix index from-tree` ([`2fbd3df`](https://github.com/Byron/gitoxide/commit/2fbd3df89373eea5d6268fa47e046c8ebad8bba9))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`56ba481`](https://github.com/Byron/gitoxide/commit/56ba481f4c48f74f10397feb1b6dc3d7dd3704fb))
    - A basic implementation of rev-list without anything fancy ([`791dd66`](https://github.com/Byron/gitoxide/commit/791dd666430fe0586c7db75b352487a72d3789e7))
</details>

## 0.15.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-45a30f0f31a99cda5cf105408e9c3905f43071f2/> Support for `-c/--config` in `gix`
 - <csr-id-5d6d5ca305615568dfedbcc10ea86294c0a0472d/> `gix remote refs` to list all remote references of a suitable remote.
   This takes into account either a named remote, or the remote associated
   with the current branch, or the default remote it could deduce or obtain
   from the configuration.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs

### Changed (BREAKING)

 - <csr-id-dda995790c260131048484a11e66185b9c841311/> remove `gix free remote ref-list` in favor of `gix remote refs`
   The functinality is the same, but the latter is built on top of a
   repository which is slightly less flexible, but preferable over
   maintaining a non-repo version.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Support for -c CLI config overrides in `gix config`. ([`19c1746`](https://github.com/Byron/gitoxide/commit/19c1746cefca9bc76537637ec99634eb4cf66a92))
    - remove `gix free remote ref-list` in favor of `gix remote refs` ([`dda9957`](https://github.com/Byron/gitoxide/commit/dda995790c260131048484a11e66185b9c841311))
    - Support for `-c/--config` in `gix` ([`45a30f0`](https://github.com/Byron/gitoxide/commit/45a30f0f31a99cda5cf105408e9c3905f43071f2))
    - refactor ([`e0be6e9`](https://github.com/Byron/gitoxide/commit/e0be6e9558add3255de63f3785306daace2707a6))
    - Add support for passing urls directly to bypass all remote repository logic. ([`df3cf18`](https://github.com/Byron/gitoxide/commit/df3cf18a6ac1e4f35f6d11d62184a43722397bbe))
    - `gix remote refs` to list all remote references of a suitable remote. ([`5d6d5ca`](https://github.com/Byron/gitoxide/commit/5d6d5ca305615568dfedbcc10ea86294c0a0472d))
    - Try to use maybe async for the simplest of possibly blocking remote interactions ([`db4df25`](https://github.com/Byron/gitoxide/commit/db4df250d7e58518015bed0b9a1e3391b209cb29))
    - basic parsing of `gix remote refs` without any implementation. ([`f8f1249`](https://github.com/Byron/gitoxide/commit/f8f124943f73bacf816c6d0055f0b66659fd3906))
 * **Uncategorized**
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - thanks clippy ([`bb6813a`](https://github.com/Byron/gitoxide/commit/bb6813abf365728d9851ee205b2c25b925a0f06a))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.14.0 (2022-08-17)

### Changed

 - <csr-id-0235111a4fcc40c7b57d973bfce27a66eddea901/> Invert behaviour to `open::Options::strict_config()`, with lenient being the default.
   This means API users will get libgit2 behaviour but commands like `gix` can
   change options to emulate `git` behaviour.

### New Features

 - <csr-id-b83f6bdf7f8d8f5cef2a57fa3932b6a0e0988db1/> `--cat-file` flag for `gix rev parse` to cat instead of resolving.
 - <csr-id-e972aad020d3653a53b20fa6e535d5759e239a45/> `gix rev previous-branches` subcommand

### Changed (BREAKING)

 - <csr-id-edf73dd4db5b0f5d9309c95bf366e11ea6723885/> `ein tools` to `ein tool` for as it's more intuitive

### New Features (BREAKING)

 - <csr-id-c5846e05aa54f0601ac7b8e2e59bcf1ffaa305f1/> `gix rev resolve --explain`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#427](https://github.com/Byron/gitoxide/issues/427)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - `--cat-file` flag for `gix rev parse` to cat instead of resolving. ([`b83f6bd`](https://github.com/Byron/gitoxide/commit/b83f6bdf7f8d8f5cef2a57fa3932b6a0e0988db1))
    - `gix rev resolve --explain` ([`c5846e0`](https://github.com/Byron/gitoxide/commit/c5846e05aa54f0601ac7b8e2e59bcf1ffaa305f1))
    - `gix rev previous-branches` subcommand ([`e972aad`](https://github.com/Byron/gitoxide/commit/e972aad020d3653a53b20fa6e535d5759e239a45))
    - support for parsing multiple specs in one invocation ([`84b5448`](https://github.com/Byron/gitoxide/commit/84b5448deb7b87f67a1b7461f00793e7ae33ef31))
    - support overriding cache settings with environment variables in `gix` ([`b838202`](https://github.com/Byron/gitoxide/commit/b8382026cb5b979a5c563ea40d1d8e483c1ca23a))
 * **Uncategorized**
    - Merge branch 'core-abbrev-handling' ([`dbaff13`](https://github.com/Byron/gitoxide/commit/dbaff13eaf7ba5f9c0ee2c90ac9f17e6078cad9e))
    - Control which command is lenient or not. That way `gix-config` can be lenient. ([`6a9c58f`](https://github.com/Byron/gitoxide/commit/6a9c58fde7ca4a52fa1c3225974a2019e7d93168))
    - Invert behaviour to `open::Options::strict_config()`, with lenient being the default. ([`0235111`](https://github.com/Byron/gitoxide/commit/0235111a4fcc40c7b57d973bfce27a66eddea901))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - Merge branch 'index-write-refactor' ([`805f432`](https://github.com/Byron/gitoxide/commit/805f432bf8e9d2dd9ede56caf959de386d5d80c7))
    - `ein tools` to `ein tool` for as it's more intuitive ([`edf73dd`](https://github.com/Byron/gitoxide/commit/edf73dd4db5b0f5d9309c95bf366e11ea6723885))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - add aliases to make revision sub-commands more accessible ([`a6d79e3`](https://github.com/Byron/gitoxide/commit/a6d79e38cb0dd7e87d00a098030bbcaa614f259d))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.13.0 (2022-07-22)

### New Features

 - <csr-id-eda39ec7d736d49af1ad9e2ad775e4aa12b264b7/> `gix config` with section and sub-section filtering.
 - <csr-id-d99453ebeb970ed493be236def299d1e82b01f83/> `gix config` lists all entries of all configuration files git considers.
   Filters allow to narrow down the output.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 47 commits contributed to the release over the course of 101 calendar days.
 - 108 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Allow reading patterns from stdin ([`0c597fe`](https://github.com/Byron/gitoxide/commit/0c597fe78acdd5672b4535a7d82620c5f7f93649))
    - Add `--show-ignore-patterns` to `gix repo exclude query` ([`09f904b`](https://github.com/Byron/gitoxide/commit/09f904b1f393f03176882d491d7fffcad4058b49))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - Support for overrides on the command-line ([`7d98b21`](https://github.com/Byron/gitoxide/commit/7d98b2196c130263ace4a948418affdd950302ed))
    - fix build ([`cb56f12`](https://github.com/Byron/gitoxide/commit/cb56f12ad83cf2932a068ef4fa0ca5ce4aa73e84))
    - refactor ([`3ff991d`](https://github.com/Byron/gitoxide/commit/3ff991d0ca0d63632fc5710680351840f51c14c3))
    - frame for `gix repo exclude query` ([`a331314`](https://github.com/Byron/gitoxide/commit/a331314758629a93ba036245a5dd03cf4109dc52))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - fix journey tests after `gix` restructuring ([`59b95c9`](https://github.com/Byron/gitoxide/commit/59b95c94aacac174e374048b7d11d2c0984a19e0))
    - `gix config` with section and sub-section filtering. ([`eda39ec`](https://github.com/Byron/gitoxide/commit/eda39ec7d736d49af1ad9e2ad775e4aa12b264b7))
    - `gix config` lists all entries of all configuration files git considers. ([`d99453e`](https://github.com/Byron/gitoxide/commit/d99453ebeb970ed493be236def299d1e82b01f83))
    - refactor ([`a437abe`](https://github.com/Byron/gitoxide/commit/a437abe8e77ad07bf25a16f19ca046ebdaef42d6))
    - move 'exclude' up one level and dissolve 'repo' subcommand ([`8e5b796`](https://github.com/Byron/gitoxide/commit/8e5b796ea3fd760839f3c29a4f65bb42b1f3e893))
    - move 'mailmap' up one level ([`5cf08ce`](https://github.com/Byron/gitoxide/commit/5cf08ce3d04d635bbfee169cb77ce259efbf6bc3))
    - move 'odb' up one level ([`0ed65da`](https://github.com/Byron/gitoxide/commit/0ed65da9b66d4cc3c85d3b70fa4bc383c7a0d1a3))
    - move 'tree' up one level ([`38a8350`](https://github.com/Byron/gitoxide/commit/38a8350d75720a8455e9c55d12f7cdf4b1742e56))
    - move 'commit' up one level ([`72876f1`](https://github.com/Byron/gitoxide/commit/72876f1fd65efc816b704db6880ab881c89cff01))
    - move 'verify' up one level ([`ac7d99a`](https://github.com/Byron/gitoxide/commit/ac7d99ac42ff8561e81f476856d0bbe86b5fa627))
    - move 'revision' one level up ([`c9c78e8`](https://github.com/Byron/gitoxide/commit/c9c78e86c387c09838404c90de420892f41f4356))
    - move 'remote' to 'free' ([`8967fcd`](https://github.com/Byron/gitoxide/commit/8967fcd009260c2d32881866244ba673894775f2))
    - move commitgraph to 'free' ([`f99c3b2`](https://github.com/Byron/gitoxide/commit/f99c3b29cea30f1cbbea7e5855abfec3de6ca630))
    - move index to 'free' ([`83585bd`](https://github.com/Byron/gitoxide/commit/83585bdfccdc42b5307255b2d56d8cb12d4136cb))
    - move 'pack' to 'free' ([`1cdecbc`](https://github.com/Byron/gitoxide/commit/1cdecbc583ae412e7f25cade73b46e00a182125f))
    - migrate mailmap to the new 'free' section ([`141c5f1`](https://github.com/Byron/gitoxide/commit/141c5f1145f9d3864e2d879089c66c62f38a2b5d))
    - first step towards moving all repository-commands one level up. ([`f4e1810`](https://github.com/Byron/gitoxide/commit/f4e1810fb711d57778be79c88f49aa583821abab))
    - make obvious what plumbing and porcelain really are ([`faaf791`](https://github.com/Byron/gitoxide/commit/faaf791cc960c37b180ddef9792dfabc7d106138))
    - adjustments due to breaking changes in `git_path` ([`4420ae9`](https://github.com/Byron/gitoxide/commit/4420ae932d5b20a9662a6d36353a27111b5cd672))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Add rough but working version of `rev-parse` ([`f3f176d`](https://github.com/Byron/gitoxide/commit/f3f176db42cef4036cc7c0ced1ee68f247424896))
    - basic infrastructure for delegate implementation ([`d3c0bc6`](https://github.com/Byron/gitoxide/commit/d3c0bc6e8d7764728f4e10500bb895152ccd0b0b))
    - Hookup explain command ([`1049b00`](https://github.com/Byron/gitoxide/commit/1049b00eaa261a67f060eaca4eb50dcda831eafd))
    - frame for `gix repo rev explain` ([`12e6277`](https://github.com/Byron/gitoxide/commit/12e6277a65a6572a0e43e8324d2d1dfb23d0bb40))
 * **Uncategorized**
    - Merge branch 'gix-repo-config' ([`afecb63`](https://github.com/Byron/gitoxide/commit/afecb6337dcf0fc51d5c74747c3c60fa06ae6346))
    - thanks clippy ([`48b3f4a`](https://github.com/Byron/gitoxide/commit/48b3f4a5077ba66d47482a80e505feb69e9ac9fc))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`0e9df36`](https://github.com/Byron/gitoxide/commit/0e9df364c4cddf006b1de18b8d167319b7cc1186))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - thanks clippy ([`056e8d2`](https://github.com/Byron/gitoxide/commit/056e8d26dc511fe7939ec87c62ef16aafd34fa9c))
    - thanks clippy ([`fdec111`](https://github.com/Byron/gitoxide/commit/fdec11135692b3503087b0a3245c12cc87554d67))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
</details>

## 0.12.0 (2022-04-05)

### New Features

 - <csr-id-7e99e6aeee9bf200a561d215c586301f5e4a8cbc/> Add `gix repo commit describe`
   It supports typical but basic flags mostly similar to the ones in git.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#298](https://github.com/Byron/gitoxide/issues/298)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use all tags by default, instead of requiring annotated tags ([`00c42ca`](https://github.com/Byron/gitoxide/commit/00c42ca36e93a22f233fc1d3f9a1afc241fd4464))
    - support for the --max-candidates flag ([`b9e6754`](https://github.com/Byron/gitoxide/commit/b9e67540801f2630be8aa1acbfddfec4202360ac))
    - Reduce amount of max candidates, add --debug flag ([`c8c13e3`](https://github.com/Byron/gitoxide/commit/c8c13e398671a21e96282547fc0e3bd445627e2f))
    - Add `gix repo commit describe` ([`7e99e6a`](https://github.com/Byron/gitoxide/commit/7e99e6aeee9bf200a561d215c586301f5e4a8cbc))
    - a first sketch of the `gix repo describe` plumbing command ([`2d6ccef`](https://github.com/Byron/gitoxide/commit/2d6ccefd5506d84ba14e3ff11c2af4cb107a386d))
</details>

## 0.11.0 (2022-04-03)

<csr-id-4d2d433e7e98ac42db858688edac06e68ee4d10d/>

Adapt to changes in `git-features` which change `Send + Sync` to `Send + Clone`. This happens to allow non-sync implementations (i.e. thread-local), along with `Sync` ones
which usually are `Clone` too as they are passed by immutable reference (which is `Clone + Copy`).

### Refactor (BREAKING)

 - <csr-id-4d2d433e7e98ac42db858688edac06e68ee4d10d/> Remove light* features, add 'lean-async' in its place; remove termion support

### Changed (BREAKING)

 - <csr-id-bf04644ab75ed1969507f957dc8d4868790d462d/> remove `Option<impl Progress>` in favor of `impl Progress`
 - <csr-id-d851bede97801096d188ff6af06c98a79fe276db/> remove unnecessary `Arc` around `should_interrupt` flag
 - <csr-id-c2679a03358b9c19d63ed1af1cd57324c6381447/> remove Sha1 mentions in `index::verify::Mode::*` variants
   The hash is repository defined and not hard-coded
 - <csr-id-51bf03feaa94bebb26690dff92262b2134070a44/> Remove lean plumbing CLI

### Bug Fixes

 - <csr-id-57ca0456cf02073099bfd403f9155290af756ecd/> Collect all stdout messages in line renderer as well
   Otherwise the threaded line renderer will interfere with genuine
   program output.

### New Features

 - <csr-id-384ed665c7423feca1b1ee1f81db10867fa813a8/> `gix mailmap verify` command
 - <csr-id-70109bee679d33a5c5fb3a78a708b479684b03b1/> `ein find --debug` to learn why it is slow
 - <csr-id-00909619ff04e247aabc9ffe3c025f0064c3092d/> --counting-threads flag to configure amount of threads when counting
   The efficiency of multi-threaded counting is low per core, and despite
   some speedups might be desirable, one might not want to commit all cores
   to this amount of waste.
 - <csr-id-25da30f3652bd72c157e84439dd6e3957471fa08/> in-manifest and in-bin documentation of feature toggles
   Unfortunately, these don't show up on docs.rs due to it being a abinary
   only crate. One could consider throwing in a lib just for good measure.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 63 commits contributed to the release over the course of 126 calendar days.
 - 165 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 12 unique issues were worked on: [#215](https://github.com/Byron/gitoxide/issues/215), [#263](https://github.com/Byron/gitoxide/issues/263), [#266](https://github.com/Byron/gitoxide/issues/266), [#279](https://github.com/Byron/gitoxide/issues/279), [#287](https://github.com/Byron/gitoxide/issues/287), [#289](https://github.com/Byron/gitoxide/issues/289), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#366](https://github.com/Byron/gitoxide/issues/366), [#67](https://github.com/Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#215](https://github.com/Byron/gitoxide/issues/215)**
    - Collect all stdout messages in line renderer as well ([`57ca045`](https://github.com/Byron/gitoxide/commit/57ca0456cf02073099bfd403f9155290af756ecd))
    - Fix compile warning ([`e4514a8`](https://github.com/Byron/gitoxide/commit/e4514a85d406aaa0aa959a18e0e32d46f1994cc8))
    - Remove reference of pretty-cli in code tree ([`4bd2f29`](https://github.com/Byron/gitoxide/commit/4bd2f29da7e37c5d6e920c97df82c7860dd9f22c))
    - Remove lean plumbing CLI ([`51bf03f`](https://github.com/Byron/gitoxide/commit/51bf03feaa94bebb26690dff92262b2134070a44))
    - Remove light* features, add 'lean-async' in its place; remove termion support ([`4d2d433`](https://github.com/Byron/gitoxide/commit/4d2d433e7e98ac42db858688edac06e68ee4d10d))
 * **[#263](https://github.com/Byron/gitoxide/issues/263)**
    - fmt ([`fbeddeb`](https://github.com/Byron/gitoxide/commit/fbeddebcab999f4898f768a3184906091f8ce0b8))
    - A mad attempt to use thread-local everywhere and avoid Sync… ([`0af5077`](https://github.com/Byron/gitoxide/commit/0af5077e1f028c1c69bbdc098bb567e486282c37))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - Provide handle with a snapshot of the store's state ([`6e0cd6d`](https://github.com/Byron/gitoxide/commit/6e0cd6d38c5df874990ace6c2c3c0b39342c4d05))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Fast-path multi-pack index verification in the CLI ([`bcde935`](https://github.com/Byron/gitoxide/commit/bcde935e7102ba5cd50c057a8323353247d3dd85))
    - Basic multi-pack index creation ([`89428b2`](https://github.com/Byron/gitoxide/commit/89428b2936fb0169606a543cf531bddaacb8187c))
    - 'index' with its own sub-commands ([`c4c5678`](https://github.com/Byron/gitoxide/commit/c4c56787b1f9165984a8bddf35cfee530554fa2f))
    - even nicer printing ([`d2bea27`](https://github.com/Byron/gitoxide/commit/d2bea270787597d6aef48ffe023ff49969c33bd9))
    - remove `Option<impl Progress>` in favor of `impl Progress` ([`bf04644`](https://github.com/Byron/gitoxide/commit/bf04644ab75ed1969507f957dc8d4868790d462d))
    - remove unnecessary `Arc` around `should_interrupt` flag ([`d851bed`](https://github.com/Byron/gitoxide/commit/d851bede97801096d188ff6af06c98a79fe276db))
    - remove Sha1 mentions in `index::verify::Mode::*` variants ([`c2679a0`](https://github.com/Byron/gitoxide/commit/c2679a03358b9c19d63ed1af1cd57324c6381447))
 * **[#287](https://github.com/Byron/gitoxide/issues/287)**
    - share and pass cli arguments for pack verification ([`db43e47`](https://github.com/Byron/gitoxide/commit/db43e47fc0a43ef45824ac1c9426c1889bdb13a3))
    - Very rough version of repository verification ([`80a4a7a`](https://github.com/Byron/gitoxide/commit/80a4a7add688d16376b9bf2ed7f1c7f655b7c912))
    - Adjustments to deal with changes to git-pack/git-odb ([`fcf8fde`](https://github.com/Byron/gitoxide/commit/fcf8fde7272974a70df808bd7ac03e925b7e39a8))
 * **[#289](https://github.com/Byron/gitoxide/issues/289)**
    - 'pack' with its own sub-commands ([`fb64af4`](https://github.com/Byron/gitoxide/commit/fb64af4d747960bfa40ec23051ecb03ea8ec5d83))
    - 'remote' with its own sub-commands ([`8677f7e`](https://github.com/Byron/gitoxide/commit/8677f7edd516ea54ec652a4a59cb220422036b90))
    - 'commitgraph' with its own sub-commands ([`db0251e`](https://github.com/Byron/gitoxide/commit/db0251e277ee9035bd3b44bf5ec9152fb64ac8c8))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - faster writing to stdout/stderr for plumbing commands ([`d04dc01`](https://github.com/Byron/gitoxide/commit/d04dc01115efa6688e71a2a0ef4ffce45d3d0db6))
    - Add 'index verify' subcommand to 'gix' ([`1ac2c21`](https://github.com/Byron/gitoxide/commit/1ac2c210c311c4b2ef835e04e2d7c477981b850f))
    - Flag to hide extension details ([`34ea001`](https://github.com/Byron/gitoxide/commit/34ea001fafa93b6453513cf458fe24327a13ff28))
    - Print basic index information, including the tree extension ([`9277cf8`](https://github.com/Byron/gitoxide/commit/9277cf877e1f2276dcad1efdeb97e0e3d96ed3f0))
    - Basic entry information ([`239e7b2`](https://github.com/Byron/gitoxide/commit/239e7b291297d6d49ebdf3d4986fb9fb86480e9a))
    - refactor ([`8bf585d`](https://github.com/Byron/gitoxide/commit/8bf585d67cd67b168d819ba05858cef7d9b90894))
    - JSON output for index entries ([`3fc1622`](https://github.com/Byron/gitoxide/commit/3fc1622488054c6ab655eb9d2f941b68cc3ccf18))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Simplify command-line options declaration ([`f790a55`](https://github.com/Byron/gitoxide/commit/f790a55ff4263bea9b9476137bac3824912044ac))
    - frame for printing index information ([`9ea98fd`](https://github.com/Byron/gitoxide/commit/9ea98fda75fbef339647a0ca03776060356d1206))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - greatly simplify render-line logic ([`a8fa53a`](https://github.com/Byron/gitoxide/commit/a8fa53a007780ec89f4768745b1549e8e73a8478))
    - pass thread-limit along to checkout ([`07e9081`](https://github.com/Byron/gitoxide/commit/07e9081fb5628e4ddc8f87e2d4ba0c7b3247bb35))
    - add thread-count and chunk-size computation; interrupt capability ([`8cbe85d`](https://github.com/Byron/gitoxide/commit/8cbe85d135898826a91939726465a9e295c1e24b))
    - a first sketch of access odb information using a sub-command ([`89b628a`](https://github.com/Byron/gitoxide/commit/89b628ab5b833a34f0b426b3a399bb182e63f3f4))
    - sub-command to print multi-index entries ([`6c10e09`](https://github.com/Byron/gitoxide/commit/6c10e097a432d81b930008abc00c6821ed7ac9be))
    - pack multi-index info subcommand ([`21c2dd5`](https://github.com/Byron/gitoxide/commit/21c2dd5da20a9e3cbae618b6311b6c9de12cf43c))
    - refactor ([`e6a3d43`](https://github.com/Byron/gitoxide/commit/e6a3d437e1a97c56fba18d80ac54928d953cb507))
    - detailed report about issues after checkout ([`613483b`](https://github.com/Byron/gitoxide/commit/613483b297b8a7e9a91cac3ef8205f2103ea946b))
    - keep-going support on the command-line ([`73a7393`](https://github.com/Byron/gitoxide/commit/73a73932f430fe991f26222ba2735332c03c0e77))
    - add tree-info subcommand to more easily test actual tree-traversal performance ([`29fb0c8`](https://github.com/Byron/gitoxide/commit/29fb0c8ff628716d33c9c41d3910e617791dcc77))
    - frame for traversing tree entries ([`0e55fbb`](https://github.com/Byron/gitoxide/commit/0e55fbb2fb0cec6f402b7a3aed7ee55078d233a1))
    - Properly use 'max-performance' feature toggle to get pack caches :D ([`a39d476`](https://github.com/Byron/gitoxide/commit/a39d4768e36f27aababefd5bd519e51f33ffa7b6))
    - allow writing empty files during checkout but also query the odb ([`5388d80`](https://github.com/Byron/gitoxide/commit/5388d8091ef02cf927478a1492847ae1666040d4))
    - support for repo to write actual objects ([`5494fb3`](https://github.com/Byron/gitoxide/commit/5494fb3e1de1234dde8c47336597283dbd8bcb29))
    - basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
    - support for unicode-precomposition for gix apps ([`e90c123`](https://github.com/Byron/gitoxide/commit/e90c123675a98ab62fc6bb22019f889cee8b7301))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - in-manifest and in-bin documentation of feature toggles ([`25da30f`](https://github.com/Byron/gitoxide/commit/25da30f3652bd72c157e84439dd6e3957471fa08))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - frame for printing mailmap entries using git-repository ([`2a01f47`](https://github.com/Byron/gitoxide/commit/2a01f4728ae858b47280b587501d343fdb86655d))
    - gix mailmap verify can now detect collisions ([`f89fe2f`](https://github.com/Byron/gitoxide/commit/f89fe2f867fa792db5d9e003ce342a337a6ac973))
    - `gix mailmap verify` command ([`384ed66`](https://github.com/Byron/gitoxide/commit/384ed665c7423feca1b1ee1f81db10867fa813a8))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - --counting-threads flag to configure amount of threads when counting ([`0090961`](https://github.com/Byron/gitoxide/commit/00909619ff04e247aabc9ffe3c025f0064c3092d))
 * **Uncategorized**
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - small build now uses the line renderer as well ([`652a0ac`](https://github.com/Byron/gitoxide/commit/652a0acdf9f06e35e65c1b66d264d5e8734ccc65))
    - Upgrade to prodash 19 ([`90c6c5a`](https://github.com/Byron/gitoxide/commit/90c6c5aec4015ff969d6e2514fa4d49873ee80f5))
    - `ein find --debug` to learn why it is slow ([`70109be`](https://github.com/Byron/gitoxide/commit/70109bee679d33a5c5fb3a78a708b479684b03b1))
    - fix clap warnings ([`aa51e05`](https://github.com/Byron/gitoxide/commit/aa51e05923695e20aecc16317331c7e26d49a2e7))
    - Merge branch 'index-verification' ([`ad3c803`](https://github.com/Byron/gitoxide/commit/ad3c8032cee02052ef3940d1d7c950270a0a299a))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - improve CLI docs ([`866530a`](https://github.com/Byron/gitoxide/commit/866530a154c3ef9383fae30c694991e31e97528c))
    - rename 'gix commitgraph' back to 'gix commit-graph' ([`d6a72e7`](https://github.com/Byron/gitoxide/commit/d6a72e70c9b4ee9b10a1172cce64ade5664599eb))
    - Merge branch 'use-midx-in-store' ([`338521b`](https://github.com/Byron/gitoxide/commit/338521b0443b9dc1007581de42ef6a950f6e0bbf))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - thanks clippy ([`b0f7328`](https://github.com/Byron/gitoxide/commit/b0f73280c0233e05b68a22b0b01f40d574786a03))
</details>

## v0.10.0 (2021-10-20)

This release pins beta versions of `clap` to avoid it to automatically fetch the latest one
during installation.

This is made possible due to `clap` itself pinning its dependency
to the `clap-derive` crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - upgrade to clap 3 beta 5 ([`2ddc4ed`](https://github.com/Byron/gitoxide/commit/2ddc4eddda23c77b5891a11a3e7215702c63882b))
</details>

## v0.9.0 (2021-10-15)

A first usable version of `git-repository` to make using `gitoxide` from your applications so much easier. It serves as a one-stop shop for application developers without sacrificing performance by default while making common use-cases more convenient.

### Feature list

* `git-repository` as hub crate for application development with focus on usability without sacrificing any knob to tune performance.
* opt-in `async` for `git-packetline`, `git-transport` and `git-protocol` for fully async git clients, along with the `light-async` feature toggle to build a `gix pack-receive` with an async client instead of a blocking one.
* Statistics for `gix pack-create` with the `-s/--statistics` flag to have data indicating the cost of the operation. Currently it's doing a lot of work that has to be avoided in order to be useable in production and the numbers underline that. Future iterations will cause key metrics to go down.
* Packs are now reproducible by default, which means that the same tip will always generate a pack with the same hash. This may be a desirable property for some kinds of packs, but not for others which is why it can be turned off for a considerable speed boost.
* `git-tempfile` crate
* `git-lock` crate
* `git-ref` crate with complete loose-ref, packed-ref and transaction support.


### Performance

* On M1, thanks to [a new release](https://github.com/RustCrypto/hashes/pull/289#event-5035369215), Sha1 is now computed much faster which unlocks a massive performance boost. In my test, verifying/decoding the entire linux kernel pack now happens in 17s, as compared to 37s for canonical `git`.
* `git-object` parsing is a few percent faster thanks a reworked error handling for objects. By default, error collection is disabled entirely making the error case zero-sized. If needed, verbose and stacked errors can be turned on using a feature toggle for applications who expect repositories with malformed objects and need detailed diagnostics.

### New Features

 - <csr-id-60c9fad8002b4e3f6b9607bba6361871752f4d3d/> control pack and object cache size in megabytes in some sub-commands

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 26 calendar days.
 - 35 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#200](https://github.com/Byron/gitoxide/issues/200), [#67](https://github.com/Byron/gitoxide/issues/67)

## v0.8.4 (2021-09-10)

This is a maintenance release.

## v0.8.3 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 8 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

## v0.8.1 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 42 commits contributed to the release over the course of 95 calendar days.
 - 98 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#83](https://github.com/Byron/gitoxide/issues/83)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release over the course of 128 calendar days.
 - 143 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.6.0 (2020-12-16)

Maintenance release without any new features.

These are created to account for breaking changes within the dependency graph of
`gitoxide` crates. Due to some blunders in the past the version on crates.io
could not be installed anymore.
This was eventually fixed with new minor releases across the ecosystem.

Finally, yet another breaking change due to the introduction of the `git-hash`
crate to break a dependency cycle between `git-object` and `git-features` caused
yet another maintenance release.

## v0.5.0 (2020-12-15)

Maintenance release without any new features.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 78 calendar days.
 - 84 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'commit-graph' into main ([`9cb09b2`](https://github.com/Byron/gitoxide/commit/9cb09b248796f0ff5c9d3f3e857de4731324cfd5))
    - Add lean-plumbing docs for path of commit-graph-verify ([`5c7b52d`](https://github.com/Byron/gitoxide/commit/5c7b52d658d5b86bf4cf05c724202e824016c0e2))
    - [commitgraph] Implement basic commit-graph file verification. ([`2571113`](https://github.com/Byron/gitoxide/commit/2571113fea516737acedac08d66632ead499b474))
    - [commitgraph] Stub out commit-graph-verify plumbing command. ([`aacf0f0`](https://github.com/Byron/gitoxide/commit/aacf0f05a909e5b7d9ffd5623ef9833e0465be93))
    - Merge branch 'main' into commit-graph ([`ca5b801`](https://github.com/Byron/gitoxide/commit/ca5b80174b73cc9ac162b3f33b5d3721ef936cb1))
</details>

## v0.4.3 (2020-09-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

## v0.4.1 (2020-09-18)

* fix installation via `cargo install`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - Finish removal of rust 2018 idioms ([`0d1699e`](https://github.com/Byron/gitoxide/commit/0d1699e0e0bc9052be0a74b1b3f3d3eeeec39e3e))
    - Provide terminal dimensions to better use horizontal space ([`11f6b84`](https://github.com/Byron/gitoxide/commit/11f6b8497a5089377e605f4cbe1cd317ef677d59))
</details>

## v0.4.0 (2020-09-12)

* add `remote-ref-list` and `pack-receive` subcommands to **gix**

### CLI Breaking

 * rename plumbing sub-command from `index-from-pack` to `pack-index-from-data`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 29 calendar days.
 - 30 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [clone] refs can now be written into a specified directory ([`fb1f048`](https://github.com/Byron/gitoxide/commit/fb1f04837be994fa5bcb9aa24f25b5f4f72e4ce0))
    - [clone] First version of writing references, but… ([`445be27`](https://github.com/Byron/gitoxide/commit/445be27cf81663ba4fe941c00262448444efbac2))
    - [clone] first journey test for pack-receive ([`46a3511`](https://github.com/Byron/gitoxide/commit/46a3511aead043bc45256ce603285ff4d0fff60a))
    - [clone] This actually works: first MVP of retrieving packs via clone ([`c06d819`](https://github.com/Byron/gitoxide/commit/c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d))
    - [ref-ls] add pretty version for ls-refs ([`487d06d`](https://github.com/Byron/gitoxide/commit/487d06d53b9cc201b5a009977e835b51f4b9f690))
    - [ref-ls] Fix progress display ([`2fcb557`](https://github.com/Byron/gitoxide/commit/2fcb557dce941eb94ca60f46ecee86b94e029db7))
    - refactor ([`b38290e`](https://github.com/Byron/gitoxide/commit/b38290e4a8fcabd758f26a15407710ab2abcdc07))
    - [ref-ls] refactor ([`35e26fc`](https://github.com/Byron/gitoxide/commit/35e26fc32978232aebda3468c9f172fb7b08b815))
    - refactor ([`f90b92f`](https://github.com/Byron/gitoxide/commit/f90b92ffc2994f594352abaf4bacd9767cbc2e6c))
    - [ref-ls] Frame for remote-ref-ls command in gitoxide-core ([`161e7df`](https://github.com/Byron/gitoxide/commit/161e7df34a53db40551879c6d2319ee775dfd551))
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive ([`5ea49c8`](https://github.com/Byron/gitoxide/commit/5ea49c8aa0d449bed98ce0147ad222ff25c27c32))
    - refactor ([`40a6412`](https://github.com/Byron/gitoxide/commit/40a64125dc5556630576ec2164b68838c76ccd79))
    - Less ambiguous name for 'index-from-pack': 'pack-index-from-data' ([`386673c`](https://github.com/Byron/gitoxide/commit/386673ccc99d18d023c7df3fcd40e86d71960b25))
    - refactor ([`b4a6e16`](https://github.com/Byron/gitoxide/commit/b4a6e16364822c0dccb56f98dbfb0ca4c8007069))
</details>

## v0.3.0 (2020-08-12)

* add `pack-explode` and `pack-index-from-data` sub-commands
* massive speed improvements for `pack-verify`

Many small and possibly breaking changes are not mentioned here.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 30 calendar days.
 - 31 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make obvious that interrupt request was received ([`34b2373`](https://github.com/Byron/gitoxide/commit/34b23737f560fe52d4f98fb886eba754652f9a5e))
    - make interrupt handler work reliably ([`e71da0f`](https://github.com/Byron/gitoxide/commit/e71da0fce6d6eab68f7b81b13cdc78ce8e9b7ee3))
    - unify used ranges for line renderer amond pretty and lean interface ([`f59f66e`](https://github.com/Byron/gitoxide/commit/f59f66e189732f567414f68c7463364e510f41c4))
    - Add percentage and throughput to tasks that matter ([`763d7ca`](https://github.com/Byron/gitoxide/commit/763d7caa4c70111b7cb3ef5733d2c3c697758c28))
    - Upgrade to latest iteration of prodash ([`3a4faec`](https://github.com/Byron/gitoxide/commit/3a4faecab56e37670c553e6563f11a46d740c333))
    - support for JSON format output ([`1931575`](https://github.com/Byron/gitoxide/commit/19315750f4f409e3f105c3c4054c4afbef91daad))
    - first pieces of the index-from-pack journey tests ([`181d69c`](https://github.com/Byron/gitoxide/commit/181d69c1da46a931c513cbd7d8bca7b2fa53351c))
    - Add versions back to main command, remove from sub-commands ([`e509373`](https://github.com/Byron/gitoxide/commit/e509373b26c9a7b120057fc6e75970568f328fc4))
    - ditch structopt in favor of clap 3.0 beta1 ([`d7591e2`](https://github.com/Byron/gitoxide/commit/d7591e24a5178732713286a5e28cbc90f5fe9ed9))
    - Move common flags to common plac ([`c0352c2`](https://github.com/Byron/gitoxide/commit/c0352c2643e2badde79778d4a22d2e392a44f0a3))
    - Write about user interfaces and the use/non-use of async ([`91ba045`](https://github.com/Byron/gitoxide/commit/91ba0457745f860b7a68cb38b13e69754747e8d9))
    - interrupt support for pretty plumbing ([`bca7ce2`](https://github.com/Byron/gitoxide/commit/bca7ce2e668a4be2600d2d04d00f46b21c82eee2))
    - Revert "Less memory for look up mode, faster start" - too slow ([`584350a`](https://github.com/Byron/gitoxide/commit/584350af91f533db4cf980327d530445384c6b5a))
    - Less memory for look up mode, faster start ([`395c7e7`](https://github.com/Byron/gitoxide/commit/395c7e78ef344ee56cf3d4ef49828942a09094bc))
    - remove memory mode entirely (and some complexity with it) ([`8812e91`](https://github.com/Byron/gitoxide/commit/8812e916a21983868a37c4aade10f79a1dc9b926))
    - turns out you never want to keep deltas in memory ([`657aa2c`](https://github.com/Byron/gitoxide/commit/657aa2c38673cf10174f42bcb97039ac37b2926e))
    - Remove support for keeping compressed memory to reduce the index size ([`1e2ec7e`](https://github.com/Byron/gitoxide/commit/1e2ec7e9d0ef2f2a4908860672080e411e945bff))
    - …but there seem to be issues with the kernel pack… ([`cc147bc`](https://github.com/Byron/gitoxide/commit/cc147bc60066c4ef31353a499958edadc960a9c4))
    - minor fixes after first local tests - it's up to twice as fast!! ([`43c7fd1`](https://github.com/Byron/gitoxide/commit/43c7fd1f81b9b4c938f99c0bf1deabdf121226b9))
    - quick and dirty impl of lean command-line for index-from-pack ([`9660bbf`](https://github.com/Byron/gitoxide/commit/9660bbffd8ace621178b067e22d227ef8c50ba84))
    - upgrade dependencies ([`44b8221`](https://github.com/Byron/gitoxide/commit/44b8221800454f9b651778a422186bd5061877f4))
    - remove invalid clap configuration ([`665696f`](https://github.com/Byron/gitoxide/commit/665696f636e152ad9969ea0ca004cb83f1641ae6))
    - prepare full 'verify' implementation ([`ee45c7f`](https://github.com/Byron/gitoxide/commit/ee45c7f47b95fc406cc5922a322c8fd6c0f52775))
    - refactor ([`0a33b24`](https://github.com/Byron/gitoxide/commit/0a33b24f5b61ccdf1358f1e9adcf0f6fd4099c1c))
    - Allow sink-compress configuration; choose best algorithm ([`29b9c23`](https://github.com/Byron/gitoxide/commit/29b9c230e35ba9b4334797b63ab9fa88c2fe59d0))
    - Nice error message on failure ([`adbc82c`](https://github.com/Byron/gitoxide/commit/adbc82c31450681fcb38233eeb8095efc5e52a18))
    - The first 'explode' implementation… ([`0d31ad1`](https://github.com/Byron/gitoxide/commit/0d31ad1b61997fa0d0692c5919fb8032ffaaa35b))
    - Get all pieces ready for action ([`1805d64`](https://github.com/Byron/gitoxide/commit/1805d64b9222d6a05a8718f04b29b789c1f42fea))
    - Pass option for safety checks down to explode(…) ([`0bcb790`](https://github.com/Byron/gitoxide/commit/0bcb790dc8c35097916876afbb68bbfcc894c369))
    - refactor ([`f66b116`](https://github.com/Byron/gitoxide/commit/f66b116ddfbee62c3e20a4c5e7cd878fbf064195))
    - basic tests and CLI args for explode pack ([`f932256`](https://github.com/Byron/gitoxide/commit/f932256a62d6fc5d5558446de079fb666ddc27da))
    - rename verify-pack to pack-verify (keeping it more formal) ([`ec8c48a`](https://github.com/Byron/gitoxide/commit/ec8c48a8fcbcd748c9c764734d881b5f0217e1e4))
    - refactor ([`d3c00c8`](https://github.com/Byron/gitoxide/commit/d3c00c841ee1aeda6bb0534fe365db13c31f8d3c))
    - Change bin names from 'gio' to 'gix' and 'gixp' ([`5e23137`](https://github.com/Byron/gitoxide/commit/5e231371432ad02c67b095448564b2aa6af76799))
    - Revert "Invert --statitics switch to become --no-statistics" ([`93a9b30`](https://github.com/Byron/gitoxide/commit/93a9b30069d9abc5742546ade90913026ac5774b))
    - Invert --statitics switch to become --no-statistics ([`aeb8778`](https://github.com/Byron/gitoxide/commit/aeb87789ecc5cf3fd0ac69d67c7d0785e4eb329c))
    - use faster algorithm by default ([`bb45c3d`](https://github.com/Byron/gitoxide/commit/bb45c3d8a2aabf87231981000240f0444abf6fc4))
    - Fix clippy ([`ec40e09`](https://github.com/Byron/gitoxide/commit/ec40e093d72f93d86168f39ebaca5b122ca0bec3))
    - Change course and do pack streaming first ([`bcb275e`](https://github.com/Byron/gitoxide/commit/bcb275e91cfd6f0a71b3cb59a2b706b60608a594))
    - get rid of annoying warnings - there is no better and easier way ([`41f38c4`](https://github.com/Byron/gitoxide/commit/41f38c442e086b1f3fb48eea25839ef6207f0cbc))
    - Fully implement --encode and --re-encode flags ([`a7cfac8`](https://github.com/Byron/gitoxide/commit/a7cfac83ddd859d9c2c25e457c0d7043738792dc))
    - prepare for re-encoding each pack object ([`afae684`](https://github.com/Byron/gitoxide/commit/afae684c72e5dc4b718976056dd5d34ed61de72a))
    - fix naming change, which was introduced accidentally ([`fbb9f98`](https://github.com/Byron/gitoxide/commit/fbb9f98508ec722e192466e28ded47aef2fb78b3))
    - refactor ([`2888f1b`](https://github.com/Byron/gitoxide/commit/2888f1b10a2baf40155544e667ddd461f3ddc938))
    - pass threadlimit down from CLIs ([`f98c5b1`](https://github.com/Byron/gitoxide/commit/f98c5b160db80a7cac530e18b9256562c25be47f))
    - add new Context argument to support more configuration options ([`7c5d8b8`](https://github.com/Byron/gitoxide/commit/7c5d8b8bb318e59a59ad74ad767a1525e2833632))
</details>

## v0.1.0 (2020-07-12)

* Initial release with `pack-verify`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 54 commits contributed to the release over the course of 765 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bring color back to 'max' versions ([`c68d9ab`](https://github.com/Byron/gitoxide/commit/c68d9ab6e9cccab3610a77a2c6839a26fb42de2d))
    - Support for disabling the cursor in the pretty line renderer ([`48c4bbd`](https://github.com/Byron/gitoxide/commit/48c4bbd35f678de801fa2243f7a8705e825bdbf9))
    - Allow TUI to terminate action properly ([`1f1b725`](https://github.com/Byron/gitoxide/commit/1f1b7257895a219b5623b1ba3beaee1282ff2b63))
    - refactor ([`cce71aa`](https://github.com/Byron/gitoxide/commit/cce71aadb53528cb2f1d173b81df939ad1df8083))
    - refactor ([`f276a05`](https://github.com/Byron/gitoxide/commit/f276a0561fcec78f802b166f09110becf22ea7ee))
    - preliminary support for line renderer in max version ([`4aa8022`](https://github.com/Byron/gitoxide/commit/4aa8022c248fc26292ea9f1c61e2e8dbadc56e7c))
    - unify frame rate across plumbing (and later potentially porcelain) ([`e2a7bdd`](https://github.com/Byron/gitoxide/commit/e2a7bdd9a549400f0e8d31b590f469660e458c89))
    - refactor ([`41e01a5`](https://github.com/Byron/gitoxide/commit/41e01a53a7f48dd3341ee287a243a33190bccea0))
    - Proper implementation of line renderer into 'lean' CLI ([`e98e7c2`](https://github.com/Byron/gitoxide/commit/e98e7c280d73e9d9ebd13202afb93a56cb2f7c9c))
    - Mild improvements to look of verbose log ([`5fff552`](https://github.com/Byron/gitoxide/commit/5fff5524c4443d9c9ae26307c19745c722334d0c))
    - first very basic version of line renderer progress - works… ([`0cc1bf2`](https://github.com/Byron/gitoxide/commit/0cc1bf25c69611f9512fec415ae8e09b608706fc))
    - prepare for optional addition of line renderer for lean version ([`aac0d34`](https://github.com/Byron/gitoxide/commit/aac0d341eb02f0dccdf740f7ef15e8f585907544))
    - upgrade to prodash version 7 ([`af02b46`](https://github.com/Byron/gitoxide/commit/af02b46cc1eff5ba1da7da20d3f524a79fad686f))
    - Make --version flags work as expected. ([`a4d978c`](https://github.com/Byron/gitoxide/commit/a4d978ccc11e73fd752055c9a28b3b23dea145ea))
    - Merge branch 'release' ([`a1a0b13`](https://github.com/Byron/gitoxide/commit/a1a0b135c991edfe5cddb71c5fbfbed25b47e3b3))
    - rename 'pretty' target into 'max', a better fit for what it is ([`5acecc5`](https://github.com/Byron/gitoxide/commit/5acecc59d2d39141f2e98b6f8556c6d457ab0965))
    - Make gio commands less cumbersome, self-document their build type (pretty, lean) ([`1f9bc03`](https://github.com/Byron/gitoxide/commit/1f9bc03dd773d90960a6f6d4ee59af3f938ad80b))
    - Allow to limit the logging depth for less cluttered output ([`fce7035`](https://github.com/Byron/gitoxide/commit/fce703531d7006f7d961d6ffa66f51f6c9bc0efc))
    - support for json in pretty-plumbing and gitoxide (on demand) ([`b3780f8`](https://github.com/Byron/gitoxide/commit/b3780f87438d34b372c48b7385199f7ea22b3965))
    - Simplify the 'keep open' logic of TUI progress window ([`13cd8ce`](https://github.com/Byron/gitoxide/commit/13cd8ce372800eb0016190960834c759c9744b9c))
    - attempt to implement progress with a mode enum ([`ac490c2`](https://github.com/Byron/gitoxide/commit/ac490c21b8f369c45ee0d7688ddb381ce6f4af94))
    - Allow for more screen space when formatting ([`6794300`](https://github.com/Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - assure pretty progress doesn't occlude the output ([`122d69f`](https://github.com/Byron/gitoxide/commit/122d69fee217eb264a335f0a056d03eba066332e))
    - fix pretty build ([`6adf615`](https://github.com/Byron/gitoxide/commit/6adf615ed7d6c488c25589940fc0a55bf0fb3d5c))
    - pass average stats through to the top level ([`5b4979c`](https://github.com/Byron/gitoxide/commit/5b4979c1dfeb9a29974dd4e6529ae5da074d0b1a))
    - refactor ([`7add82c`](https://github.com/Byron/gitoxide/commit/7add82c39169e3c2fff76c48cdd318fe6040d7bc))
    - Now ACTUALLY stop TUI when there is no progress anymore :D ([`3bf3321`](https://github.com/Byron/gitoxide/commit/3bf33210a96e1e3bc2a81782b339b5c67344ac34))
    - Automatically close the TUI when there is no progress anymore. ([`c416152`](https://github.com/Byron/gitoxide/commit/c416152b04051958de7bd161a8a2ee42ca163275))
    - Assure we wait for GUI thread to finish ([`60eaea0`](https://github.com/Byron/gitoxide/commit/60eaea0ee01214202ab9f23514dc45a9909d7888))
    - pretty progress in a generalized form ([`caa883b`](https://github.com/Byron/gitoxide/commit/caa883b96827deb63b5c8787ed820d22f2c85249))
    - neater progress log messages: don't show the module it originates from ([`026a0dd`](https://github.com/Byron/gitoxide/commit/026a0dd1faf28c4668f58bd1790c168a0134559f))
    - refactor ([`30925e6`](https://github.com/Byron/gitoxide/commit/30925e654144a05365908f7d2aa90deb7b2952d3))
    - support for logging in pretty binaries ([`67026e4`](https://github.com/Byron/gitoxide/commit/67026e479f0aa3e47ff3fd230c8741a7a5dbe99c))
    - --verbose flag for lean plumbing binary ([`aaf4825`](https://github.com/Byron/gitoxide/commit/aaf482584d1ee080c0a6c091c4675736c4c8d6a7))
    - first very basic progress implementation ([`b820717`](https://github.com/Byron/gitoxide/commit/b8207177daee8a9ffa23c7c052cf9ca651b15804))
    - Pass progress everywhere, for now just to discard it ([`da3ae1c`](https://github.com/Byron/gitoxide/commit/da3ae1c82cd726b8fae9b8d26069719930e9ba99))
    - split plumbing into separate binary ([`b1e51d6`](https://github.com/Byron/gitoxide/commit/b1e51d6a83ca7a00923b39209d0a2bfb3b78de0d))
    - refactor ([`0fbba9f`](https://github.com/Byron/gitoxide/commit/0fbba9fe7597af03912f956c251c88472b48c3eb))
    - refactor ([`ba6a8ef`](https://github.com/Byron/gitoxide/commit/ba6a8ef064a9884066414c82f4f7d1bb72ab524f))
    - add initial version of 'lean-cli' feature toggle, but… ([`f01c298`](https://github.com/Byron/gitoxide/commit/f01c2985732ac05b24a7fcbc3752ef52dd1bc438))
    - Support for verifying pack files and index files ([`b09b4e1`](https://github.com/Byron/gitoxide/commit/b09b4e1f35c3802dfd3418bda42b96828acd9ec8))
    - reorganize crates to make 'gitoxide' the CLI, and 'gitoxide-core' the library ([`0ac9c5a`](https://github.com/Byron/gitoxide/commit/0ac9c5af0cbb562d3cb48a661736afd98dd1a940))
    - Add simple pack verification to gio ([`8c0e0b5`](https://github.com/Byron/gitoxide/commit/8c0e0b5bb79c8c337eed03d37cbf818d8bb9c924))
    - goodbye git-core, hello git-repository ([`7cec2b6`](https://github.com/Byron/gitoxide/commit/7cec2b648f86fc665b4fc5bfe269e9ca16679a55))
    - document existing use of unsafe, deny everywhere else ([`41f4bce`](https://github.com/Byron/gitoxide/commit/41f4bce9d9a492f8e20a6eb5b3eaf5adc6d78329))
    - cargo clippy ([`1179ac1`](https://github.com/Byron/gitoxide/commit/1179ac16ea2bb84816f9b615d1191f8a2d4e775b))
    - move parsing tests close to actual parsing ([`3ca2c59`](https://github.com/Byron/gitoxide/commit/3ca2c592d91c9aa8fab8ed749871d6d96f2ef4e2))
    - color for all grit commands/subcommands ([`aa8efdd`](https://github.com/Byron/gitoxide/commit/aa8efdd922d45bdab668dc71e8b30adf79930667))
    - use structopt instead of clap ([`eb7388c`](https://github.com/Byron/gitoxide/commit/eb7388c5d51e4ef3ea928d5f8f9e5b218cdbbd57))
    - Remove failure from grit binary, too ([`417c34b`](https://github.com/Byron/gitoxide/commit/417c34b82469bcc3391706646dd39c7f6d1ad69c))
    - refactor ([`87c8a2e`](https://github.com/Byron/gitoxide/commit/87c8a2e288140b04e163fe85266d040d039ec69c))
    - cargo fmt ([`2aa0857`](https://github.com/Byron/gitoxide/commit/2aa085752aa3e99b51034a3dec882aea8c27ad94))
    - implement git-init ([`57737c2`](https://github.com/Byron/gitoxide/commit/57737c2c48ff898a327ba57712fea21b5d83188e))
    - Initial commit - based on standard project template ([`c3d319f`](https://github.com/Byron/gitoxide/commit/c3d319f2b3076a0bb169bcd8a7b6a011f6aba9a5))
</details>

