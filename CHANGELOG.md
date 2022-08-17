# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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

 - 15 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 5 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - Control which command is lenient or not. That way `gix-config` can be lenient. ([`6a9c58f`](https://github.com/Byron/gitoxide/commit/6a9c58fde7ca4a52fa1c3225974a2019e7d93168))
    - Invert behaviour to `open::Options::strict_config()`, with lenient being the default. ([`0235111`](https://github.com/Byron/gitoxide/commit/0235111a4fcc40c7b57d973bfce27a66eddea901))
    - `ein tools` to `ein tool` for as it's more intuitive ([`edf73dd`](https://github.com/Byron/gitoxide/commit/edf73dd4db5b0f5d9309c95bf366e11ea6723885))
    - add aliases to make revision sub-commands more accessible ([`a6d79e3`](https://github.com/Byron/gitoxide/commit/a6d79e38cb0dd7e87d00a098030bbcaa614f259d))
    - Merge branch 'write-index-files' into write-index-v2 ([`cddc2ca`](https://github.com/Byron/gitoxide/commit/cddc2ca06f63f66e887ff821452d1f56fb08fe6a))
    - Merge branch 'write-index-files' into rev-parse-delegate ([`370110d`](https://github.com/Byron/gitoxide/commit/370110d3356528af38150c2280ed505354ceca5b))
    - Merge branch 'main' into rev-parse-delegate ([`4ae2bed`](https://github.com/Byron/gitoxide/commit/4ae2bedfc25d1881d58ebdc54aca0936c68d4859))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.13.0 (2022-07-22)

### New Features

 - <csr-id-eda39ec7d736d49af1ad9e2ad775e4aa12b264b7/> `gix config` with section and sub-section filtering.
 - <csr-id-d99453ebeb970ed493be236def299d1e82b01f83/> `gix config` lists all entries of all configuration files git considers.
   Filters allow to narrow down the output.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 38 commits contributed to the release over the course of 101 calendar days.
 - 108 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - thanks clippy ([`48b3f4a`](https://github.com/Byron/gitoxide/commit/48b3f4a5077ba66d47482a80e505feb69e9ac9fc))
    - make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
    - thanks clippy ([`056e8d2`](https://github.com/Byron/gitoxide/commit/056e8d26dc511fe7939ec87c62ef16aafd34fa9c))
    - thanks clippy ([`fdec111`](https://github.com/Byron/gitoxide/commit/fdec11135692b3503087b0a3245c12cc87554d67))
</details>

## 0.12.0 (2022-04-05)

### New Features

 - <csr-id-7e99e6aeee9bf200a561d215c586301f5e4a8cbc/> Add `gix repo commit describe`
   It supports typical but basic flags mostly similar to the ones in git.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
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

 - 61 commits contributed to the release over the course of 126 calendar days.
 - 165 days passed between releases.
 - 10 commits where understood as [conventional](https://www.conventionalcommits.org).
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
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
    - fix clap warnings ([`aa51e05`](https://github.com/Byron/gitoxide/commit/aa51e05923695e20aecc16317331c7e26d49a2e7))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - improve CLI docs ([`866530a`](https://github.com/Byron/gitoxide/commit/866530a154c3ef9383fae30c694991e31e97528c))
    - rename 'gix commitgraph' back to 'gix commit-graph' ([`d6a72e7`](https://github.com/Byron/gitoxide/commit/d6a72e70c9b4ee9b10a1172cce64ade5664599eb))
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
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
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

 - 4 commits contributed to the release over the course of 26 calendar days.
 - 35 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#200](https://github.com/Byron/gitoxide/issues/200), [#67](https://github.com/Byron/gitoxide/issues/67)

## v0.8.4 (2021-09-10)

This is a maintenance release.

## v0.8.3 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 8 calendar days.
 - 20 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## v0.8.1 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 95 calendar days.
 - 98 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#83](https://github.com/Byron/gitoxide/issues/83)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 128 calendar days.
 - 143 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

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

 - 3 commits contributed to the release over the course of 65 calendar days.
 - 84 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add lean-plumbing docs for path of commit-graph-verify ([`5c7b52d`](https://github.com/Byron/gitoxide/commit/5c7b52d658d5b86bf4cf05c724202e824016c0e2))
    - [commitgraph] Implement basic commit-graph file verification. ([`2571113`](https://github.com/Byron/gitoxide/commit/2571113fea516737acedac08d66632ead499b474))
    - [commitgraph] Stub out commit-graph-verify plumbing command. ([`aacf0f0`](https://github.com/Byron/gitoxide/commit/aacf0f05a909e5b7d9ffd5623ef9833e0465be93))
</details>

## v0.4.3 (2020-09-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## v0.4.1 (2020-09-18)

* fix installation via `cargo install`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 6 calendar days.
 - 6 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
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
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

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
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

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

 - 53 commits contributed to the release over the course of 765 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

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

