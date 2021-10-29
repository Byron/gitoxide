# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.10.0 (2021-10-19)

This release pins beta versions of `clap` to avoid it to automatically fetch the latest one
during installation.

This is made possible due to `clap` itself pinning its dependency
to the `clap-derive` crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com//Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com//Byron/gitoxide/issues/222)**
    - upgrade to clap 3 beta 5 ([`2ddc4ed`](https://github.com//Byron/gitoxide/commit/2ddc4eddda23c77b5891a11a3e7215702c63882b))
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
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#200](https://github.com//Byron/gitoxide/issues/200), [#67](https://github.com//Byron/gitoxide/issues/67)

## v0.8.4 (2021-09-10)

This is a maintenance release.

## v0.8.3 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## v0.8.1 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 95 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#83](https://github.com//Byron/gitoxide/issues/83)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.7.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 128 calendar days.
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
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add lean-plumbing docs for path of commit-graph-verify ([`5c7b52d`](https://github.com//Byron/gitoxide/commit/5c7b52d658d5b86bf4cf05c724202e824016c0e2))
    - [commitgraph] Implement basic commit-graph file verification. ([`2571113`](https://github.com//Byron/gitoxide/commit/2571113fea516737acedac08d66632ead499b474))
    - [commitgraph] Stub out commit-graph-verify plumbing command. ([`aacf0f0`](https://github.com//Byron/gitoxide/commit/aacf0f05a909e5b7d9ffd5623ef9833e0465be93))
</details>

## v0.4.3 (2020-09-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

## v0.4.1 (2020-09-18)

* fix installation via `cargo install`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 6 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Finish removal of rust 2018 idioms ([`0d1699e`](https://github.com//Byron/gitoxide/commit/0d1699e0e0bc9052be0a74b1b3f3d3eeeec39e3e))
    - Provide terminal dimensions to better use horizontal space ([`11f6b84`](https://github.com//Byron/gitoxide/commit/11f6b8497a5089377e605f4cbe1cd317ef677d59))
</details>

## v0.4.0 (2020-09-12)

* add `remote-ref-list` and `pack-receive` subcommands to **gix**

### CLI Breaking

 * rename plumbing sub-command from `index-from-pack` to `pack-index-from-data`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 29 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [clone] refs can now be written into a specified directory ([`fb1f048`](https://github.com//Byron/gitoxide/commit/fb1f04837be994fa5bcb9aa24f25b5f4f72e4ce0))
    - [clone] First version of writing references, but… ([`445be27`](https://github.com//Byron/gitoxide/commit/445be27cf81663ba4fe941c00262448444efbac2))
    - [clone] first journey test for pack-receive ([`46a3511`](https://github.com//Byron/gitoxide/commit/46a3511aead043bc45256ce603285ff4d0fff60a))
    - [clone] This actually works: first MVP of retrieving packs via clone ([`c06d819`](https://github.com//Byron/gitoxide/commit/c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d))
    - [ref-ls] add pretty version for ls-refs ([`487d06d`](https://github.com//Byron/gitoxide/commit/487d06d53b9cc201b5a009977e835b51f4b9f690))
    - [ref-ls] Fix progress display ([`2fcb557`](https://github.com//Byron/gitoxide/commit/2fcb557dce941eb94ca60f46ecee86b94e029db7))
    - refactor ([`b38290e`](https://github.com//Byron/gitoxide/commit/b38290e4a8fcabd758f26a15407710ab2abcdc07))
    - [ref-ls] refactor ([`35e26fc`](https://github.com//Byron/gitoxide/commit/35e26fc32978232aebda3468c9f172fb7b08b815))
    - refactor ([`f90b92f`](https://github.com//Byron/gitoxide/commit/f90b92ffc2994f594352abaf4bacd9767cbc2e6c))
    - [ref-ls] Frame for remote-ref-ls command in gitoxide-core ([`161e7df`](https://github.com//Byron/gitoxide/commit/161e7df34a53db40551879c6d2319ee775dfd551))
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive ([`5ea49c8`](https://github.com//Byron/gitoxide/commit/5ea49c8aa0d449bed98ce0147ad222ff25c27c32))
    - refactor ([`40a6412`](https://github.com//Byron/gitoxide/commit/40a64125dc5556630576ec2164b68838c76ccd79))
    - Less ambiguous name for 'index-from-pack': 'pack-index-from-data' ([`386673c`](https://github.com//Byron/gitoxide/commit/386673ccc99d18d023c7df3fcd40e86d71960b25))
    - refactor ([`b4a6e16`](https://github.com//Byron/gitoxide/commit/b4a6e16364822c0dccb56f98dbfb0ca4c8007069))
</details>

## v0.3.0 (2020-08-12)

* add `pack-explode` and `pack-index-from-data` sub-commands
* massive speed improvements for `pack-verify`

Many small and possibly breaking changes are not mentioned here.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 30 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make obvious that interrupt request was received ([`34b2373`](https://github.com//Byron/gitoxide/commit/34b23737f560fe52d4f98fb886eba754652f9a5e))
    - make interrupt handler work reliably ([`e71da0f`](https://github.com//Byron/gitoxide/commit/e71da0fce6d6eab68f7b81b13cdc78ce8e9b7ee3))
    - unify used ranges for line renderer amond pretty and lean interface ([`f59f66e`](https://github.com//Byron/gitoxide/commit/f59f66e189732f567414f68c7463364e510f41c4))
    - Add percentage and throughput to tasks that matter ([`763d7ca`](https://github.com//Byron/gitoxide/commit/763d7caa4c70111b7cb3ef5733d2c3c697758c28))
    - Upgrade to latest iteration of prodash ([`3a4faec`](https://github.com//Byron/gitoxide/commit/3a4faecab56e37670c553e6563f11a46d740c333))
    - support for JSON format output ([`1931575`](https://github.com//Byron/gitoxide/commit/19315750f4f409e3f105c3c4054c4afbef91daad))
    - first pieces of the index-from-pack journey tests ([`181d69c`](https://github.com//Byron/gitoxide/commit/181d69c1da46a931c513cbd7d8bca7b2fa53351c))
    - Add versions back to main command, remove from sub-commands ([`e509373`](https://github.com//Byron/gitoxide/commit/e509373b26c9a7b120057fc6e75970568f328fc4))
    - ditch structopt in favor of clap 3.0 beta1 ([`d7591e2`](https://github.com//Byron/gitoxide/commit/d7591e24a5178732713286a5e28cbc90f5fe9ed9))
    - Move common flags to common plac ([`c0352c2`](https://github.com//Byron/gitoxide/commit/c0352c2643e2badde79778d4a22d2e392a44f0a3))
    - Write about user interfaces and the use/non-use of async ([`91ba045`](https://github.com//Byron/gitoxide/commit/91ba0457745f860b7a68cb38b13e69754747e8d9))
    - interrupt support for pretty plumbing ([`bca7ce2`](https://github.com//Byron/gitoxide/commit/bca7ce2e668a4be2600d2d04d00f46b21c82eee2))
    - Revert "Less memory for look up mode, faster start" - too slow ([`584350a`](https://github.com//Byron/gitoxide/commit/584350af91f533db4cf980327d530445384c6b5a))
    - Less memory for look up mode, faster start ([`395c7e7`](https://github.com//Byron/gitoxide/commit/395c7e78ef344ee56cf3d4ef49828942a09094bc))
    - remove memory mode entirely (and some complexity with it) ([`8812e91`](https://github.com//Byron/gitoxide/commit/8812e916a21983868a37c4aade10f79a1dc9b926))
    - turns out you never want to keep deltas in memory ([`657aa2c`](https://github.com//Byron/gitoxide/commit/657aa2c38673cf10174f42bcb97039ac37b2926e))
    - Remove support for keeping compressed memory to reduce the index size ([`1e2ec7e`](https://github.com//Byron/gitoxide/commit/1e2ec7e9d0ef2f2a4908860672080e411e945bff))
    - …but there seem to be issues with the kernel pack… ([`cc147bc`](https://github.com//Byron/gitoxide/commit/cc147bc60066c4ef31353a499958edadc960a9c4))
    - minor fixes after first local tests - it's up to twice as fast!! ([`43c7fd1`](https://github.com//Byron/gitoxide/commit/43c7fd1f81b9b4c938f99c0bf1deabdf121226b9))
    - quick and dirty impl of lean command-line for index-from-pack ([`9660bbf`](https://github.com//Byron/gitoxide/commit/9660bbffd8ace621178b067e22d227ef8c50ba84))
    - upgrade dependencies ([`44b8221`](https://github.com//Byron/gitoxide/commit/44b8221800454f9b651778a422186bd5061877f4))
    - remove invalid clap configuration ([`665696f`](https://github.com//Byron/gitoxide/commit/665696f636e152ad9969ea0ca004cb83f1641ae6))
    - prepare full 'verify' implementation ([`ee45c7f`](https://github.com//Byron/gitoxide/commit/ee45c7f47b95fc406cc5922a322c8fd6c0f52775))
    - refactor ([`0a33b24`](https://github.com//Byron/gitoxide/commit/0a33b24f5b61ccdf1358f1e9adcf0f6fd4099c1c))
    - Allow sink-compress configuration; choose best algorithm ([`29b9c23`](https://github.com//Byron/gitoxide/commit/29b9c230e35ba9b4334797b63ab9fa88c2fe59d0))
    - Nice error message on failure ([`adbc82c`](https://github.com//Byron/gitoxide/commit/adbc82c31450681fcb38233eeb8095efc5e52a18))
    - The first 'explode' implementation… ([`0d31ad1`](https://github.com//Byron/gitoxide/commit/0d31ad1b61997fa0d0692c5919fb8032ffaaa35b))
    - Get all pieces ready for action ([`1805d64`](https://github.com//Byron/gitoxide/commit/1805d64b9222d6a05a8718f04b29b789c1f42fea))
    - Pass option for safety checks down to explode(…) ([`0bcb790`](https://github.com//Byron/gitoxide/commit/0bcb790dc8c35097916876afbb68bbfcc894c369))
    - refactor ([`f66b116`](https://github.com//Byron/gitoxide/commit/f66b116ddfbee62c3e20a4c5e7cd878fbf064195))
    - basic tests and CLI args for explode pack ([`f932256`](https://github.com//Byron/gitoxide/commit/f932256a62d6fc5d5558446de079fb666ddc27da))
    - rename verify-pack to pack-verify (keeping it more formal) ([`ec8c48a`](https://github.com//Byron/gitoxide/commit/ec8c48a8fcbcd748c9c764734d881b5f0217e1e4))
    - refactor ([`d3c00c8`](https://github.com//Byron/gitoxide/commit/d3c00c841ee1aeda6bb0534fe365db13c31f8d3c))
    - Change bin names from 'gio' to 'gix' and 'gix' ([`5e23137`](https://github.com//Byron/gitoxide/commit/5e231371432ad02c67b095448564b2aa6af76799))
    - Revert "Invert --statitics switch to become --no-statistics" ([`93a9b30`](https://github.com//Byron/gitoxide/commit/93a9b30069d9abc5742546ade90913026ac5774b))
    - Invert --statitics switch to become --no-statistics ([`aeb8778`](https://github.com//Byron/gitoxide/commit/aeb87789ecc5cf3fd0ac69d67c7d0785e4eb329c))
    - use faster algorithm by default ([`bb45c3d`](https://github.com//Byron/gitoxide/commit/bb45c3d8a2aabf87231981000240f0444abf6fc4))
    - Fix clippy ([`ec40e09`](https://github.com//Byron/gitoxide/commit/ec40e093d72f93d86168f39ebaca5b122ca0bec3))
    - Change course and do pack streaming first ([`bcb275e`](https://github.com//Byron/gitoxide/commit/bcb275e91cfd6f0a71b3cb59a2b706b60608a594))
    - get rid of annoying warnings - there is no better and easier way ([`41f38c4`](https://github.com//Byron/gitoxide/commit/41f38c442e086b1f3fb48eea25839ef6207f0cbc))
    - Fully implement --encode and --re-encode flags ([`a7cfac8`](https://github.com//Byron/gitoxide/commit/a7cfac83ddd859d9c2c25e457c0d7043738792dc))
    - prepare for re-encoding each pack object ([`afae684`](https://github.com//Byron/gitoxide/commit/afae684c72e5dc4b718976056dd5d34ed61de72a))
    - fix naming change, which was introduced accidentally ([`fbb9f98`](https://github.com//Byron/gitoxide/commit/fbb9f98508ec722e192466e28ded47aef2fb78b3))
    - refactor ([`2888f1b`](https://github.com//Byron/gitoxide/commit/2888f1b10a2baf40155544e667ddd461f3ddc938))
    - pass threadlimit down from CLIs ([`f98c5b1`](https://github.com//Byron/gitoxide/commit/f98c5b160db80a7cac530e18b9256562c25be47f))
    - add new Context argument to support more configuration options ([`7c5d8b8`](https://github.com//Byron/gitoxide/commit/7c5d8b8bb318e59a59ad74ad767a1525e2833632))
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
    - bring color back to 'max' versions ([`c68d9ab`](https://github.com//Byron/gitoxide/commit/c68d9ab6e9cccab3610a77a2c6839a26fb42de2d))
    - Support for disabling the cursor in the pretty line renderer ([`48c4bbd`](https://github.com//Byron/gitoxide/commit/48c4bbd35f678de801fa2243f7a8705e825bdbf9))
    - Allow TUI to terminate action properly ([`1f1b725`](https://github.com//Byron/gitoxide/commit/1f1b7257895a219b5623b1ba3beaee1282ff2b63))
    - refactor ([`cce71aa`](https://github.com//Byron/gitoxide/commit/cce71aadb53528cb2f1d173b81df939ad1df8083))
    - refactor ([`f276a05`](https://github.com//Byron/gitoxide/commit/f276a0561fcec78f802b166f09110becf22ea7ee))
    - preliminary support for line renderer in max version ([`4aa8022`](https://github.com//Byron/gitoxide/commit/4aa8022c248fc26292ea9f1c61e2e8dbadc56e7c))
    - unify frame rate across plumbing (and later potentially porcelain) ([`e2a7bdd`](https://github.com//Byron/gitoxide/commit/e2a7bdd9a549400f0e8d31b590f469660e458c89))
    - refactor ([`41e01a5`](https://github.com//Byron/gitoxide/commit/41e01a53a7f48dd3341ee287a243a33190bccea0))
    - Proper implementation of line renderer into 'lean' CLI ([`e98e7c2`](https://github.com//Byron/gitoxide/commit/e98e7c280d73e9d9ebd13202afb93a56cb2f7c9c))
    - Mild improvements to look of verbose log ([`5fff552`](https://github.com//Byron/gitoxide/commit/5fff5524c4443d9c9ae26307c19745c722334d0c))
    - first very basic version of line renderer progress - works… ([`0cc1bf2`](https://github.com//Byron/gitoxide/commit/0cc1bf25c69611f9512fec415ae8e09b608706fc))
    - prepare for optional addition of line renderer for lean version ([`aac0d34`](https://github.com//Byron/gitoxide/commit/aac0d341eb02f0dccdf740f7ef15e8f585907544))
    - upgrade to prodash version 7 ([`af02b46`](https://github.com//Byron/gitoxide/commit/af02b46cc1eff5ba1da7da20d3f524a79fad686f))
    - Make --version flags work as expected. ([`a4d978c`](https://github.com//Byron/gitoxide/commit/a4d978ccc11e73fd752055c9a28b3b23dea145ea))
    - rename 'pretty' target into 'max', a better fit for what it is ([`5acecc5`](https://github.com//Byron/gitoxide/commit/5acecc59d2d39141f2e98b6f8556c6d457ab0965))
    - Make gio commands less cumbersome, self-document their build type (pretty, lean) ([`1f9bc03`](https://github.com//Byron/gitoxide/commit/1f9bc03dd773d90960a6f6d4ee59af3f938ad80b))
    - Allow to limit the logging depth for less cluttered output ([`fce7035`](https://github.com//Byron/gitoxide/commit/fce703531d7006f7d961d6ffa66f51f6c9bc0efc))
    - support for json in pretty-plumbing and gitoxide (on demand) ([`b3780f8`](https://github.com//Byron/gitoxide/commit/b3780f87438d34b372c48b7385199f7ea22b3965))
    - Simplify the 'keep open' logic of TUI progress window ([`13cd8ce`](https://github.com//Byron/gitoxide/commit/13cd8ce372800eb0016190960834c759c9744b9c))
    - attempt to implement progress with a mode enum ([`ac490c2`](https://github.com//Byron/gitoxide/commit/ac490c21b8f369c45ee0d7688ddb381ce6f4af94))
    - Allow for more screen space when formatting ([`6794300`](https://github.com//Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - assure pretty progress doesn't occlude the output ([`122d69f`](https://github.com//Byron/gitoxide/commit/122d69fee217eb264a335f0a056d03eba066332e))
    - fix pretty build ([`6adf615`](https://github.com//Byron/gitoxide/commit/6adf615ed7d6c488c25589940fc0a55bf0fb3d5c))
    - pass average stats through to the top level ([`5b4979c`](https://github.com//Byron/gitoxide/commit/5b4979c1dfeb9a29974dd4e6529ae5da074d0b1a))
    - refactor ([`7add82c`](https://github.com//Byron/gitoxide/commit/7add82c39169e3c2fff76c48cdd318fe6040d7bc))
    - Now ACTUALLY stop TUI when there is no progress anymore :D ([`3bf3321`](https://github.com//Byron/gitoxide/commit/3bf33210a96e1e3bc2a81782b339b5c67344ac34))
    - Automatically close the TUI when there is no progress anymore. ([`c416152`](https://github.com//Byron/gitoxide/commit/c416152b04051958de7bd161a8a2ee42ca163275))
    - Assure we wait for GUI thread to finish ([`60eaea0`](https://github.com//Byron/gitoxide/commit/60eaea0ee01214202ab9f23514dc45a9909d7888))
    - pretty progress in a generalized form ([`caa883b`](https://github.com//Byron/gitoxide/commit/caa883b96827deb63b5c8787ed820d22f2c85249))
    - neater progress log messages: don't show the module it originates from ([`026a0dd`](https://github.com//Byron/gitoxide/commit/026a0dd1faf28c4668f58bd1790c168a0134559f))
    - refactor ([`30925e6`](https://github.com//Byron/gitoxide/commit/30925e654144a05365908f7d2aa90deb7b2952d3))
    - support for logging in pretty binaries ([`67026e4`](https://github.com//Byron/gitoxide/commit/67026e479f0aa3e47ff3fd230c8741a7a5dbe99c))
    - --verbose flag for lean plumbing binary ([`aaf4825`](https://github.com//Byron/gitoxide/commit/aaf482584d1ee080c0a6c091c4675736c4c8d6a7))
    - first very basic progress implementation ([`b820717`](https://github.com//Byron/gitoxide/commit/b8207177daee8a9ffa23c7c052cf9ca651b15804))
    - Pass progress everywhere, for now just to discard it ([`da3ae1c`](https://github.com//Byron/gitoxide/commit/da3ae1c82cd726b8fae9b8d26069719930e9ba99))
    - split plumbing into separate binary ([`b1e51d6`](https://github.com//Byron/gitoxide/commit/b1e51d6a83ca7a00923b39209d0a2bfb3b78de0d))
    - refactor ([`0fbba9f`](https://github.com//Byron/gitoxide/commit/0fbba9fe7597af03912f956c251c88472b48c3eb))
    - refactor ([`ba6a8ef`](https://github.com//Byron/gitoxide/commit/ba6a8ef064a9884066414c82f4f7d1bb72ab524f))
    - add initial version of 'lean-cli' feature toggle, but… ([`f01c298`](https://github.com//Byron/gitoxide/commit/f01c2985732ac05b24a7fcbc3752ef52dd1bc438))
    - Support for verifying pack files and index files ([`b09b4e1`](https://github.com//Byron/gitoxide/commit/b09b4e1f35c3802dfd3418bda42b96828acd9ec8))
    - reorganize crates to make 'gitoxide' the CLI, and 'gitoxide-core' the library ([`0ac9c5a`](https://github.com//Byron/gitoxide/commit/0ac9c5af0cbb562d3cb48a661736afd98dd1a940))
    - Add simple pack verification to gio ([`8c0e0b5`](https://github.com//Byron/gitoxide/commit/8c0e0b5bb79c8c337eed03d37cbf818d8bb9c924))
    - goodbye git-core, hello git-repository ([`7cec2b6`](https://github.com//Byron/gitoxide/commit/7cec2b648f86fc665b4fc5bfe269e9ca16679a55))
    - document existing use of unsafe, deny everywhere else ([`41f4bce`](https://github.com//Byron/gitoxide/commit/41f4bce9d9a492f8e20a6eb5b3eaf5adc6d78329))
    - cargo clippy ([`1179ac1`](https://github.com//Byron/gitoxide/commit/1179ac16ea2bb84816f9b615d1191f8a2d4e775b))
    - move parsing tests close to actual parsing ([`3ca2c59`](https://github.com//Byron/gitoxide/commit/3ca2c592d91c9aa8fab8ed749871d6d96f2ef4e2))
    - color for all grit commands/subcommands ([`aa8efdd`](https://github.com//Byron/gitoxide/commit/aa8efdd922d45bdab668dc71e8b30adf79930667))
    - use structopt instead of clap ([`eb7388c`](https://github.com//Byron/gitoxide/commit/eb7388c5d51e4ef3ea928d5f8f9e5b218cdbbd57))
    - Remove failure from grit binary, too ([`417c34b`](https://github.com//Byron/gitoxide/commit/417c34b82469bcc3391706646dd39c7f6d1ad69c))
    - refactor ([`87c8a2e`](https://github.com//Byron/gitoxide/commit/87c8a2e288140b04e163fe85266d040d039ec69c))
    - cargo fmt ([`2aa0857`](https://github.com//Byron/gitoxide/commit/2aa085752aa3e99b51034a3dec882aea8c27ad94))
    - implement git-init ([`57737c2`](https://github.com//Byron/gitoxide/commit/57737c2c48ff898a327ba57712fea21b5d83188e))
    - Initial commit - based on standard project template ([`c3d319f`](https://github.com//Byron/gitoxide/commit/c3d319f2b3076a0bb169bcd8a7b6a011f6aba9a5))
</details>

