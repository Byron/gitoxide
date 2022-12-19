# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.8.0 (2022-12-19)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'http-config' ([`a4ff140`](https://github.com/Byron/gitoxide/commit/a4ff140a0d3607cf282c49228c1248bd36d464fd))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
</details>

## 0.7.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 14 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Improve docs ever so slightly ([`ca5d89c`](https://github.com/Byron/gitoxide/commit/ca5d89c6c94ca5e26098fcbe449a723e6a6b4b69))
</details>

## 0.6.1 (2022-11-06)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 2 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
</details>

## 0.6.0 (2022-10-10)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.5.0 (2022-09-20)

<csr-id-d95029eac0e9179a7cd730d1d60a08696584bfd1/>
<csr-id-b8c54f03fdb6060caf9c04557c0530c090e7a975/>
<csr-id-4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1/>

### Changed

 - <csr-id-43656d5ce84834c847cf8650d4c486c634f209b6/> use `git-config-value` crate

### New Features

 - <csr-id-15f1afccb7ed0ebaf217cbbdd58e6ae651a31e42/> `protocol::Context::to_bstring()`, and use it in `example/git-credential-lite`
 - <csr-id-b1d528ae60001ae51dd89b29c26ea505eacbef45/> an example implementing a custom credential helper program
 - <csr-id-eaff67c14366f149ccca346acb46af12531a24e6/> `helper::main` to easily create credential helper implementations
 - <csr-id-a253d30093122e37b5560ff86a7888f8062c7014/> add `helper::Action::get_for_url(…)`
 - <csr-id-64bc2ec666dacba486bd1de2fbd95f97f2efc7a5/> `helper::invoke(helper, action, context)` function that allows for more flexible helper invocation

### Other

 - <csr-id-d95029eac0e9179a7cd730d1d60a08696584bfd1/> :main::Action::as_str()`

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-783a1a7dfd64a64fa765fa3d3ef41b1e954413ee/> rename `git()` to `builtin()`
 - <csr-id-bfa2545883daf8c4d9e97d2fc91c9328d73ab0eb/> rename `Program::Custom*` variants to `Program::External*`
   It's more true to what it is.
 - <csr-id-811985aba024385465104ed826a9989961555201/> differentiate between top-level functions and those which are invoked
   That way it's easier to use as it can assure an account was actually
   provided.
 - <csr-id-49b9bd501f33f1e10ce0180e814b84e293bd3898/> invoke::Outcome can now represent partial identities
   That way these can be assembled by multiple helpers called in a row.
 - <csr-id-4b7d0b6d2c43cac9823885bc69510cc4bb6a3f00/> move `helper::(Next)Action` into `helper::invoke::` module
   These are only relevant for invoke, hence the change.
 - <csr-id-ddd53988a6d5da17fc65451a059bed1bfa2eb454/> rename `helper::NextAction` variants to `store` and `erase`
 - <csr-id-2073b583dc2bd83b800584edda6592bb71a01538/> rename `helper::Action` variants to 'Get', 'Store', 'Erase'
   It's more obvious what it does and is more typical for what credentials
   helpers do.
 - <csr-id-9c6f024f838d866645937a67cd67dffb8be17259/> Use `helper::Context` in `helper::Action::Fill()`
   That way additional information, like from configuration, can be passed
   as well.
 - <csr-id-71f651930e6fd53e3c3f9e82dfd95828e4981d92/> move `helper::invoke()` related types into `helper::invoke` module.
   Also allow to pass arbitrary bytes (more or less) as context by not
   forcing it all into a string. Values can now be everything, which
   helps with passing paths or other values.
 - <csr-id-4c1a1a28558c4f8d084b8046afd5d87a11fd25b7/> use `thiserror` instead of `quickerror`
 - <csr-id-081934ca4452e550cf2663026905bce67253af81/> hide `helper::action()` in favor of single path via `helper()`

### Other (BREAKING)

 - <csr-id-b8c54f03fdb6060caf9c04557c0530c090e7a975/> `helper::Kind` to `program::Kind`
 - <csr-id-4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1/> `helper::(encode|decode)_message(…)` to `::message::(encode|decode)(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 113 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 21 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - :main::Action::as_str()` ([`d95029e`](https://github.com/Byron/gitoxide/commit/d95029eac0e9179a7cd730d1d60a08696584bfd1))
    - `protocol::Context::to_bstring()`, and use it in `example/git-credential-lite` ([`15f1afc`](https://github.com/Byron/gitoxide/commit/15f1afccb7ed0ebaf217cbbdd58e6ae651a31e42))
    - assure that protocol::Context::to_url() never shows passwords ([`e9f4d40`](https://github.com/Byron/gitoxide/commit/e9f4d40b6f04414c04f153f74f13d2e1fe89e43a))
    - Make sure the helper::Cascade never sends the URL to helper programs ([`9059de8`](https://github.com/Byron/gitoxide/commit/9059de825d310c2c28f28d4009b09115acccc2bf))
    - fix docs ([`9a5ec7b`](https://github.com/Byron/gitoxide/commit/9a5ec7bd8b23bbef2c21de07638681160a7bbdee))
    - move `program::Cascade` to `helper::Cascade` which is what it is ([`6149a14`](https://github.com/Byron/gitoxide/commit/6149a14af1742bcfc30bfbe65656b411c6f771c9))
    - An example on how to invoke the git credential helper driver ([`be0f834`](https://github.com/Byron/gitoxide/commit/be0f83411371e445beceabfcc6e0458eedccf31a))
    - Context has to serialize url or else the builtin credential helper may not work. ([`87ae404`](https://github.com/Byron/gitoxide/commit/87ae40493cc0dbe11e5de5fd21e2391caa7161db))
    - credential context won't send url and quit fields to helpers ([`337a53b`](https://github.com/Byron/gitoxide/commit/337a53b945da26e253c9ba1c72b623d6a06d2e7c))
    - Cascade supports `use_http_path` and respects it when setting up the context ([`959c0bd`](https://github.com/Byron/gitoxide/commit/959c0bdfb6a634f590969f2c26d13ff8c05a4bb8))
    - make it less easy to start a cascade with platform_defaults() ([`4b5d63f`](https://github.com/Byron/gitoxide/commit/4b5d63f7e0ea6bc43f54c95dd30f823ead9fa1a2))
    - make clearer what platform builtins actually are ([`9788e30`](https://github.com/Byron/gitoxide/commit/9788e3070edc5c1d84099a2fc5fa9262604170e7))
    - credential-cascade now passes on prompt options ([`baad8a0`](https://github.com/Byron/gitoxide/commit/baad8a077ffd556cb29da93fb0081b245f4663ff))
    - refactor ([`c8f1b41`](https://github.com/Byron/gitoxide/commit/c8f1b41408f2ace5b01292ef95189b9e66ab4d8e))
    - always compile prompting support in ([`bd0ea68`](https://github.com/Byron/gitoxide/commit/bd0ea68225a73fb83c9fc1b8594fc6ad288a77a9))
    - set version of git-prompt to 0.1 and turn prompting on ([`7657693`](https://github.com/Byron/gitoxide/commit/7657693b8e23dfb69d6da4376bcd1b8e4e264f7e))
    - fix warnings ([`e011242`](https://github.com/Byron/gitoxide/commit/e011242c0c9f6779632f5d33dc7b185495f3868e))
    - more helpful prompt error messages when asking for credentials ([`b0c6863`](https://github.com/Byron/gitoxide/commit/b0c6863e6b91ded34ed1860ed449f797d28be81e))
    - use `git-config-value` crate ([`43656d5`](https://github.com/Byron/gitoxide/commit/43656d5ce84834c847cf8650d4c486c634f209b6))
    - proper prompt generation ([`63ee38d`](https://github.com/Byron/gitoxide/commit/63ee38dab45fd9d07532f6c01afc2d8dd1c1e904))
    - remove rustyline in favor of `git-prompt` ([`b3e5e59`](https://github.com/Byron/gitoxide/commit/b3e5e59cafaab0d4866c52722cd2a67aa313b395))
    - add interactive example for prompt, but… ([`a3fadea`](https://github.com/Byron/gitoxide/commit/a3fadea7759a20fe409762e48d0f1bb9c07f39ba))
    - blindly implement prompting if it is allowed ([`c78f4b8`](https://github.com/Byron/gitoxide/commit/c78f4b80d1554fdae49d8d5e7d1cfef6c1bf3b05))
    - frame to support prompting (as compile-time feature) ([`afaae28`](https://github.com/Byron/gitoxide/commit/afaae2880a77c30f845ccf2b3c2b7dc5210665f8))
    - another test ([`569b7bc`](https://github.com/Byron/gitoxide/commit/569b7bc3d8d8acfe8ad16fe1bc0480e3dbd263d2))
    - remove unnecessary `Helper` trait ([`19b84f0`](https://github.com/Byron/gitoxide/commit/19b84f0636f6a8d28e938c3a56b3e2cf0a3b4711))
    - use fixtures in all tests ([`24da911`](https://github.com/Byron/gitoxide/commit/24da911f2fcbc0073fcdab1a217686ac3e3b1c79))
    - fix tests on linux ([`89db8ee`](https://github.com/Byron/gitoxide/commit/89db8ee938f05f8f9066f34325619f434a5ea00f))
    - more tests ([`57e9060`](https://github.com/Byron/gitoxide/commit/57e906094683860b43f5b7ff71e0189bd2fd0a91))
    - refactor ([`561bb35`](https://github.com/Byron/gitoxide/commit/561bb356850715c2f4377dd36d1daff69126f543))
    - another test ([`52d2e54`](https://github.com/Byron/gitoxide/commit/52d2e547b18aa5a00d9d1ada9c88bd84e951e1ed))
    - fix CI ([`d526c6d`](https://github.com/Byron/gitoxide/commit/d526c6d111bfa05dfa20aca8426d78217ae41558))
    - improve path normalization; a new ignored test ([`ece5a3f`](https://github.com/Byron/gitoxide/commit/ece5a3f16bfbf84eddce42c64c32736ad98b5356))
    - parse 'quit' according to spec ([`5e260da`](https://github.com/Byron/gitoxide/commit/5e260dab2edd40092501ab52684f6370104a4eb1))
    - Allow disabling stderr on credential programs ([`4abec50`](https://github.com/Byron/gitoxide/commit/4abec50dc620e965fc03dda4c801753365839691))
    - refactor ([`cdfcea4`](https://github.com/Byron/gitoxide/commit/cdfcea4eb92097927d4c90639fc211e427b6415c))
    - url-preprocessing for scripts ([`c00cc35`](https://github.com/Byron/gitoxide/commit/c00cc35493cec8f0b2673248caf1f0d83590dd54))
    - breaking credential helpers don't stop everything ([`0cdbde7`](https://github.com/Byron/gitoxide/commit/0cdbde78a200ff8585fb217bab3daf81ff46dd6e))
    - refactor; try harder not to spill secrets in errors ([`525fa97`](https://github.com/Byron/gitoxide/commit/525fa9748b966d515fbdeaa48abd34798e97b78e))
    - first step towards our own `git credential` -like implementation ([`1d1622a`](https://github.com/Byron/gitoxide/commit/1d1622a0dd66ce181d0fa07fc440f85ad0212791))
    - refactor ([`ce16f51`](https://github.com/Byron/gitoxide/commit/ce16f513dc0a482583cdff168dcdbe2cdd379ad7))
    - Platform specific defaults for the program cascade ([`b66258f`](https://github.com/Byron/gitoxide/commit/b66258f3827e8ca4c7da4a5bca7768888a09e6d5))
    - refactor ([`85f8cd9`](https://github.com/Byron/gitoxide/commit/85f8cd9b9ef9e93c6495495a83b1ec96437672a5))
    - refactor ([`23fb302`](https://github.com/Byron/gitoxide/commit/23fb3025112d2f627896383fb0f74f7e91139116))
    - A sketch of how a custom 'git credential' could look like ([`4767a14`](https://github.com/Byron/gitoxide/commit/4767a14d2390edacf46d5436a07685b7d7b79cab))
    - make 'quit' handler request representable and raise it to an error ([`39b6514`](https://github.com/Byron/gitoxide/commit/39b6514928304807b3d43bd60be716a7f42169c7))
    - rename `git()` to `builtin()` ([`783a1a7`](https://github.com/Byron/gitoxide/commit/783a1a7dfd64a64fa765fa3d3ef41b1e954413ee))
    - fix docs ([`f86364c`](https://github.com/Byron/gitoxide/commit/f86364c4e2d9efd04027978679232946494a4734))
    - rename `Program::Custom*` variants to `Program::External*` ([`bfa2545`](https://github.com/Byron/gitoxide/commit/bfa2545883daf8c4d9e97d2fc91c9328d73ab0eb))
    - refactor ([`52e958d`](https://github.com/Byron/gitoxide/commit/52e958d62cdf49c35ed56cb26699b155ee0e7732))
    - fix build ([`99958c6`](https://github.com/Byron/gitoxide/commit/99958c6f87a09b99f21b88e42095a1326d6b8a82))
    - differentiate between top-level functions and those which are invoked ([`811985a`](https://github.com/Byron/gitoxide/commit/811985aba024385465104ed826a9989961555201))
    - invoke::Outcome can now represent partial identities ([`49b9bd5`](https://github.com/Byron/gitoxide/commit/49b9bd501f33f1e10ce0180e814b84e293bd3898))
    - make clear what `helper()` does by renaming it to `git` ([`2edb58b`](https://github.com/Byron/gitoxide/commit/2edb58b6c7395b67c8a7f7c9f6162e6e7c290aac))
    - Make clear in the error type that the helper program couldn't be started ([`c09d223`](https://github.com/Byron/gitoxide/commit/c09d2234cb7e89a2b6ae54e7c8497e86b38621f0))
    - improved error when identity could not be obtained ([`08c1287`](https://github.com/Byron/gitoxide/commit/08c12876d763a4ade3d4013ce1be66d9594e4ff1))
    - support for `quit` field in context ([`5a50528`](https://github.com/Byron/gitoxide/commit/5a50528a6f2b1a547fdc9a656e5ea2ca07396ecf))
    - refactor ([`7487b5a`](https://github.com/Byron/gitoxide/commit/7487b5a4142679ef423c5bd996e40e473c5dfc27))
    - support for non-consuming operation of `Program` ([`bcfe5ca`](https://github.com/Byron/gitoxide/commit/bcfe5ca22636114bb232d1208ab7c9d78d1fe1de))
    - disable test that seems to fail on linux ([`419ca56`](https://github.com/Byron/gitoxide/commit/419ca56f7a97cdb0c0e18a4a6f8fda6320692518))
    - More tests for custom helper scripts ([`b396032`](https://github.com/Byron/gitoxide/commit/b3960320d1ef86b42fe8d42c8d7f7abfe66e1710))
    - Support for script invocations ([`377cf14`](https://github.com/Byron/gitoxide/commit/377cf142996279394af38179ad5b51c419642f90))
    - use git_path::is_absolute() ([`2ba836f`](https://github.com/Byron/gitoxide/commit/2ba836f3e9e5231e8bc42d57d8ff939d85acfe16))
    - fix tests on windows ([`f4bc860`](https://github.com/Byron/gitoxide/commit/f4bc86011d4aafb5bdeafadd43adb0022ff9b449))
    - also fill in the git credential command prefix ([`b2f4fe8`](https://github.com/Byron/gitoxide/commit/b2f4fe8f96785222edc3c0472ccef3acf1f069f8))
    - initial version of parsing of custom helper definitions ([`2b2cd00`](https://github.com/Byron/gitoxide/commit/2b2cd0001babdc16e940fa7242c6d723fc9f789b))
    - `helper::Kind` to `program::Kind` ([`b8c54f0`](https://github.com/Byron/gitoxide/commit/b8c54f03fdb6060caf9c04557c0530c090e7a975))
    - sketch additional credentials programs ([`46e3045`](https://github.com/Byron/gitoxide/commit/46e3045e04e5197560d8c786642b8f1924a577f9))
    - first test for launching the git credential helper ([`4d7b1dd`](https://github.com/Byron/gitoxide/commit/4d7b1ddec6ef747665edcfddbba68ed12e3970c2))
    - an example implementing a custom credential helper program ([`b1d528a`](https://github.com/Byron/gitoxide/commit/b1d528ae60001ae51dd89b29c26ea505eacbef45))
    - add docs ([`a360594`](https://github.com/Byron/gitoxide/commit/a360594fac3102cd48aac0039efbe71693c5fa25))
    - `helper::main` to easily create credential helper implementations ([`eaff67c`](https://github.com/Byron/gitoxide/commit/eaff67c14366f149ccca346acb46af12531a24e6))
    - move `helper::(Next)Action` into `helper::invoke::` module ([`4b7d0b6`](https://github.com/Byron/gitoxide/commit/4b7d0b6d2c43cac9823885bc69510cc4bb6a3f00))
    - sketch for helper::invoke (get) test ([`c48eb39`](https://github.com/Byron/gitoxide/commit/c48eb390a2f95954f542992806d4e8667ee97981))
    - refactor ([`cb9d32a`](https://github.com/Byron/gitoxide/commit/cb9d32a3611463f983afea3b3ea875c33092207b))
    - rename `helper::NextAction` variants to `store` and `erase` ([`ddd5398`](https://github.com/Byron/gitoxide/commit/ddd53988a6d5da17fc65451a059bed1bfa2eb454))
    - fix docs ([`d9b4ba5`](https://github.com/Byron/gitoxide/commit/d9b4ba5a00c1c9f9c199ac218da76cb716896b75))
    - add `helper::Action::get_for_url(…)` ([`a253d30`](https://github.com/Byron/gitoxide/commit/a253d30093122e37b5560ff86a7888f8062c7014))
    - rename `helper::Action` variants to 'Get', 'Store', 'Erase' ([`2073b58`](https://github.com/Byron/gitoxide/commit/2073b583dc2bd83b800584edda6592bb71a01538))
    - Use `helper::Context` in `helper::Action::Fill()` ([`9c6f024`](https://github.com/Byron/gitoxide/commit/9c6f024f838d866645937a67cd67dffb8be17259))
    - remaining decode tests ([`0e76efe`](https://github.com/Byron/gitoxide/commit/0e76efe035a48f9d042096342ac79804f1eeebdc))
    - test context value validation ([`20dde9e`](https://github.com/Byron/gitoxide/commit/20dde9eb93ecfb56e72bc5d59caacf31328a55e4))
    - basic round-tripping of fully fleshed-out context ([`280e4a3`](https://github.com/Byron/gitoxide/commit/280e4a3f69699e11428decc6858711b35ae8249e))
    - flesh out `helper::Context` as it will soon be used. ([`0cb1ed4`](https://github.com/Byron/gitoxide/commit/0cb1ed4600c614169118b2a94fed83699989a6be))
    - move `helper::invoke()` related types into `helper::invoke` module. ([`71f6519`](https://github.com/Byron/gitoxide/commit/71f651930e6fd53e3c3f9e82dfd95828e4981d92))
    - refactor ([`03bf747`](https://github.com/Byron/gitoxide/commit/03bf747292af7792bc175c4f06939b1e02f7234c))
    - express `helper()` in terms of `helper::invoke()` ([`f2a2c5e`](https://github.com/Byron/gitoxide/commit/f2a2c5ebb7d7428460fe22e9b84dec242a992302))
    - `helper::invoke(helper, action, context)` function that allows for more flexible helper invocation ([`64bc2ec`](https://github.com/Byron/gitoxide/commit/64bc2ec666dacba486bd1de2fbd95f97f2efc7a5))
    - refactor ([`af27d20`](https://github.com/Byron/gitoxide/commit/af27d20909d14f2737fbad5edd9a6c9d86c93e24))
    - prepare for more additional implementations of helpers ([`486ef98`](https://github.com/Byron/gitoxide/commit/486ef98b792cc57412a4a90d2cf28586a06d7041))
    - refactor ([`167b521`](https://github.com/Byron/gitoxide/commit/167b5215326ff2f39e89f2130ff575f4ef6c02d6))
    - fix docs ([`db46b60`](https://github.com/Byron/gitoxide/commit/db46b60d8f9b4341cf215da6e2cd74bf554fe4b8))
    - re-add `Result` type ([`de92fce`](https://github.com/Byron/gitoxide/commit/de92fce44496b050e5697aab6d6d1ea98a5954dc))
    - use `thiserror` instead of `quickerror` ([`4c1a1a2`](https://github.com/Byron/gitoxide/commit/4c1a1a28558c4f8d084b8046afd5d87a11fd25b7))
    - hide `helper::action()` in favor of single path via `helper()` ([`081934c`](https://github.com/Byron/gitoxide/commit/081934ca4452e550cf2663026905bce67253af81))
    - `helper::(encode|decode)_message(…)` to `::message::(encode|decode)(…)` ([`4c39521`](https://github.com/Byron/gitoxide/commit/4c39521a47419bb4b0f0edbe51aa509fb4e2a7f1))
    - refactor ([`a395308`](https://github.com/Byron/gitoxide/commit/a395308fdc01b5449a851b1dcb6c3e97a205a5d0))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - thanks clippy ([`c1399d1`](https://github.com/Byron/gitoxide/commit/c1399d155889e6142eafd65b9bbd2ed005f580dd))
    - thanks clippy ([`e8e80f5`](https://github.com/Byron/gitoxide/commit/e8e80f5b176ebca4ee81727a551d83383a0b38f8))
    - thanks clippy ([`9b8a6d6`](https://github.com/Byron/gitoxide/commit/9b8a6d6afeab13968dea61619b1e742e93f60fab))
    - thanks clippy ([`8317b46`](https://github.com/Byron/gitoxide/commit/8317b4672c8cd38520ed90c42eaadd539ea4df66))
    - thanks clippy ([`01efe88`](https://github.com/Byron/gitoxide/commit/01efe88cff52e75ba0b76ecc27a41994ee908d2c))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.4.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Changed (BREAKING)

 - <csr-id-12589cc6f08e4d7aabae30bcdadaa0c2b4850229/> adapt to changes in `git-url` and use `BString` to represent URLs.
   They can contain paths, which is why `String` can't repsent a URL
   losslessly.
   
   For HTTP urls these are ultimately UTF-8 strings though.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 30 calendar days.
 - 32 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - adapt to changes in `git-url` and use `BString` to represent URLs. ([`12589cc`](https://github.com/Byron/gitoxide/commit/12589cc6f08e4d7aabae30bcdadaa0c2b4850229))
 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - refactor ([`1dc342f`](https://github.com/Byron/gitoxide/commit/1dc342f9a60cb20e1fafa8c7e913c4a957367662))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
</details>

## 0.3.0 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 33 calendar days.
 - 39 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.2.0 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 25 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.1.0 (2022-05-18)

### New Features

 - <csr-id-3d339d5c24630fac0192b5d27f9b1cbd94418730/> use `git-sec::Identity` type
   It's shared across crates.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 33 calendar days.
 - 33 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#386](https://github.com/Byron/gitoxide/issues/386)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - more details for path permissions ([`ca26659`](https://github.com/Byron/gitoxide/commit/ca26659eb870c8e947962fe0647a07d01b3e95e4))
    - adapt to changes in git-sec ([`c5e2346`](https://github.com/Byron/gitoxide/commit/c5e2346cee53019b1b321e45cf080b210e60bb7a))
    - use `git-sec::Identity` type ([`3d339d5`](https://github.com/Byron/gitoxide/commit/3d339d5c24630fac0192b5d27f9b1cbd94418730))
    - fill git-credentials with existing impleemntation ([`6016c22`](https://github.com/Byron/gitoxide/commit/6016c2252aea6892a813b7dc1b0c870a156b3cfd))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'main' into repo-status ([`9679d6b`](https://github.com/Byron/gitoxide/commit/9679d6b0e68c28438e22cb65c554d0b31dfaf159))
    - Merge branch 'git-sec' ([`cd723b5`](https://github.com/Byron/gitoxide/commit/cd723b5ae11148e7e9fd07daf28bc04455d5c46f))
</details>

## 0.0.0 (2022-04-15)

An empty crate without any content to reserve the name for the gitoxide project.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#386](https://github.com/Byron/gitoxide/issues/386)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - add frame for git-credentials crate ([`be7a9cf`](https://github.com/Byron/gitoxide/commit/be7a9cf776f958ac7228457bb4e1415f86f8e575))
 * **Uncategorized**
    - Release git-credentials v0.0.0 ([`7db45ab`](https://github.com/Byron/gitoxide/commit/7db45abb822b7c28ac84bf0229ec23ce0f46c8f2))
</details>

