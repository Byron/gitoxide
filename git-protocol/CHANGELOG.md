# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.12.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com//Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com//Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com//Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - stabilize changelogs ([`920e832`](https://github.com//Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com//Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
</details>

## v0.11.0 (2021-10-15)

<csr-id-da68bfb8104ecf58e73e3f99d87f81c90712a2ca/>
<csr-id-c77bd7a01820110154f2c66cd954c1ccfff173c1/>

### Bug Fixes

 - <csr-id-c77bd7a01820110154f2c66cd954c1ccfff173c1/> '(null)' symref targets are turned into direct refs instead
   
   because that's what 'git' would do if it would care enough.
   On the client side this means one has to deal with detached heads
   and it's obvious that these are detached. It's not clear anymore
   why this is the case, and probably it's good to hide it as the
   current git behaviour is accidental and may change in future, and
   do so in a way we predict.
   
   There is the possibility that git would abort the entire
   fetch/upload-pack operation, but that's already handled correctly
   by our implementation as well as we understand ERR messages in packet
   lines.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 27 commits contributed to the release over the course of 31 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#198](https://github.com//Byron/gitoxide/issues/198), [#200](https://github.com//Byron/gitoxide/issues/200), [#205](https://github.com//Byron/gitoxide/issues/205)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com//Byron/gitoxide/issues/198)**
    - deduplicate conventional message ids ([`e695eda`](https://github.com//Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com//Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Support writing whole bodies in conventional messages… ([`c1f3c9d`](https://github.com//Byron/gitoxide/commit/c1f3c9d2bd5a8e123ac9b376c257e3d5630876a0))
    - Support for paragraphs in conventional items ([`7f52823`](https://github.com//Byron/gitoxide/commit/7f528239089788f4dd1f75a85bee1d0492285d60))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com//Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com//Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com//Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com//Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com//Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com//Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com//Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com//Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - fix docs ([`f1fa6dd`](https://github.com//Byron/gitoxide/commit/f1fa6dd98c3bc0380a126adbb059a2dc1e8a0924))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com//Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com//Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com//Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`2f43a54`](https://github.com//Byron/gitoxide/commit/2f43a54298e7ecfff2334627df149fe0882b5d1d))
 * **[#200](https://github.com//Byron/gitoxide/issues/200)**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… ([`f293b63`](https://github.com//Byron/gitoxide/commit/f293b633d16c0f7393d0ede64e12f14e47d0296b))
 * **[#205](https://github.com//Byron/gitoxide/issues/205)**
    - '(null)' symref targets are turned into direct refs instead… ([`c77bd7a`](https://github.com//Byron/gitoxide/commit/c77bd7a01820110154f2c66cd954c1ccfff173c1))
    - fetch::Ref::Symbolic::target is now an option… ([`da68bfb`](https://github.com//Byron/gitoxide/commit/da68bfb8104ecf58e73e3f99d87f81c90712a2ca))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com//Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com//Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com//Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com//Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com//Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com//Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.10.4 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.4 ([`898ee08`](https://github.com//Byron/gitoxide/commit/898ee08befa1eb7dd22980063c7633f83d0a8958))
    - thanks clippy ([`4701296`](https://github.com//Byron/gitoxide/commit/4701296bd5e2c4ad2f80f4e1de498db49f93385a))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.10.3 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.3 ([`aa90f98`](https://github.com//Byron/gitoxide/commit/aa90f98eb45e93b629462b629660e38b1824c405))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com//Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
</details>

## v0.10.2 (2021-08-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.2 ([`54a4400`](https://github.com//Byron/gitoxide/commit/54a44009e3507ee1c53a51a5f3b6735b6115a887))
    - [various #184] configure docs.rs build features ([`cc50249`](https://github.com//Byron/gitoxide/commit/cc502492c512293e93e95610ca80a71896076ded))
</details>

## v0.10.1 (2021-08-27)

- instruct docs.rs which features to use for more useful documentation

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.10.1 ([`cec8ee3`](https://github.com//Byron/gitoxide/commit/cec8ee3709ed401303cdd412a53e73f91eced619))
    - [protocol #174] fix tests… ([`cdc16fc`](https://github.com//Byron/gitoxide/commit/cdc16fc0ef42df4a17ec4fde4be4511ee2cdaed6))
</details>

## v0.10.0 (2021-08-27)

- Various minor updates of pre-release dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 3 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-protocol v0.10.0 ([`82d5a0b`](https://github.com//Byron/gitoxide/commit/82d5a0bb38903a8389e43cd5416e02e5496e661a))
    - Bump git-transport v0.11.0 ([`1149f1b`](https://github.com//Byron/gitoxide/commit/1149f1b716624f8f4fdaed20c803530aebc45599))
    - Bump git-packetline v0.10.0 ([`b09f391`](https://github.com//Byron/gitoxide/commit/b09f3912e0addd7b4b0ef22bc3a24869d5011646))
    - [packetline #178] rename PacketLine to PacketLineRef… ([`d4c16a9`](https://github.com//Byron/gitoxide/commit/d4c16a93946244177606b58cc702b81a16424ad4))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com//Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com//Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com//Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
</details>

## v0.9.0 (2021-08-17)

### BREAKING

- Add fifth argument to `fetch(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [protocol] prepare release to fix crates-io instalations ([`83d7423`](https://github.com//Byron/gitoxide/commit/83d74239108df420bd464c340762c1dfcb6ae78a))
    - bump git-protocol to v0.9.0 as there are breaking changes ([`b4e3340`](https://github.com//Byron/gitoxide/commit/b4e33408b8eb12c9418704f663322385fd1dfb25))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.8.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-protocol v0.8.1 ([`b57c339`](https://github.com//Byron/gitoxide/commit/b57c3397706940354c493cadf1ab93916b79f917))
    - Release git-transport v0.10.0 ([`b944278`](https://github.com//Byron/gitoxide/commit/b94427835bf922aa9388cdd78200c79a3c31da43))
    - Release git-packetline v0.9.0 ([`7ffbd60`](https://github.com//Byron/gitoxide/commit/7ffbd602c08605026b0bb97ab85216907badaf09))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com//Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - bump transport version to 0.10 ([`f26a3d3`](https://github.com//Byron/gitoxide/commit/f26a3d3a2745f3eb69d76e0cfd718a90cf74f003))
    - (cargo-release) version 0.8.0 ([`ad6d7f9`](https://github.com//Byron/gitoxide/commit/ad6d7f9c2b4f8879d466e758fc9b51ece6879e96))
    - (cargo-release) version 0.7.0 ([`2ef3106`](https://github.com//Byron/gitoxide/commit/2ef3106eb84981e2dabd84f81362b4e44f938ea6))
    - [transport] A much better name for 'is_stateful()` ([`f15f1e8`](https://github.com//Byron/gitoxide/commit/f15f1e85fda76eef72c3754d625cf51e3c454eea))
    - [protocol] Make fetch-connection usage explicit ([`29696f9`](https://github.com//Byron/gitoxide/commit/29696f9b8e3ba3a72af1b099dac1c0866194d5ce))
</details>

## v0.8.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 101 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com//Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com//Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - [protocol RL-#741] Respect delegate configuration when running only ls-refs ([`65ce8e1`](https://github.com//Byron/gitoxide/commit/65ce8e1812ce820e3a0c40e39170339bf73234e5))
    - [protocol #145] Unify the `previous` and `previous_result` parameters… ([`96f77c7`](https://github.com//Byron/gitoxide/commit/96f77c78a08e975d367ca25ac5d07eb2253cf4e5))
    - [protocol] remove misleading documentation about ref-in-want ([`9a8f6b5`](https://github.com//Byron/gitoxide/commit/9a8f6b5480bf55f52315ddf86ac28771147a4664))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com//Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com//Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Bump async-trait from 0.1.50 to 0.1.51 ([`ce0b81e`](https://github.com//Byron/gitoxide/commit/ce0b81e8f5c652d389ff876844bc42bcfa687921))
    - Bump futures-io from 0.3.15 to 0.3.16 ([`3c23820`](https://github.com//Byron/gitoxide/commit/3c23820d3f0d3567f44215cdb0ad13ab675a201f))
    - [protocol] Delegate will indicate end-of-operation when fetch is done ([`928f75a`](https://github.com//Byron/gitoxide/commit/928f75ad939e35a159d3d2751d5d0f9d00d796af))
    - [protocol] Let 'fetch()' only be used via `git_protocol::fetch` ([`4bae2f9`](https://github.com//Byron/gitoxide/commit/4bae2f959bdf3f4a12b378f5734d9abdc25af36d))
    - thanks clippy ([`eccbecb`](https://github.com//Byron/gitoxide/commit/eccbecb938f0c84b63ad7e1ee17fb8113ce89c2e))
    - [protocol] fix build ([`38aca40`](https://github.com//Byron/gitoxide/commit/38aca4076037a6f8288c2cf483f134ea16c328d5))
    - [protocol] Allow both preparation delegate methods to fail ([`d89393b`](https://github.com//Byron/gitoxide/commit/d89393bbd5fce130a50855316ef364083c62eccd))
    - [protocol] start trying LsRefsAction::Abort(Box<dyn Error>)… ([`660b9dc`](https://github.com//Byron/gitoxide/commit/660b9dcc4e5249506a7656b038333f64b109261d))
    - [protocol] adjust description of fetch::Error to match io::Error sources ([`23dafc6`](https://github.com//Byron/gitoxide/commit/23dafc6e24377ad00b70c0235fd7a8ff107eee0a))
    - Revert "[ref] Try using BorrowMut to avoid blanket trait impls, but…" ([`8212536`](https://github.com//Byron/gitoxide/commit/8212536376341e673a6ef05221d20815659d92d3))
    - [ref] Try using BorrowMut to avoid blanket trait impls, but… ([`4bb9bba`](https://github.com//Byron/gitoxide/commit/4bb9bbad5b4e0c2e64a48a8e4a70a1b3af1ca3e3))
    - [protocol] only send flush packets in stateful connections ([`0995c22`](https://github.com//Byron/gitoxide/commit/0995c225c92a0dcccd2514b53abcf8400d9342e1))
    - [transport] remove Transport::close()… ([`4268a9b`](https://github.com//Byron/gitoxide/commit/4268a9bcf733413f7326be7af487a8fcdec1f71c))
    - [ref] rename Action::Close to Action::Cancel… ([`cac1f6c`](https://github.com//Byron/gitoxide/commit/cac1f6c757709797d193c6bca30e99fe40466ddc))
    - [transport] impl Delegate for &mut T: Delegate; refactor fetch() signature ([`2ded7f9`](https://github.com//Byron/gitoxide/commit/2ded7f9b2659ab8705ad6b896aaf6ca5afb12a6c))
    - [transport] implement Transport for &mut T: Transport as well ([`372fb81`](https://github.com//Byron/gitoxide/commit/372fb8183aff19bd0f2d17ea74409b2ca3a08511))
    - [protocol] fallible negotiation ([`e269a2c`](https://github.com//Byron/gitoxide/commit/e269a2cde18f604a36b33efb7e53f31ea5c45e2d))
    - [protocol] refactor ([`11b2fd1`](https://github.com//Byron/gitoxide/commit/11b2fd1250ff902dd084d8664f06732d4b69b4b3))
    - [protocol] refactor ([`967946a`](https://github.com//Byron/gitoxide/commit/967946a65f67cb1fc5d7bf6944a7e900ff3521c7))
    - [protocol] refactor ([`8dc425f`](https://github.com//Byron/gitoxide/commit/8dc425ff91d00d3315903f429d4009df6410ba77))
    - [protocol] assure we don't coerce refs into UTF-8 representation ([`5ceb64d`](https://github.com//Byron/gitoxide/commit/5ceb64dfed67b942100e2e36715903492d870c71))
    - [protocol] support ref-in-want ([`b6df400`](https://github.com//Byron/gitoxide/commit/b6df400dccd66ad2f01c80d2fa05b8f9bb130b23))
    - [transport] tests for extra parameters ([`fffd926`](https://github.com//Byron/gitoxide/commit/fffd926a3d5c6abfa732aa2305a4a05fdd06254d))
    - [protocol] extra_parameters are forwarded from delegate to handshake ([`03e3db3`](https://github.com//Byron/gitoxide/commit/03e3db3809bd031d7d0c151ada2542214d7e32c0))
    - [transport] unsupported protocol versions now abort the fetch operation ([`812aa3b`](https://github.com//Byron/gitoxide/commit/812aa3bc02a823cb9277847db905e76a50ee7413))
    - [transport] flexible version of version support check doesn't actually work :D ([`2b220f0`](https://github.com//Byron/gitoxide/commit/2b220f0758cb7a96a66b256552f13a020cdee3fc))
    - [protocol] make refs parsing functionality public ([`d6da891`](https://github.com//Byron/gitoxide/commit/d6da891419f66208a8820185dd165e62b7a01a6e))
    - [protocol] async-io path handles improved refs parsing ([`328ab9c`](https://github.com//Byron/gitoxide/commit/328ab9c4ce739fe79f12ae539ea37e50c541b786))
    - [protocol] first step towards keeping InternalRef internal in blocking-io ([`6c4ed2d`](https://github.com//Byron/gitoxide/commit/6c4ed2d4dd352b4218419a1a79269a49cc91a992))
    - refactor ([`24697bc`](https://github.com//Byron/gitoxide/commit/24697bc66363f8e8b1ff14a59fdf303ffdab132d))
    - [async-client] cleanup Send bounds! ([`c7dee44`](https://github.com//Byron/gitoxide/commit/c7dee44267462d5ece491b8a45cf35afa904ce81))
    - [async-client] refactor ([`b252932`](https://github.com//Byron/gitoxide/commit/b252932ee3eb26bb26560a849a9b13aca11cf00f))
    - [async-client] unblock the async delegate in the cheapest possible way… ([`a3b5d75`](https://github.com//Byron/gitoxide/commit/a3b5d75d387dc5d6c44f695f63df8803613637a2))
    - Revert "[async-client] a taste of what it means to unblock the delegate" ([`2ba452f`](https://github.com//Byron/gitoxide/commit/2ba452ff1c9659f7433328b12732d792e7871102))
    - [async-client] a taste of what it means to unblock the delegate ([`4d6c10a`](https://github.com//Byron/gitoxide/commit/4d6c10a6956bb9a81144a61ebb6bcab3aedb840e))
    - [async-client] prepare for unblocking the protocol delegate ([`796c7d5`](https://github.com//Byron/gitoxide/commit/796c7d54a20ef32a581be572e1d681f9727482de))
    - [async-client] refactor ([`0d5b911`](https://github.com//Byron/gitoxide/commit/0d5b911ad5f47ab8f044d6bbe660a6d1dfeecb5f))
    - Revert "[async-client] Try to bring 'Send' back but…" ([`52eb953`](https://github.com//Byron/gitoxide/commit/52eb953fcc44cce19604b1df6a600237b8c81392))
    - [async-client] Try to bring 'Send' back but… ([`3a06adb`](https://github.com//Byron/gitoxide/commit/3a06adb41f6b2946f78044e4ab1385e6441fc40f))
    - [git-protocol] fix test ([`e30ea36`](https://github.com//Byron/gitoxide/commit/e30ea363311aa82486828c59755a012cc76751b1))
    - [git-protocol] no warnings when building without client ([`2f30666`](https://github.com//Byron/gitoxide/commit/2f3066659280f7b43ca39d285166f11192ac7fa9))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com//Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - [git-protocol] remove compile warnings if no client type is specified… ([`478a980`](https://github.com//Byron/gitoxide/commit/478a98056afd2504050391262dabc921b59425c5))
    - thanks clippy ([`57106e2`](https://github.com//Byron/gitoxide/commit/57106e21089ae3c3a529295bceb8c0a515e2c2b6))
    - [git-protocol] builds without features work ([`a1945ff`](https://github.com//Byron/gitoxide/commit/a1945ff22f3412be1fbfac76236d487896ec4685))
    - [git-protocol] async fetch tests work ([`fe434a5`](https://github.com//Byron/gitoxide/commit/fe434a58d321b3ac12644827e65eb4db11cfe5fb))
    - [git-protocol] fetch tests nearly compile in async ([`97fb186`](https://github.com//Byron/gitoxide/commit/97fb186df5661fb297c2c9485186dbfe0ed1d504))
    - [git-protocol] fetch in sync and async… ([`4776039`](https://github.com//Byron/gitoxide/commit/47760399bffd030c848e0ef6df52a4765d8fb566))
    - [git-protocol] refactor ([`80379fd`](https://github.com//Byron/gitoxide/commit/80379fd32aae02f2975d8637326188655f85b474))
    - [git-protocol] build should fail if mutually exclusiive features are set ([`72cf940`](https://github.com//Byron/gitoxide/commit/72cf9401dda6e1bb465cce8d65ce66a7cc6a03fd))
    - Bump maybe-async from 0.2.4 to 0.2.6 ([`d99a1a8`](https://github.com//Byron/gitoxide/commit/d99a1a815809d22c7384c6ecb1275e39fb911d91))
    - [git-protocol] fix build ([`4cce648`](https://github.com//Byron/gitoxide/commit/4cce6487d6d514541afee1a9aa92043f186136d3))
    - [git-protocol] async Delegate ([`1aa6781`](https://github.com//Byron/gitoxide/commit/1aa678172f0eb75af76017addd3dff4d7e62ff41))
    - thanks clippy ([`0759ade`](https://github.com//Byron/gitoxide/commit/0759ade3e8e97927f452eabd11e249bb93aa54e2))
    - [git-protocol] refactor ([`94d7be4`](https://github.com//Byron/gitoxide/commit/94d7be4a16f2c2e68a9dacf120eef7a417a8a6b9))
    - [git-protocol] refactor ([`990099b`](https://github.com//Byron/gitoxide/commit/990099b01bfd54b926f0f4e7ecf727c423a23b8e))
    - [git-protocol] refactor ([`d623cf7`](https://github.com//Byron/gitoxide/commit/d623cf7db4488815ad5a2afd2d1bcbbbda275d2c))
    - [git-protocol] async response ([`c498557`](https://github.com//Byron/gitoxide/commit/c49855738bc164f65130cb307ba612b71c3fa83e))
    - [git-protocol] refactor ([`a8dc078`](https://github.com//Byron/gitoxide/commit/a8dc078e00d8a5689ba0d8070732421d35df50c8))
    - refactor ([`2eefe17`](https://github.com//Byron/gitoxide/commit/2eefe1712131a69298be02e94df8b6ba844afcd9))
    - [git-protocol] prepare response module for async ([`08b891b`](https://github.com//Byron/gitoxide/commit/08b891b089081a3ec3c44ed27b1aca316391d0de))
    - [git-protocol] fix tests without any feature toggles ([`1da0b1a`](https://github.com//Byron/gitoxide/commit/1da0b1ab9e22040d5b273a5604954859990e0334))
    - thanks clippy ([`91fdfba`](https://github.com//Byron/gitoxide/commit/91fdfba7cabc7331598903106a1dd7cea3b49eeb))
    - [git-protocol] refs now available in async ([`3a5b2cf`](https://github.com//Byron/gitoxide/commit/3a5b2cfcc50a48e09a6495c4c15af69596f966df))
    - [git-protocol] refactor ([`abf0b9d`](https://github.com//Byron/gitoxide/commit/abf0b9d41a2509d35102970602e77fb45e898d52))
    - [git-protocol] prepare to translate refs ([`bf79c91`](https://github.com//Byron/gitoxide/commit/bf79c91b30be61135dd33122bb93b3cf3a49f586))
    - [git-protocol] no warnings if there is no client feature set ([`335e831`](https://github.com//Byron/gitoxide/commit/335e83136efa7cb0913bc5e317bb49d616ee0290))
    - [git-protocol] fix tests in case there is no client feature set ([`1ee5518`](https://github.com//Byron/gitoxide/commit/1ee551878ef21d20925fab00b3eef044ada97065))
    - [git-protocol] refactor ([`0b4ff16`](https://github.com//Byron/gitoxide/commit/0b4ff166175dd51cded5131bcebf1edd80335abe))
    - [git-protocol] refactor ([`e99a03b`](https://github.com//Byron/gitoxide/commit/e99a03b360e4bb757904a03834297f14df67838f))
    - [git-protocol] async capabilities and arguments abstractions ([`aa3eacb`](https://github.com//Byron/gitoxide/commit/aa3eacbd53665d6b76bd9706d801d1189a970261))
    - [git-protocol] now just a dummy async transport impl and… ([`c7f0b80`](https://github.com//Byron/gitoxide/commit/c7f0b80182c08430a3720474eda41519b6814f17))
    - [git-protocol] a big step towards getting 'Arguments' test into async ([`5d1c30f`](https://github.com//Byron/gitoxide/commit/5d1c30f3ceae6fe26a0d9961d135b44f371d9cd7))
    - [git-protocol] move everything into `blocking_io` for later translation… ([`fa03374`](https://github.com//Byron/gitoxide/commit/fa03374fd42e127f5be7fb4da2bac85ea38c8afa))
    - [git-protocol] all blocking fetch tests ([`0d39b5d`](https://github.com//Byron/gitoxide/commit/0d39b5d23659d29a9f0e33428db401a3a887c007))
    - [git-protocol] re-introduce credentials helper code ([`6a5575f`](https://github.com//Byron/gitoxide/commit/6a5575fa7dbfa2a835fabf6746494097c3af23c2))
    - [git-protocol] separate test configuration for async mode ([`62a117c`](https://github.com//Byron/gitoxide/commit/62a117c4e6bd205c4bb1d224db7d8e80ba46322f))
    - [git-transport] fix git-protocol ([`0cc9537`](https://github.com//Byron/gitoxide/commit/0cc9537036003c86584223aa61f9c207a2c5c2df))
    - [git-protocol] simplify test setup ([`189ed2c`](https://github.com//Byron/gitoxide/commit/189ed2c32636ef59975dd15ec0ef61e8a62b98c0))
    - refactor ([`2ba9f91`](https://github.com//Byron/gitoxide/commit/2ba9f915035a518bef3eb8b0ed1c9972c4a47cfa))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com//Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - Switch to latest nom ([`859e57e`](https://github.com//Byron/gitoxide/commit/859e57eae93c3490523b7ed98f7a606acbd87a2f))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com//Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - Put prodash behind a feature toggle, too ([`966058d`](https://github.com//Byron/gitoxide/commit/966058d611c548e90c050462de52e36f1925e775))
    - [git-packetline] refactor ([`1328c5b`](https://github.com//Byron/gitoxide/commit/1328c5b4001f380936beff73e1f822f14e41e98b))
    - (cargo-release) version 0.6.0 ([`ec5a54e`](https://github.com//Byron/gitoxide/commit/ec5a54e9f3543afddc9f972f16135edc6ef6ff5b))
    - [git-packetline] refactor ([`e5769d1`](https://github.com//Byron/gitoxide/commit/e5769d1e7668ae54c667d2593c0c22e7723710c0))
    - (cargo-release) version 0.8.0 ([`ccea4b6`](https://github.com//Byron/gitoxide/commit/ccea4b6bcdaba0ee6c6a6236d225ea1276d2547c))
    - (cargo-release) version 0.9.0 ([`18f6d01`](https://github.com//Byron/gitoxide/commit/18f6d011043203523f1d0dacf657704ed3f9cf89))
    - [git-transport] simplify parsing capabilities from lines ([`401af09`](https://github.com//Byron/gitoxide/commit/401af0974742f10c8b9b3c9752e9d30205e96c16))
    - [git-protocol] separate tests those who need feature toggles ([`4a49d64`](https://github.com//Byron/gitoxide/commit/4a49d6406c9c39d75ab5021b6e213fd2c9d63adb))
    - [git-transport] remove default features to force being explicit everywhere ([`d1b39f8`](https://github.com//Byron/gitoxide/commit/d1b39f8093c032a172237a584c9208479611a866))
    - Fix git-protocol ([`284f8af`](https://github.com//Byron/gitoxide/commit/284f8af0599bee4e3de0e385b69a389713cee9f7))
    - refactor ([`1412282`](https://github.com//Byron/gitoxide/commit/141228219d33e8056489514f91221d803888edd8))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

## v0.7.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 21 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 ([`069184e`](https://github.com//Byron/gitoxide/commit/069184e55057a1655d2754cb1fd68a4424beff34))
    - (cargo-release) version 0.8.0 ([`411a05e`](https://github.com//Byron/gitoxide/commit/411a05ead1546c76fe51f359fbcb961a1140535e))
    - (cargo-release) version 0.5.0 ([`8c4cc3f`](https://github.com//Byron/gitoxide/commit/8c4cc3fb5922d1a761463bbbad65e59f91cce4cb))
    - thanks clippy ([`17258cc`](https://github.com//Byron/gitoxide/commit/17258cc58767caa6e71227898decd160ad0cdf13))
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com//Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com//Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com//Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
</details>

## v0.6.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com//Byron/gitoxide/issues/63)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com//Byron/gitoxide/issues/63)**
    - git-protocol uses `oid` type ([`3930a6f`](https://github.com//Byron/gitoxide/commit/3930a6ff508f5bb2249fb2c2f21e00b74fecda22))
    - refactor; better errors for invalid hash sizes ([`be84b36`](https://github.com//Byron/gitoxide/commit/be84b36129694a2e89d1b81d932f2eba23aedf54))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com//Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Remove all public exports of git-hash types in git-object ([`accf89d`](https://github.com//Byron/gitoxide/commit/accf89d25560e5ded6f44a1c4a898ee65d14f8f6))
    - Remove re-export of git_object::borrowed::Id ([`a3f2816`](https://github.com//Byron/gitoxide/commit/a3f28169c1268c1129852f279631d5a7f7540cdf))
    - Make git-hash Error usage explicit (it's for decoding only) ([`4805cfc`](https://github.com//Byron/gitoxide/commit/4805cfc8d837bb111424b5e32f46d0fb9b12365a))
 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`8513f0f`](https://github.com//Byron/gitoxide/commit/8513f0fafbf8ae61d86df2d8b0aefa52d3eb1680))
    - (cargo-release) version 0.7.0 ([`334b7e1`](https://github.com//Byron/gitoxide/commit/334b7e1b838b5201f2484be42dee3c4d2fd789d7))
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com//Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com//Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
</details>

## v0.5.0 (2021-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 60 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`3cc4a57`](https://github.com//Byron/gitoxide/commit/3cc4a5799fa1f487452b5c346b57fea97e45b47e))
    - (cargo-release) version 0.6.0 ([`50fb6f2`](https://github.com//Byron/gitoxide/commit/50fb6f25e9afa900ac1c3cfb88d7ca0d5a9a95f7))
    - thanks clippy ([`0fc239c`](https://github.com//Byron/gitoxide/commit/0fc239cf9b773f72928b7c42344b578c6ff5d19f))
    - thanks clippy ([`749ceba`](https://github.com//Byron/gitoxide/commit/749ceba246fb8a4cb8d48fa86184619fef500108))
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com//Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.4.1 (2021-01-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`6244fb4`](https://github.com//Byron/gitoxide/commit/6244fb4cfbc40d35f46f4d1942519414a04ac355))
    - finish docs for `git-protocol` crate ([`598f700`](https://github.com//Byron/gitoxide/commit/598f700ce2a273a6f430c8d2442dbd71e21a2704))
    - revise trait documentation of git-protocol ([`5271128`](https://github.com//Byron/gitoxide/commit/52711283d456eefcbdc37ac8f8da36149afc1322))
    - docs for response in git-protocol ([`487de13`](https://github.com//Byron/gitoxide/commit/487de1383d801fb442abc8101666a0d9a050af15))
    - more docs for git-protocol ([`bca0cbd`](https://github.com//Byron/gitoxide/commit/bca0cbd98ab02b63ac24b1d15baea602b02e1623))
    - docs for fetch::refs ([`6a97a3e`](https://github.com//Byron/gitoxide/commit/6a97a3e5883d9a6c0011a68b16966d1f8be589d7))
    - docs for git credentials helper utilities ([`eb6bb6e`](https://github.com//Byron/gitoxide/commit/eb6bb6ee2fe22ad0621f7e1743a7e56adbc54bd1))
    - first pieces of docs for git-protocol ([`12d8a83`](https://github.com//Byron/gitoxide/commit/12d8a83fbc1b70bd2612ad62aa1a69e87914fe39))
    - thanks clippy ([`343ab9a`](https://github.com//Byron/gitoxide/commit/343ab9adb62da1dde495fc209c179137bbe59a10))
</details>

## v0.4.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`28df5e9`](https://github.com//Byron/gitoxide/commit/28df5e9131aec3efb2b68db204662b92b232b33c))
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com//Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - use git-hash in git-features ([`5b307e0`](https://github.com//Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
</details>

## v0.3.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`e60dbe6`](https://github.com//Byron/gitoxide/commit/e60dbe6c21843eab44d6f05fe70927252453cb41))
    - (cargo-release) version 0.4.0 ([`32aefc0`](https://github.com//Byron/gitoxide/commit/32aefc051c7ad9d1a160f77db070df7fa4843dbc))
    - (cargo-release) version 0.4.0 ([`72eaece`](https://github.com//Byron/gitoxide/commit/72eaeceed135e4cc5c943685f4c902d03597c4d2))
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com//Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com//Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
</details>

## v0.2.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 90 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`a476a46`](https://github.com//Byron/gitoxide/commit/a476a46b7b933a3c2fa4aa8c285beec1777a3f2d))
    - (cargo-release) version 0.3.0 ([`d19ee35`](https://github.com//Byron/gitoxide/commit/d19ee35cc6683c63e0eabd717e4758075faeaa71))
    - (cargo-release) version 0.3.0 ([`eade7d1`](https://github.com//Byron/gitoxide/commit/eade7d101e071153055b07d9c6ae3c1452493a21))
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com//Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com//Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com//Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - thanks clippy ([`b9e0a87`](https://github.com//Byron/gitoxide/commit/b9e0a87996b8f3c4531a392607c353a1f0824ce6))
    - remove dash in all repository links ([`98c1360`](https://github.com//Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - refactor ([`7c3c80a`](https://github.com//Byron/gitoxide/commit/7c3c80acf487296014ae9f2f9b88865c6aa6d98e))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.1.1 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`9ef184e`](https://github.com//Byron/gitoxide/commit/9ef184e35712f938fb4f9f6da7390a8777a9284e))
    - (cargo-release) version 0.1.1 ([`bb38c6b`](https://github.com//Byron/gitoxide/commit/bb38c6b66e8de2b6743bb873c94afb187c8c8dd3))
    - Support V2 shallow-info section ([`6679c91`](https://github.com//Byron/gitoxide/commit/6679c918628979efc73e68c60e0968058cd220db))
    - Tests for V2 shallow section parsing ([`5bf58ab`](https://github.com//Byron/gitoxide/commit/5bf58ab344cb6b670ae535c7f7bca8a7f99a726c))
    - Support for the 'deepen-relative' argument ([`b86fed6`](https://github.com//Byron/gitoxide/commit/b86fed6e415183f52bb34c68d8b503566740f671))
    - V1 parsing of shallow and unshallow lines… ([`8bcf535`](https://github.com//Byron/gitoxide/commit/8bcf535a8b07d9b1d53fb84c73ba55c76a318daf))
    - remove unused fixtures ([`6ae69f5`](https://github.com//Byron/gitoxide/commit/6ae69f5f57ab371684e8c35cc77803aea05edd7b))
    - Fix wants/haves separator handling for stateful V1 ([`1629575`](https://github.com//Byron/gitoxide/commit/16295757a33cdbdb8c69ba6c487ae8b298f612cd))
    - Make really clear that V2 is stateless no matter what the transport supports :D ([`c296845`](https://github.com//Byron/gitoxide/commit/c296845201b379273ff8077489ace9ed33f416b7))
    - Assure the first 'want' in V1 is always first ([`e729ec8`](https://github.com//Byron/gitoxide/commit/e729ec8f075a6c3122b42e367486a15c5367960f))
    - Properly handle statelessness in V2 protocol ([`1b49f1e`](https://github.com//Byron/gitoxide/commit/1b49f1ef6d7a40e2dec07f9c08036b1b1d460f6b))
    - add some samples for deepen clones ([`61bc41a`](https://github.com//Byron/gitoxide/commit/61bc41a6f97decd3bdd96f874001ffb45251aca4))
    - Switch to prodash 10 and safe a lot of trait bounds in the process ([`e2fb1d9`](https://github.com//Byron/gitoxide/commit/e2fb1d944b4d803a11c91f868b831d406fb5e35f))
</details>

## v0.1.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 182 commits contributed to the release over the course of 29 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`0d7b60e`](https://github.com//Byron/gitoxide/commit/0d7b60e856325009431172e1df742a1cd2165575))
    - (cargo-release) version 0.2.0 ([`779e9d0`](https://github.com//Byron/gitoxide/commit/779e9d0ad67c20fa9cec14359e87774ca2d74ee4))
    - (cargo-release) version 0.2.0 ([`da830de`](https://github.com//Byron/gitoxide/commit/da830defc9cfa81ce159f6d908da828227760845))
    - (cargo-release) version 0.5.0 ([`82b7313`](https://github.com//Byron/gitoxide/commit/82b73131b79ec3c42a712dad1c0766a72209d737))
    - [clone] Assure we don't hang due to unprocessed headers when peeking lines! ([`d9ced27`](https://github.com//Byron/gitoxide/commit/d9ced2711dba702d73b28f0e1b9399cd7eab5183))
    - [clone] more correct handling of 'no-done'/done when sending wants/haves… ([`50f4516`](https://github.com//Byron/gitoxide/commit/50f4516adfa458f4b16e301340a39b3c34ddbef0))
    - [clone] Don't try to explicitly close the connection… ([`17200b3`](https://github.com//Byron/gitoxide/commit/17200b3c494a24de19b7c6ec3191e61551a54380))
    - [clone] Fix encoding of V1 capabilities in first want ([`b68a5c5`](https://github.com//Byron/gitoxide/commit/b68a5c57a6bd35391d8efb6436bb36e032851b49))
    - [clone] When unpacking peeled refs, use the object that refers to the tag… ([`fe8bb39`](https://github.com//Byron/gitoxide/commit/fe8bb3985bd5529a36c71fa170ca48df91060491))
    - [clone] none the wiser - it really looks like everything is alright… ([`3b8d613`](https://github.com//Byron/gitoxide/commit/3b8d613c6de349defce9af06d56f73ac2c0d0d25))
    - [clone] it looks like in order to figure out the issue, it needs tests higher up… ([`edf1540`](https://github.com//Byron/gitoxide/commit/edf1540d2014eb26cd5b98aa1baaa1e0c99662bd))
    - [clone] Don't send V2 capabilities that don't have a value… ([`9c9a4ee`](https://github.com//Byron/gitoxide/commit/9c9a4ee2a9c93612fd80844e8d2338461ee82ccc))
    - [clone] Handle remote progress name prefixing (more) correctly ([`51d4d15`](https://github.com//Byron/gitoxide/commit/51d4d15028a4162fae2d4e68a8fbb34c6ba93cc7))
    - [clone] This actually works: first MVP of retrieving packs via clone ([`c06d819`](https://github.com//Byron/gitoxide/commit/c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d))
    - Use git attributes to prevent crlf conversion of fixtures on windows ([`80ca8b2`](https://github.com//Byron/gitoxide/commit/80ca8b24b5565d82bc1f8e7d92c942f985e6ea3b))
    - [clone] Support for reading multi-step negoritaions, but… ([`507d342`](https://github.com//Byron/gitoxide/commit/507d342dfe2a714a4dd0bc100d96ed9e64a58243))
    - [clone] refactor ([`ded46fd`](https://github.com//Byron/gitoxide/commit/ded46fd5eafcb1fa1ef99dcbdd933ee8631ed7dc))
    - [clone] support for progress that can handle writing pack files ([`46e0055`](https://github.com//Byron/gitoxide/commit/46e0055eab47e402807b15c63b6a4577f5c0b7bb))
    - [clone] leave aborting the negotiation loop in the hands of the delegate ([`ea83ce7`](https://github.com//Byron/gitoxide/commit/ea83ce73b16b24409dec4009f09a0cbf203a89f7))
    - [clone] sideband-all support ([`ecc8e09`](https://github.com//Byron/gitoxide/commit/ecc8e091fb97a5d44828cd56412358b7043e47ba))
    - [clone] Actually pass pack file to the delegate ([`94c5e62`](https://github.com//Byron/gitoxide/commit/94c5e62b274b0fc39f64ee5b04273db5ead4a470))
    - [clone] Response parsing up to (optional) pack ([`24064c7`](https://github.com//Byron/gitoxide/commit/24064c77f2969380fb92ea66109df86e84060324))
    - [clone] FAIL: try to model pack reading using ownership… ([`4ee14e3`](https://github.com//Byron/gitoxide/commit/4ee14e322d904cafa297ad989a0d653e7f8e5d2f))
    - [clone] properly handle V2 response parsing ([`0d7d768`](https://github.com//Byron/gitoxide/commit/0d7d768278234824e03c5e74dacaafca3ee65713))
    - refactor ([`f2c31ec`](https://github.com//Byron/gitoxide/commit/f2c31ec4f245ce4e42e1371c4c9095fc4124cf16))
    - refactor ([`fab9f99`](https://github.com//Byron/gitoxide/commit/fab9f99a1f73378747b07f2f27f69492da899cba))
    - [clone] Provide a handle to the packfile, if it is present in the response ([`fcb4cc1`](https://github.com//Byron/gitoxide/commit/fcb4cc1b011edb2597686fcf24ad383819a52389))
    - [ref-ls] A way to abort on multiple delimiters; first tests work ([`8d44912`](https://github.com//Byron/gitoxide/commit/8d44912e7215b85c6931b7b829bd73ac38584424))
    - refactor ([`feec5be`](https://github.com//Byron/gitoxide/commit/feec5be335a99a4c47ba98f93803863044575838))
    - [ref-ls] Allow multiple delimiters at the same time ([`cfae63a`](https://github.com//Byron/gitoxide/commit/cfae63a5f7d2d99560dd857f7220980d70c4c4d8))
    - [ref-ls] basic V2 acknowledgement and packfile parsing, but… ([`549f404`](https://github.com//Byron/gitoxide/commit/549f404378535390195dea4d6c5b6485db34b81e))
    - thanks clippy ([`ac88eef`](https://github.com//Byron/gitoxide/commit/ac88eefd56095995841f60f0cfdca78295006584))
    - [ref-ls] parse all V1 acknowledgements, without duplication ([`f7c1580`](https://github.com//Byron/gitoxide/commit/f7c15809d74729b92e4c64a71543a4850765a8f8))
    - [ref-ls] first stab at V1 acknowledgement parsing ([`1d21cd4`](https://github.com//Byron/gitoxide/commit/1d21cd4a59c28fe5c631a12a10f332f4cc8fd3f3))
    - [ref-ls] It would be practical to simply have access to the line provider… ([`5fba787`](https://github.com//Byron/gitoxide/commit/5fba78796d3bcc16f812dc3202d521ee057e86f9))
    - thanks clippy ([`27f30df`](https://github.com//Byron/gitoxide/commit/27f30df9a8046fe4e872837e36dd497096660282))
    - [ref-ls] support for line peeking in packet line readers ([`0c0c575`](https://github.com//Byron/gitoxide/commit/0c0c57522972f2a49ed5261474114da062e6ab15))
    - [ref-ls] Let's make Acks copy, because owned::Id is as well ([`1f9cc44`](https://github.com//Byron/gitoxide/commit/1f9cc44275d226a7e80e24ed592f6d6bd98de31a))
    - refactor ([`935d5fe`](https://github.com//Byron/gitoxide/commit/935d5fea48b0d8710be822e2d64c77a7008143c4))
    - [ref-ls] first sketch of V1 tests for result parsing (ack + pack) ([`fd16a5f`](https://github.com//Byron/gitoxide/commit/fd16a5f265764ae9f18b9b9fc0f713ccfaaf2944))
    - [ref-ls] tests for stateless V1/V2 ([`d34afc6`](https://github.com//Byron/gitoxide/commit/d34afc6fcbcdc175c09b12d6697b01611dcd02ed))
    - [ref-ls] first step towards parsing negotiation result ([`51ecf7e`](https://github.com//Byron/gitoxide/commit/51ecf7e248724cd0b499e7a8662df4511f24d6ee))
    - refactor ([`61e9812`](https://github.com//Byron/gitoxide/commit/61e98128ddd85cde1a352b70f83870fdea0c6bac))
    - thanks clippy ([`6b1294a`](https://github.com//Byron/gitoxide/commit/6b1294a7046af84a13e34c3c43f8ddd2b3b1cb97))
    - [ref-ls] Argument tests for fetches ([`50cd260`](https://github.com//Byron/gitoxide/commit/50cd260866b7dbc44653d8c193e6517e770f44eb))
    - [ref-ls] first argument tests for clone ([`83490ef`](https://github.com//Byron/gitoxide/commit/83490ef764c2625ac34e42c27de7364d5445cdd6))
    - [ref-ls] Also add 'haves' in V2; some more assertions ([`3e6bfb1`](https://github.com//Byron/gitoxide/commit/3e6bfb1d144f9de4d45502b8257ea0f278d49376))
    - [ref-ls] Do feature assertions to not have to support old servers ([`9980ff9`](https://github.com//Byron/gitoxide/commit/9980ff9e52b466a56418857ca15fbcdc0d17b6b8))
    - [ref-ls] don't do anything on drop ([`9f18d9b`](https://github.com//Byron/gitoxide/commit/9f18d9b9062d61d6da6e2bb7564fe5edbb1528c4))
    - [ref-ls] A step towards getting the negotiation right, really need tests ([`abb56d8`](https://github.com//Byron/gitoxide/commit/abb56d855d49f232d25e0326fdef13732605df5b))
    - [ref-ls] Transport layer knows whether it's stateful or not ([`22c3640`](https://github.com//Byron/gitoxide/commit/22c3640b70bb6925d72794eeaeda48b0687f2047))
    - [ref-ls] Also re-send V1 features in each request, independently of statefulness for now ([`f8669d6`](https://github.com//Byron/gitoxide/commit/f8669d60cb349b6217227eea0d76664e8da9a458))
    - [ref-ls] potentially fix 'is-done' logic ([`f9e338f`](https://github.com//Byron/gitoxide/commit/f9e338f244806aa9f0e24352912091cb7d8e0e80))
    - [ref-ls] Sketch of sending arguments in V1 & V2 ([`e1d27b6`](https://github.com//Byron/gitoxide/commit/e1d27b6693adca053bfb42d841c03ef16a256d88))
    - [ref-ls] first step towards supporting negotiation ([`27b6d2d`](https://github.com//Byron/gitoxide/commit/27b6d2d24a92c1ffc1579a116a044cece50d9d20))
    - [ref-ls] probably all it takes to handle all capabilities of fetch arguments ([`d956ecc`](https://github.com//Byron/gitoxide/commit/d956ecc7d66544157d9233c4803b27fdc3fee1c4))
    - [ref-ls] first sketch of argument utility to help creating wants/haves ([`b0b0166`](https://github.com//Byron/gitoxide/commit/b0b0166c8dcc1094d7294ddf63e20c0ced2c85e7))
    - [ref-ls] fix feature validation in V2 ([`eb387d2`](https://github.com//Byron/gitoxide/commit/eb387d24267d90e731b41897c7e4071131508ce2))
    - update tasks ([`079fc02`](https://github.com//Byron/gitoxide/commit/079fc02608432fb6c5539759813e336c3c9f6c58))
    - [ref-ls] Always send a flush before closing the connection ([`918f19f`](https://github.com//Byron/gitoxide/commit/918f19f0c2dc202ed2014e30b7247e63a0f6a51e))
    - [ref-ls] Make credentials helper truly work ([`7f3c3a7`](https://github.com//Byron/gitoxide/commit/7f3c3a71db7eeba1d37481ba1b522d5ded654237))
    - [ref-ls] And it even doesn't work if it is the very same transport ([`4ba50fe`](https://github.com//Byron/gitoxide/commit/4ba50fe06f7423c31f4cd78079d51ef3ffd51920))
    - [clone] support automatic downgrade to protocol version 1 ([`4cf3643`](https://github.com//Byron/gitoxide/commit/4cf36436f11eb95d420c1147a1ec8adb618ea5fb))
    - [clone] basic progress for fetch in protocol ([`1925d02`](https://github.com//Byron/gitoxide/commit/1925d020b1ab922465f9555515f691b06aaba46a))
    - refactor ([`aa7e8b1`](https://github.com//Byron/gitoxide/commit/aa7e8b1eeccaa1182cfdc668592f61d8b28867d7))
    - refactor ([`b97507e`](https://github.com//Byron/gitoxide/commit/b97507ec5cd041d0433977d78006fc0d9a35e88e))
    - [clone] update README, improve delegate docs ([`dc7908f`](https://github.com//Byron/gitoxide/commit/dc7908f1546239ade71f4147a389a001769311f5))
    - [clone] test ls-remote V2 ([`0907771`](https://github.com//Byron/gitoxide/commit/09077710fb489b7a6dfa2bace4fda47609a97e78))
    - thanks clippy ([`baf0b2c`](https://github.com//Byron/gitoxide/commit/baf0b2c253b005e64762226dcf628b401b1684d4))
    - [clone] more tests for fetch features and arguments ([`a946861`](https://github.com//Byron/gitoxide/commit/a9468614e1f40de4e7442b3915c6ce09d58f8c01))
    - [clone] features for V1 fetch ([`5b24a55`](https://github.com//Byron/gitoxide/commit/5b24a559dfb03c99ee360e9997650c443fd30077))
    - [clone] assert on ref-prefix for ls-refs command ([`70347a5`](https://github.com//Byron/gitoxide/commit/70347a5406e66a77c490010cd695ceffd80fb7e2))
    - thanks clippy ([`d55cd56`](https://github.com//Byron/gitoxide/commit/d55cd56c6721ab591157f9add4ba44507373398c))
    - refactor ([`f02232d`](https://github.com//Byron/gitoxide/commit/f02232d6698a217e7ff87164bda2869777c54e33))
    - [clone] Getting there with feature handling for ls-refs ([`27c5adc`](https://github.com//Byron/gitoxide/commit/27c5adca6c428343492de69bdf2e4bd3ac9c89f3))
    - [clone] Remove intermediary mutable Capabilities implementation ([`f59344a`](https://github.com//Byron/gitoxide/commit/f59344a6e39dac579624fa2a9db64cb10afcdb75))
    - refactor ([`5ea42ba`](https://github.com//Byron/gitoxide/commit/5ea42ba9eece2f7d9557456fe6adda5058d0ae1a))
    - [clone] first step towards better organizing features/capabilities/argument names ([`7d45f3a`](https://github.com//Byron/gitoxide/commit/7d45f3abad100e8fe5691430ea3e3b95c7ae068a))
    - dependency update ([`dea0028`](https://github.com//Byron/gitoxide/commit/dea002855ef949a58851b1a3f853a59c57e4d164))
    - [clone] first sign of somethign working: ls-remote ([`df58fa1`](https://github.com//Byron/gitoxide/commit/df58fa15bc01cb047115577da58fec867f118cf9))
    - refactor; thanks clippy ([`03c3d17`](https://github.com//Byron/gitoxide/commit/03c3d176fc4c534798df9a6faf80d0722dcf0b33))
    - refactor ([`25122f2`](https://github.com//Byron/gitoxide/commit/25122f2acc95c363ee573fa875d8573ad0ee7586))
    - [clone] V2 ref parsing ([`455fa0f`](https://github.com//Byron/gitoxide/commit/455fa0f3a607cdbf24f0833e05a8a4e75ddca0c2))
    - [clone] A better way to set the agent in V2 invocations ([`325d3a2`](https://github.com//Byron/gitoxide/commit/325d3a26e45c78aa953400229d131f2119f06f75))
    - [clone] Make the actual ls-refs call ([`898cb8b`](https://github.com//Byron/gitoxide/commit/898cb8b0d672420536387926f8c6b26fba698b81))
    - [clone] sketch of delegating simple commands along with arg/feature verification ([`c2ebc48`](https://github.com//Byron/gitoxide/commit/c2ebc4875587db0936648d59440e07cc941f9503))
    - refactor ([`a6bcdc4`](https://github.com//Byron/gitoxide/commit/a6bcdc42a82b63d544b6ca6fd32d123f5ea0f4ae))
    - ignore keep-alive packages in case of 'sideband-all' ([`2e77b86`](https://github.com//Byron/gitoxide/commit/2e77b862896c5070246184290c138a68cefbe313))
    - refactor ([`ad0b2e9`](https://github.com//Byron/gitoxide/commit/ad0b2e9df98ad8f5a687849af32cb4593be9ae53))
    - thanks clippy ([`8b1ea29`](https://github.com//Byron/gitoxide/commit/8b1ea290f8f132e5a3b11828acfe4859c3d19bc1))
    - [clone] apply another mild workaround to be able to use 'transport.close()' ([`ea636ae`](https://github.com//Byron/gitoxide/commit/ea636aea6d4486edee79280c33770961a422e6bf))
    - [clone] remove workaround ([`55cf167`](https://github.com//Byron/gitoxide/commit/55cf16744126137ee70b06513c2daba116645aa9))
    - [clone] more safety checks ([`6f5a9f3`](https://github.com//Byron/gitoxide/commit/6f5a9f370542fd1d79a318e57fba65263f05028b))
    - thanks clippy ([`423458e`](https://github.com//Byron/gitoxide/commit/423458e8013b69a901a127c954281b8cb323fb26))
    - refactor ([`f29ea65`](https://github.com//Byron/gitoxide/commit/f29ea65de4693a6096d979531add42d1e0f3d04f))
    - [clone] proper parsing of V1 refs ([`d262307`](https://github.com//Byron/gitoxide/commit/d26230727ef795a819852bc82d6c2e9956809d8c))
    - [clone] A little more ref V1 parsing ([`4bc7842`](https://github.com//Byron/gitoxide/commit/4bc78425aba304b4e4967fb7599460366322ef41))
    - [clone] preparation of test for proper ref parsing (V1) ([`85cd580`](https://github.com//Byron/gitoxide/commit/85cd5806299a2fd92e786e242f946fe9e29853c1))
    - refactor ([`99247f4`](https://github.com//Byron/gitoxide/commit/99247f46673ff6772796bf55662e920200ba0c38))
    - refactor ([`c985370`](https://github.com//Byron/gitoxide/commit/c9853702e4b63dc217e94a838de8c5ee5c877a4d))
    - [clone] symref parsing from capabilities ([`8c2ff64`](https://github.com//Byron/gitoxide/commit/8c2ff640cce4f5f42a3424405efc15b18f4aa7f4))
    - [clone] A step closer to parsing symrefs correctly ([`250a340`](https://github.com//Byron/gitoxide/commit/250a34045c26ae0f5c2e06b1943479887edfe412))
    - [clone] attempt to make refs more accessible… ([`fa1112c`](https://github.com//Byron/gitoxide/commit/fa1112c69911b4cee8b2d768f907114b910832ac))
    - refactor ([`c138059`](https://github.com//Byron/gitoxide/commit/c138059434885536984996cd8fec002aba3d5fe1))
    - [clone] Prevent accidental leakage by transforming back to the 'right' type ([`2d469c6`](https://github.com//Byron/gitoxide/commit/2d469c66ec47be2e1bc3e0b1f3d17dfea5050970))
    - thanks clippy ([`9afa7f9`](https://github.com//Byron/gitoxide/commit/9afa7f9c95635559426395f61f670dfcd6f6154d))
    - [clone] a better workaround for the 'drop scope' issue ([`3ccf32b`](https://github.com//Byron/gitoxide/commit/3ccf32be15efea134bd72bbcc59c3f79252eeb3b))
    - [clone] First step of workarounding rusts drop rules ([`6b47923`](https://github.com//Byron/gitoxide/commit/6b479239cd2a60ebfe7a4b11f9e2df0a8ea4a096))
    - [clone] update tracking ticket information ([`650c452`](https://github.com//Byron/gitoxide/commit/650c4520ffc12b3c3861d406a7b8ffa2df5b5c04))
    - [clone] add Rustc issue to see if this is just my bad ([`ccb9b53`](https://github.com//Byron/gitoxide/commit/ccb9b53bfecd0e6adcccfd6dc155e8c3033cf16e))
    - thanks clippy ([`fd6f9e5`](https://github.com//Byron/gitoxide/commit/fd6f9e5c9c2ac8f68ab885d9bbf2d5f7a77a732a))
    - [clone] Workaround for the drop-issue ([`43c6159`](https://github.com//Byron/gitoxide/commit/43c61597b8907eba572eecf39b90bdca438ef7c3))
    - [clone] first attempt at adding authentication logic, but… ([`a36d14a`](https://github.com//Byron/gitoxide/commit/a36d14a6b916f6aafc2c5757acda7c32415370c5))
    - [clone] first rough sketch of (mutable) capabailities in the protocol side ([`13f7ecb`](https://github.com//Byron/gitoxide/commit/13f7ecbf493d4de633fd872f9b75292378449165))
    - refactor ([`a567b24`](https://github.com//Byron/gitoxide/commit/a567b24cb9e040d92c49364e6c4e45ff77895629))
    - refactor ([`88ecda1`](https://github.com//Byron/gitoxide/commit/88ecda11dc1d97a7460a449350945dcac2f13752))
    - [clone] frame for first 'fetch' tests ([`2da70f6`](https://github.com//Byron/gitoxide/commit/2da70f688da95434e256ba1f355dbb809100604a))
    - refactor ([`89aabde`](https://github.com//Byron/gitoxide/commit/89aabde074b26a3d36579227912eec0b74ca5a91))
    - refactor ([`51f6142`](https://github.com//Byron/gitoxide/commit/51f6142913ce520329f9829976ee364e226a41a7))
    - [clone] support for git-credentials helper ([`a6546da`](https://github.com//Byron/gitoxide/commit/a6546dab8d6d0dc4453052b77278cf5bb96aaade))
    - refactor ([`cf0e45a`](https://github.com//Byron/gitoxide/commit/cf0e45a7f129e91d377d15558378724ac0c1aca8))
    - [clone] decoding of credential message replies ([`1c2f56d`](https://github.com//Byron/gitoxide/commit/1c2f56d0fd10d3592d0a6de298360b136b34467a))
    - [clone] encode message for git credentials helper ([`143549e`](https://github.com//Byron/gitoxide/commit/143549e0757d4fa7a8347aa1b8b4734e9b62bf04))
    - [clone] sketch for identity handling ([`b23f470`](https://github.com//Byron/gitoxide/commit/b23f47029fba50c7bba23a6ebe135e129ee9392a))
    - [clone] put remaining remote progress parsing code into protocol ([`e03e0e5`](https://github.com//Byron/gitoxide/commit/e03e0e58191c71220ea1f8b9207bab96b3f9b303))
    - refactor - decouple protocol from packetline ([`dc98db2`](https://github.com//Byron/gitoxide/commit/dc98db28b77cc6a0bff2248167942224e58cdd2e))
    - [clone] move packet-line code into own crate ([`879af67`](https://github.com//Byron/gitoxide/commit/879af671fcde405d3d08ddbc07ea70d0bee23ef1))
    - [clone] move packet-lint into transport layer ([`c0dd831`](https://github.com//Byron/gitoxide/commit/c0dd8315089243164d82c444499a459756a0337b))
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive ([`5ea49c8`](https://github.com//Byron/gitoxide/commit/5ea49c8aa0d449bed98ce0147ad222ff25c27c32))
    - [url] basic frame and first failing test ([`60aacf0`](https://github.com//Byron/gitoxide/commit/60aacf0c279d277c4abf13e62697a51feeee26fd))
    - [protocol] properly implement remote progress reporting ([`a81954a`](https://github.com//Byron/gitoxide/commit/a81954a6a37afacd51add6661a656b8fb663ca54))
    - refactor ([`66e9cd1`](https://github.com//Byron/gitoxide/commit/66e9cd1fa1d17cfaac1235b573ba0230230e549c))
    - thanks clippy ([`7f6e290`](https://github.com//Byron/gitoxide/commit/7f6e29033ae05285afad846157f9c44b8c8710a5))
    - [protocol] prepare passing most of remote progress on to prodash… ([`b8a34e5`](https://github.com//Byron/gitoxide/commit/b8a34e5cf26c469ff69f29fd5d02c61605887929))
    - refactor ([`df8ebdc`](https://github.com//Byron/gitoxide/commit/df8ebdc443458fa95f9fc7fbb43ca2b6d874d972))
    - refactor ([`2ea3288`](https://github.com//Byron/gitoxide/commit/2ea3288e57ddd5204821fd6efee6cbb05231e311))
    - refactor ([`2102cab`](https://github.com//Byron/gitoxide/commit/2102cabc9860900e2b5d9391cdfde6e59ad4a119))
    - [protocol] remote::Progress can now parse the usual progress ([`b0e5601`](https://github.com//Byron/gitoxide/commit/b0e5601ae2d96b96b267b36b68ff7426c75ee3a8))
    - [protocol] first steps towards parsing remote progress ([`c3d0e7a`](https://github.com//Byron/gitoxide/commit/c3d0e7a490cfa4d114bf8c13b5b3803eb6187290))
    - [protocol] even starting to parse remote progress by hand is painful… ([`d68db3c`](https://github.com//Byron/gitoxide/commit/d68db3ca8a187d6e9b7e341dae3058ea210197fd))
    - Less ambiguous name for 'index-from-pack': 'pack-index-from-data' ([`386673c`](https://github.com//Byron/gitoxide/commit/386673ccc99d18d023c7df3fcd40e86d71960b25))
    - [protocol] handle errors as well; transmit progress (first part) ([`c484398`](https://github.com//Byron/gitoxide/commit/c48439818dbde32007a4ec350bc0599c5cbb0cf2))
    - [protocol] first successful test with pack reading ([`ad1e8bf`](https://github.com//Byron/gitoxide/commit/ad1e8bf7668a935733b0ba6a0f1573de2250eced))
    - [protocol] first stab at decoding sidebands in Read ([`51fe596`](https://github.com//Byron/gitoxide/commit/51fe5960a84e48e41544ee6d8523b7bb1e2c6a82))
    - [protocol] allow Reader delimiter to be configured ([`5a01596`](https://github.com//Byron/gitoxide/commit/5a01596ba4c9fc50beaa99260ff2b263f64e99a0))
    - refactor ([`78f27d8`](https://github.com//Byron/gitoxide/commit/78f27d8bd0dada168bf2502937cc82ee9b6cfcfe))
    - Revert "[protocol] an alternative version with external buffer" ([`157d810`](https://github.com//Byron/gitoxide/commit/157d810e50f3cc8dd12586ccd128be1d7c8a331a))
    - Revert "[protocol] But external buffers also don't help at all" ([`579a697`](https://github.com//Byron/gitoxide/commit/579a697536ff7de9727f5a7e517b83a3feb75540))
    - [protocol] But external buffers also don't help at all ([`8e711df`](https://github.com//Byron/gitoxide/commit/8e711df01b812aac9e4197a196582cad47ee6bbe))
    - [protocol] an alternative version with external buffer ([`a862d22`](https://github.com//Byron/gitoxide/commit/a862d22aaadbd1f096400d4bcd06bc5c1ce17425))
    - [protocol] a struggle - putting buffers in Read adapters = bad idea ([`e257426`](https://github.com//Byron/gitoxide/commit/e257426f3583b079120ed75e0bda2f035e70d94b))
    - [protocol] FAIL: keep referenced PacketLine for minimal copy ([`7e4d1f3`](https://github.com//Byron/gitoxide/commit/7e4d1f304b6821118f38a6cdab599cc02e6e949c))
    - [protocol] sketch of Read impl for pack line iterator ([`fe3b050`](https://github.com//Byron/gitoxide/commit/fe3b050ca7218aa7b4adf99e702534f5a6eaa70c))
    - refactor ([`c81caa3`](https://github.com//Byron/gitoxide/commit/c81caa3d178671c447846f346d08b60f59b313c4))
    - Revert "[protocol] FAIL: attempt to add an actual Iterator impl for packet lines" ([`2989781`](https://github.com//Byron/gitoxide/commit/2989781250e85042a5e26632df4b3471abe8adee))
    - [protocol] FAIL: attempt to add an actual Iterator impl for packet lines ([`a6e4cb1`](https://github.com//Byron/gitoxide/commit/a6e4cb13be7a3157d08fb899a7b9137a4f81c5b7))
    - refactor ([`20b10c5`](https://github.com//Byron/gitoxide/commit/20b10c5a52ed408a4d45e1f361dfa6faeb952850))
    - [protocol] thanks clippy ([`10b9017`](https://github.com//Byron/gitoxide/commit/10b9017f1ced471a612713ab364e7c702078e756))
    - [protocol] tests for the reader ([`86d1a40`](https://github.com//Byron/gitoxide/commit/86d1a40d735d88b4da4b654fa573e53c67c5f3c4))
    - [protocol] A chance for the reader to actually work ([`d6aebed`](https://github.com//Byron/gitoxide/commit/d6aebed49320fc52dd1f11a42ec6dc54b2de8824))
    - refactor ([`8ebdcbd`](https://github.com//Byron/gitoxide/commit/8ebdcbd7e6ae9ecb874dabf689c8a4f7a2bc4f67))
    - [protocol] FAIL: finally the reader compiles with the 'slice split technique'… ([`58543cb`](https://github.com//Byron/gitoxide/commit/58543cb13d88201f27ba015786d4916ee854ce67))
    - [protocol] FAIL3: giving up - it's quite impossible to do that without 'bytes' ([`047d67c`](https://github.com//Byron/gitoxide/commit/047d67c9ed4d329718494076f1b741da16343906))
    - [protocol] reader FAIL: wherever the loop moves, it will not borrowcheck ([`cb154f2`](https://github.com//Byron/gitoxide/commit/cb154f25d0ca6431ea3be278b573d80fa43fc66d))
    - [protocol] FAIL2: lifetime issues with loop ([`c2ff070`](https://github.com//Byron/gitoxide/commit/c2ff0700a2ea7088cdfd1c66d140bc393b7a85ce))
    - [protocol] decode-band can fail on malformed input ([`0f468f9`](https://github.com//Byron/gitoxide/commit/0f468f983efe082900689b900a10ae81ffab0157))
    - refactor ([`ed1f364`](https://github.com//Byron/gitoxide/commit/ed1f3649a89cdb224efa0ce62a63372fd973cc3b))
    - [protocol] better handling of text-lines ([`7ad1db0`](https://github.com//Byron/gitoxide/commit/7ad1db0cc1efd486b4ce9ecfef6f6a763f8d6aac))
    - [protocol] attempt to implement a streaming pack line reader (FAIL :D) ([`cc45cec`](https://github.com//Byron/gitoxide/commit/cc45cec34c43e93348fed7149c4ad5abd81dd775))
    - [protocol] add cargo-diet assertions ([`831b758`](https://github.com//Byron/gitoxide/commit/831b7587828b819844341ff451baf54694e7641c))
    - refactor ([`73e24c9`](https://github.com//Byron/gitoxide/commit/73e24c9c1966206125bea0bfa627b50ef339ce11))
    - [protocol] side-band channel encoding and decoding ([`9b4fb3e`](https://github.com//Byron/gitoxide/commit/9b4fb3eeecc7c383c7c9b9d890e7adf771ddc80a))
    - [protocol] suppot for V2 special lines ([`4e46719`](https://github.com//Byron/gitoxide/commit/4e467194d19c2804b49f5f1c445f62a5d2dc7c44))
    - Encode and decode errors ([`3f4fd90`](https://github.com//Byron/gitoxide/commit/3f4fd90333f80fc4a6b395dfb476d4ae0be921c7))
    - decode ERR lines as actual errors ([`1f58568`](https://github.com//Byron/gitoxide/commit/1f58568f670b8b3dfc996b6e7dbd2d5ef59f0f28))
    - more tests ([`c34d88b`](https://github.com//Byron/gitoxide/commit/c34d88b18a23ee499b0df8e499bd772d41a9b8e1))
    - the first succeeding tests for streaming decoding :D ([`7ea25c5`](https://github.com//Byron/gitoxide/commit/7ea25c5e94967d4480dd81bb2f3e4ad18a9d226e))
    - first stab at implementing streaming decoding of packet line… ([`843c6fb`](https://github.com//Byron/gitoxide/commit/843c6fb51e001fe9384e0f1c2cde8ec906250ee5))
    - cargo fmt ([`60cd21b`](https://github.com//Byron/gitoxide/commit/60cd21b7a2df78dbf57efbb51ab6e7a507b4f187))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com//Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
    - refactor ([`7e3f67d`](https://github.com//Byron/gitoxide/commit/7e3f67dbb8bd17cc2ee0888db08c716d7c81539a))
    - packet line encoding with flush support ([`e924a59`](https://github.com//Byron/gitoxide/commit/e924a595a4d9c9bd8647a72fd728f1bcb3f0db1a))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 10 times to make code idiomatic. 

## v0.0.0 (2020-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - first bunch of tasks I see after studying parts of the protocol docs ([`9bd97ba`](https://github.com//Byron/gitoxide/commit/9bd97bafd299efefd063dde73cef53fde9d36670))
</details>

