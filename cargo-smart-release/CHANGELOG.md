# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

The `v` prefix is not enforced anymore and is handled depending on what's already present.

This helps to handle changelogs with slightly different styles as well.

### New Features

 - <csr-id-3613a95d730d0aeef87d9c256f93bd528d4945bb/> Support for lack of prefixes in version headers.
   
   These are also inherited so once set by a single versioned release
   section, fully generated sections will inherit their prefix from
   that one.

### Bug Fixes

 - <csr-id-9d0d1fd71196b129b229a7d9475fdd6b99e8675b/> Assume manifests cannot necessarily be read by `cargo_toml::Manifest` and fallback.

   This prevents errors to occur in some configurations when no crate is specified on the command-line.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#228](https://github.com/Byron/gitoxide/issues/228)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#228](https://github.com/Byron/gitoxide/issues/228)**
    - Flexible tag parsing allows to find any version tags ([`a1b12e6`](https://github.com/Byron/gitoxide/commit/a1b12e695c08e344becbfcddb6192e34c3cf8ae5))
    - Support for no prefixes in version headers ([`3613a95`](https://github.com/Byron/gitoxide/commit/3613a95d730d0aeef87d9c256f93bd528d4945bb))
    - Assume manifests cannot necessarily be read by `cargo_toml::Manifest` and fallback ([`9d0d1fd`](https://github.com/Byron/gitoxide/commit/9d0d1fd71196b129b229a7d9475fdd6b99e8675b))
</details>

## v0.5.4 (2021-10-20)

### Bug Fixes

 - <csr-id-77f433e806e43c8d355b3e176ed740ba4de9777c/> create github release only after tags were created and pushed

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#227](https://github.com/Byron/gitoxide/issues/227)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#227](https://github.com/Byron/gitoxide/issues/227)**
    - create github release only after tags were created and pushed ([`77f433e`](https://github.com/Byron/gitoxide/commit/77f433e806e43c8d355b3e176ed740ba4de9777c))
 * **Uncategorized**
    - Release cargo-smart-release v0.5.4 ([`447d689`](https://github.com/Byron/gitoxide/commit/447d689776a6eaebf00bbccb5f84e0906876d547))
</details>

## v0.5.3 (2021-10-20)

### Bug Fixes

 - <csr-id-a3aaa3e0fa38085530bc20443de176306fc8d5d2/> strip `.git` suffix from repository paths when using it in urls
 - <csr-id-53ee1a751e5d79aa3e325a5fd3c3a211fc3d06a1/> remove extra '/' after https://github.com/ based URLs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - fix smart-release journey test expecations ([`4b638ae`](https://github.com/Byron/gitoxide/commit/4b638ae72d070bb0d362f358f5eaad035db0e2ae))
 * **Uncategorized**
    - Release cargo-smart-release v0.5.3 ([`0953239`](https://github.com/Byron/gitoxide/commit/0953239faebccfce05dc7fef3bf07c43340b3e7f))
    - strip `.git` suffix from repository paths when using it in urls ([`a3aaa3e`](https://github.com/Byron/gitoxide/commit/a3aaa3e0fa38085530bc20443de176306fc8d5d2))
    - remove extra '/' after https://github.com/ based URLs ([`53ee1a7`](https://github.com/Byron/gitoxide/commit/53ee1a751e5d79aa3e325a5fd3c3a211fc3d06a1))
</details>

## v0.5.2 (2021-10-19)

Releases will be more atomic and it will try hard to complete all pending operations even in the light
of failure. Now GitHub releases will be created right after a publish succeeded, and tags will be pushed
for all successful publishes.

### New Features

 - <csr-id-db3cb11c466fff57f3f272d7269dc95a636e1c1f/> Add `-d` short flag for `--allow-dirty` in `changelog`

### Bug Fixes

 - <csr-id-8c3ca9cf58c44af627fc9b3c4138891635b1c554/> Push all available tags even if an error occurred.
   
   That way, tags don't remain unpushed despite having been created
   successfully, just because one crate later in the publishing
   process fails.
 - <csr-id-b769c47079a16042ef592a0199cb2d0f6afeeb5e/> Create GitHub release right after publishing succeeds.
   
   This is more atomic and prevents loosing all github releases if one
   publish fails later on.
 - <csr-id-ae8570050a313457bb2fd6659e31f34fd29bc325/> `src/` dir of root packages is only used if there is multiple workspace members.
   
   Otherwise one part of the dependency resolver might have concluded that there are changes, while another part would not have.
   The outcome would have been the same, but the messaging around it would have been different unnecessarily.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - adjust changelog ([`6ce09b7`](https://github.com/Byron/gitoxide/commit/6ce09b7b73c42f8c58f27a2460829d2de387d25a))
    - Add `-d` short flag for `--allow-dirty` in `changelog` ([`db3cb11`](https://github.com/Byron/gitoxide/commit/db3cb11c466fff57f3f272d7269dc95a636e1c1f))
    - adjust changelog ([`2db6d88`](https://github.com/Byron/gitoxide/commit/2db6d88f390d6577c8660b9da00f94a4a3943ebd))
    - push all available tags even if an error occurred ([`8c3ca9c`](https://github.com/Byron/gitoxide/commit/8c3ca9cf58c44af627fc9b3c4138891635b1c554))
    - create GitHub release right after publishing succeeds ([`b769c47`](https://github.com/Byron/gitoxide/commit/b769c47079a16042ef592a0199cb2d0f6afeeb5e))
    - src/ dir of root packages is only used if there is multiple workspace members ([`ae85700`](https://github.com/Byron/gitoxide/commit/ae8570050a313457bb2fd6659e31f34fd29bc325))
 * **Uncategorized**
    - Release cargo-smart-release v0.5.2 ([`69b7142`](https://github.com/Byron/gitoxide/commit/69b714256346f7e459aa70100ac8a261d5403c86))
</details>

## v0.5.1 (2021-10-19)

This release contains an important bugfix which may have caused panics when the root-package didn't have changes.

### New Features

 - <csr-id-ed8abfdac40f5c8b17981b8a990572f6f07c8862/> `changelog` subcommand fails if there is nothing to do

### Bug Fixes

 - <csr-id-ce68733379a8ab4644c849ba1571bc7063962c64/> Fix panic due to unexpected internal state.
   
   When there was no change in the src/ directory of the top-level crate,
   the dependency resolution would not be able to auto-bump the version
   as no change occurred, but another part would usually detect a change
   as it wasn't confined to the top-level src/ directory.
   
   This could lead to a panic as an invariant wasn't upheld.
   
   This was fixed by letting both parts agree to use the src/ directory
   to determine changes of the top-level directory, and by making panics
   impossible while improving the messaging around this state should it
   still occur. The latter is rough, probably rare, but usable.
 - <csr-id-6ee4f5d20c832a54ca5d841773d93f0927a16f25/> Correct the reporting of manifest changes.
   
   Previously even unchanged crates would trigger workspace crates
   to be recorded for manifest changes.
   
   Now only crates that are to receive manifest changes will be triggering
   this.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 3 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - adjust changelog for release, now definitely ([`7133f5d`](https://github.com/Byron/gitoxide/commit/7133f5d7738e70b7bdf9ce033f3f9a0485dc844f))
    - correct reporting of manifest changes ([`6ee4f5d`](https://github.com/Byron/gitoxide/commit/6ee4f5d20c832a54ca5d841773d93f0927a16f25))
    - adjust changelog for smart-release release ([`210b09a`](https://github.com/Byron/gitoxide/commit/210b09ae63be853fb233547e8173e7176ca9a4d0))
    - `changelog` subcommand fails if there is nothing to do ([`ed8abfd`](https://github.com/Byron/gitoxide/commit/ed8abfdac40f5c8b17981b8a990572f6f07c8862))
    - panic due to unexpected internal state ([`ce68733`](https://github.com/Byron/gitoxide/commit/ce68733379a8ab4644c849ba1571bc7063962c64))
    - crude fix to avoid version related invariants to kick in during dependency resolution ([`3cdebf5`](https://github.com/Byron/gitoxide/commit/3cdebf5c34845ecef195ce762e344dbff7c1b035))
 * **Uncategorized**
    - Release cargo-smart-release v0.5.1 ([`31a1481`](https://github.com/Byron/gitoxide/commit/31a148153c4c9faa320de60af2a55cfb2131c797))
</details>

## v0.5.0 (2021-10-19)

<csr-id-07372dd045de88f283d35d8f3dcc4c079dce88e9/>

A release with breaking changes as the dependency engine was reworked to handle even more cases
and make future improvements easier.

### New Features

 - <csr-id-6d4edfa3b2d2c6700e0956716a575831b940cb50/> Respect `publish=false` in cargo manifest
 - <csr-id-7648bf3c7554352bec8e1355f9b593d891b2b17f/> Perform safety bumps without forcing a publish.
   
   This is what's required to assure that future publishes of such
   transitively dependent crates won't cause downstream breakage the next time the tool is run.
 - <csr-id-b806a9c982da1e5ff42c268e430c67363f3a7918/> Inform about safety bumps more explicitly,
   and generally greatly improve the way the course of action is described.

### Bug Fixes

 - <csr-id-501c1d102c0e5e4635120bb1aa857e97a2b537b4/> Dependency resolution.
   
   Previously the ordering of crates for release might not have been
   correct due to this issue that is now fixed.
   
   We need depth-first traversals and previously it would extend skipped
   dependencies, effectively putting them into their own ordering.
   
   Previously it would restore that ordering, but not anymore, causing
   this bug that was entirely unnecessary.
 - <csr-id-5e98e5559707cf308e2cd64494fe73a99f9e9c8e/> `--no-changelog` during smart-release is now actually working
   
   Previously the flag had no effect and changelogs would always be
   generated, possibly stopping the release as at least one of them
   needed manual work.
 - <csr-id-dfc588b25ede3faa578eb8e131e73c857117a6df/> Pin version of clap to beta 5.
   
   This assures we don't get broken automatically in future.
   Previously that wasn't possible as the dependency of `clap`,
   `clap-derive` was also using a beta verion and wasn't constrained,
   hence it would be updated and cause breaking changes with pinned
   versions of consumers of `clap`.

### Changed (BREAKING)

<csr-id-2f87196217a6e685dc447b4af091842926aed6d0/>

 - <csr-id-59302ae24db791988c22322c2c1ad72e2918f89a/> `changelog` subcommand inverts `--dependencies` to `--no-dependencies`
 - Remove `--no-multi-crate-release` support entirely
  
   As the default is to do multi-crate releases and now having to deal
   with single-create releases reduces maintenance burden.

   The solution to this problem is to not specify version constraints in
   dev-dependencies to workspace crates.

   We also don't check for this anymore, which might be re-added
   at some point if there is demand.This makes dependency resolution similar to cargo smart-release by default and is less surprising.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 82 commits contributed to the release over the course of 3 calendar days.
 - 10 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#221](https://github.com/Byron/gitoxide/issues/221), [#222](https://github.com/Byron/gitoxide/issues/222), [#224](https://github.com/Byron/gitoxide/issues/224)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Add yet another video ([`dc8f7ca`](https://github.com/Byron/gitoxide/commit/dc8f7ca20424e31ef5621cab2cc802f2d2b1686a))
    - Update Asciinema link in readme ([`b56a31e`](https://github.com/Byron/gitoxide/commit/b56a31e69c625ca41c923c14e411da9ee71428e7))
 * **[#221](https://github.com/Byron/gitoxide/issues/221)**
    - Add tests which indicate the problem: safety-bump not applied to auto-publishes… ([`32e1f1a`](https://github.com/Byron/gitoxide/commit/32e1f1aa1d97b061e07878ae94f23ec99f56d64d))
    - --no-changelog-preview isn't needed anymore in dry-run mode ([`1b6a4ad`](https://github.com/Byron/gitoxide/commit/1b6a4adecbb884ad973d5b1e2cbc420b163ad390))
    - refactor ([`aff053f`](https://github.com/Byron/gitoxide/commit/aff053f0e2b1e3f5920d3db5a44292a0dc3ac708))
    - Inform about safety bumps more explicitly ([`b806a9c`](https://github.com/Byron/gitoxide/commit/b806a9c982da1e5ff42c268e430c67363f3a7918))
    - refactor ([`23073e8`](https://github.com/Byron/gitoxide/commit/23073e88c8d083d064cd5b79800c063a9fdc949f))
 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - refactor ([`72bda30`](https://github.com/Byron/gitoxide/commit/72bda306c00642571cd22d909775b4b7cc2c85b6))
    - refactor ([`c8c0dcd`](https://github.com/Byron/gitoxide/commit/c8c0dcd167556e7c45baf8eff34e4aeff8e28379))
    - refactor ([`f7a8847`](https://github.com/Byron/gitoxide/commit/f7a8847c8afb5504cf9bfd779a6a99f7c996e05d))
    - fix merging of dependency graphs for multiple crates ([`d332cdf`](https://github.com/Byron/gitoxide/commit/d332cdf9532add5e34c1ade16d4b775ba56171e5))
    - Revert "FAIL: try to assure that breaking changes are always published in correct order" ([`f25e7c7`](https://github.com/Byron/gitoxide/commit/f25e7c706d340c02d851ce2b5fe06ef43a0ce95c))
    - try to assure that breaking changes are always published in correct order ([`3519f9a`](https://github.com/Byron/gitoxide/commit/3519f9a1f4002232aec752dadf7d3737bd97ce3d))
    - update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - Respect user selection when re-adding crates for manifest change ([`72d16bf`](https://github.com/Byron/gitoxide/commit/72d16bf935bccf0faff9274156ce399a72540e73))
    - dependency resolution ([`501c1d1`](https://github.com/Byron/gitoxide/commit/501c1d102c0e5e4635120bb1aa857e97a2b537b4))
    - --no-changelog during smart-release is now actually working ([`5e98e55`](https://github.com/Byron/gitoxide/commit/5e98e5559707cf308e2cd64494fe73a99f9e9c8e))
    - replace TODO with runtime logging ([`f457e65`](https://github.com/Byron/gitoxide/commit/f457e659623ea2e14ca6ab0678b22329ef7a7763))
    - unify presentation even more ([`7c32409`](https://github.com/Byron/gitoxide/commit/7c32409e49b21a6b0e3017357e0fe1755faaa467))
    - adjust expectations in smart-release journey tests ([`1f96a72`](https://github.com/Byron/gitoxide/commit/1f96a7215b78bfeb56074b1894bb6bbc8b598011))
    - group skipped items by skipped reason ([`ba28746`](https://github.com/Byron/gitoxide/commit/ba287464731bb15785930183290cecdd9694e458))
    - unify reporting style ([`99be2e1`](https://github.com/Byron/gitoxide/commit/99be2e16cba34a613d41fd2e46cf3576a511ee1c))
    - fix reporting of skipped crates, consider adjustment ([`ac91016`](https://github.com/Byron/gitoxide/commit/ac91016841348476f1c1f32c2d1121359986e9f6))
    - Abort if not a single provided crate would need publishing ([`478c4ea`](https://github.com/Byron/gitoxide/commit/478c4eaa3ff60f0c83933581a3d0170429a95381))
    - improved reporting of skipped/refused crates; abort operation if there is nothing to publish ([`f9358f1`](https://github.com/Byron/gitoxide/commit/f9358f124726d69dc11e6103d649c5cab30c738b))
    - better reporting of crates that where refused to be published ([`1d7142a`](https://github.com/Byron/gitoxide/commit/1d7142a861636f088694500855a1f7acbcdbeded))
    - 'changelog' subcommand change --dependencies to --no-dependencies ([`59302ae`](https://github.com/Byron/gitoxide/commit/59302ae24db791988c22322c2c1ad72e2918f89a))
    - Properly resolve breaking propagation through the graph ([`4f25236`](https://github.com/Byron/gitoxide/commit/4f252365147aae2f8a9f12a0ae6087adc0ed4644))
    - multi-round discovery of breaking changes from published packages ([`dc93e1a`](https://github.com/Byron/gitoxide/commit/dc93e1a828c6cd97fcb64aa92293cb8f3899316a))
    - Verify and partially fix journey tests ([`e53a7f6`](https://github.com/Byron/gitoxide/commit/e53a7f6b4d67da52ac7fee7706dfd067b67e0275))
    - remove all now unused items ([`40f2da2`](https://github.com/Byron/gitoxide/commit/40f2da20395213b48d4a8517cf2b63513f901e93))
    - use Dependency in manifest editor ([`d5c905a`](https://github.com/Byron/gitoxide/commit/d5c905ab4132626eb1af2a8e5410440f8fdc7a8f))
    - upgrade to clap 3 beta 5 ([`2ddc4ed`](https://github.com/Byron/gitoxide/commit/2ddc4eddda23c77b5891a11a3e7215702c63882b))
    - Show only changelogs that would be published ([`e20f498`](https://github.com/Byron/gitoxide/commit/e20f498b0d07d39a47b36a454c384068404119ad))
    - refactor ([`244431f`](https://github.com/Byron/gitoxide/commit/244431fbb12de811feb8f53e0faaeb9d683aa834))
    - Fix reporting of skipped crates ([`a305232`](https://github.com/Byron/gitoxide/commit/a305232bc1f027d65ef3d7dc7898931745cf652c))
    - Respect publish=false in cargo manifest ([`6d4edfa`](https://github.com/Byron/gitoxide/commit/6d4edfa3b2d2c6700e0956716a575831b940cb50))
    - more consistent reporting of what would be done ([`47ce4b0`](https://github.com/Byron/gitoxide/commit/47ce4b0a6a6545be6cd8b3928289478694a2f764))
    - refactor ([`369fa93`](https://github.com/Byron/gitoxide/commit/369fa93a16ed9af3ea0b70c08c8426759bdc7d11))
    - don't try to change crates that are already at the correct version ([`561aac3`](https://github.com/Byron/gitoxide/commit/561aac30a709974fb48fc02cb5d21828d7df1e54))
    - keep ordering of causes for breaking changes when printing ([`f4a0970`](https://github.com/Byron/gitoxide/commit/f4a0970ba0d0a4175972c6f231eceba1ff1c33fb))
    - better safety bumps to be more concise ([`7c8335a`](https://github.com/Byron/gitoxide/commit/7c8335a5f0b0168997f8a08d4da942e9d60e71d4))
    - Perform safety bumps without forcing a publish ([`7648bf3`](https://github.com/Byron/gitoxide/commit/7648bf3c7554352bec8e1355f9b593d891b2b17f))
    - refactor ([`ebec001`](https://github.com/Byron/gitoxide/commit/ebec001a2ca6f1faa17f27847ea274146506e9ce))
    - inform about the crates seeing a mnifest update too; only show fully-skipped crates ([`7f2a927`](https://github.com/Byron/gitoxide/commit/7f2a927eb0d880c58f5b9eed59e3a9640adf5c95))
    - fix:! breaking changes cause intermediate (otherwise skipped) crates to be published. ([`fb6b909`](https://github.com/Byron/gitoxide/commit/fb6b909e49d8428e53da6e2ce3c2f878025e00f7))
    - reverse-bumping for safety works, including publishing :) ([`5e1713c`](https://github.com/Byron/gitoxide/commit/5e1713c4bf0772d23678a28ff281cc36a77b5991))
    - track root-cause as well ([`7f8e720`](https://github.com/Byron/gitoxide/commit/7f8e720283416d101c0bbea545bbd504cc3f7204))
    - sketch backwards search for lifting crates to be published ([`0b018c0`](https://github.com/Byron/gitoxide/commit/0b018c0decf2d45eb58a5eaf3704d62c46b0a72c))
    - Realize that the search can't be 'flat' ([`13db698`](https://github.com/Byron/gitoxide/commit/13db6985159d24c3e6806a70120f17c81999803b))
    - start sketching backward traversal… ([`de1d7f7`](https://github.com/Byron/gitoxide/commit/de1d7f7242ddc6d357dc165532f1336a239b472b))
    - sumarize manifest updates rather than spelling out each one ([`8cf00e0`](https://github.com/Byron/gitoxide/commit/8cf00e06f1017fff1c328afe4a97428d56c1cca6))
    - update test expectations and formulate 'the algorithm' ([`c0693ae`](https://github.com/Byron/gitoxide/commit/c0693aebb3bc4306124be7287a54c1c1f1a31a65))
    - refactor ([`0bfb1b1`](https://github.com/Byron/gitoxide/commit/0bfb1b17ff7fc25aed1ad108fa407b56f35c7274))
    - assure changelog picks up safety bumps as well ([`f2a497b`](https://github.com/Byron/gitoxide/commit/f2a497b3eebecd0ca94801ac656d4fc2852505f2))
    - Collect crates for manifest updates ([`56ccdd2`](https://github.com/Byron/gitoxide/commit/56ccdd2195802a920fa084f85eae797e2cf17da7))
    - Remove --no-multi-crate-release support entirely ([`07372dd`](https://github.com/Byron/gitoxide/commit/07372dd045de88f283d35d8f3dcc4c079dce88e9))
    - remove performance measurements ([`37bacee`](https://github.com/Byron/gitoxide/commit/37bacee619fadf9dc1ff645a85c4e340cb84a569))
    - refactor ([`ac85cdf`](https://github.com/Byron/gitoxide/commit/ac85cdfccd0545b7da6276f0df086b32a7a9dfc0))
    - no newlines in gh traces ([`afd9b9e`](https://github.com/Byron/gitoxide/commit/afd9b9ebffa5db09b0ed69b29c878ccfd156a528))
    - refactor ([`03c7dba`](https://github.com/Byron/gitoxide/commit/03c7dbabff14bd5dd150bd5174f53148d4ee0fec))
    - Simplify use of 'verbose' flag by using log::trace! as well ([`4dc5f4b`](https://github.com/Byron/gitoxide/commit/4dc5f4b5e54a35f2794bb61ebc4c00758447bf75))
    - refactor ([`e256949`](https://github.com/Byron/gitoxide/commit/e256949f4a679ff74bece435b302490998f1cc6e))
    - refactor ([`e4ffa71`](https://github.com/Byron/gitoxide/commit/e4ffa71161d949cd134bc5289963ed7533607def))
    - try to represent safety-bump versions ([`9f3001f`](https://github.com/Byron/gitoxide/commit/9f3001f3300b5ceb350b157f541a30bf54a51549))
    - refactor ([`6f84e0b`](https://github.com/Byron/gitoxide/commit/6f84e0b6e7da2fce4ef7c4f43a6c5a67f4930e0d))
    - Simple version bumping logic based on what currently exists, with printout ([`81e5785`](https://github.com/Byron/gitoxide/commit/81e5785fca30c6cb71c962132ddcd573ba950d72))
    - fully data-driven presentation of dependency tracking results… ([`fd53e22`](https://github.com/Byron/gitoxide/commit/fd53e22a2f975010fd4ca6a95513339bc1102740))
    - refactor ([`51a5d36`](https://github.com/Byron/gitoxide/commit/51a5d365f71bf44ab60ece4511d8dce1a77f5442))
    - refactor ([`b8a5fc8`](https://github.com/Byron/gitoxide/commit/b8a5fc8bbe1dc58813c8c86cf0ad0dcdd5bff8ad))
    - refactor ([`10aa1eb`](https://github.com/Byron/gitoxide/commit/10aa1eb344fdc42528717f240b2446be60da360b))
    - refactor ([`cfec54d`](https://github.com/Byron/gitoxide/commit/cfec54d02d7df8fbc1c7cec5459ea267e7f3f236))
    - Remove `--only` alias and invert `--no-dependencies` to `--dependencies` ([`2f87196`](https://github.com/Byron/gitoxide/commit/2f87196217a6e685dc447b4af091842926aed6d0))
    - Keep track of skipped crate names for future use ([`f0a04c7`](https://github.com/Byron/gitoxide/commit/f0a04c7148729bf9c322692610c501b78c9557a9))
 * **[#224](https://github.com/Byron/gitoxide/issues/224)**
    - pin version of clap to beta 5 ([`dfc588b`](https://github.com/Byron/gitoxide/commit/dfc588b25ede3faa578eb8e131e73c857117a6df))
 * **Uncategorized**
    - Release cargo-smart-release v0.5.0 ([`c03e8cb`](https://github.com/Byron/gitoxide/commit/c03e8cb31d61401450564bef9cd18d6638c681b7))
    - changelog update ([`7fcd02e`](https://github.com/Byron/gitoxide/commit/7fcd02e3baf49bc498a702ed87511d42f2e71f05))
    - Adjusting changelogs prior to release of cargo-smart-release v0.5.0 ([`11f55d3`](https://github.com/Byron/gitoxide/commit/11f55d36b2db19dc9e43c7fbed5d3fb4a8cdc9e1))
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
    - thanks clippy ([`7496ba3`](https://github.com/Byron/gitoxide/commit/7496ba38ef815d4f4cb6b78bdead5226fb48f2b6))
    - thanks clippy ([`b717323`](https://github.com/Byron/gitoxide/commit/b7173235ef4b118d96a0989f671424e06910ef46))
    - thanks clippy ([`c4efd9d`](https://github.com/Byron/gitoxide/commit/c4efd9dc1be51049b38c0dd2a3263437ca51fee0))
</details>

## v0.4.0 (2021-10-15)

<csr-id-3c0a6389fe5ff981dadca20e8a4a4a0d2ef66e13/>
<csr-id-77ed17c703e502e132cda9a94eb8c63db0b627ad/>
<csr-id-1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7/>
<csr-id-ae8780e08303946412cedc19ea4d2679be49ec97/>
<csr-id-509550f8aa8210f3688c78167a56a21fc1817515/>
<csr-id-11b64fce4630371633b6415f227eecdc6b42b20b/>
<csr-id-0ebfeb614264ca06ab763189e55e6c016c9997af/>
<csr-id-80b8331092f4856f52afa1d85fa375ae688bdd28/>
<csr-id-e59f901f47fb0180211494a1591aed62b856406a/>
<csr-id-19fc134d2a34f2ea84b2cc8fbd15ca55c55df35e/>
<csr-id-e668bf23ddba9a676a885f1f401d2d2885784eef/>

This major release adds **changelog** support to automatically generate scaffolding to be filled in by hand. The feature is driven by
[conventional commit](https://www.conventionalcommits.org) messages which are used sparingly to mark important changes only.
Furthermore, it will deduce the require version bump, i.e. patch, minor or major, automatically by looking at the commit history
and interpreting _'conventional commit'_ messages. This means that from time to time one would sprinkle in a specifically formatted
commit message to leave enough information to determine the correct release version and create changelog scaffolding.

If you have 10 minutes, the following video gives the whirlwind tour through the new features (_and note that all issues discovered there
have been fixed :)_).

[![12 minute introduction video](https://img.youtube.com/vi/EOft_uMDVYE/0.jpg)](https://www.youtube.com/watch?v=EOft_uMDVYE)

If you have 30 minutes, there is also [a long version of the video](https://youtu.be/a4CzzxJ7ecE).

And there is another one showing `cargo smart-release` releasing `gitoxide 0.9.0`, along with some explanation on how it works. 

[![8 minute video releasing gitoxide](https://img.youtube.com/vi/ZS9fwPDYLpI/0.jpg)](https://www.youtube.com/watch?v=ZS9fwPDYLpI)

### Changelog Support in `cargo smart-release`

When using `cargo smart-release` in dry-run mode (_default_), additional information regarding changelog will be printed.
This informs you a release would be attempted, or if manual adjustments to the changelogs would be required, for example as
they are fully generated with statistical information only.

If there is no issue with the initial changelogs, passing the `--execute` flag will write the changelogs after
providing them to you for preview (using `bat`) for a last chance to abort the operation. Otherwise the publishing
will proceed, which includes the creation of tag objects containing the relevant section of the changelog, along with
a GitHub release which is annotated with the same section (_only if the repository is hosted on GitHub_).

If there are issues to be corrected, there will be suggestions to run `cargo changelog --write --only <crate-name>`
one by one, or the release operation will have left a single commit with all changelogs written out.
In any case, it's recommended to re-write the changelog after editing to assure it is indeed stable and won't change each time
the generator is run.

For more information, run `cargo smart-release -h`.

### The `cargo changelog` Sub-Command

This new sub-command sports the same dependency resolution as `smart-release` itself, operates in dry-run mode by default
to preview changelogs that would be written. Use the `--write` flag to actually write changes to disk.

It's primary use is to conveniently generate changelogs from time to time to add the final polish by hand before
actually releasing them along with the crate with `smart-release`.

For more information, run `cargo changelog -h`.

### Other BREAKING Changes

- renamed `--skip-*` flags to `--no-*` for consistency
- rename `--skip-dependencies` to `--no-dependencies` to be more inline with existing terminology of other flags.
- rename short name for `--execute` to `-e` from `-n` for consistency

### Other Changes

 - <csr-id-e668bf23ddba9a676a885f1f401d2d2885784eef/> `--no-dependencies` now has `--only` as alias

### Bug Fixes

 - <csr-id-11eebdcc572a72b2e66a9db3cae0a01f12a81619/> Previously it might have been possible to see that it won't use a 'new' crate version as it's already in the manifest, _even_ if these are the same. This is now fixed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 281 commits contributed to the release over the course of 36 calendar days.
 - 19 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#192](https://github.com/Byron/gitoxide/issues/192), [#197](https://github.com/Byron/gitoxide/issues/197), [#198](https://github.com/Byron/gitoxide/issues/198), [#200](https://github.com/Byron/gitoxide/issues/200), [#213](https://github.com/Byron/gitoxide/issues/213), [#67](https://github.com/Byron/gitoxide/issues/67)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 27 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#192](https://github.com/Byron/gitoxide/issues/192)**
    - smart-release: assure the current package version is actually breaking ([`fb750b6`](https://github.com/Byron/gitoxide/commit/fb750b65ca64c894ffb79cd0049f10a8db255ab6))
    - smart-release: better verbosity handling when comparing to crates-index ([`f6f2d1b`](https://github.com/Byron/gitoxide/commit/f6f2d1b2c1c50d36ee046ed58ffffed0444cd25a))
    - smart-release(feat): turn off safety bump with its own flag ([`a040f7d`](https://github.com/Byron/gitoxide/commit/a040f7d882eb5f6db0d54ba7e32437da3579a075))
    - smart-release(refactor): ([`443f000`](https://github.com/Byron/gitoxide/commit/443f000015de2117eae08fedf7d23f0d1ac6abff))
 * **[#197](https://github.com/Byron/gitoxide/issues/197)**
    - smart-release: improved safety bump log message ([`9b78c34`](https://github.com/Byron/gitoxide/commit/9b78c344ee287c4c2908ccbe64bd64c2c9648459))
    - smart-release: commit message reveals safety bumps ([`b1a3904`](https://github.com/Byron/gitoxide/commit/b1a39046056bf4a862cebe69f44f3ea1e53a2069))
    - smart-release: released crates only receive minor bumps… ([`ecf38b8`](https://github.com/Byron/gitoxide/commit/ecf38b8c013e46a33aa0b2c1b4e9cf547c8393c4))
    - smart-release(chore): update changelog ([`342b443`](https://github.com/Byron/gitoxide/commit/342b443a4f49736a10c2b311d69841dbf581ceec))
    - smart-release(test): way more tests to nail current log output ([`0d30094`](https://github.com/Byron/gitoxide/commit/0d30094f4d397f932288f8c04ffd01f956113dc8))
    - smart-release: dependency upgrade works ([`a56bd7b`](https://github.com/Byron/gitoxide/commit/a56bd7b134d315e22e5c8d01ca2d927de75955a9))
    - smart-release: calculate new version of dependent ([`c50704a`](https://github.com/Byron/gitoxide/commit/c50704a0595884c3fb20629aba0f22bf99893cbf))
    - smart-release(fix): don't claim "conservative" updates for major version change ([`681d743`](https://github.com/Byron/gitoxide/commit/681d743e5579197d7262c40237dda0116fc4af1c))
    - smart-release: assure we can find non-sequential connections ([`798b650`](https://github.com/Byron/gitoxide/commit/798b650ad848001b10018087ed6c5d8a4055ece8))
    - smart-release: all logic to calculate dependent version bumps ([`7ca029c`](https://github.com/Byron/gitoxide/commit/7ca029c73eee51302d6828c6f9e8862d3fd4fbd4))
    - smart-release: an algorithm to collect dependencies by 'growing' ([`73794a4`](https://github.com/Byron/gitoxide/commit/73794a4e382404cb7b684c9054278fb4ff8a84ce))
    - smart-release: foundation for bumping versions ([`d1145d1`](https://github.com/Byron/gitoxide/commit/d1145d1a6219ddafa7a41c82d6149b289f033640))
 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Polish README a little more ([`455c45d`](https://github.com/Byron/gitoxide/commit/455c45d9a534805cf9659b9c33c3995673e8709f))
    - First version of updated README ([`45dcc68`](https://github.com/Byron/gitoxide/commit/45dcc684e16017cb0289cff209fd1d436fa50c2c))
    - Finish changelog ([`e341b22`](https://github.com/Byron/gitoxide/commit/e341b221086cb75a24053da61ed90aed166538cd))
    - Enforce an empty line after user sections ([`79f0093`](https://github.com/Byron/gitoxide/commit/79f00933f4bbf24551fc093e33e8d94ff8365eb6))
    - Respect release-level removed-id list even when inserting sections ([`2970fff`](https://github.com/Byron/gitoxide/commit/2970fffc681657d0ab393b4c20d9be20675d808d))
    - rename short name for `--execute` to `-e` from `-n` for consistency ([`19fc134`](https://github.com/Byron/gitoxide/commit/19fc134d2a34f2ea84b2cc8fbd15ca55c55df35e))
    - `--no-dependencies` now has `--only` as alias ([`e668bf2`](https://github.com/Byron/gitoxide/commit/e668bf23ddba9a676a885f1f401d2d2885784eef))
    - Write more of the smart-release changelog to learn --no-dependencies needs an alias ([`65468c8`](https://github.com/Byron/gitoxide/commit/65468c88c241914847a91a563663c60b8931ef9f))
    - Show how many more changelogs are going to be previewed… ([`94a6788`](https://github.com/Byron/gitoxide/commit/94a678843edb7b0da98f2227745900f5c89b9b56))
    - Start writing the 0.4 changelog ([`5f18bc9`](https://github.com/Byron/gitoxide/commit/5f18bc96147a48226be957de2c996f14ba55f1bc))
    - Only use src/ directory for top-level crate change tracking… ([`f26b581`](https://github.com/Byron/gitoxide/commit/f26b58143491300c3375a815f3ffaa1a7ea2bcea))
    - refactor ([`78c4ad5`](https://github.com/Byron/gitoxide/commit/78c4ad5d05a9bd02131238be4d503080cade8924))
    - Don't show previews in dry-run mode; provide help on how to fix this before release ([`cdb8db4`](https://github.com/Byron/gitoxide/commit/cdb8db412fad2063f78f0e4c677a3bb429c0fd76))
    - Fix naughty issue that isn't even reproducible… ([`95cb79c`](https://github.com/Byron/gitoxide/commit/95cb79cc7927c886080c0e7fef540e173eb51c3e))
    - Correctly parse back single-word conventional messages ([`bfa0777`](https://github.com/Byron/gitoxide/commit/bfa0777719303e732c8a3314f1652bc3a33f6bc0))
    - Fix logic to determine if breaking changes are already handled by package version ([`cb06e9d`](https://github.com/Byron/gitoxide/commit/cb06e9d74b2afd648ec81b1b279a2a416253579d))
    - greatly simplify dry-run preview for clear visuals ([`2990028`](https://github.com/Byron/gitoxide/commit/2990028a7812790654293d0958713391018e15d3))
    - Update expectations for log messages ([`494e8e5`](https://github.com/Byron/gitoxide/commit/494e8e51210bb5b392b507c6826953bae34d9f31))
    - Use correct title for github release to match name of tag ([`90f39ad`](https://github.com/Byron/gitoxide/commit/90f39ad693e0998bc3307bf553fccdc37c8dc0c8))
    - Fix logic to determine if links should be used… ([`dcc5c1c`](https://github.com/Byron/gitoxide/commit/dcc5c1c7d8a635869da73b58dd579636f69e06ff))
    - Fix logic to determine if there are conventional headlines to fix - ignore non-breaking ([`f80b7fc`](https://github.com/Byron/gitoxide/commit/f80b7fc9ac7d85c52376d539f21ba9ecbe06d560))
    - Fix commit subject line when release would stop due changelog ([`2cdc852`](https://github.com/Byron/gitoxide/commit/2cdc85223b30c93e73fb73f2f14c9961140d4d02))
    - Fix github release invocation ([`6f0fdbf`](https://github.com/Byron/gitoxide/commit/6f0fdbfaf8bae53bd75adeac81d17bc124468bb0))
    - less surprising location of the 'prepare release' message ([`0dd739b`](https://github.com/Byron/gitoxide/commit/0dd739b58b04e74090bbc2917c610498e788e5fb))
    - Much better preview titles ([`b70f815`](https://github.com/Byron/gitoxide/commit/b70f81540ed69386f50e8876bd0913764b85c7ac))
    - Use --file-name flag to shorten displayed path ([`6e6dcda`](https://github.com/Byron/gitoxide/commit/6e6dcda283dc56bd2c0d4342da1c2cb222cc05ce))
    - Fix crate name and version for --version flag ([`4cc0213`](https://github.com/Byron/gitoxide/commit/4cc0213ac728e1c27a1d7c9a4167645e8bd8ebe7))
    - clap second pass with arg headlines and better help messages ([`624076f`](https://github.com/Byron/gitoxide/commit/624076f4de0e0ad3483a5c0dec8a49c6d4210f43))
    - First pass of using clap instead of argh ([`836837c`](https://github.com/Byron/gitoxide/commit/836837c53337c1c5f52804e33bfae93dab5f0bd3))
    - Use fmt::Display instead of io::Write when creating markdown docs… ([`fb946b6`](https://github.com/Byron/gitoxide/commit/fb946b6d879e54244886079b3158456d611bec65))
    - even cleaner release text, just with detail tags… ([`52a6cc8`](https://github.com/Byron/gitoxide/commit/52a6cc81e3152b1805ecc3422fc479c300d8dc05))
    - less verbose gh tool logging in dry-run mode ([`75ebf0b`](https://github.com/Byron/gitoxide/commit/75ebf0bb35ee5757497964a0736dd3769bb34026))
    - try to do github releases for multi-crate releases, too ([`552ae4f`](https://github.com/Byron/gitoxide/commit/552ae4f4e1aff192c767fe8ba4ad83b159c8ae63))
    - improve commit message titles and simplify tag-name logic ([`4aa68bd`](https://github.com/Byron/gitoxide/commit/4aa68bdeac7f863a5e7ee9c833a1aa691bf13f4c))
    - refactor ([`a6d3bb1`](https://github.com/Byron/gitoxide/commit/a6d3bb168096f1174e45a4bc544429c045859aa2))
    - First sketch of running gh tool to create releases ([`bf7f020`](https://github.com/Byron/gitoxide/commit/bf7f02075b664ab6477fbe7e791b23c90a9ef7e8))
    - support for ssh->https github urls; more robustness in general ([`ab7ea71`](https://github.com/Byron/gitoxide/commit/ab7ea7170f987991952da0c1c062513062f0891f))
    - Add flag to allow disabling github releases ([`5f6c4de`](https://github.com/Byron/gitoxide/commit/5f6c4de7b09250d24f447571c47c80e1b8afabe7))
    - sketch incorporation of github CLI support ([`5aa6ef9`](https://github.com/Byron/gitoxide/commit/5aa6ef9483498a18ee5aa548b7c29df7f228d3fb))
    - git::remote_url() is now optional ([`e16603b`](https://github.com/Byron/gitoxide/commit/e16603b15b5488b81563c583cd8f5292ab9d24a2))
    - Inform about the difference between tag objects and references in verbose logs ([`98a9f10`](https://github.com/Byron/gitoxide/commit/98a9f10fa0a544ea27f9dd98eeb008470f1616df))
    - rename `ObjectAccessExt::tag(…)` to `*::tag_reference(…)`, add `easy::Object::try_to_tag()` ([`e59f901`](https://github.com/Byron/gitoxide/commit/e59f901f47fb0180211494a1591aed62b856406a))
    - add easy::ext::ObjectAccessExt::tag(…) to create tag objects ([`80b8331`](https://github.com/Byron/gitoxide/commit/80b8331092f4856f52afa1d85fa375ae688bdd28))
    - Allow to skip writing section titles and html tags ([`7b29406`](https://github.com/Byron/gitoxide/commit/7b29406d1b5814956a8474aa187d1e60e5eddf38))
    - Allow to turn off changelog links ([`b33e737`](https://github.com/Byron/gitoxide/commit/b33e7375509a74762c43f03ffc74e33b69c8f800))
    - pass release section text to function soon creating a tag object ([`a4ac96c`](https://github.com/Byron/gitoxide/commit/a4ac96c6ca834b91e5311f89f6cd35acb3f85f54))
    - precise change tracking for changelogs ([`d038c06`](https://github.com/Byron/gitoxide/commit/d038c0673f3ee48446aa5fade071766ce23c5c6a))
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - less verbose changelog and smart-release sub-commands related to changelogs ([`c096805`](https://github.com/Byron/gitoxide/commit/c09680524a8c07b09f8bf421381ce93b1ae4610b))
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - Handle changelogs with upcoming version section if they were left for editing ([`0f5f47d`](https://github.com/Byron/gitoxide/commit/0f5f47da4662b596cbbbd9c0d83e135e2cc52c11))
    - refactor ([`6d30e2c`](https://github.com/Byron/gitoxide/commit/6d30e2c7e20ce1572afbebeee232d0c138a38462))
    - Automatically stop releases if changelogs are fully generated, and a flag to disable that ([`7161340`](https://github.com/Byron/gitoxide/commit/7161340ba7c4f2802e1a87cb02268d0adea8c0f8))
    - Check for changelog sections which are purely generated and warn about those ([`a9b8321`](https://github.com/Byron/gitoxide/commit/a9b83214cf425ec8853dacfbc96cba65e2005373))
    - See how it deals with major versions and auto-bumping in journey tests ([`b450bf3`](https://github.com/Byron/gitoxide/commit/b450bf3fb26fc399b405fc45972820d50281cef3))
    - more consistent log messages pertaining crate names ([`b32d8d6`](https://github.com/Byron/gitoxide/commit/b32d8d63841fed8c95436b9ae611aef9c11291cf))
    - first working version of version auto-bumping based on changelog ([`5ca7b1d`](https://github.com/Byron/gitoxide/commit/5ca7b1d1d703387d2e690a5a32a4033d87742217))
    - issue links for category headlines ([`425d3df`](https://github.com/Byron/gitoxide/commit/425d3dfc114e62db16c8c16c0b3e7c6b4a2a3ae4))
    - prepare for arrival of 'auto' bump mode ([`306035c`](https://github.com/Byron/gitoxide/commit/306035cf68dcc29466e736081ca8cdd3a5f57134))
    - Fix git-url re-export to respect feature flags ([`ec4e3ca`](https://github.com/Byron/gitoxide/commit/ec4e3ca4c7211655549a76cae252742633da1083))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - link up github issue ids in statistics ([`661cd39`](https://github.com/Byron/gitoxide/commit/661cd3928002ef2f288d7410b37a046a6f0ea21b))
    - format links for commit ids ([`9426db5`](https://github.com/Byron/gitoxide/commit/9426db53537162d58a65648f3f3a3a3b65f621dc))
    - pass actual repository url down from commands ([`4e03515`](https://github.com/Byron/gitoxide/commit/4e03515622afd79b145db081ef9e3cb301ce6e97))
    - Make `git_url::Url` available under `git_repository::Url` ([`0ebfeb6`](https://github.com/Byron/gitoxide/commit/0ebfeb614264ca06ab763189e55e6c016c9997af))
    - Foundation for rendering links if needed ([`ba4ce96`](https://github.com/Byron/gitoxide/commit/ba4ce96e32676b2aed529330ee526da2fc2f6d49))
    - Rename title for "Fixed" to "Bug Fixes" ([`e766b01`](https://github.com/Byron/gitoxide/commit/e766b01c73813dd80c72e13e43c5acdda741521a))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Also parse 'style' if there are breaking changes ([`bc9d85a`](https://github.com/Byron/gitoxide/commit/bc9d85a15d94a54dd2dbc67f20f1ffdbdf2b4789))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Support writing whole bodies in conventional messages… ([`c1f3c9d`](https://github.com/Byron/gitoxide/commit/c1f3c9d2bd5a8e123ac9b376c257e3d5630876a0))
    - Support for paragraphs in conventional items ([`7f52823`](https://github.com/Byron/gitoxide/commit/7f528239089788f4dd1f75a85bee1d0492285d60))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Only write headlines that we can parse back… ([`d44369a`](https://github.com/Byron/gitoxide/commit/d44369ab5d849720dda9a9c0edc1ba1a3c1a78b5))
    - handle all possible changelog headlines and add roundtrip tests ([`fda5ccf`](https://github.com/Byron/gitoxide/commit/fda5ccfcb224f9dcbb79be501a2ef639a906a493))
    - First basic parsing of conventional user and generated messages ([`56cd4ac`](https://github.com/Byron/gitoxide/commit/56cd4ac11a25710db889a8038d9ba8eb902b544b))
    - parsing of removed conventional messages from changelogs ([`c593252`](https://github.com/Byron/gitoxide/commit/c5932522178af3e2b1c22eb6e5f0b3a282f12f07))
    - first basic merging of conventional messages… ([`9af3248`](https://github.com/Byron/gitoxide/commit/9af3248b9402a4e1cf63cbb03ac53ab3d7fbf015))
    - Trivially emulate gits way of handling commit dates… ([`f58b30a`](https://github.com/Byron/gitoxide/commit/f58b30a78078f222f0db8b70d2c98c83af59c1a0))
    - Also consider changes of changelogs themselves… ([`8a2042c`](https://github.com/Byron/gitoxide/commit/8a2042cd2aa8aa212e456587187ab33ed0f70e3e))
    - Adjust date of upcoming version as well ([`fab4649`](https://github.com/Byron/gitoxide/commit/fab4649f3319fac2cc61cf2deba1e150f85206b0))
    - assure git-conventional is treated like user generated content for statistics ([`1fd5acb`](https://github.com/Byron/gitoxide/commit/1fd5acbcbcc038fc28cdfa529c3a108cbe22f706))
    - merge doesn't consider user generated sections, only the ones it would want to add ([`ebbebdd`](https://github.com/Byron/gitoxide/commit/ebbebdd70aeec9aa3ad453d61375429a7f555bbc))
    - Quick and dirty writing of conventional messages… ([`adfbd0d`](https://github.com/Byron/gitoxide/commit/adfbd0d812718868063a5d3142e02b026e3cf2fc))
    - basic generation of git-conventional information ([`77b0feb`](https://github.com/Byron/gitoxide/commit/77b0feb954232d34e4618e502f22a59dda7e6a2d))
    - Sketch out data structure for git-conventional segments ([`2713c02`](https://github.com/Byron/gitoxide/commit/2713c022317a72cd3af60698e380d370093ea499))
    - refactor ([`bcdec5e`](https://github.com/Byron/gitoxide/commit/bcdec5e62f8e5b6e97b8ead9e2d9abc0a61779b3))
    - smart-release with --changelog-without option… ([`ae8780e`](https://github.com/Byron/gitoxide/commit/ae8780e08303946412cedc19ea4d2679be49ec97))
    - changelog command learns the --without <section> option ([`509550f`](https://github.com/Byron/gitoxide/commit/509550f8aa8210f3688c78167a56a21fc1817515))
    - Easy removal of statistical sections, by just removing them… ([`91efd68`](https://github.com/Byron/gitoxide/commit/91efd68aea84dcd22569c429f22e06c5fc7f8f6e))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - reorder headlines according to version ordering… ([`2ff0c20`](https://github.com/Byron/gitoxide/commit/2ff0c2078d12f6d17862a6f64bbec19fcc227be8))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Use most relevant parent tree for change comparison… ([`5b9dd14`](https://github.com/Byron/gitoxide/commit/5b9dd148289d6c82dff5f74d8ebf7fabafc0c463))
    - Use hashmap based lookup for trees… ([`48a0c76`](https://github.com/Byron/gitoxide/commit/48a0c76ab163b6e35b19dd2a9efc2e101a721633))
    - refactor and improve path filtering to find relevant commits… ([`01b2466`](https://github.com/Byron/gitoxide/commit/01b246644c76d842892a8dfcf8392026baf288d8))
    - The first headline level controls all the other ones ([`715ea54`](https://github.com/Byron/gitoxide/commit/715ea54624a2651a4828ccd8cd035889495212b8))
    - adapt to git-hash refactor ([`925d668`](https://github.com/Byron/gitoxide/commit/925d6685df58a4a1135e426a70c370280f2ac142))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Only use short hashes for logs, without detecting ambiguity for now ([`772525c`](https://github.com/Byron/gitoxide/commit/772525c8b46136654e907b5b6362792806e6a897))
    - boost allowed package sizes… ([`1b21d71`](https://github.com/Byron/gitoxide/commit/1b21d71b9cb28ded42b6c2fb2c6b7e3c134b281e))
    - Stable smart-release journey tests… ([`fc79188`](https://github.com/Byron/gitoxide/commit/fc791887f4286411d33db676ebb0ee35591557a4))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Put commit details to the end of generated segments ([`054d207`](https://github.com/Byron/gitoxide/commit/054d207ae40452ae024693162a4586c63b489df0))
    - Use message commit id instead of body… ([`9b46f32`](https://github.com/Byron/gitoxide/commit/9b46f3212a62e96bbbdaa4d0af443c73f5d657ee))
    - fix md formatting on github ([`262c000`](https://github.com/Byron/gitoxide/commit/262c00095a7eb16c2c6e9990e9247d1e9ef9bb1d))
    - create details headline based on log message ([`04bbcbb`](https://github.com/Byron/gitoxide/commit/04bbcbb9109abe2e0715cdb5446d9fd2231fc9a5))
    - Add details behind a fold, but… ([`3360b2e`](https://github.com/Byron/gitoxide/commit/3360b2e2740e265ee46fd1b9a28596de5ebb8a2e))
    - Use the notion of 'changes after merge' only to drive previews… ([`634267c`](https://github.com/Byron/gitoxide/commit/634267cad2f3243b58603df224dc2831c45cd5fc))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - refactor ([`7a83103`](https://github.com/Byron/gitoxide/commit/7a83103e632be4fff50391caa8aff5237bc4baca))
    - Also provide a duration in days for preparing a release as part of statistics ([`bd12cac`](https://github.com/Byron/gitoxide/commit/bd12cac57898951eea0846e193839ccdbd41da89))
    - Fix tests ([`6c98afc`](https://github.com/Byron/gitoxide/commit/6c98afc351fca32d4f056a2f398328676c4c8163))
    - refactor ([`65fa0a4`](https://github.com/Byron/gitoxide/commit/65fa0a49f20b0895083e06c738dc68baa932dd7d))
    - More commit statistics ([`0840e7e`](https://github.com/Byron/gitoxide/commit/0840e7e67e107aea0b5c8a6e8efcdb584990875e))
    - Basic commit statistics with round-trip, more actual information to come ([`6d097ae`](https://github.com/Byron/gitoxide/commit/6d097ae29d2c3afd8a23a81d58712ebecf89b563))
    - refactor… ([`ce0dda2`](https://github.com/Byron/gitoxide/commit/ce0dda259d61725898190de6769e1577d32068d4))
    - More robust parsing of read-only sections ([`a3954f4`](https://github.com/Byron/gitoxide/commit/a3954f4949695e3fdb910ea6bc94ae4eca7e25de))
    - treat clippy as generated statistical section… ([`1cff425`](https://github.com/Byron/gitoxide/commit/1cff425d5797c181a8c3709d241381091b14e487))
    - Add new section type and write it out: clippy ([`6fca2ac`](https://github.com/Byron/gitoxide/commit/6fca2ac8421f300e64429de6cf4581168d8db409))
    - introduce notion of essential sections in a changelog… ([`be891e2`](https://github.com/Byron/gitoxide/commit/be891e20cb0152af52ceec47400cf3401e2112fb))
    - Preview changelog support for smart-release as well ([`b9e6de1`](https://github.com/Byron/gitoxide/commit/b9e6de124eab5e961c1effe797a5e54e23228284))
    - Detect changes after merge; add flag for controlling changelog preview ([`6beb734`](https://github.com/Byron/gitoxide/commit/6beb7345f0329592081c2955cf7ad2c9adf0e68a))
    - A lot of logic to handle messaging around changelog generation and halting… ([`28f6e18`](https://github.com/Byron/gitoxide/commit/28f6e181ff0e14e52704544bc6ed5f41bd7fb234))
    - Unconditional changelog creation in smart-release ([`48b5228`](https://github.com/Byron/gitoxide/commit/48b52281f789a93415fefe38d661228ab582a107))
    - rename --skip-* flags to --no-* for consistency ([`3c0a638`](https://github.com/Byron/gitoxide/commit/3c0a6389fe5ff981dadca20e8a4a4a0d2ef66e13))
    - fix windows tests by transforming line endings ([`e276d77`](https://github.com/Byron/gitoxide/commit/e276d777eb7a88dc424badbf88a929b5f567e5de))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Implement --write actually ([`69d36ff`](https://github.com/Byron/gitoxide/commit/69d36ffbeea68259add2d8e15a9eb74137b14156))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`2f43a54`](https://github.com/Byron/gitoxide/commit/2f43a54298e7ecfff2334627df149fe0882b5d1d))
    - a test case showing that headlines are currently ignored, and links too ([`2a57b75`](https://github.com/Byron/gitoxide/commit/2a57b755c5513544987be74b3b4b65d35e7718c9))
    - don't try to run tests in binaries that have none… ([`d453fe5`](https://github.com/Byron/gitoxide/commit/d453fe5086f819e590af78bba1083659fcc44c01))
    - It's already getting there, even though a few parts are completely missing ([`ee4aa08`](https://github.com/Byron/gitoxide/commit/ee4aa08fc0ed4bd06c7a987b2a9c86425400d68a))
    - only parse into 'unknown' catch all in special cases… ([`c0296c4`](https://github.com/Byron/gitoxide/commit/c0296c4d28016044f7d82afeba10971a526eca36))
    - first basic parsing of unknown parts as segments in sections ([`f265982`](https://github.com/Byron/gitoxide/commit/f265982a58600b68674f8552252e1ea156fe163d))
    - quick and dirty switch to getting access to a range of parsed input… ([`f5902f2`](https://github.com/Byron/gitoxide/commit/f5902f2fa9a6b876497278c9c62a91e58de1c31f))
    - setup test for old method of parsing unknown text… ([`996c39d`](https://github.com/Byron/gitoxide/commit/996c39d002d1781fd7193dabe958af6045936411))
    - refactor tests: unit to integration level ([`4326322`](https://github.com/Byron/gitoxide/commit/43263226420c0bd9db5d4920f5ce2f76c5367aa8))
    - Don't add a date to unreleased versions ([`ba4d024`](https://github.com/Byron/gitoxide/commit/ba4d02404e0a00c1b0d1553032c8062806d09b84))
    - Actually integrated generated changelog with existing ones… ([`aa095e2`](https://github.com/Byron/gitoxide/commit/aa095e2447fff350492c38600c7303d38ae38824))
    - inform about 'bat's  absence ([`c82c5bc`](https://github.com/Byron/gitoxide/commit/c82c5bc682f6b4cc53b5965e3a124a826933718f))
    - rename --no-bat to --no-preview… ([`1087dd8`](https://github.com/Byron/gitoxide/commit/1087dd81ce869de9c886379766a962ec30c93e36))
    - basic merging now works ([`6c6c200`](https://github.com/Byron/gitoxide/commit/6c6c20002cf7632e8fed11b83a1e2f69b669d907))
    - sketch for finding insertion points and merging sections ([`2a49033`](https://github.com/Byron/gitoxide/commit/2a4903348f6179f6939e6b87d3477e5643acb7b7))
    - Sketch merging logic… ([`1932e2c`](https://github.com/Byron/gitoxide/commit/1932e2ca848db57f3907b93e804553524dfa27ac))
    - prepare test for basic merging… ([`0a14ced`](https://github.com/Byron/gitoxide/commit/0a14cedbd68058ac296e34a84ab1fe1083a0bf5e))
    - nicer 'thanks clippy' message ([`4344216`](https://github.com/Byron/gitoxide/commit/43442162aa22f561a33cab78936514d05d8214a0))
    - Show with simple example how the round-tripping works, neat ([`9510d9b`](https://github.com/Byron/gitoxide/commit/9510d9bd2c3b2d5cffe32485d7bc3fff374343ee))
    - collect unknown text so things don't get lost entirely… ([`60040c9`](https://github.com/Byron/gitoxide/commit/60040c9301e6468c72a0c52095c0b86f8b3041f5))
    - parse back what we write out, perfectly… ([`5cab315`](https://github.com/Byron/gitoxide/commit/5cab315b0f28d9b9f6f6b4e037d053fb501fdfaa))
    - fix journey test ([`3006e59`](https://github.com/Byron/gitoxide/commit/3006e5975e023c9ad56e62ce3163dd65964c0c57))
    - Write new changelogs with bat if available ([`cca8e52`](https://github.com/Byron/gitoxide/commit/cca8e52fdd2ebd16b08247d428ed5387a1058cd5))
    - Use `cargo diet` to reduce package size ([`cc5709e`](https://github.com/Byron/gitoxide/commit/cc5709e812aea79e9d9a6f16637d09f22cb73f81))
    - Write markdown changelog to lock file ([`400046e`](https://github.com/Byron/gitoxide/commit/400046ec65100a15cd1757143c1abba05091f129))
    - refactor ([`b05ce15`](https://github.com/Byron/gitoxide/commit/b05ce15a31aba9b0084792b7f0e7155b73b46e2d))
    - Basic serialization of ChangeLog ([`205b569`](https://github.com/Byron/gitoxide/commit/205b5698072c6919036190cacac120a7dd5dbd73))
    - support for generated headers ([`bcc4323`](https://github.com/Byron/gitoxide/commit/bcc4323785c5aca698e5af2ee5cf32e171727ed3))
    - refactor ([`1ebb736`](https://github.com/Byron/gitoxide/commit/1ebb7365ce564d55bd4f16f7316375b9458b4659))
    - Use 'to_*' when converting `easy::Object` to specific object kind ([`1cb41f8`](https://github.com/Byron/gitoxide/commit/1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7))
    - transform history segments into changelog parts ([`348b05c`](https://github.com/Byron/gitoxide/commit/348b05cbe6e93e871393a6db9d1ebfea59ec7fdb))
    - layout structure for ChangeLog generation from history items ([`40e9075`](https://github.com/Byron/gitoxide/commit/40e9075238f7272c08497851f55d0b525f47f2db))
    - more general commit history ([`39522ec`](https://github.com/Byron/gitoxide/commit/39522ec59d2eb7f439c75a5cc5dc0315db9497d5))
    - Invert meaning of changelog's --dependencies flag… ([`51eb8cb`](https://github.com/Byron/gitoxide/commit/51eb8cba67edf431ebe3e32232022dbf8971e6ac))
    - rename --skip-dependencies to --no-dependencies… ([`77ed17c`](https://github.com/Byron/gitoxide/commit/77ed17c703e502e132cda9a94eb8c63db0b627ad))
    - Remove strong-weak typing for conventional type ([`b71c579`](https://github.com/Byron/gitoxide/commit/b71c5790fd8c14f10df00a96f3a344c121278418))
    - Fix panic related to incorrect handling of character boundaries ([`9e92cff`](https://github.com/Byron/gitoxide/commit/9e92cff33f4f53d3b2d6b55a722d577c2dd6a4f2))
    - Parse message fully (and own it) to allow markdown generation ([`b8107e5`](https://github.com/Byron/gitoxide/commit/b8107e5d33da70f91225e9fd37443e3ba2b20f7c))
    - tests for conventional and unconventional description parsing ([`faade3f`](https://github.com/Byron/gitoxide/commit/faade3f95f861736ec0ccf7f0a811c1cf12831cd))
    - Make use of fixed git-conventional ([`b7b92b6`](https://github.com/Byron/gitoxide/commit/b7b92b6c72051d462ab01c7645ea09d7d21cb918))
    - update git-conventional dependency ([`2d369e8`](https://github.com/Byron/gitoxide/commit/2d369e863b15269ba8714b025fe596f69e5b1217))
    - first test and sketch for stripping of additional title values ([`55b7fe8`](https://github.com/Byron/gitoxide/commit/55b7fe8c9391e3a9562e084ae7524bb9f83ec36c))
    - Basic message parsing, either conventional or not, without additions ([`b3b6a2d`](https://github.com/Byron/gitoxide/commit/b3b6a2dc07c2eff38556ee66b9290b0c66b463ed))
    - Sketch Message fields from which change logs can be built ([`b167d39`](https://github.com/Byron/gitoxide/commit/b167d39ecf0cd306dcf4d2c00413251cbfd02ed6))
    - Fix build ([`d0a956f`](https://github.com/Byron/gitoxide/commit/d0a956fdb5a822dbd116792bfbe70d1532a95ec9))
    - More message parsing tests, now with legit failure… ([`625be8d`](https://github.com/Byron/gitoxide/commit/625be8dbd4204ea1a7131ade9b17f63dcc7e30d7))
    - Sketch data for parsed messages ([`32dd280`](https://github.com/Byron/gitoxide/commit/32dd280eaada635994e11b4f2722a4efc59faa8f))
    - smart-release: add git-conventional ([`0c355ed`](https://github.com/Byron/gitoxide/commit/0c355ed24eb230e9834e797d5c8dc72ae21f0c46))
    - smart-release: consider nom for custom parsing, but… ([`5fc3326`](https://github.com/Byron/gitoxide/commit/5fc33266b2626a07b19d2f5bd075e2c600204a3d))
    - smart-release: refactor ([`17322fa`](https://github.com/Byron/gitoxide/commit/17322fa378fdecad80ad1349292aaaee8bcd00f6))
    - smart-release: refactor ([`ac0696b`](https://github.com/Byron/gitoxide/commit/ac0696b8226a1478fa90b932306f35e5dbf464b1))
    - smart-release: refactor ([`87ebacc`](https://github.com/Byron/gitoxide/commit/87ebacc65f56f8765eb787fea1bd27f2c99dfd97))
    - smart-release: a seemingly slow version of path lookup, but… ([`41afad3`](https://github.com/Byron/gitoxide/commit/41afad3386461b658ee859225785b6de86d13cfb))
    - smart-release: fast filter by single-component path ([`ae7def4`](https://github.com/Byron/gitoxide/commit/ae7def47388aeb56c7df4a73fd13ff508cee7017))
    - smart-release: prepare for fast lookup of paths ([`fbf267e`](https://github.com/Byron/gitoxide/commit/fbf267eeb424bf90649be278ee847fe3f2a3db80))
    - configure caches with env vars using `apply_environment()` ([`d422b9a`](https://github.com/Byron/gitoxide/commit/d422b9a31a37a03551bec4382039aaf3a7e49902))
    - refactor ([`e7c061b`](https://github.com/Byron/gitoxide/commit/e7c061b10c263001eb4abf03098d6694b770f828))
    - set package cache via RepositoryAccessExt ([`66292fd`](https://github.com/Byron/gitoxide/commit/66292fd1076c2c9db4694c5ded09799a0be11a03))
    - object-cache to allow for a speed boost… ([`06996e0`](https://github.com/Byron/gitoxide/commit/06996e032b1e451a674395ebaca94434fac46f05))
    - smart-release: actually build the segment vec, without pruning for now ([`422701b`](https://github.com/Byron/gitoxide/commit/422701be4ed6d2a61361af9b6eb0f4f470d1d782))
    - smart-release: build commit history for later use in changelog generation ([`daec716`](https://github.com/Byron/gitoxide/commit/daec7167df524b329daad7dabb1b9920b6ef8936))
    - smart-release: sketch history acquisition ([`debe009`](https://github.com/Byron/gitoxide/commit/debe0094826f83839f907523715def929133fd58))
    - add 'Head::peeled()' method ([`56e39fa`](https://github.com/Byron/gitoxide/commit/56e39fac54bfa3871c42bbf76a9f7c49486b85be))
    - smart-release: some performance logging ([`1954b46`](https://github.com/Byron/gitoxide/commit/1954b467cf1e97e22629c55487b4a66cb1380a89))
    - smart-release: build ref lookup table ([`9062a47`](https://github.com/Byron/gitoxide/commit/9062a472ac63887900562ed341c7b68665b8587a))
    - loose reference iteration with non-dir prefixes… ([`293bfc0`](https://github.com/Byron/gitoxide/commit/293bfc0278c5983c0beaec93253fb51f00d81156))
    - Add 'references().all().peeled().'… ([`6502412`](https://github.com/Byron/gitoxide/commit/650241251a420602f74037babfc24c9f64df78d8))
    - smart-release: filter refs correctly, but… ([`2b4a615`](https://github.com/Byron/gitoxide/commit/2b4a61589a7cba3f7600710e21304e731ae3b36a))
    - smart-release: find tag references by name… ([`72e1752`](https://github.com/Byron/gitoxide/commit/72e175209441b12f3d4630e5118e21a3156146df))
    - git-ref(docs): improve changelog format ([`90e6128`](https://github.com/Byron/gitoxide/commit/90e6128727932f917c485f411e623fc6a9c2ad4d))
    - smart-release: sketch first step of info generation ([`ff894e5`](https://github.com/Byron/gitoxide/commit/ff894e5b0257722c31578772ed694324194c0741))
    - smart-release: changelog gets crates to work on ([`78d31d9`](https://github.com/Byron/gitoxide/commit/78d31d9de2710b4369862c1226f18d4a2d79a9c4))
    - smart-release: handle unborn heads ([`0e02831`](https://github.com/Byron/gitoxide/commit/0e02831fff83f6d6b0ea8889d54196e54e4e4aff))
    - smart-release: fmt ([`d66c5ae`](https://github.com/Byron/gitoxide/commit/d66c5aea01a7d1df2cc539c52b789ad39a058ad2))
    - smart-release: refactor ([`d4ffb4f`](https://github.com/Byron/gitoxide/commit/d4ffb4f2ac935f6345bdc7d03cc1878007609503))
    - smart-release: refactor ([`9fc15f9`](https://github.com/Byron/gitoxide/commit/9fc15f92ddec4ccfd0803d2b1231ed08d424cf33))
    - smart-release: refactor ([`9e430df`](https://github.com/Byron/gitoxide/commit/9e430df135e87ee9e9673e7d52f072f39abaf4d9))
    - smart-release: initial test for changelog ([`a33dd5d`](https://github.com/Byron/gitoxide/commit/a33dd5d21039441556ab89c997195f1bcc5bc543))
    - smart-release: very basic support for changelog command… ([`1a683a9`](https://github.com/Byron/gitoxide/commit/1a683a91a2850d663cf87fb326e5ab66ae86fc96))
    - smart-release: add 'cargo changelog' sub-command binary ([`3677b78`](https://github.com/Byron/gitoxide/commit/3677b782f8bc63a38d4d49b8555b5a6b9a618f84))
    - smart-release(test): add changelog to most tests ([`cdf4199`](https://github.com/Byron/gitoxide/commit/cdf41998360527161a1b04821bab377489f6c5f0))
 * **[#200](https://github.com/Byron/gitoxide/issues/200)**
    - parse issue numbers from description and clean it up ([`95c0a51`](https://github.com/Byron/gitoxide/commit/95c0a510f875e8fd889b87caee356a4c1e099ea8))
 * **[#213](https://github.com/Byron/gitoxide/issues/213)**
    - fix version logic to handle breaking version updates correctly ([`67ed644`](https://github.com/Byron/gitoxide/commit/67ed6449c410cca61ac5b40589408695eee4df69))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - split data::output::count::objects into files ([`8fe4612`](https://github.com/Byron/gitoxide/commit/8fe461281842b58aa11437445637c6e587bedd63))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - thanks clippy ([`2113d79`](https://github.com/Byron/gitoxide/commit/2113d7989b5e5dde5fc7594e1c63abef0bfba650))
    - thanks clippy ([`7c78dcf`](https://github.com/Byron/gitoxide/commit/7c78dcf468a2947e7b46103f275c27eb49b1547c))
    - thanks clippy ([`fc9da4c`](https://github.com/Byron/gitoxide/commit/fc9da4c3eef70bcc780224f42e0b78e477f3b199))
    - thanks clippy ([`41ed695`](https://github.com/Byron/gitoxide/commit/41ed695a6a739df00d39bf86dae2cc12b8e280b6))
    - thanks clippy ([`2b62956`](https://github.com/Byron/gitoxide/commit/2b629561ba7d08c6861746c512bd21dc5324e1bb))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - thanks clippy ([`87d2f49`](https://github.com/Byron/gitoxide/commit/87d2f491b4c177bd5b67eea57e6a4e516f25d1e8))
    - thanks clippy ([`a1ebd80`](https://github.com/Byron/gitoxide/commit/a1ebd800e46094ada7dbd8298a63b33724de0431))
    - thanks clippy ([`ca0d943`](https://github.com/Byron/gitoxide/commit/ca0d9432869c40135cc8db26af29bec44f3ae74a))
    - thanks clippy ([`8b3d9ea`](https://github.com/Byron/gitoxide/commit/8b3d9ea5aa7f161d2baebeafc4c1ab966583f5ac))
    - thanks clippy ([`ce48e18`](https://github.com/Byron/gitoxide/commit/ce48e184f37bf0a9c558f8e9a0eaa3b4526fdc2e))
    - thanks clippy ([`af9d137`](https://github.com/Byron/gitoxide/commit/af9d13745ae4e14d9553d3a4aa5a82cc15957a7e))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - thanks clippy ([`bf514a2`](https://github.com/Byron/gitoxide/commit/bf514a27b6b79d4ad680092019039f292c94b0f1))
    - thanks clippy ([`ead04f2`](https://github.com/Byron/gitoxide/commit/ead04f23d671039ee08ee3e6809edadfe9732ed9))
    - thanks clippy ([`e4f1c09`](https://github.com/Byron/gitoxide/commit/e4f1c091ac6cce21ee313d93bd0b0ace38aa131b))
    - thanks clippy ([`b856da4`](https://github.com/Byron/gitoxide/commit/b856da409e6a8fdc81ea32ebb4a534b0e70baebc))
    - thanks clippy ([`31498bb`](https://github.com/Byron/gitoxide/commit/31498bbee4b2bc766b42171dfd6529d885d3bc84))
    - thanks clippy ([`c55f909`](https://github.com/Byron/gitoxide/commit/c55f90977756c794939454072e4cc648f1e4348f))
    - thanks clippy ([`b200ee8`](https://github.com/Byron/gitoxide/commit/b200ee8d7522f0c83e0e01f0d793784cba7028aa))
    - thanks clippy ([`4b3407d`](https://github.com/Byron/gitoxide/commit/4b3407d0baf32b6eeb04cee07faa4bb9c1270e4e))
    - thanks clippy ([`1dece2b`](https://github.com/Byron/gitoxide/commit/1dece2b8dd18d0266210152c749c39595d70db5a))
    - thanks clippy ([`a89d08c`](https://github.com/Byron/gitoxide/commit/a89d08c4ce28f0c466f01758e9f4db09eeb02458))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com/Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - don't claim to change manifest version if it's the same one ([`11eebdc`](https://github.com/Byron/gitoxide/commit/11eebdcc572a72b2e66a9db3cae0a01f12a81619))
    - thanks clippy ([`68ea77d`](https://github.com/Byron/gitoxide/commit/68ea77dcdd5eb8033618e7af2e3eb0989007b89b))
    - thanks clippy ([`7899ef1`](https://github.com/Byron/gitoxide/commit/7899ef10f2f4a6df43beed598ddf396991dcd9e5))
    - thanks clippy ([`2b55427`](https://github.com/Byron/gitoxide/commit/2b5542761ab160cd9460b133928efd6f0cb55e75))
    - thanks clippy ([`a554b9d`](https://github.com/Byron/gitoxide/commit/a554b9d356d4e44c9504f7b35aa2c4f9c660df9b))
    - Bump git-repository v0.10.0 ([`5a10dde`](https://github.com/Byron/gitoxide/commit/5a10dde1bcbc03157f3ba45104638a8b5b296cb9))
    - thanks clippy ([`d15fded`](https://github.com/Byron/gitoxide/commit/d15fded08224c45dcbd34cf742398e2594f39964))
    - [repository #164] fix build ([`1db5542`](https://github.com/Byron/gitoxide/commit/1db554216e99c5df62a2fc7fa3f8693fdc35b3eb))
    - Release git-repository v0.9.1 ([`262c122`](https://github.com/Byron/gitoxide/commit/262c1229d6d2d55c70fe0e199ab15d10954d967b))
    - [smart-release] auto-detect changes in production crates as well ([`24bc1bd`](https://github.com/Byron/gitoxide/commit/24bc1bd678602e6b1af771b0b47eb3a39f8aa3a7))
    - [smart-release #195] update test output to match CI… ([`f864386`](https://github.com/Byron/gitoxide/commit/f86438609a1f99173efbe6b1fe91229433c1fc76))
    - [smart-release #195] better error for untracked files. ([`f5266f9`](https://github.com/Byron/gitoxide/commit/f5266f9756b1dbb9dc9846ba6efb863bdc12ae35))
    - [smart-release #195] assure dependent packages are not packages to be published ([`6792ebc`](https://github.com/Byron/gitoxide/commit/6792ebc9d09aec81ebc81b3b0fa58ca7c6ce4fcc))
    - [smart-release #195] refactor ([`f354b61`](https://github.com/Byron/gitoxide/commit/f354b61b986369865de3471ab4eed2ae7bcc60e3))
    - [smart-release #195] refactor ([`968b6e1`](https://github.com/Byron/gitoxide/commit/968b6e19894a1b42546c15ed3cf5c8485dbc701c))
    - [smart-release #195] don't tout changes that aren't really there… ([`5931012`](https://github.com/Byron/gitoxide/commit/5931012d0183b97e29de58eb93d07055f855a34f))
    - [smart-release #195] another test to validate log output ([`6148ebf`](https://github.com/Byron/gitoxide/commit/6148ebf361363f362f281bc2bdf0d37a6618f4fc))
    - [smart-release #195] a test that in theory should trigger the desired behaviour ([`fd50208`](https://github.com/Byron/gitoxide/commit/fd5020868c7141e377a604c0d34cbc527d4959f9))
    - [smart-release #194] basic journey test setup ([`d5d90a6`](https://github.com/Byron/gitoxide/commit/d5d90a654170c32750ef26872b72a6080184ac5d))
    - thanks clippy ([`8fedb68`](https://github.com/Byron/gitoxide/commit/8fedb686bcf195bf69eadd828cbacb77ff19f386))
    - [smart-release #194] conservative pre-release version updates ([`f23442d`](https://github.com/Byron/gitoxide/commit/f23442d90e710bde63dd597ae6c4509b1f909a34))
    - Bump git-repository v0.9.0 ([`b797fc1`](https://github.com/Byron/gitoxide/commit/b797fc10f3f3d1fbc23916a4ff6e5e860e2dd4ed))
</details>

## v0.3.1 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.3.1 ([`1bcea9a`](https://github.com/Byron/gitoxide/commit/1bcea9a9b3be1bbb19a279ae9a8143d008fcefe3))
    - [repository #190] refactor ([`e7188e0`](https://github.com/Byron/gitoxide/commit/e7188e047529cb0f4b20b3876f36b4592e9d2dc4))
    - [repository #190] fix build ([`f5e118c`](https://github.com/Byron/gitoxide/commit/f5e118c8871e45ed3db9da9cd6bc63a5ea99621e))
    - [repository #190] a major step forward with `head()` access ([`43ac4f5`](https://github.com/Byron/gitoxide/commit/43ac4f5acbe3ace5d43ed3ed1bc394d721f0e273))
</details>

## v0.3.0 (2021-08-27)

- add `--skip-dependencies` flag
- add `--verbose` flag and be less verbose in dry-runs by default to provide only essential information
- improvements to notification clarity

### Breaking

- Use short flag for `--no-bump-on-demand` in `--bump-dependencies`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 49 commits contributed to the release over the course of 11 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.3.0 ([`82f0cec`](https://github.com/Byron/gitoxide/commit/82f0cec9c8f0f5610ddbd6cd1ac0716a9633d7c6))
    - [smart-release #174] add asciinema recording of failed release ([`6668527`](https://github.com/Byron/gitoxide/commit/6668527ee961df214bda41619d6fb76540b0dda1))
    - [smart-release #174] prepare changelog ([`0d9a2b8`](https://github.com/Byron/gitoxide/commit/0d9a2b802d5a544a08ba1c88f9fd8fe62e8e3dc6))
    - Bump git-repository v0.8.0 ([`cdb45ff`](https://github.com/Byron/gitoxide/commit/cdb45ffa0810e9fcc9fd25bff7b696c2d27eeef5))
    - [smart-release] Adjust commit message depending on whether we are skipping the publish… ([`c190c6b`](https://github.com/Byron/gitoxide/commit/c190c6b963dbaaf80a70a51135e591ee2cb9c157))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com/Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [ref #175] make 'mutable' module private ([`a80dbcf`](https://github.com/Byron/gitoxide/commit/a80dbcf083bfcf2e291013f7b13bba9e787c5cb4))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com/Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - [stability #171] git-ref is now ST1 and available through git-repository ([`50154cd`](https://github.com/Byron/gitoxide/commit/50154cd02fdd90930a1d7c5a4406d53c8067cb4b))
    - [smart-release #171] Try to avoid unstable git-repository features… ([`c8f325b`](https://github.com/Byron/gitoxide/commit/c8f325bed5d644eded035109702098f9fed3fba3))
    - Merge branch 'main' into stability ([`11bae43`](https://github.com/Byron/gitoxide/commit/11bae437e473fef6ed09c178d54ad11eee001b1d))
    - [stability #171] Don't provide access to less stable crates in `Respository` ([`e4c5b58`](https://github.com/Byron/gitoxide/commit/e4c5b58ad935c907dfbd0d61049453dcb64a7e19))
    - [stability #171] Don't leak unstable plumbing crates in git-repository… ([`71eb30f`](https://github.com/Byron/gitoxide/commit/71eb30f1caa41c1f9fe5d2785b71c9d77922c2af))
    - [stability #171] finish tier description… ([`4fe1259`](https://github.com/Byron/gitoxide/commit/4fe125973304b765f0171deb1c26bca64bbff5d7))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com/Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [ref #165] refactor ([`66624c3`](https://github.com/Byron/gitoxide/commit/66624c3ef1faf7048ee86ed73cf5f622802c061e))
    - [repository #165] refactor ([`1547d0b`](https://github.com/Byron/gitoxide/commit/1547d0b062e35bad2229dac532e6f30bf105db73))
    - [repository #165] refactor; fine grained allow(missing_docs)… ([`aa0511f`](https://github.com/Byron/gitoxide/commit/aa0511f80f11de8e83fc333e78db369ceb9b2794))
    - [repository #165] prepare for writing light docs for Easy ([`f8834c9`](https://github.com/Byron/gitoxide/commit/f8834c9c8d2ab2ce87857c6773c6204f60df240e))
    - [repository #165] refactor ([`3a0160e`](https://github.com/Byron/gitoxide/commit/3a0160ed1c5bc33d330ad4e9189c4937d194e98d))
    - [repository #165] a sample of a simpler way to create a tag ([`fb8f584`](https://github.com/Byron/gitoxide/commit/fb8f58412cdd32991a182a41cbc0d463127a4e0e))
    - [smart-release #165] Use generic edit-reference functionality ([`be3e57f`](https://github.com/Byron/gitoxide/commit/be3e57f6221dc87505ba1aad1166e28c328c3b54))
    - [repository #165] refactor ([`00ec15d`](https://github.com/Byron/gitoxide/commit/00ec15dcfdb839095e508139d238df384ea418eb))
    - [repository #165] offer panicking type conversions for objects ([`f802f8c`](https://github.com/Byron/gitoxide/commit/f802f8c8c382f8063fa615fda022857a740a974a))
    - [repository #165] try a more common naming convention for fallbile things… ([`fc70393`](https://github.com/Byron/gitoxide/commit/fc703937a078937840ea1c254f11e64aaf31de90))
    - [smart-release #162] use TreeRef capabilities to lookup path ([`51d1943`](https://github.com/Byron/gitoxide/commit/51d19433e6704fabb6547a0ba1b5c32afce43d8b))
    - [repository #162] finally let smart-release use the correct abstraction for peeling ([`ba243a3`](https://github.com/Byron/gitoxide/commit/ba243a35ff6f059e5581c6f7ff80e1253ceca6f8))
    - [repository #162] Add id field to ObjectRef… ([`f5ba98e`](https://github.com/Byron/gitoxide/commit/f5ba98ebd0e1d7d0491871be58476cb6882b8436))
    - [repository #162] experiment with finding objects… ([`312a692`](https://github.com/Byron/gitoxide/commit/312a69256a67a0f9d3f3f5c5f9eaf51b50971c5e))
    - [repository #162] Cannot ever store a RefCell Ref in an object… ([`5c17199`](https://github.com/Byron/gitoxide/commit/5c171995383fa9a3698b6aaf3fbd9537110c0299))
    - [repository #162] experiemnt with optionally keeping data in Object ([`b8a8e08`](https://github.com/Byron/gitoxide/commit/b8a8e08e1d972e5069b136c30407c079825b7e1d))
    - [smart-release #162] Fix short flags ([`08f3418`](https://github.com/Byron/gitoxide/commit/08f3418a0b763b7860d95536446fe615cf361adf))
    - [smart-release #162] don't throw away work… ([`b43b780`](https://github.com/Byron/gitoxide/commit/b43b780c0382683edc859e3fbd27739716a47141))
    - [smart-release #162] refactor ([`7f2421b`](https://github.com/Byron/gitoxide/commit/7f2421bddf7510d1cd6a12fa1457e3e842b38879))
    - [smart-release #162] peeling objects to a certain target kind… ([`5785136`](https://github.com/Byron/gitoxide/commit/57851361f3fc729b964fd0ca5dca9f084fe20f5e))
    - [smart-release #162] a single import path for ReferenceExt ([`7060797`](https://github.com/Byron/gitoxide/commit/7060797031e5bdbb8d635cc2da3269996bdfc4cc))
    - [smart-release #162] replace reference peeling with git_easy ([`7cfd5f9`](https://github.com/Byron/gitoxide/commit/7cfd5f9e0a7f828152594f0393a919617c60a9d6))
    - [smart-release #162] smart-release uses Easy repository in 'plumbing' mode ([`4fb672a`](https://github.com/Byron/gitoxide/commit/4fb672a6e7116722577cbbeeee67887871f583bf))
    - [smart-release #164] improve handling of empty commits ([`bd93fcb`](https://github.com/Byron/gitoxide/commit/bd93fcbbf372099abc1cd3a56cb57105581232ad))
    - [smart-release #164] Make it easier to change a single crate's version only ([`38d28ce`](https://github.com/Byron/gitoxide/commit/38d28ceb1b57da36d59ce0ec418a3dbd9f6fd8fb))
    - [smart-release #162] only warn if there is working tree modifications in dry-run mode… ([`f8ce62f`](https://github.com/Byron/gitoxide/commit/f8ce62fec67845ad89be4bb5482452e9ca7d0035))
    - [smart-release #162] clearer messages ([`aa7417f`](https://github.com/Byron/gitoxide/commit/aa7417fb8ab58761ae31ff926898855c76a8fd9f))
    - thanks clippy ([`45c5c3c`](https://github.com/Byron/gitoxide/commit/45c5c3cb4679721f296ac72db382b8536f8774c7))
    - [smart-release #162] top-level crate uses version-only tag ([`85e5b1a`](https://github.com/Byron/gitoxide/commit/85e5b1a6e24107f4a26c2b3119c94bbb67fd6068))
    - [smart-release #162] FAIL: single-crate workspaces use version-only tags ([`c5947c4`](https://github.com/Byron/gitoxide/commit/c5947c42eb330bc2cc84889755c461858925cc2e))
    - [smart-release] better --verbosity handling ([`8cccb11`](https://github.com/Byron/gitoxide/commit/8cccb1181e8ad708205524886ac0188ab74da163))
    - [smart-release] properly obtain top-level crate name using manifest ([`d74b32e`](https://github.com/Byron/gitoxide/commit/d74b32eb57c45bef4f6257b4fbe7a9dfc5a41a78))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.2.4 (2021-08-15)

- Fix auto-push functionality

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.4 ([`19f21a4`](https://github.com/Byron/gitoxide/commit/19f21a4d53c8fcb237ee79c098d39510830806ed))
    - [smart-release #160] fix auto-push issue ([`73051d3`](https://github.com/Byron/gitoxide/commit/73051d3c85a2b0356286deb5da6863e7f9e72b35))
</details>

## v0.2.3 (2021-08-15)

- Less verbosity by default which is helpful on the first run to get an overview. Use `--verbose/-v` for all the details.
- Also push tags and HEAD by default, unless `--skip-push` is specified.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.3 ([`f50bac8`](https://github.com/Byron/gitoxide/commit/f50bac894363d008f670d1d0f15a03bdad98b9c2))
    - [smart-release #160] update chnagelog ([`7c4ff64`](https://github.com/Byron/gitoxide/commit/7c4ff64492c584bf5cfa99432aed714c9baeaa9c))
    - [smart-release #160] Add the --skip-push flag… ([`6ebfc85`](https://github.com/Byron/gitoxide/commit/6ebfc854c723799466f2136e77986d1ffb2b6f63))
    - [smart-release #160] Push after creating a single tag ([`6add57f`](https://github.com/Byron/gitoxide/commit/6add57f321610de446f67d1c1395a683660b54a4))
    - [smart-release #160] a seemingly nice '--verbose' mode… ([`bf55679`](https://github.com/Byron/gitoxide/commit/bf55679d973bc4a36faf426d33cd5d91d6783656))
    - thanks clippy ([`bc7c9a8`](https://github.com/Byron/gitoxide/commit/bc7c9a89c56bf0c6ddb2a9edb2bee6c6aea5b746))
    - [smart-release #160] avoid trying to use an empty path when detecting changes… ([`836324e`](https://github.com/Byron/gitoxide/commit/836324ea67b16dd2dd3dd2f09e6e04c5ae39fb35))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.2.2 (2021-08-15)

- support for unsorted packed-refs files

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.2 ([`f73c729`](https://github.com/Byron/gitoxide/commit/f73c72972abca7ebf7c7ad52c078e3df3157ae7b))
</details>

## v0.2.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-smart-release v0.2.1 ([`a3c45de`](https://github.com/Byron/gitoxide/commit/a3c45de6b0e1cc75ab016bf9c3b0bfa7039ba6c7))
    - [smart-release #155] Another note ([`5feb437`](https://github.com/Byron/gitoxide/commit/5feb4379ac400086468b9838c22d95504d0c5ea5))
</details>

## v0.2.0 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [smart-release #155] how to increase version numbers ([`0bad7b7`](https://github.com/Byron/gitoxide/commit/0bad7b7a20bc4d8d73d6ac0d308c47bcd9368a23))
    - Release cargo-smart-release v0.2.0 ([`b95d7ed`](https://github.com/Byron/gitoxide/commit/b95d7ed464c499694784de153b63461c70f0dbe0))
    - [smart-release #155] keep dependency versions by default ([`4f53287`](https://github.com/Byron/gitoxide/commit/4f5328743c2d5dd80f8f1c17f90c21a7142e45f9))
    - [smart-release #155] fix bug :D ([`3d2a044`](https://github.com/Byron/gitoxide/commit/3d2a044252830c7c6e3092aa36184f5d25a7c855))
    - [smart-release #155] workflow notes and inversion of flag for comfort ([`1ffb66c`](https://github.com/Byron/gitoxide/commit/1ffb66c6f3b8ec199809d0485bcd19d71d879385))
    - thanks clippy ([`c50bd73`](https://github.com/Byron/gitoxide/commit/c50bd735a3764bcd25d9e312da81bed60c711133))
    - [smart-release #155] inform about latest features ([`133e43a`](https://github.com/Byron/gitoxide/commit/133e43a776403af1115b0f09eb046d02e779e12e))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.0 (2021-08-13)

- initial release

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 45 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [smart-release #155] refactor ([`21192b8`](https://github.com/Byron/gitoxide/commit/21192b899a246e5824b1d5156c4123cab2cc404e))
    - [smart-release #155] prepare release ([`4684557`](https://github.com/Byron/gitoxide/commit/4684557651237ba41e52e648f42efcddd18489d3))
    - [smart-release #155] even smarter bumping ([`1f38680`](https://github.com/Byron/gitoxide/commit/1f38680ada9a33966bef3e5e752b795c0c005224))
    - [smart-release #155] --bump-dependencies only ([`19d87a6`](https://github.com/Byron/gitoxide/commit/19d87a651f9f3a8db89bed533d4c31758f5bfc1f))
    - [smart-release #155] incorporate crates-index for additional version check ([`08bd13d`](https://github.com/Byron/gitoxide/commit/08bd13d7f94390e58ba2516c9328303e023805e5))
    - [smart-release #155] prepare for crates-index; refactor ([`498b6cc`](https://github.com/Byron/gitoxide/commit/498b6cc11f625c60ca7ccc40b014fc6a7d20183d))
    - [smart-release #155] make it an actual depth-first traversal :D ([`b05a21f`](https://github.com/Byron/gitoxide/commit/b05a21f668f8d0ef176b450a160b4def23d3d79b))
    - [smart-release #155] sanity check for dry-run/no-dry-run-cargo-publish ([`2fa7b0b`](https://github.com/Byron/gitoxide/commit/2fa7b0b3053644f1132f1e8689d2f685c4e5b95d))
    - [smart-release #155] update README, add changelog ([`b5dd553`](https://github.com/Byron/gitoxide/commit/b5dd55333b48869c89dd38f437dbba2d217c14d8))
    - thanks clippy ([`7709ca0`](https://github.com/Byron/gitoxide/commit/7709ca0eeed793825a3b2f3c3fd84e9feff1e494))
    - [smart-release #155] graceful handling of unspecified crate to publish ([`e65b657`](https://github.com/Byron/gitoxide/commit/e65b657300208513f957ad52cedc3af64666cd6d))
    - [smart-release #155] rely only on cargo metadata for root paths ([`217dafb`](https://github.com/Byron/gitoxide/commit/217dafbd2453e079d8da82fb95753b965b359569))
    - [smart-release #155] also ignore provided crate names if they didn't change ([`2110a8c`](https://github.com/Byron/gitoxide/commit/2110a8c3da4266083f2d46e75e8956d212598c86))
    - [smart-release #155] gracefully fail when encountering unknown comparators ([`bee367b`](https://github.com/Byron/gitoxide/commit/bee367bfc816247316dede4b4f638fafa69d0fba))
    - [smart-release #155] don't set versions if the new ones match ([`dd0f428`](https://github.com/Byron/gitoxide/commit/dd0f42848ce9906cdaff3418498f3e42b2c41e2c))
    - [smart-release #155] refactor ([`07dc6d8`](https://github.com/Byron/gitoxide/commit/07dc6d81377b1830b8d1f76118dd9c220516d9fd))
    - [smart-release #155] remove dia-semver ([`07f84c7`](https://github.com/Byron/gitoxide/commit/07f84c76e10b17e36cfaae05b4becfe186e2bebe))
    - [smart-release #155] don't set versions where there are none when fixing manifests ([`a1cc79f`](https://github.com/Byron/gitoxide/commit/a1cc79f4b97182a54d54f6cb8b41b756cd75ff81))
    - [smart-release #155] also find renamed dependencies when updating versions ([`06bc6a9`](https://github.com/Byron/gitoxide/commit/06bc6a9dd0715b0deb158968e62aa216d1902014))
    - [smart-release #155] a note ([`a726225`](https://github.com/Byron/gitoxide/commit/a726225df5967a02776e7caf26d9499ab0cfb262))
    - [smart-release #155] invert meaning of cargo-publish dryrun flag ([`cc57eb8`](https://github.com/Byron/gitoxide/commit/cc57eb8100f4502b0cb9ac0223f37444141884a3))
    - [smart-release #155] allow dry-running cargo publish, too… ([`15e611e`](https://github.com/Byron/gitoxide/commit/15e611e8abb770f4b9c424faf678fbf7e6e541d5))
    - [smart-release #155] allow dry-running cargo-publish, too ([`a3add55`](https://github.com/Byron/gitoxide/commit/a3add5510395e47bddfea3ba9ad4a6e5aeba8ff7))
    - [smart-release #155] Flag to auto-publish dependent stable crates as well ([`bded12f`](https://github.com/Byron/gitoxide/commit/bded12ffd4c92fdb97c320a813a3eccde824c47f))
    - [smart-release #155] don't auto-add stable crates but suggest to do something about it ([`d1dca70`](https://github.com/Byron/gitoxide/commit/d1dca70f5893e4df5bc0fd7ecaffd739d007f1ee))
    - [smart-release #155] refactor ([`8e78e77`](https://github.com/Byron/gitoxide/commit/8e78e77248066f03ff26e8ab1556377f57f6b901))
    - thanks clippy ([`507cb94`](https://github.com/Byron/gitoxide/commit/507cb94c1be97c6e3c0f15a8142c88291bfe1482))
    - [smart-release #155] refactor ([`fb1fb57`](https://github.com/Byron/gitoxide/commit/fb1fb57230fd8ae3b6b2654d33b4c130478f2781))
    - [smart-release #155] don't rely on cargo resolution order for cyclic case/publish groups ([`7c97fa4`](https://github.com/Byron/gitoxide/commit/7c97fa4eeeb261ec12a93fde5de90d11db1b6e60))
    - [smart-release #155] avoid using cargo resolution order ([`4b7d9d1`](https://github.com/Byron/gitoxide/commit/4b7d9d1704c7236ff343634eb5d120beff6ff18c))
    - [smart-release #155] properly handle multi-crate dependencies (if there is no cycle) ([`e8838a9`](https://github.com/Byron/gitoxide/commit/e8838a97e143f67efe92fd98dc70b868d3ab3487))
    - [smart-release #155] trust our own resolution order more… ([`a977925`](https://github.com/Byron/gitoxide/commit/a977925262f000d2f33a25f80e298d5efce33347))
    - [smart-release #155] refactor ([`0841088`](https://github.com/Byron/gitoxide/commit/0841088f9ca70d727ca221ffb05daf6f5bf7b888))
    - [smart-release #155] don't check cycles on dependencies without version ([`9eeaa2f`](https://github.com/Byron/gitoxide/commit/9eeaa2f11ee063dec88b783d0be2c64902cfe093))
    - [smart-release #155] refactor ([`3f887a7`](https://github.com/Byron/gitoxide/commit/3f887a7f59b8c56a9e4aaa042bbab5f00382d089))
    - [smart-release #155] refactor ([`680675b`](https://github.com/Byron/gitoxide/commit/680675b5a37c1a7ab77357460b8daf2df347a11f))
    - [smart-release #155] refactor ([`20a3aef`](https://github.com/Byron/gitoxide/commit/20a3aef84d480cecaa437a258d23e0904d004cb3))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - [smart-release #155] prepare release ([`1330dff`](https://github.com/Byron/gitoxide/commit/1330dff97d6a94e9653c98b0aa4330ea9b441ad1))
    - [smart-release #155] cargo compatibility ([`d432a8e`](https://github.com/Byron/gitoxide/commit/d432a8e95dd88224b3c18cc173035458ef57faea))
    - [smart-release #155] add readme ([`86252eb`](https://github.com/Byron/gitoxide/commit/86252ebb2f1bd8b5430600c09e01516359f4274f))
    - [smart-release #155] --skip-tag flag ([`469de34`](https://github.com/Byron/gitoxide/commit/469de34e19ea25174b7461361e595815d1554343))
    - [smart-release #155] --bump option ([`552d244`](https://github.com/Byron/gitoxide/commit/552d24422e0b4a91bb0cb1f7e98dc101e0e19a5b))
    - [smart-release #155] remove subcommands ([`9f82828`](https://github.com/Byron/gitoxide/commit/9f828280307648be37926c803e19b51ade8dee8b))
    - [smart-release #155] rename from 'utils' ([`a9e6fcc`](https://github.com/Byron/gitoxide/commit/a9e6fccda617ea44eb8593f4da18519eff56bf8c))
</details>

