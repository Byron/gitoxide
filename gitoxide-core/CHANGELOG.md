# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed (BREAKING)

 - <csr-id-89a41bf2b37db29b9983b4e5492cfd67ed490b23/> remove local-time-support feature toggle.
   We treat local time as default feature without a lot of fuzz, and
   will eventually document that definitive support needs a compile
   time switch in the compiler (`--cfg unsound_local_offset` or something).
   
   One day it will perish. Failure is possible anyway and we will write
   code to deal with it while minimizing the amount of system time
   fetches when asking for the current local time.
 - <csr-id-6f4eea936d64fb9827277c160f989168e7b1dba2/> Associate `file::Metadata` with each `File`.
   This is the first step towards knowing more about the source of each
   value to filter them based on some properties.
   
   This breaks various methods handling the instantiation of configuration
   files as `file::Metadata` typically has to be provided by the caller
   now or be associated with each path to read configuration from.

### New Features

 - <csr-id-eda39ec7d736d49af1ad9e2ad775e4aa12b264b7/> `gix config` with section and sub-section filtering.
 - <csr-id-d99453ebeb970ed493be236def299d1e82b01f83/> `gix config` lists all entries of all configuration files git considers.
   Filters allow to narrow down the output.
 - <csr-id-7f67b23b9462b805591b1fe5a8406f8d7404f372/> Use `git-config` to write config file on initialization, including `logallrefupdates` and `precomposeunicode`.

### Bug Fixes

 - <csr-id-5667a7c1bafcfdff1a278b3ad0e1198cd0cc4653/> `ein tool organize` now ignores worktrees.
   Previously it would report an error due to invalid assumptions.
   The new behaviour acknowledges that worktrees are placed by hand
   and moving them is almost always not what the user would want,
   even ignoring the added complexity in doing so correctly.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 60 commits contributed to the release over the course of 101 calendar days.
 - 107 days passed between releases.
 - 6 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - fix build ([`d7dac11`](https://github.com/Byron/gitoxide/commit/d7dac11be455ee99299a8d7dfd412853f0d709f3))
    - refactor ([`7b5fe1d`](https://github.com/Byron/gitoxide/commit/7b5fe1de5332ca8a85741c7c0872130b5ebd31f2))
    - adjust to changes in git-repository ([`e1dbf85`](https://github.com/Byron/gitoxide/commit/e1dbf85999f2b6a56467419687319b79b65899a9))
    - fix build ([`40b8d4c`](https://github.com/Byron/gitoxide/commit/40b8d4c072b62f0c4ef3eb452578055cd332b6a9))
    - Allow reading patterns from stdin ([`0c597fe`](https://github.com/Byron/gitoxide/commit/0c597fe78acdd5672b4535a7d82620c5f7f93649))
    - Add `--show-ignore-patterns` to `gix repo exclude query` ([`09f904b`](https://github.com/Byron/gitoxide/commit/09f904b1f393f03176882d491d7fffcad4058b49))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - Support for overrides on the command-line ([`7d98b21`](https://github.com/Byron/gitoxide/commit/7d98b2196c130263ace4a948418affdd950302ed))
    - preliminary access to a fully configured exclusion cache ([`259d015`](https://github.com/Byron/gitoxide/commit/259d015c4c0195fb77d372545d790ea4c4d01b8a))
    - fix build ([`cb56f12`](https://github.com/Byron/gitoxide/commit/cb56f12ad83cf2932a068ef4fa0ca5ce4aa73e84))
    - sketch `open_index()` on `Worktree`, but… ([`ff76261`](https://github.com/Byron/gitoxide/commit/ff76261f568f6b717a93b1f2dcf5d8e8b63acfca))
    - a sketch of basic Worktree support ([`732f6fb`](https://github.com/Byron/gitoxide/commit/732f6fb0aa9cdc843087352b12bed2cd142ed6ec))
    - refactor ([`3ff991d`](https://github.com/Byron/gitoxide/commit/3ff991d0ca0d63632fc5710680351840f51c14c3))
    - frame for `gix repo exclude query` ([`a331314`](https://github.com/Byron/gitoxide/commit/a331314758629a93ba036245a5dd03cf4109dc52))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - fix build ([`ffe92ca`](https://github.com/Byron/gitoxide/commit/ffe92ca3cf066c09020bf6fa875bea06552cbd0d))
    - Make attributes and ignore configuration possible, but… ([`8a75fd7`](https://github.com/Byron/gitoxide/commit/8a75fd745a194786f0da7c1fd660211446ea51f7))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Group similarly named sections together more by not separating them with newline ([`4c69541`](https://github.com/Byron/gitoxide/commit/4c69541cd7192ebd5bdd696a833992d5a52cd9b6))
    - Make lossy-configuration configurable ([`b0e4da6`](https://github.com/Byron/gitoxide/commit/b0e4da621114d188a73b9f40757f59564da3c079))
    - update README with `gix config` information ([`c19d9fd`](https://github.com/Byron/gitoxide/commit/c19d9fdc569528972f7f6255760ae86ba99848cc))
    - remove local-time-support feature toggle. ([`89a41bf`](https://github.com/Byron/gitoxide/commit/89a41bf2b37db29b9983b4e5492cfd67ed490b23))
    - `gix config` with section and sub-section filtering. ([`eda39ec`](https://github.com/Byron/gitoxide/commit/eda39ec7d736d49af1ad9e2ad775e4aa12b264b7))
    - `gix config` lists all entries of all configuration files git considers. ([`d99453e`](https://github.com/Byron/gitoxide/commit/d99453ebeb970ed493be236def299d1e82b01f83))
    - Associate `file::Metadata` with each `File`. ([`6f4eea9`](https://github.com/Byron/gitoxide/commit/6f4eea936d64fb9827277c160f989168e7b1dba2))
    - Use `git-config` to write config file on initialization, including `logallrefupdates` and `precomposeunicode`. ([`7f67b23`](https://github.com/Byron/gitoxide/commit/7f67b23b9462b805591b1fe5a8406f8d7404f372))
    - adjust to changes in `git-config` ([`c52cb95`](https://github.com/Byron/gitoxide/commit/c52cb958f85b533e791ec6b38166a9d819f12dd4))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - Handle 'kind' changes which completes 'explain' ([`45022a0`](https://github.com/Byron/gitoxide/commit/45022a0efe6e71404868a7ba816c6972050098b9))
    - Support for explaining all navitation ([`ace9c89`](https://github.com/Byron/gitoxide/commit/ace9c8953bebc4a808c639e365010ed53c031622))
    - start navigation implementation ([`ea1c009`](https://github.com/Byron/gitoxide/commit/ea1c009e1b064deccf242fc60876a8535f4814b5))
    - Implement `Revision` anchors ([`a1f0e3d`](https://github.com/Byron/gitoxide/commit/a1f0e3d463397be201f4df40184ce38b830f3bde))
    - basic infrastructure for delegate implementation ([`d3c0bc6`](https://github.com/Byron/gitoxide/commit/d3c0bc6e8d7764728f4e10500bb895152ccd0b0b))
    - Hookup explain command ([`1049b00`](https://github.com/Byron/gitoxide/commit/1049b00eaa261a67f060eaca4eb50dcda831eafd))
 * **Uncategorized**
    - make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - fix build after changes to `git-url` and `git-config` ([`1f02420`](https://github.com/Byron/gitoxide/commit/1f0242034071ce317743df75cc685e7428b604b0))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - Merge branch 'main' into cont_include_if ([`0e9df36`](https://github.com/Byron/gitoxide/commit/0e9df364c4cddf006b1de18b8d167319b7cc1186))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - Refact. ([`a342e53`](https://github.com/Byron/gitoxide/commit/a342e53dac58cea1787a94eaa1a9d24fb1389df2))
    - `ein tool organize` now ignores worktrees. ([`5667a7c`](https://github.com/Byron/gitoxide/commit/5667a7c1bafcfdff1a278b3ad0e1198cd0cc4653))
    - Revert "ignore worktrees in 'organize', but…" ([`f59471f`](https://github.com/Byron/gitoxide/commit/f59471f0cf883176594ab4635248b4029bcb6caf))
    - ignore worktrees in 'organize', but… ([`e501c9e`](https://github.com/Byron/gitoxide/commit/e501c9e6348e1595fee4a5e0bd712fc2433b10df))
    - Merge branch 'davidkna-admin-sec' ([`3d0e2c2`](https://github.com/Byron/gitoxide/commit/3d0e2c2d4ebdbe3dff01846aac3375128353a2e1))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - fix most of docs ([`1fe053f`](https://github.com/Byron/gitoxide/commit/1fe053f60fa4843e7da6a6328fc293b4bcd25277))
    - refactor ([`07e0f5e`](https://github.com/Byron/gitoxide/commit/07e0f5e91b3c41614b9182cf9716120fe41ddf40))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
    - thanks clippy ([`d011d4e`](https://github.com/Byron/gitoxide/commit/d011d4ec3b58234cc3ea07cf6808a8ce580811c5))
    - thanks clippy ([`fdec111`](https://github.com/Byron/gitoxide/commit/fdec11135692b3503087b0a3245c12cc87554d67))
    - Release git-glob v0.2.0, safety bump 3 crates ([`ab6bed7`](https://github.com/Byron/gitoxide/commit/ab6bed7e2aa19eeb9990441741008c430f373708))
</details>

## 0.14.0 (2022-04-05)

### New Features

 - <csr-id-7e99e6aeee9bf200a561d215c586301f5e4a8cbc/> Add `gix repo commit describe`
   It supports typical but basic flags mostly similar to the ones in git.
 - <csr-id-654f4afb794a370b7cd9d9502ff6d0c3378ec417/> `Commit::describe()`
   A way to fluidly configure a `git describe` operation and run it.
   
   Along that, a new `Tag` top-level object was added as well to provide
   convenient access to otherwise lower-level objects. It's not strictly
   required for our implementation here but it's needed for a symmetric
   API.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 2 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#298](https://github.com/Byron/gitoxide/issues/298)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - restrict signature changes to 'Ancestores::sorting()` ([`d71bd9d`](https://github.com/Byron/gitoxide/commit/d71bd9ded1e5e5a61a27be3d55f4b85ee4049bcf))
    - Adjust to changes in git-traverse ([`8240622`](https://github.com/Byron/gitoxide/commit/824062215865e6ec12afeb2d51b3c63f15291244))
    - support for the --max-candidates flag ([`b9e6754`](https://github.com/Byron/gitoxide/commit/b9e67540801f2630be8aa1acbfddfec4202360ac))
    - Reduce amount of max candidates, add --debug flag ([`c8c13e3`](https://github.com/Byron/gitoxide/commit/c8c13e398671a21e96282547fc0e3bd445627e2f))
    - Add `gix repo commit describe` ([`7e99e6a`](https://github.com/Byron/gitoxide/commit/7e99e6aeee9bf200a561d215c586301f5e4a8cbc))
    - `Commit::describe()` ([`654f4af`](https://github.com/Byron/gitoxide/commit/654f4afb794a370b7cd9d9502ff6d0c3378ec417))
 * **Uncategorized**
    - Release git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0 ([`f041c00`](https://github.com/Byron/gitoxide/commit/f041c00a7df2455ca52fac7b83af1e9f335f5688))
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
</details>

## 0.13.0 (2022-04-03)

<csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/>
<csr-id-4d2d433e7e98ac42db858688edac06e68ee4d10d/>
<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>
<csr-id-424c9b3a2b467f5a1e339700257cd4ab72e2e692/>

### Refactor (BREAKING)

 - <csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/> clarify different repository types much better
 - <csr-id-4d2d433e7e98ac42db858688edac06e68ee4d10d/> Remove light* features, add 'lean-async' in its place; remove termion support
 - <csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/> remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 - <csr-id-2290d006705ff47ad780b009fe58ee422b3285af/> move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

### Other

 - <csr-id-424c9b3a2b467f5a1e339700257cd4ab72e2e692/> Try to make Handle usable for pack creation
   It's nearly there, but for some reason the boxed dyn traits don't get to
   be Send even though it's specified.

### Bug Fixes

 - <csr-id-84ade1d23060f10bf6c8529f8f693d06660b4f4e/> Allow resolution of in-pack ref-deltas
   This finally allows delta tree caches to be used on typical small packs
   returned by GitHub.
 - <csr-id-d33795f9e99d5ad7191c807e42a6fce368932f6b/> keep non "git" repository name extensions
   When converting a remote URL to a destination directory name,
   keep extension strings if they are not equal to "git". For example,
 - <csr-id-6d3f52dc13d7243a6bce6dab89a985114a75d94b/> Avoid the dashmap being cloned for each thread
   Intead, share it by reference, it's sync after all.
   
   This issue was introduced when switching to a `Send + Clone` model,
   instead of `Send + Sync`, to allow thread-local caches in database
   handles of all kinds.

### New Features

 - <csr-id-30b22a870f22a8ad124f15a7d5395a707c1c8fa4/> ein tool estimate-hours with shallow clone support
 - <csr-id-def80df2e165b74f4b053e4030f563902b7d34a4/> `ein tool estimate-hours` now supports mailmaps
 - <csr-id-d2388d8d80f379eccc9ee84ebe07acd67d154630/> `gix repository mailmap entries`
 - <csr-id-384ed665c7423feca1b1ee1f81db10867fa813a8/> `gix mailmap verify` command
 - <csr-id-e3bc1b410409a9e27894a5cac48b06d8c3295e36/> unstable mailmap module
 - <csr-id-3f28e2037fcb07fa022220e52bcf967d6d6ef647/> `ein find` with support for worktree checkouts
 - <csr-id-70109bee679d33a5c5fb3a78a708b479684b03b1/> `ein find --debug` to learn why it is slow
 - <csr-id-00909619ff04e247aabc9ffe3c025f0064c3092d/> --counting-threads flag to configure amount of threads when counting
   The efficiency of multi-threaded counting is low per core, and despite
   some speedups might be desirable, one might not want to commit all cores
   to this amount of waste.
 - <csr-id-aa3795d69926c4506e5bc2d14f447d64bc4e6c2c/> in-manifest and in-lib documentation of feature toggles

### Changed (BREAKING)

 - <csr-id-15d429bb50602363292453606902bdce5042d9a5/> file::Store::(try_)find(…, packed) was removed
   The packed buffer is now handled internally while loading it on demand.
   When compiled with `git-features/parallel` the `file::Store` remains
   send and sync.
   
   The packed refs buffer is shared across clones and it's recommended
   to clone one `file::Store` instance per thread, each of which can
   use its own namespace.
 - <csr-id-e7526b2a7b51cbac4018e1ab3b623a85987fadc2/> parallel utilities now use `Send + Clone` insted of `Send + Sync`
   This helps to assure that thread-local computations always work with the
   kind of types we provide. The ones that are carrying out actions are
   notably not `Sync` anymore.
   
   We cater to that by defining our bounds accordingly, but for those
   who want to use other utilities that need Sync, using types like
   `Repository` and `thread_local!()` is the only way to make this
   work.
 - <csr-id-e8b091943f0c9a26317da0003f7fcdf5a56ef21a/> Rename gix->ein and gixp->gix
 - <csr-id-bf04644ab75ed1969507f957dc8d4868790d462d/> remove `Option<impl Progress>` in favor of `impl Progress`
 - <csr-id-6829e5e5d6aed1e6c87647144e2dd76a1e4b9f1f/> multi-index integrity check; use `integrity::Outcome` for various integrity checks
 - <csr-id-d851bede97801096d188ff6af06c98a79fe276db/> remove unnecessary `Arc` around `should_interrupt` flag
 - <csr-id-c2679a03358b9c19d63ed1af1cd57324c6381447/> remove Sha1 mentions in `index::verify::Mode::*` variants
   The hash is repository defined and not hard-coded
 - <csr-id-80b120d3278e46429f848df7af3db13413c36649/> introduce `index::File::verify_integrity(…, pack: Option<PackContext>, …)`, replacing tuple
   This allows for more documentation on what input is required there and
   generally makes for an easier to use API.
 - <csr-id-de4fa64843e2c26b86eaaeadd9d6514c96029a9f/> consistently use `object_hash` instead of `hash_kind`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 144 commits contributed to the release over the course of 156 calendar days.
 - 165 days passed between releases.
 - 26 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 14 unique issues were worked on: [#215](https://github.com/Byron/gitoxide/issues/215), [#247](https://github.com/Byron/gitoxide/issues/247), [#263](https://github.com/Byron/gitoxide/issues/263), [#266](https://github.com/Byron/gitoxide/issues/266), [#279](https://github.com/Byron/gitoxide/issues/279), [#287](https://github.com/Byron/gitoxide/issues/287), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#333](https://github.com/Byron/gitoxide/issues/333), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366), [#67](https://github.com/Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#215](https://github.com/Byron/gitoxide/issues/215)**
    - Remove light* features, add 'lean-async' in its place; remove termion support ([`4d2d433`](https://github.com/Byron/gitoxide/commit/4d2d433e7e98ac42db858688edac06e68ee4d10d))
 * **[#247](https://github.com/Byron/gitoxide/issues/247)**
    - Rename gix->ein and gixp->gix ([`e8b0919`](https://github.com/Byron/gitoxide/commit/e8b091943f0c9a26317da0003f7fcdf5a56ef21a))
 * **[#263](https://github.com/Byron/gitoxide/issues/263)**
    - Adjust to chagne signature of peel_to_id_in_place (again?) ([`dd5341f`](https://github.com/Byron/gitoxide/commit/dd5341f6d258c72441e84b3874eede45798d87cc))
    - file::Store::(try_)find(…, packed) was removed ([`15d429b`](https://github.com/Byron/gitoxide/commit/15d429bb50602363292453606902bdce5042d9a5))
    - parallel utilities now use `Send + Clone` insted of `Send + Sync` ([`e7526b2`](https://github.com/Byron/gitoxide/commit/e7526b2a7b51cbac4018e1ab3b623a85987fadc2))
    - A mad attempt to use thread-local everywhere and avoid Sync… ([`0af5077`](https://github.com/Byron/gitoxide/commit/0af5077e1f028c1c69bbdc098bb567e486282c37))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - upgrade dependencies ([`c301abe`](https://github.com/Byron/gitoxide/commit/c301abe7a7f0f0a2613250c772d4cc74faca0e5e))
    - Make single-threaded programs possible to use with git-repository ([`dde5c6b`](https://github.com/Byron/gitoxide/commit/dde5c6ba76ff849f69f742c985b4bc65ca830883))
    - Use new store in git-repository ([`2f9e342`](https://github.com/Byron/gitoxide/commit/2f9e342b63f9e5c925d8e85ebc0a0be693ca0901))
    - Adjust object-acess to test new contains method ([`8488b41`](https://github.com/Byron/gitoxide/commit/8488b41651751d9177f53a23233b7ddd655dd696))
    - Use handle with default cache configuration in 'ein hours' ([`f71806f`](https://github.com/Byron/gitoxide/commit/f71806f1221daaf86b6c7b8846a85a134d0ac1ce))
    - adapt to changes in git-repository ([`fae309b`](https://github.com/Byron/gitoxide/commit/fae309b43f5cbc2aeb6b44b40e20f09ac0af5ca1))
    - Adjustments to match changes in `git-repository` ([`117d5f8`](https://github.com/Byron/gitoxide/commit/117d5f8625fd3af8f501e48eb0fad6d09fa814ba))
    - Adapt to changes in git-repository ([`3ab9b03`](https://github.com/Byron/gitoxide/commit/3ab9b03eee7d449b7bb87cb7dcbf164fdbe4ca48))
    - Adapt to changes in git-repository ([`3266c47`](https://github.com/Byron/gitoxide/commit/3266c47a04b1cbd296df469765f4df0727062ca5))
    - Adjust pack-create to changes in git-pack ([`12db899`](https://github.com/Byron/gitoxide/commit/12db899a72da6decccd82931637d074059b578f5))
    - Cache-creators are indeed shared across threads, must be sync ([`c326cb3`](https://github.com/Byron/gitoxide/commit/c326cb35cc684a5751e007c0ece3f02edf162ecc))
    - Try to make Handle usable for pack creation ([`424c9b3`](https://github.com/Byron/gitoxide/commit/424c9b3a2b467f5a1e339700257cd4ab72e2e692))
    - Inform when multi-threaded counting is rejected ([`cc3b070`](https://github.com/Byron/gitoxide/commit/cc3b070d18ebb257e799c16435bffb117b9262b7))
    - remove pack-cache from `Find::try_find(…)` ([`ebc7f47`](https://github.com/Byron/gitoxide/commit/ebc7f47708a63c3df4415ba0e702660d976dfb3e))
    - move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait ([`2290d00`](https://github.com/Byron/gitoxide/commit/2290d006705ff47ad780b009fe58ee422b3285af))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Fast-path multi-pack index verification in the CLI ([`bcde935`](https://github.com/Byron/gitoxide/commit/bcde935e7102ba5cd50c057a8323353247d3dd85))
    - Add a less thorough and faster way of verifying multi-indices ([`7517482`](https://github.com/Byron/gitoxide/commit/75174825e1012cfb4c34c18391c681b49c2f0d29))
    - Handle large multi-pack indices correctly ([`4f6b030`](https://github.com/Byron/gitoxide/commit/4f6b0308f06b7705163ff624a98694e1d928fee1))
    - Fix progress and handling of large of multi-pack index offsets ([`5dc1f81`](https://github.com/Byron/gitoxide/commit/5dc1f813ead64ad13edb2b5ed9bd660d198c7ddb))
    - Basic multi-pack index creation ([`89428b2`](https://github.com/Byron/gitoxide/commit/89428b2936fb0169606a543cf531bddaacb8187c))
    - Add frame for writing a multi-pack index ([`9ce1e7f`](https://github.com/Byron/gitoxide/commit/9ce1e7f2d8c7133590f571919850eaa763f789e3))
    - adjust to changes in git_pack ([`7907ca8`](https://github.com/Byron/gitoxide/commit/7907ca83b9106384b7ced0295c9edda4dd7940c4))
    - even nicer printing ([`d2bea27`](https://github.com/Byron/gitoxide/commit/d2bea270787597d6aef48ffe023ff49969c33bd9))
    - nicer printing of index verification results ([`e3dfa12`](https://github.com/Byron/gitoxide/commit/e3dfa123b368e66f39567bd2a8f5d7d9c09d4fe6))
    - very first experimental support for multi-pack index verification ([`bb35c69`](https://github.com/Byron/gitoxide/commit/bb35c6994765ec3bbbcfde247911d1ffe711a23d))
    - remove `Option<impl Progress>` in favor of `impl Progress` ([`bf04644`](https://github.com/Byron/gitoxide/commit/bf04644ab75ed1969507f957dc8d4868790d462d))
    - multi-index integrity check; use `integrity::Outcome` for various integrity checks ([`6829e5e`](https://github.com/Byron/gitoxide/commit/6829e5e5d6aed1e6c87647144e2dd76a1e4b9f1f))
    - remove unnecessary `Arc` around `should_interrupt` flag ([`d851bed`](https://github.com/Byron/gitoxide/commit/d851bede97801096d188ff6af06c98a79fe276db))
    - remove Sha1 mentions in `index::verify::Mode::*` variants ([`c2679a0`](https://github.com/Byron/gitoxide/commit/c2679a03358b9c19d63ed1af1cd57324c6381447))
    - introduce `index::File::verify_integrity(…, pack: Option<PackContext>, …)`, replacing tuple ([`80b120d`](https://github.com/Byron/gitoxide/commit/80b120d3278e46429f848df7af3db13413c36649))
    - Adjust to changes in git-odb ([`710780c`](https://github.com/Byron/gitoxide/commit/710780cd355793ea638767213f250e026997a530))
    - refactor ([`005fba7`](https://github.com/Byron/gitoxide/commit/005fba79f2c4bc17e7eb8b655f570bb15bacae9d))
    - consistently use `object_hash` instead of `hash_kind` ([`de4fa64`](https://github.com/Byron/gitoxide/commit/de4fa64843e2c26b86eaaeadd9d6514c96029a9f))
    - adapt to changes in git-pack ([`28dba20`](https://github.com/Byron/gitoxide/commit/28dba20d0ba6197d02e1c9b665279392dad8d707))
    - Deal with changes to git-odb `Write` trait ([`4d67122`](https://github.com/Byron/gitoxide/commit/4d6712210555c7ac88940be2a271471ee1e7cb97))
    - adapt to changes to `git-odb` ([`5b0e2b9`](https://github.com/Byron/gitoxide/commit/5b0e2b927eac75548d5a9f3cf302aa5eda70a795))
    - adapt to changes in git-hash ([`82fec95`](https://github.com/Byron/gitoxide/commit/82fec95e9ed4b924849bfcc84b5b2691a925a5b3))
 * **[#287](https://github.com/Byron/gitoxide/issues/287)**
    - lower-case json fields for consistency ([`f6c0e6d`](https://github.com/Byron/gitoxide/commit/f6c0e6d76c00f03bdc1c69f814c46e30af51eec7))
    - basic output for 'repo verify' json only ([`9f8d61f`](https://github.com/Byron/gitoxide/commit/9f8d61f164fb3fbdb76cc44fbd634ca5db35b3b8))
    - share and pass cli arguments for pack verification ([`db43e47`](https://github.com/Byron/gitoxide/commit/db43e47fc0a43ef45824ac1c9426c1889bdb13a3))
    - Allow resolution of in-pack ref-deltas ([`84ade1d`](https://github.com/Byron/gitoxide/commit/84ade1d23060f10bf6c8529f8f693d06660b4f4e))
    - Very rough version of repository verification ([`80a4a7a`](https://github.com/Byron/gitoxide/commit/80a4a7add688d16376b9bf2ed7f1c7f655b7c912))
    - frame for loose-db validation ([`a24307d`](https://github.com/Byron/gitoxide/commit/a24307dfd0b7322472f85ec83687a04488d28cff))
    - Adjustments to deal with changes to git-pack/git-odb ([`fcf8fde`](https://github.com/Byron/gitoxide/commit/fcf8fde7272974a70df808bd7ac03e925b7e39a8))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - Assert store tree cache matches actual source objects ([`b062bec`](https://github.com/Byron/gitoxide/commit/b062becd01058f5c519538f89d9d8fec8342114d))
    - Sketch a surprisingly difficult way of loading objects in verify_extension() ([`3baeab4`](https://github.com/Byron/gitoxide/commit/3baeab4ab216132536d5c182b3e316ce65095085))
    - Also verify the index of the default workspace ([`15b8372`](https://github.com/Byron/gitoxide/commit/15b8372c5a36c093c0bf3193f6b5bcd764b12a66))
    - First stab at tree verification ([`f928350`](https://github.com/Byron/gitoxide/commit/f9283500e8316ab949fc0ff9c2fc13a498380873))
    - Add 'index verify' subcommand to 'gix' ([`1ac2c21`](https://github.com/Byron/gitoxide/commit/1ac2c210c311c4b2ef835e04e2d7c477981b850f))
    - refactor ([`d0725bd`](https://github.com/Byron/gitoxide/commit/d0725bd40f0b9af0e0af34ffe77c2d8406c6d24c))
    - Fix tree-extension loading for empty trees ([`2e13989`](https://github.com/Byron/gitoxide/commit/2e1398985edfaf9e62ff5643cf4756d9d9717862))
    - Now we are able to load indices correctly ([`762efa3`](https://github.com/Byron/gitoxide/commit/762efa3f5e4ebda4d3bcc6a9bba43c6cdb407937))
    - refactor ([`3541e33`](https://github.com/Byron/gitoxide/commit/3541e3329574fbb694450bead62b71a9af1d336e))
    - Print extension names instead of count ([`1cc07e0`](https://github.com/Byron/gitoxide/commit/1cc07e0cfdae74e388abb29d7acb9c6f643278b4))
    - Flag to hide extension details ([`34ea001`](https://github.com/Byron/gitoxide/commit/34ea001fafa93b6453513cf458fe24327a13ff28))
    - Print basic index information, including the tree extension ([`9277cf8`](https://github.com/Byron/gitoxide/commit/9277cf877e1f2276dcad1efdeb97e0e3d96ed3f0))
    - Basic entry information ([`239e7b2`](https://github.com/Byron/gitoxide/commit/239e7b291297d6d49ebdf3d4986fb9fb86480e9a))
    - refactor ([`8bf585d`](https://github.com/Byron/gitoxide/commit/8bf585d67cd67b168d819ba05858cef7d9b90894))
    - JSON output for index entries ([`3fc1622`](https://github.com/Byron/gitoxide/commit/3fc1622488054c6ab655eb9d2f941b68cc3ccf18))
    - fix build ([`e3977fe`](https://github.com/Byron/gitoxide/commit/e3977fe033550bfd3297cdd674934e40476aa38b))
    - refactor ([`9b28b18`](https://github.com/Byron/gitoxide/commit/9b28b18262c763608d60fba65e91fcb9ca3ddb3e))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - adapt to changes in `git-repository' ([`16a1c36`](https://github.com/Byron/gitoxide/commit/16a1c360113b9bc910d5b0812384c3ab32cfc780))
    - clarify different repository types much better ([`bbc6efe`](https://github.com/Byron/gitoxide/commit/bbc6efeceb26050973e1425e68a52e51b9df4572))
    - Also print stage of entries ([`003515f`](https://github.com/Byron/gitoxide/commit/003515f3c90a49fbe9db9b84987233486711beb8))
    - simple printing of basic entry information ([`329538b`](https://github.com/Byron/gitoxide/commit/329538b9c3f44bb8e70a4567ba90dc3b594c2dfc))
    - frame for printing index information ([`9ea98fd`](https://github.com/Byron/gitoxide/commit/9ea98fda75fbef339647a0ca03776060356d1206))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - pass thread-limit along to checkout ([`07e9081`](https://github.com/Byron/gitoxide/commit/07e9081fb5628e4ddc8f87e2d4ba0c7b3247bb35))
    - conversions from Rc to arc for Handle ([`c19331e`](https://github.com/Byron/gitoxide/commit/c19331e001e587e4fca74f3e9fec28a7df922c0a))
    - proper handling of interruptions during checkout ([`7575a58`](https://github.com/Byron/gitoxide/commit/7575a5854ebe61a5941177efb470143192223ef3))
    - add thread-count and chunk-size computation; interrupt capability ([`8cbe85d`](https://github.com/Byron/gitoxide/commit/8cbe85d135898826a91939726465a9e295c1e24b))
    - refactor ([`542f49b`](https://github.com/Byron/gitoxide/commit/542f49beb811f7f9bf9dff3cd19694498f6cf9e2))
    - return proper errors during checkout object lookup ([`f9beac0`](https://github.com/Byron/gitoxide/commit/f9beac0471a38cb4c3b070ecb576ed1a39456bd6))
    - more safety around recursion and invariants when resolving ref-deltas ([`dddb4a5`](https://github.com/Byron/gitoxide/commit/dddb4a51f417ff84a53da64959ad668ab26ebd93))
    - elaborate odb info and simple entries printing ([`0f65282`](https://github.com/Byron/gitoxide/commit/0f65282fd2719234f745473e33bd42637be5fd3b))
    - a first sketch of access odb information using a sub-command ([`89b628a`](https://github.com/Byron/gitoxide/commit/89b628ab5b833a34f0b426b3a399bb182e63f3f4))
    - sub-command to print multi-index entries ([`6c10e09`](https://github.com/Byron/gitoxide/commit/6c10e097a432d81b930008abc00c6821ed7ac9be))
    - pack multi-index info subcommand ([`21c2dd5`](https://github.com/Byron/gitoxide/commit/21c2dd5da20a9e3cbae618b6311b6c9de12cf43c))
    - bring back more detailed errors in case of keep-going ([`8198817`](https://github.com/Byron/gitoxide/commit/8198817507a5e9c6e6fb847a45ac47bd38de68f6))
    - use progress to print errors right when they happen ([`af03686`](https://github.com/Byron/gitoxide/commit/af03686b5abf9548300a83329500b27acd66e16a))
    - detailed report about issues after checkout ([`613483b`](https://github.com/Byron/gitoxide/commit/613483b297b8a7e9a91cac3ef8205f2103ea946b))
    - keep-going support on the command-line ([`73a7393`](https://github.com/Byron/gitoxide/commit/73a73932f430fe991f26222ba2735332c03c0e77))
    - add tree-info subcommand to more easily test actual tree-traversal performance ([`29fb0c8`](https://github.com/Byron/gitoxide/commit/29fb0c8ff628716d33c9c41d3910e617791dcc77))
    - fix 'tree entries' performance by not decoding every object ([`53e79c8`](https://github.com/Byron/gitoxide/commit/53e79c88239a9191506c095294a42ead959c8af2))
    - first basic tree visualization with --recursive and flat display ([`111400f`](https://github.com/Byron/gitoxide/commit/111400f9353d1e2c6951260eda2dbe6e5ba64ded))
    - frame for traversing tree entries ([`0e55fbb`](https://github.com/Byron/gitoxide/commit/0e55fbb2fb0cec6f402b7a3aed7ee55078d233a1))
    - fix progress - there is no max value for bytes written ([`537e5aa`](https://github.com/Byron/gitoxide/commit/537e5aaa80fb21800d8ad856595c09018428f3eb))
    - allow writing empty files during checkout but also query the odb ([`5388d80`](https://github.com/Byron/gitoxide/commit/5388d8091ef02cf927478a1492847ae1666040d4))
    - support for repo to write actual objects ([`5494fb3`](https://github.com/Byron/gitoxide/commit/5494fb3e1de1234dde8c47336597283dbd8bcb29))
    - basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - in-manifest and in-lib documentation of feature toggles ([`aa3795d`](https://github.com/Byron/gitoxide/commit/aa3795d69926c4506e5bc2d14f447d64bc4e6c2c))
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
    - remove os-str-bytes everywhere ([`71a086a`](https://github.com/Byron/gitoxide/commit/71a086aaf0835c31c834aa32d968552de490f2e7))
    - gitoxide-core without os-str-bytes ([`909aa14`](https://github.com/Byron/gitoxide/commit/909aa1402c82c3128052023613a297b213716e3d))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - add some precaution to avoid strange interactions with packs ([`b052a9a`](https://github.com/Byron/gitoxide/commit/b052a9a3e9127fd9a4029594ea9de6e436db03c6))
    - ein tool estimate-hours with shallow clone support ([`30b22a8`](https://github.com/Byron/gitoxide/commit/30b22a870f22a8ad124f15a7d5395a707c1c8fa4))
    - simplify estimate-hours by just looking at the author signature ([`beb478f`](https://github.com/Byron/gitoxide/commit/beb478fb50581139afadbec84d24ff4d419b0756))
    - Full error handling for CommitRefIter ([`b94471a`](https://github.com/Byron/gitoxide/commit/b94471a0ced50204156cf5d4126c676f0258a5eb))
    - Adjust to breaking changes in `git-traverse` ([`d79b506`](https://github.com/Byron/gitoxide/commit/d79b5064eab2d1bef445e6c9e62a53466a8d5225))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - `ein tool estimate-hours` now supports mailmaps ([`def80df`](https://github.com/Byron/gitoxide/commit/def80df2e165b74f4b053e4030f563902b7d34a4))
    - `gix repository mailmap entries` ([`d2388d8`](https://github.com/Byron/gitoxide/commit/d2388d8d80f379eccc9ee84ebe07acd67d154630))
    - frame for printing mailmap entries using git-repository ([`2a01f47`](https://github.com/Byron/gitoxide/commit/2a01f4728ae858b47280b587501d343fdb86655d))
    - gix mailmap verify can now detect collisions ([`f89fe2f`](https://github.com/Byron/gitoxide/commit/f89fe2f867fa792db5d9e003ce342a337a6ac973))
    - `gix mailmap verify` command ([`384ed66`](https://github.com/Byron/gitoxide/commit/384ed665c7423feca1b1ee1f81db10867fa813a8))
    - unstable mailmap module ([`e3bc1b4`](https://github.com/Byron/gitoxide/commit/e3bc1b410409a9e27894a5cac48b06d8c3295e36))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - --counting-threads flag to configure amount of threads when counting ([`0090961`](https://github.com/Byron/gitoxide/commit/00909619ff04e247aabc9ffe3c025f0064c3092d))
    - Avoid the dashmap being cloned for each thread ([`6d3f52d`](https://github.com/Byron/gitoxide/commit/6d3f52dc13d7243a6bce6dab89a985114a75d94b))
 * **Uncategorized**
    - Release git-commitgraph v0.7.0, gitoxide-core v0.13.0, gitoxide v0.11.0 ([`ab08a7f`](https://github.com/Byron/gitoxide/commit/ab08a7f066fb65671868424315d958ae985d76d8))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'svetli-n-refactor_git_config_tests' ([`babaa9f`](https://github.com/Byron/gitoxide/commit/babaa9f5725ab8cdf14e0c7e002c2e1de09de103))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - adapt to breaking changes in git-actor ([`40c48c3`](https://github.com/Byron/gitoxide/commit/40c48c390eb796b427ebd516dde92e9538ce5fb7))
    - `ein find` with support for worktree checkouts ([`3f28e20`](https://github.com/Byron/gitoxide/commit/3f28e2037fcb07fa022220e52bcf967d6d6ef647))
    - `ein find --debug` to learn why it is slow ([`70109be`](https://github.com/Byron/gitoxide/commit/70109bee679d33a5c5fb3a78a708b479684b03b1))
    - thanks clippy ([`804d5f1`](https://github.com/Byron/gitoxide/commit/804d5f141787a1feac25909e17692ee626881082))
    - thanks clippy ([`5db3993`](https://github.com/Byron/gitoxide/commit/5db39936fc003a79f18e545a8317305fe18af74d))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Merge branch 'index-verification' ([`ad3c803`](https://github.com/Byron/gitoxide/commit/ad3c8032cee02052ef3940d1d7c950270a0a299a))
    - Release git-config v0.1.11 ([`a605b67`](https://github.com/Byron/gitoxide/commit/a605b67294773628590220600f5017c63911f620))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
    - thanks clippy ([`3aba4b4`](https://github.com/Byron/gitoxide/commit/3aba4b4877a11b720a02f4a246e6fa6ac6327119))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - thanks clippy ([`5a68d2f`](https://github.com/Byron/gitoxide/commit/5a68d2feffc551ad5f07e90efb2307e966d2636b))
    - Merge branch 'use-midx-in-store' ([`338521b`](https://github.com/Byron/gitoxide/commit/338521b0443b9dc1007581de42ef6a950f6e0bbf))
    - keep non "git" repository name extensions ([`d33795f`](https://github.com/Byron/gitoxide/commit/d33795f9e99d5ad7191c807e42a6fce368932f6b))
    - thanks clippy ([`533a532`](https://github.com/Byron/gitoxide/commit/533a532c86bcf0dae27558e66b1a5cd2e52983df))
    - Release git-chunk v0.2.0, safety bump 4 crates ([`b792fab`](https://github.com/Byron/gitoxide/commit/b792fabf9f5f93ab906ac5a5bb3e4f01c179290a))
    - Merge branch 'oknozor-feat/traversal-sort-by-committer-date' ([`6add377`](https://github.com/Byron/gitoxide/commit/6add3773c64a9155c236a36bd002099c218882eb))
    - Fix pack/create ([`4d8de93`](https://github.com/Byron/gitoxide/commit/4d8de93603cc143ca092a07b30dfe8660ffe8fcb))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Merge branch 'git-loose-objects' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-git-loose-objects ([`ee737cd`](https://github.com/Byron/gitoxide/commit/ee737cd237ad70bf9f2c5e0d3e4557909e495bca))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Merge branch 'header-field-multi-improve' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-header-field-multi-improve ([`d88e377`](https://github.com/Byron/gitoxide/commit/d88e377c21e566bf86c274d5e87eff06100698b9))
</details>

## v0.12.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.11.0 (2021-10-15)

<csr-id-ac3b9efb7b90958274ce55800959d930f8641115/>
<csr-id-a19567eceab0dd7f5478b83c2ff9ce79754db308/>
<csr-id-da68bfb8104ecf58e73e3f99d87f81c90712a2ca/>
<csr-id-c77bd7a01820110154f2c66cd954c1ccfff173c1/>
<csr-id-71c628d46088ab455b54eb2330d24dcff96c911d/>
<csr-id-8fe461281842b58aa11437445637c6e587bedd63/>
<csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/>
<csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/>
<csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/>
<csr-id-0456312dddbd9ffd01e29c6705bb794cb1abb414/>

This is a maintenance release signalling breaking changes because some of the crates it depends on have breaking changes.

### Refactor

 - <csr-id-71c628d46088ab455b54eb2330d24dcff96c911d/> Use 'cache::Object' trait where it matters
 - <csr-id-8fe461281842b58aa11437445637c6e587bedd63/> split data::output::count::objects into files

### Other

 - <csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/> try to create persistent Easy iterator, but can't make it Send…
   …which is fair as it contains borrowed RefCells, which really would have
   to be owned to work for this, which would in turn require the Ancestor's
   struct to be kind of self-referential
 - <csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/> set package cache via RepositoryAccessExt
 - <csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/> prepare for configurable pack cache

### Chore

 - <csr-id-0456312dddbd9ffd01e29c6705bb794cb1abb414/> fix immediate dereference warning

### New Features

 - <csr-id-60c9fad8002b4e3f6b9607bba6361871752f4d3d/> control pack and object cache size in megabytes in some sub-commands
 - <csr-id-5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc/> object cache size is configurable in some sub-commands

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 34 calendar days.
 - 35 days passed between releases.
 - 12 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#164](https://github.com/Byron/gitoxide/issues/164), [#198](https://github.com/Byron/gitoxide/issues/198), [#200](https://github.com/Byron/gitoxide/issues/200), [#205](https://github.com/Byron/gitoxide/issues/205), [#67](https://github.com/Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#164](https://github.com/Byron/gitoxide/issues/164)**
    - rename path::is_git to path::is ([`ac3b9ef`](https://github.com/Byron/gitoxide/commit/ac3b9efb7b90958274ce55800959d930f8641115))
    - rename ObjectIdExt::ancestors_iter() to *::ancestors() ([`a19567e`](https://github.com/Byron/gitoxide/commit/a19567eceab0dd7f5478b83c2ff9ce79754db308))
 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - A changelog for gitoxide-core ([`b9f6a37`](https://github.com/Byron/gitoxide/commit/b9f6a37b9c27f2405694795adb476c01574b31ed))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - set package cache via RepositoryAccessExt ([`66292fd`](https://github.com/Byron/gitoxide/commit/66292fd1076c2c9db4694c5ded09799a0be11a03))
    - prepare for configurable pack cache ([`7d2b6b6`](https://github.com/Byron/gitoxide/commit/7d2b6b66e09ff39727fccd68d190679b52d90126))
 * **[#200](https://github.com/Byron/gitoxide/issues/200)**
    - feat: Add --reference/-r flag to gixp pack-receive ([`637d12c`](https://github.com/Byron/gitoxide/commit/637d12cf368e044f59ccde37c6365d9528d2c43f))
 * **[#205](https://github.com/Byron/gitoxide/issues/205)**
    - '(null)' symref targets are turned into direct refs instead… ([`c77bd7a`](https://github.com/Byron/gitoxide/commit/c77bd7a01820110154f2c66cd954c1ccfff173c1))
    - fetch::Ref::Symbolic::target is now an option… ([`da68bfb`](https://github.com/Byron/gitoxide/commit/da68bfb8104ecf58e73e3f99d87f81c90712a2ca))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - control pack and object cache size in megabytes ([`60c9fad`](https://github.com/Byron/gitoxide/commit/60c9fad8002b4e3f6b9607bba6361871752f4d3d))
    - Use 'cache::Object' trait where it matters ([`71c628d`](https://github.com/Byron/gitoxide/commit/71c628d46088ab455b54eb2330d24dcff96c911d))
    - split data::output::count::objects into files ([`8fe4612`](https://github.com/Byron/gitoxide/commit/8fe461281842b58aa11437445637c6e587bedd63))
    - object cache size is configurable ([`5a8c2da`](https://github.com/Byron/gitoxide/commit/5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc))
    - Count ref-deltas in thin packs as well ([`80c6994`](https://github.com/Byron/gitoxide/commit/80c6994149d19917c25e36e1bdf0dc8c9678365e))
    - Use Easy in the one spot where it is possible… ([`6a97bfa`](https://github.com/Byron/gitoxide/commit/6a97bfabcec6597efe9282e6d5c9f0ac3ada61dc))
    - try to create persistent Easy iterator, but can't make it Send… ([`54a64a5`](https://github.com/Byron/gitoxide/commit/54a64a588ff72515451a3d0343306ac4abe1cb35))
    - Add '--thin' flag to pack-create and pass it on ([`2664d73`](https://github.com/Byron/gitoxide/commit/2664d73f531a4b1f4bc784c1fe3a991711c86475))
 * **Uncategorized**
    - Release git-commitgraph v0.5.0, gitoxide-core v0.11.0, gitoxide v0.9.0 ([`960eb0e`](https://github.com/Byron/gitoxide/commit/960eb0e5e5a7df117ed2ae2a8e2ec167b074c332))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - fix immediate dereference warning ([`0456312`](https://github.com/Byron/gitoxide/commit/0456312dddbd9ffd01e29c6705bb794cb1abb414))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com/Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - Bump git-repository v0.10.0 ([`5a10dde`](https://github.com/Byron/gitoxide/commit/5a10dde1bcbc03157f3ba45104638a8b5b296cb9))
    - Release git-repository v0.9.1 ([`262c122`](https://github.com/Byron/gitoxide/commit/262c1229d6d2d55c70fe0e199ab15d10954d967b))
</details>

## v0.10.5 (2021-09-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.5 ([`590e59b`](https://github.com/Byron/gitoxide/commit/590e59b2b41a419574443e6b850bdb119a172279))
    - Bump git-repository v0.9.0 ([`b797fc1`](https://github.com/Byron/gitoxide/commit/b797fc10f3f3d1fbc23916a4ff6e5e860e2dd4ed))
    - [repository #193] Add feature flags for async/blocking ([`57f482c`](https://github.com/Byron/gitoxide/commit/57f482c59ac47b7a5f1abf01b4a3e25364e061c2))
</details>

## v0.10.4 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.4 ([`5ae584c`](https://github.com/Byron/gitoxide/commit/5ae584c65cc6e9ad306f077abb609159a15c6375))
    - [ref #190] move remaining file store functions to extension trait ([`60fc215`](https://github.com/Byron/gitoxide/commit/60fc215ccac529b4a14cb9d8260ab9ddec86758a))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com/Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [repository #190] Make local-offset available on demand only… ([`1927be7`](https://github.com/Byron/gitoxide/commit/1927be7764f6af04ecc715dd52c631a3c8e16577))
    - [repository #185] support for initializing bare repositories ([`9e8a39e`](https://github.com/Byron/gitoxide/commit/9e8a39e3cbd620bd48f379743df0d5783c33a86f))
    - [repository #185] refactor ([`63089ff`](https://github.com/Byron/gitoxide/commit/63089ff356ea0f62963ae213ea0dbb09f891ada6))
    - [repository #185] refactor repository initialization… ([`5ff7eaa`](https://github.com/Byron/gitoxide/commit/5ff7eaa86bddfa94aec97355a5d6adb117045693))
</details>

## v0.10.3 (2021-08-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.3 ([`e132680`](https://github.com/Byron/gitoxide/commit/e1326808a24fa7e797106cbd4bf3f34aba59b148))
    - [various #184] configure docs.rs build features ([`cc50249`](https://github.com/Byron/gitoxide/commit/cc502492c512293e93e95610ca80a71896076ded))
    - Bump git-repository v0.8.0 ([`cdb45ff`](https://github.com/Byron/gitoxide/commit/cdb45ffa0810e9fcc9fd25bff7b696c2d27eeef5))
    - Bump git-protocol v0.10.0 ([`82d5a0b`](https://github.com/Byron/gitoxide/commit/82d5a0bb38903a8389e43cd5416e02e5496e661a))
    - [odb #180] refactor ([`eff21da`](https://github.com/Byron/gitoxide/commit/eff21dae1083042412f45cd2f7a0faaf7d6400e6))
    - [pack #179] refactor bundle ([`420dca2`](https://github.com/Byron/gitoxide/commit/420dca29bccca6e7d759880d8342f23b33eead0d))
    - [pack #179] refactor ([`ab6554b`](https://github.com/Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com/Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177]  commit::RefIter -> CommitRefIter ([`e603306`](https://github.com/Byron/gitoxide/commit/e603306e81f392af97aa5afd232653de56bf3ce9))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - [ref #175] follow (try_)find(_what) naming convention ([`679895c`](https://github.com/Byron/gitoxide/commit/679895cf866d643e768e353af614a55aeed2ba5c))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com/Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [stability #171] Simply commit on git-ref/git-config stability tier 1… ([`f6560ff`](https://github.com/Byron/gitoxide/commit/f6560ffe8b9280c7e9c32afe0294ea3ee169dcf5))
    - Merge branch 'main' into stability ([`11bae43`](https://github.com/Byron/gitoxide/commit/11bae437e473fef6ed09c178d54ad11eee001b1d))
    - cleanup imports ([`e669303`](https://github.com/Byron/gitoxide/commit/e6693032f1391416fd704c21617051ddfb862a3a))
    - [stability #171] Don't leak unstable plumbing crates in git-repository… ([`71eb30f`](https://github.com/Byron/gitoxide/commit/71eb30f1caa41c1f9fe5d2785b71c9d77922c2af))
    - [pack #170] there can only be one ([`dce4f97`](https://github.com/Byron/gitoxide/commit/dce4f97a84aa6a73e31e7397501cfce27241c5b8))
    - [pack #170] clru allows for free lists, reducing allocation pressure... ([`4d820d2`](https://github.com/Byron/gitoxide/commit/4d820d2f94dc3afc062bbd25e969c87410212c3a))
    - Revert "[pack #67] Don't pre-fetch packed objects during counting" ([`811bb54`](https://github.com/Byron/gitoxide/commit/811bb54991636f7e517087b62cf0c8c8cc2ad9e6))
    - [pack #67] Don't pre-fetch packed objects during counting ([`d08b673`](https://github.com/Byron/gitoxide/commit/d08b6739d8e9294b795aba75e9c7f9f20645af2b))
    - [pack #67] refactor ([`14717f6`](https://github.com/Byron/gitoxide/commit/14717f6132672a5d271832a68de0b323b73abb2a))
    - [pack #67] Optimize caches based on cache debugging ([`1271c01`](https://github.com/Byron/gitoxide/commit/1271c01d2635ab49474add61a9feb78b98bd6180))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com/Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [pack #167] a single-threaded special case for counting… ([`65e29de`](https://github.com/Byron/gitoxide/commit/65e29de45a92c82cebd832634ab194db19a1b590))
    - [pack #167] Error handling for object input ([`0aac40c`](https://github.com/Byron/gitoxide/commit/0aac40c88a5c26f7c295db8433b510b168f15ca3))
    - [pack #167] remove iterator based count objects impl… ([`7ec2f2b`](https://github.com/Byron/gitoxide/commit/7ec2f2b40e83aaa218360a8b5989792cd67de2ed))
    - [pack] A non-iterator version of parallel object counting… ([`04fe855`](https://github.com/Byron/gitoxide/commit/04fe855a37577d3da5bbd619807b44e449947893))
    - [ref #165] refactor ([`66624c3`](https://github.com/Byron/gitoxide/commit/66624c3ef1faf7048ee86ed73cf5f622802c061e))
    - [repository #165] prepare for writing light docs for Easy ([`f8834c9`](https://github.com/Byron/gitoxide/commit/f8834c9c8d2ab2ce87857c6773c6204f60df240e))
    - [repository #165] refactor ([`3a0160e`](https://github.com/Byron/gitoxide/commit/3a0160ed1c5bc33d330ad4e9189c4937d194e98d))
    - thanks clippy ([`1f2d458`](https://github.com/Byron/gitoxide/commit/1f2d4584f8b650f7e751c8d2df9a5d27725f4f2f))
    - [smart-release #162] rename git-repository::object -> objs ([`ac70d81`](https://github.com/Byron/gitoxide/commit/ac70d81791cad04ffdeb04916d7a2a6b533eee6c))
</details>

## v0.10.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.2 ([`b96a518`](https://github.com/Byron/gitoxide/commit/b96a518610256bb2a684c940908aca26089db54b))
    - bump git-protocol to v0.9.0 as there are breaking changes ([`b4e3340`](https://github.com/Byron/gitoxide/commit/b4e33408b8eb12c9418704f663322385fd1dfb25))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.10.1 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 3 calendar days.
 - 4 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release gitoxide-core v0.10.1 ([`8b21d82`](https://github.com/Byron/gitoxide/commit/8b21d8214ddc0ad05ff559261aedb4a010ba8726))
    - [protocol] Make fetch-connection usage explicit ([`29696f9`](https://github.com/Byron/gitoxide/commit/29696f9b8e3ba3a72af1b099dac1c0866194d5ce))
</details>

## v0.10.0 (2021-08-10)

<csr-id-8802fa7e28ea8fcd3ef8dbca84be4e1f55eca665/>

### Other

 - <csr-id-8802fa7e28ea8fcd3ef8dbca84be4e1f55eca665/> this version fails to detect any git repo

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 146 commits contributed to the release over the course of 90 calendar days.
 - 93 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#83](https://github.com/Byron/gitoxide/issues/83)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#83](https://github.com/Byron/gitoxide/issues/83)**
    - [organize] Auto-strip .git suffix for non-bare repos ([`ea0ecc2`](https://github.com/Byron/gitoxide/commit/ea0ecc2f9b0dc25bbaa7788aac4eeed566f075cb))
 * **Uncategorized**
    - (cargo-release) version 0.10.0 ([`310dd22`](https://github.com/Byron/gitoxide/commit/310dd22cc5a01980ca604a0110e78d9b804902a6))
    - (cargo-release) version 0.7.0 ([`1c5dfb8`](https://github.com/Byron/gitoxide/commit/1c5dfb86028f266435475ca8bdddc57f95002330))
    - [core] refactor ([`e3d708f`](https://github.com/Byron/gitoxide/commit/e3d708f953f0e88180d3c2c6bd7c028e07faa583))
    - [core] refactor ([`869d162`](https://github.com/Byron/gitoxide/commit/869d162276ec5dbc9e8b02875dd4695a9e5256cb))
    - [gitoxide-core] avoid lossy path conversions ([`63c2951`](https://github.com/Byron/gitoxide/commit/63c2951970391fb7a708bf874661417a51a4cb97))
    - Use AsRef<Path> when opening from path ([`515d256`](https://github.com/Byron/gitoxide/commit/515d2564e430da77c092ceb9414a3b3e7071c158))
    - [protocol #145] Unify the `previous` and `previous_result` parameters… ([`96f77c7`](https://github.com/Byron/gitoxide/commit/96f77c78a08e975d367ca25ac5d07eb2253cf4e5))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Bump async-trait from 0.1.50 to 0.1.51 ([`ce0b81e`](https://github.com/Byron/gitoxide/commit/ce0b81e8f5c652d389ff876844bc42bcfa687921))
    - Bump serde_json from 1.0.64 to 1.0.65 ([`9117feb`](https://github.com/Byron/gitoxide/commit/9117feb5a228fa62f6f17fc4a0918717ccdb0b14))
    - [ref #140] do actual tag peeling in programs that matter ([`e404852`](https://github.com/Byron/gitoxide/commit/e40485233ee7a32c11426665a3d50f939d9637eb))
    - [ref #140] sketch ref tag peeling ([`ef90652`](https://github.com/Byron/gitoxide/commit/ef90652dfcd84b2fc140c38e1364b42578fdfbde))
    - [pack] fix build ([`e680854`](https://github.com/Byron/gitoxide/commit/e680854b12603ea898713e900a6b4407e93ebe91))
    - Bump futures-io from 0.3.15 to 0.3.16 ([`3c23820`](https://github.com/Byron/gitoxide/commit/3c23820d3f0d3567f44215cdb0ad13ab675a201f))
    - [pack] Make use of thin-pack resolver when writing bundles… ([`9f43bf0`](https://github.com/Byron/gitoxide/commit/9f43bf029624f7c94346646465e366609b89e2e1))
    - [pack] it seems git is just skipping bad objects during pack-gen ([`0f29b82`](https://github.com/Byron/gitoxide/commit/0f29b82b48f45f509016eb16ea92af7f6dbf65a6))
    - [pack] In single-threaded mode, use a huge cache for some speedup ([`aec8a9b`](https://github.com/Byron/gitoxide/commit/aec8a9b4b9deb102b06390a19727eab7660621f9))
    - [pack] pack-create with immediate counting and traversing… ([`b74a98f`](https://github.com/Byron/gitoxide/commit/b74a98fc87a92a8ccbaec59aeea5284731e2fe49))
    - [pack] refactor; entry-iterator now produces delta-objects ([`5dc370b`](https://github.com/Byron/gitoxide/commit/5dc370ba01d25a6e8b7f4bfa03259c83e6b1d758))
    - [pack] support poor reference resolution if input is not an object hash… ([`1b985a1`](https://github.com/Byron/gitoxide/commit/1b985a195e09c5681a6b6732c9a23895053dbcd2))
    - [pack] better identify the currently implemented pack generation mode. ([`f9e3b3c`](https://github.com/Byron/gitoxide/commit/f9e3b3ca3bbf063e8d71c62fe607b812c745a969))
    - [pack] refactor ([`78d46c1`](https://github.com/Byron/gitoxide/commit/78d46c13d0510ee3e2e2f33cd60d624d63e85900))
    - [ref] fix build ([`0b732e1`](https://github.com/Byron/gitoxide/commit/0b732e1349760eebf9d954fe7904d3c4b218b8b2))
    - [ref] figure out how peeling works with packed-refs… ([`2801f7a`](https://github.com/Byron/gitoxide/commit/2801f7aa137c6167bd392ca585f1aad378cae0b4))
    - [ref] fix build ([`83002df`](https://github.com/Byron/gitoxide/commit/83002df0296a431de839ebb3522f57d42a17515f))
    - [ref] rename find_one to 'find' in git-ref… ([`ae7746a`](https://github.com/Byron/gitoxide/commit/ae7746a0815bb94659de67383ba372ac522d53b8))
    - Bump anyhow from 1.0.41 to 1.0.42 ([`352e468`](https://github.com/Byron/gitoxide/commit/352e4689959dee169f08b8fbd7be3c1f234202b8))
    - Bump async-io from 1.4.1 to 1.6.0 ([`99e4732`](https://github.com/Byron/gitoxide/commit/99e4732f5148787b767afc6ad57666e31faac960))
    - [protocol] fix build ([`38aca40`](https://github.com/Byron/gitoxide/commit/38aca4076037a6f8288c2cf483f134ea16c328d5))
    - [protocol] fallible negotiation ([`e269a2c`](https://github.com/Byron/gitoxide/commit/e269a2cde18f604a36b33efb7e53f31ea5c45e2d))
    - [ref] rename Action::Close to Action::Cancel… ([`cac1f6c`](https://github.com/Byron/gitoxide/commit/cac1f6c757709797d193c6bca30e99fe40466ddc))
    - [protocol] support ref-in-want ([`b6df400`](https://github.com/Byron/gitoxide/commit/b6df400dccd66ad2f01c80d2fa05b8f9bb130b23))
    - [actor] fix gix hours ([`b4e95fd`](https://github.com/Byron/gitoxide/commit/b4e95fdbb6664adcb2603d9cb6e6a69182de050f))
    - [actor] git-object uses git-actor ([`d01dd2f`](https://github.com/Byron/gitoxide/commit/d01dd2f9e9e8e2b81cdb1131a436d32b5819b731))
    - clippy cleanup; fix CI build ([`3e943f2`](https://github.com/Byron/gitoxide/commit/3e943f2afd5f0cfe7294a21cca8e0344c7dd0216))
    - thanks clippy ([`3f7e27b`](https://github.com/Byron/gitoxide/commit/3f7e27b91e2c7d66959f5f4c1a667f3315111cd6))
    - Fix everything up so that… ([`5930563`](https://github.com/Byron/gitoxide/commit/5930563601d6c2148cf39e109f69f8b7c7dfcb36))
    - A first attempt to make intrerupt tools work, but… ([`8fb8d37`](https://github.com/Byron/gitoxide/commit/8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5))
    - fix build ([`ea2bfac`](https://github.com/Byron/gitoxide/commit/ea2bfac65f742ca7617bc77a50376c29156c4ea5))
    - refactor ([`7f9be36`](https://github.com/Byron/gitoxide/commit/7f9be36ea909ee67555591287bcb140fdc54c801))
    - And one less usage of the global interrupt handler… ([`5da57a3`](https://github.com/Byron/gitoxide/commit/5da57a3b0efef75ad82cb4d1cd496fc7fc0f1c23))
    - Make most interrupts local to the method or function ([`4588993`](https://github.com/Byron/gitoxide/commit/458899306a3f3c8578f185d7ecbf1ade2a7142dd))
    - [hours] use new interrupt::Iter; refactor ([`2355f0b`](https://github.com/Byron/gitoxide/commit/2355f0b2de4a6d7b1e206aa2a445814947983e55))
    - [pack-create] also show throughput ([`74d8d57`](https://github.com/Byron/gitoxide/commit/74d8d57f5da84b55219c2d3b115709dc0b422897))
    - [tempfile] interruptible traversal ([`4eeaa1b`](https://github.com/Byron/gitoxide/commit/4eeaa1bb9ca4af2eb21807007eabeb714c98fdfe))
    - [pack-create] better handling of input paths ([`1825e1a`](https://github.com/Byron/gitoxide/commit/1825e1a68d2a3274b0cc7d6ae56a31cb8145d944))
    - [pack-create] progress for ancestor traversal ([`9349286`](https://github.com/Byron/gitoxide/commit/9349286c3afa89a472e33d6281414dd6bf2b90a2))
    - refactor ([`e0b7f69`](https://github.com/Byron/gitoxide/commit/e0b7f695ee6bd1032544a29d91906f9b75e12d46))
    - [pack] refactor ([`25f04ba`](https://github.com/Byron/gitoxide/commit/25f04baa100bd1996f48fbeb4c87e40ff1b27d90))
    - [pack] validate tips as well… ([`ec8864f`](https://github.com/Byron/gitoxide/commit/ec8864ff23f18a00fe39d5d0061a1ab73810e283))
    - [pack] refactor ([`18cabb8`](https://github.com/Byron/gitoxide/commit/18cabb8618ffc324412302bfda208948abffb61f))
    - [pack] Force single-threading (with toggle) for counting phase… ([`8d3ba0b`](https://github.com/Byron/gitoxide/commit/8d3ba0b863f82d4eed0d9ca1ddf439f6feaf5041))
    - [pack] also put counts in order for stable packs ([`f299160`](https://github.com/Byron/gitoxide/commit/f299160cafd00f0fea00a2402901570f5ddf27d5))
    - [pack] gixp pack-create uses in-order adapter as well ([`365c582`](https://github.com/Byron/gitoxide/commit/365c58286b9e09c9a8b1b5d6ee3b76484a458ca7))
    - [pack] refactor ([`cfdf802`](https://github.com/Byron/gitoxide/commit/cfdf8021ea1448ac4844b1f3bf252fefde2572fa))
    - [pack] print the pack file name even if there is no output directory ([`832fa29`](https://github.com/Byron/gitoxide/commit/832fa291595aaae7f7862d95bb1cbebcc34f2271))
    - [pack] refactor ([`9d9def3`](https://github.com/Byron/gitoxide/commit/9d9def30784a1b90f27c8181bfb0b0ba4ed4f1c8))
    - [pack] pack-create --output-directory is now optional ([`2150be8`](https://github.com/Byron/gitoxide/commit/2150be816f8a9d0ec049841045c904be1bb57ed6))
    - [pack] print statistics for entries iteration as well ([`eb6554b`](https://github.com/Byron/gitoxide/commit/eb6554b84131a09e5779edee302709c8ab62f47d))
    - [pack] add --statistics flag to pack-create ([`51a3077`](https://github.com/Byron/gitoxide/commit/51a307730b8514acffa75c78ecca3f02b1eb467b))
    - refactor ([`24697bc`](https://github.com/Byron/gitoxide/commit/24697bc66363f8e8b1ff14a59fdf303ffdab132d))
    - [async-receive] refactor ([`7e28831`](https://github.com/Byron/gitoxide/commit/7e288316a4cc402bd32489dbf5ca0050f84cfb18))
    - Bump anyhow from 1.0.40 to 1.0.41 ([`f6d48c8`](https://github.com/Byron/gitoxide/commit/f6d48c8c0ad2d92b587ba9cfc5f6e941203c7c4d))
    - [pack] write packs to a directory with the proper name ([`3fbca7d`](https://github.com/Byron/gitoxide/commit/3fbca7dd62752a7dd752b83a39ec8dfd7b2f2ea8))
    - [pack] refactor ([`f10adea`](https://github.com/Byron/gitoxide/commit/f10adea76d92eada3ca204fe69e7b5f81a06d8cc))
    - [pack] fix build ([`81ee633`](https://github.com/Byron/gitoxide/commit/81ee633c7f482746bc28a2a43d74ebbaded7af5f))
    - [pack] refactor ([`0514f1d`](https://github.com/Byron/gitoxide/commit/0514f1df113c5f6bf1c934b15741ca8ea47316ae))
    - [pack] refactor ([`37922d1`](https://github.com/Byron/gitoxide/commit/37922d12765c221e747fad4ca813597490525279))
    - Bump itertools from 0.10.0 to 0.10.1 ([`b54f21d`](https://github.com/Byron/gitoxide/commit/b54f21da9d41aa4fc67e5b1bf7ab979ec1bd9760))
    - [async-client] refactor ([`e7d115c`](https://github.com/Byron/gitoxide/commit/e7d115c4be758b48172b07d94139810b6fcc7fa3))
    - [async-client] cleanup Send bounds! ([`c7dee44`](https://github.com/Byron/gitoxide/commit/c7dee44267462d5ece491b8a45cf35afa904ce81))
    - [async-client] refactor ([`89e6f66`](https://github.com/Byron/gitoxide/commit/89e6f66e6e549fcc9bf72e4e837ff4e3dce66d2d))
    - Revert "[async-client] FAIL with the brutal copy-paste way" ([`7f29adc`](https://github.com/Byron/gitoxide/commit/7f29adc2936e1266a0e2c698c1a4677cf822a5f6))
    - [async-client] FAIL with the brutal copy-paste way ([`b91ecb5`](https://github.com/Byron/gitoxide/commit/b91ecb536c9c3e1a77647025f7a72fb098e83082))
    - Revert "[async-client] the beginning of an unholy transformation…" ([`c8423a8`](https://github.com/Byron/gitoxide/commit/c8423a83b5212b5381ae03accf386be2f882e78c))
    - [async-client] the beginning of an unholy transformation… ([`1f314df`](https://github.com/Byron/gitoxide/commit/1f314df4f0101bc3970201b66298afa8a35bf22c))
    - [async-client] refactor ([`b252932`](https://github.com/Byron/gitoxide/commit/b252932ee3eb26bb26560a849a9b13aca11cf00f))
    - [async-client] unblock the async delegate in the cheapest possible way… ([`a3b5d75`](https://github.com/Byron/gitoxide/commit/a3b5d75d387dc5d6c44f695f63df8803613637a2))
    - [async-client] prepare for unblocking the protocol delegate ([`796c7d5`](https://github.com/Byron/gitoxide/commit/796c7d54a20ef32a581be572e1d681f9727482de))
    - [async-client] refactor ([`0d5b911`](https://github.com/Byron/gitoxide/commit/0d5b911ad5f47ab8f044d6bbe660a6d1dfeecb5f))
    - Revert "[async-client] Try to bring 'Send' back but…" ([`52eb953`](https://github.com/Byron/gitoxide/commit/52eb953fcc44cce19604b1df6a600237b8c81392))
    - [async-client] Try to bring 'Send' back but… ([`3a06adb`](https://github.com/Byron/gitoxide/commit/3a06adb41f6b2946f78044e4ab1385e6441fc40f))
    - [async-client] refactor ([`dc742df`](https://github.com/Byron/gitoxide/commit/dc742dfbc877f4a39b5659ea4960408ce0a1d247))
    - [async-client] Unblock printing in pack-receive ([`156bed6`](https://github.com/Byron/gitoxide/commit/156bed6be1d830eb853d90dcb98c81978725d958))
    - [async-client] Sketch of (partially blocking) pack-receive ([`e58859d`](https://github.com/Byron/gitoxide/commit/e58859d133ee23b098df9107b6da5c0cc9bb696a))
    - [async-client] ls-remote in async (but for git protocol only) ([`fd8edca`](https://github.com/Byron/gitoxide/commit/fd8edca42a58a901e749d599eb552315d7b24a78))
    - [async-client] basic git_connect functionality using async_io/async_net ([`af60297`](https://github.com/Byron/gitoxide/commit/af60297cf2b80d862880a2178e08f3f23b796f1d))
    - [async-client] frame for async connect ([`9ada080`](https://github.com/Byron/gitoxide/commit/9ada0805fc5896f8ef1a31dc821b789b7f0438a6))
    - [async-client] frame from A to Z to actually implement it… ([`ac4715c`](https://github.com/Byron/gitoxide/commit/ac4715c53798c8438fb30802d6b83c868915522b))
    - Separate networking via feature toggles and pass that through in the main crate ([`2c749f1`](https://github.com/Byron/gitoxide/commit/2c749f10dd03ea0b027fb046e8c40c77869fb2e9))
    - [git-protocol] refactor ([`94d7be4`](https://github.com/Byron/gitoxide/commit/94d7be4a16f2c2e68a9dacf120eef7a417a8a6b9))
    - [gix-organize] fast-prefilter + close look at the repository itself ([`eda440a`](https://github.com/Byron/gitoxide/commit/eda440ab7efc81749b20a0f21a46825c945ff6db))
    - this version fails to detect any git repo ([`8802fa7`](https://github.com/Byron/gitoxide/commit/8802fa7e28ea8fcd3ef8dbca84be4e1f55eca665))
    - [gix-organize] use git-repository a little more ([`20f76a5`](https://github.com/Byron/gitoxide/commit/20f76a5fc93c9a59e26688dce3e82114ccaeffe3))
    - Revert 'gix-organize' to normal thanks to performance regression ([`eda452e`](https://github.com/Byron/gitoxide/commit/eda452e14564e802e9314d94993ae8c8590c5301))
    - (cargo-release) version 0.6.0 ([`d35c55d`](https://github.com/Byron/gitoxide/commit/d35c55d8ff4b52e25befb8bff839d805b9f3caf4))
    - thanks clippy ([`6a80d5c`](https://github.com/Byron/gitoxide/commit/6a80d5c02d01ab1fc6388eb0eb79d0a4407efab6))
    - [git-repository] gitoxide-core uses more of git-repository ([`bb5b074`](https://github.com/Byron/gitoxide/commit/bb5b0747dfd3a3985a904b7748f296a591fcb26e))
    - [git-repository] replaces git-features and git-protocol in gitoxide-core ([`081d20f`](https://github.com/Byron/gitoxide/commit/081d20f927f222daa69f2a1a492957fd3146bfc1))
    - refactor ([`2ba9f91`](https://github.com/Byron/gitoxide/commit/2ba9f915035a518bef3eb8b0ed1c9972c4a47cfa))
    - [git-repository] used by gix-hours ([`24e0258`](https://github.com/Byron/gitoxide/commit/24e0258b9691b82df5c35a35111d19df56087cdc))
    - [git-repository] refactor ([`b5ebcfa`](https://github.com/Byron/gitoxide/commit/b5ebcfa278a0be85ea10893fd40a8b3e2e28efd5))
    - [git-repository] now used by gixp-organize ([`aa91fad`](https://github.com/Byron/gitoxide/commit/aa91fad3cf237f6d6f9d588ed390baa6e55f6540))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com/Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - [git-odb] much better docs; cleanup exposed API ([`3d5b229`](https://github.com/Byron/gitoxide/commit/3d5b229c2605060f2cac9695ff2479777deabdd0))
    - (cargo-release) version 0.2.0 ([`b213628`](https://github.com/Byron/gitoxide/commit/b213628feeb8dfa87dab489c7d3155a60e6a236d))
    - [git-odb] refactor ([`2958145`](https://github.com/Byron/gitoxide/commit/2958145a0ae1ef582bbf88352f5567d5c2b5eaf0))
    - [git-odb] refactor ([`1eab15d`](https://github.com/Byron/gitoxide/commit/1eab15dfb42c819050b0277c4cb6a1045d2fd58d))
    - [git-pack] compilation ([`b392a55`](https://github.com/Byron/gitoxide/commit/b392a55b97a30b10ac0db94a96230e22ea7ab0dc))
    - [git-pack] refactor ([`157b6ff`](https://github.com/Byron/gitoxide/commit/157b6ff7b55ba2b7f8f90f66864212906426f8d7))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com/Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-pack] refactor ([`e5b00ee`](https://github.com/Byron/gitoxide/commit/e5b00ee257b712477413f48448b0bccf9a06bfaf))
    - [git-pack] the world compiles again ([`f0c0e36`](https://github.com/Byron/gitoxide/commit/f0c0e36a1fb15d44776678567162ac754fdd26c0))
    - [git-odb] refactor ([`e07478c`](https://github.com/Byron/gitoxide/commit/e07478c7b212e4d1d21ce151d9eb26d0fae422a8))
    - [git-odb] refactor ([`721303d`](https://github.com/Byron/gitoxide/commit/721303db232f87857aae58e12b342e5fb0139306))
    - [git-odb] refactor ([`ea224e9`](https://github.com/Byron/gitoxide/commit/ea224e9ee5f7efcbf4942a2a6fc7e4d790b2be50))
    - [git-odb] refactor ([`6a1b16a`](https://github.com/Byron/gitoxide/commit/6a1b16ae98edc9a694b945a12a7866eb17fc6be3))
    - [git-odb] refactor ([`47c4042`](https://github.com/Byron/gitoxide/commit/47c4042f16a0e0e6a536bab7150b7cb21958a7ed))
    - Configure git-features properly for gitoxide-core… ([`251e690`](https://github.com/Byron/gitoxide/commit/251e69030c2c25493a7e2ff0cb79ca01dfa228f5))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com/Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - Merge pull request #88 from avoidscorn/traverse-partial-ancestors ([`966f058`](https://github.com/Byron/gitoxide/commit/966f058beac9bec8277abb26b7cb3caf76df0cbf))
    - Prevent pack-index-from-data to block if stdin is a terminal ([`39dec0e`](https://github.com/Byron/gitoxide/commit/39dec0e25b23162cfd8171bc44477c4d936fc00a))
    - [pack-gen] release a little memory, hopefully ([`f25293a`](https://github.com/Byron/gitoxide/commit/f25293ae7885a21db72b84a3aa49eca3aafbdaef))
    - Revert "[pack-gen] remove tree-diff as traversal option." ([`2907a5f`](https://github.com/Byron/gitoxide/commit/2907a5facb08a7decbdfa652e76eb0ebd5e29dcf))
    - [pack-gen] remove tree-diff as traversal option. ([`8373671`](https://github.com/Byron/gitoxide/commit/8373671fd4f3f7e9d78c480e9f68c0a7ae423c69))
    - [pack-gen] a lot more progress, even though it's not perfect yet ([`480f8b7`](https://github.com/Byron/gitoxide/commit/480f8b720d84502bddd06cdbb35bf5cb69f9249d))
    - [pack-gen] basic progress for entry generation ([`953190d`](https://github.com/Byron/gitoxide/commit/953190d70a5df22b54dc1fffe78d41dc7d81cc61))
    - [pack-gen] better progress ([`fdee381`](https://github.com/Byron/gitoxide/commit/fdee381073459dc7d1e2e964a930aaf8db36def5))
    - [pack-gen] the first barely working progress ([`5b89a0e`](https://github.com/Byron/gitoxide/commit/5b89a0e4203d405a50bc2e8de9d87b79e545412d))
    - [pack-gen] the basics to get the program going ([`03b67b0`](https://github.com/Byron/gitoxide/commit/03b67b09e4127ae4bd791501d74794d9360f7ef6))
    - [pack-gen] very close to a basic impl of count + entries-gen… ([`c927429`](https://github.com/Byron/gitoxide/commit/c9274295e62f59cd8db06a307cc4a69d096a006e))
    - [pack-gen] Try to just ignore the amount of objects inside… ([`918b222`](https://github.com/Byron/gitoxide/commit/918b222343dbcb5fb0177526a997d0f3cb4ac585))
    - thanks clippy ([`89b1ee4`](https://github.com/Byron/gitoxide/commit/89b1ee48d4c93e8ecee7630bc894c8ca994cb989))
    - [pack-gen] And it shows we really need to let the traversal be done first ([`a870eb2`](https://github.com/Byron/gitoxide/commit/a870eb2b46a95e8ea69632eceef3fc4e37bbac4c))
    - [pack-gen] And now it creates an entries iterator ([`27c9bc1`](https://github.com/Byron/gitoxide/commit/27c9bc1e8a254689d6c337677f71d51518f6800e))
    - [pack-gen] A step further, but it looks like input object iteration is tricky ([`abf4276`](https://github.com/Byron/gitoxide/commit/abf427674805f8624ec381a00d8c70c569515878))
    - [pack-gen] Frame for plumbing command ([`a2203ca`](https://github.com/Byron/gitoxide/commit/a2203ca7a403ece79dda5c568f0bf6da34535882))
    - (cargo-release) version 0.10.0 ([`5d7ee6a`](https://github.com/Byron/gitoxide/commit/5d7ee6a105abbb6efeed8624bade936bb59dbc55))
    - refactor ([`9f0a8cc`](https://github.com/Byron/gitoxide/commit/9f0a8cc1561589088f44a1775832110449a4f1ab))
    - (cargo-release) version 0.3.0 ([`684de4b`](https://github.com/Byron/gitoxide/commit/684de4b376ecd4cc5330f7ac8643352ea9580ed3))
    - (cargo-release) version 0.8.0 ([`ccea4b6`](https://github.com/Byron/gitoxide/commit/ccea4b6bcdaba0ee6c6a6236d225ea1276d2547c))
    - [git-transport] remove default features to force being explicit everywhere ([`d1b39f8`](https://github.com/Byron/gitoxide/commit/d1b39f8093c032a172237a584c9208479611a866))
    - [organize] Be clear about what the traversal really does ([`ed945ab`](https://github.com/Byron/gitoxide/commit/ed945abfd80a4e5f994a3dee1b1deae30f57a3aa))
    - refactor ([`ef80fd6`](https://github.com/Byron/gitoxide/commit/ef80fd693204d42fdc125ea89f1c26643e99bde9))
</details>

## v0.9.0 (2021-05-09)

<csr-id-747a13e9a1fe5200c53055dd961507c9fef667e1/>
<csr-id-e7971a924df0ab958d56239f48eaafda30f15159/>

### Other

 - <csr-id-747a13e9a1fe5200c53055dd961507c9fef667e1/> :borrowed::Object => git-odb::data::Object
 - <csr-id-e7971a924df0ab958d56239f48eaafda30f15159/> pack-verify: be explicit about pack-cache choice in relation to algorithm
   Only when doing less-memory the pack cache is even used.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 36 commits contributed to the release over the course of 27 calendar days.
 - 30 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.9.0 ([`e6cdd84`](https://github.com/Byron/gitoxide/commit/e6cdd8423a57b8b3982adb168c2652676df5fa37))
    - (cargo-release) version 0.7.0 ([`069184e`](https://github.com/Byron/gitoxide/commit/069184e55057a1655d2754cb1fd68a4424beff34))
    - (cargo-release) version 0.15.0 ([`d91b241`](https://github.com/Byron/gitoxide/commit/d91b2412381e3c8c1f24c38469e821c3c3960e34))
    - (cargo-release) version 0.2.0 ([`3fb8377`](https://github.com/Byron/gitoxide/commit/3fb8377ff36422fe7607fb9172edf8bd5a4db995))
    - (cargo-release) version 0.9.0 ([`84897fd`](https://github.com/Byron/gitoxide/commit/84897fd8e6e1b0269da0303d6a0de8f9e0eb58e5))
    - Merge branch 'patch-2' ([`f01dc54`](https://github.com/Byron/gitoxide/commit/f01dc54010683b232c5f5813bd5370e93f1681f5))
    - Merge branch 'patch-1' ([`5edc076`](https://github.com/Byron/gitoxide/commit/5edc0762524112bb6716b3afcf23b2a4a0f5efd3))
    - [hours-tool] interruptability of long-running commit interations ([`4fd8a63`](https://github.com/Byron/gitoxide/commit/4fd8a63d82e87e2a9f3b0268726d08e7b954942c))
    - Add missing docs; add local-only snapshot file ([`7c56366`](https://github.com/Byron/gitoxide/commit/7c56366f4d34e67cfd31dbbcdb6f96ba61fd1668))
    - [hours-tool] Better error messages ([`86b4570`](https://github.com/Byron/gitoxide/commit/86b4570effdb756d86c105926ae0dc942399981c))
    - [hours-tool] integrate progress, remove direct writes to stderr ([`2778447`](https://github.com/Byron/gitoxide/commit/27784478c6e365bc92cb0ae7d7b372f073c47293))
    - [hours-tool] bring in all the code, mostly unchanged. ([`df16b3c`](https://github.com/Byron/gitoxide/commit/df16b3c269c0a2fa4d487988fd4fdd029b3a26f7))
    - [hours-tool] hookup new gitoxide-core command ([`680f274`](https://github.com/Byron/gitoxide/commit/680f2742a04749a9d692741381b4c662f28f3179))
    - thanks clippy ([`17258cc`](https://github.com/Byron/gitoxide/commit/17258cc58767caa6e71227898decd160ad0cdf13))
    - refactor ([`8b10434`](https://github.com/Byron/gitoxide/commit/8b1043483cb46fd1b7f47a90c9dce24a65d58d1b))
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com/Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
    - (cargo-release) version 0.14.0 ([`d9514ee`](https://github.com/Byron/gitoxide/commit/d9514eec64579ef77c9f2ac5dfe87cd302180eb9))
    - rename 'Locate' to 'Find' - shorter and just as good ([`60f72f5`](https://github.com/Byron/gitoxide/commit/60f72f573a7696323e09bf4add80d5fbce22c99d))
    - (cargo-release) version 0.13.0 ([`5c791af`](https://github.com/Byron/gitoxide/commit/5c791af217fac6a171d174ad9f4ee5f4d5282892))
    - (cargo-release) version 0.8.0 ([`a1ce210`](https://github.com/Byron/gitoxide/commit/a1ce210003ff07bf11291018bb182cbc7913647b))
    - (cargo-release) version 0.3.0 ([`e9665c7`](https://github.com/Byron/gitoxide/commit/e9665c784ae7e5cdaf662151395ee2355e9b57b6))
    - Don't mention skips anymore… ([`afb87d9`](https://github.com/Byron/gitoxide/commit/afb87d9be442a1f62a069ed58948e49cd7595a3a))
    - refactor ([`c1013dd`](https://github.com/Byron/gitoxide/commit/c1013dddbc221b366b91d186cfd1732f1d72be10))
    - refactor ([`ca98221`](https://github.com/Byron/gitoxide/commit/ca98221d5a512dabf683cc1da56d40a17285f2fb))
    - refactor ([`d490b65`](https://github.com/Byron/gitoxide/commit/d490b65ebbc6666cd59d88f8677dc1c52bfe1e1c))
    - refactor ([`08fafaa`](https://github.com/Byron/gitoxide/commit/08fafaa03144fc3ddea9120a4a1943e18c454ae8))
    - :borrowed::Object => git-odb::data::Object ([`747a13e`](https://github.com/Byron/gitoxide/commit/747a13e9a1fe5200c53055dd961507c9fef667e1))
    - bump git-odb minor version ([`5c833ce`](https://github.com/Byron/gitoxide/commit/5c833ce64babd00b7ced3e3a1c9ed3dbd260c9f4))
    - Remove loose::Object entirely #(67) ([`5cf4840`](https://github.com/Byron/gitoxide/commit/5cf4840b10a3fac43266bc9defa72977a004bf8c))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com/Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
    - (cargo-release) version 0.11.0 ([`fd698e3`](https://github.com/Byron/gitoxide/commit/fd698e334e44d5c478c162f98d09afd9ce7a6895))
    - Introduce pack_id for use in pack cache, preventing (most collisions) ([`ad04ad3`](https://github.com/Byron/gitoxide/commit/ad04ad3b8ac54e78bee307dd44c85c1389edced2))
    - Feature toggle for uluru based Lru cache ([`98eec48`](https://github.com/Byron/gitoxide/commit/98eec4837d605a408b026a859e53a7e2eae8e4da))
    - pack-verify: be explicit about pack-cache choice in relation to algorithm ([`e7971a9`](https://github.com/Byron/gitoxide/commit/e7971a924df0ab958d56239f48eaafda30f15159))
    - refactor ([`d624d09`](https://github.com/Byron/gitoxide/commit/d624d097784eed216f8d0e94544d8b62ef6c3010))
    - LruCache with const-generics ([`93618d1`](https://github.com/Byron/gitoxide/commit/93618d107e9defadb603209251f77948caddc121))
</details>

## v0.8.0 (2021-04-08)

<csr-id-b85a3891a08248aaa0d7ec429940c9793a2ddcd1/>
<csr-id-0f4265fa93060f47637ac7e9bd286d4918b3db62/>

### Other

 - <csr-id-b85a3891a08248aaa0d7ec429940c9793a2ddcd1/> make it work with bare and non-bare repositories
   This was previously handled correctly by `git` itself, and unfortunately
   the journey tests don't cover this particular case.
   
   Maybe something to improve at some point, note taken.
 - <csr-id-0f4265fa93060f47637ac7e9bd286d4918b3db62/> Make client state meaning explicit

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 56 commits contributed to the release over the course of 98 calendar days.
 - 113 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#63](https://github.com/Byron/gitoxide/issues/63)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#63](https://github.com/Byron/gitoxide/issues/63)**
    - git-protocol uses `oid` type ([`3930a6f`](https://github.com/Byron/gitoxide/commit/3930a6ff508f5bb2249fb2c2f21e00b74fecda22))
    - Use new `oid` where possible in git-odb ([`68a709e`](https://github.com/Byron/gitoxide/commit/68a709e0337d4969138d30a5c25d60b7dbe51a73))
    - Make ObjectId/oid happen! ([`ca78d15`](https://github.com/Byron/gitoxide/commit/ca78d15373ec988d909be8f240baefe75555e077))
    - Remove all public exports of git-hash types in git-object ([`accf89d`](https://github.com/Byron/gitoxide/commit/accf89d25560e5ded6f44a1c4a898ee65d14f8f6))
 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`02df134`](https://github.com/Byron/gitoxide/commit/02df1345a22889a573adfc1be80bda271b2dc9a5))
    - (cargo-release) version 0.8.0 ([`1a2a5cc`](https://github.com/Byron/gitoxide/commit/1a2a5cc093139cecb1516e8235f087ad12cfb703))
    - (cargo-release) version 0.6.0 ([`8513f0f`](https://github.com/Byron/gitoxide/commit/8513f0fafbf8ae61d86df2d8b0aefa52d3eb1680))
    - (cargo-release) version 0.10.0 ([`3161777`](https://github.com/Byron/gitoxide/commit/316177729e42f8d000a40ab01b9b97621e7179e8))
    - (cargo-release) version 0.7.0 ([`b900914`](https://github.com/Byron/gitoxide/commit/b900914a00292217ba7b9bcac260591800395287))
    - (cargo-release) version 0.4.0 ([`06612eb`](https://github.com/Byron/gitoxide/commit/06612eb12d4679bec7dae08a511dd87d80087151))
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com/Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
    - (cargo-release) version 0.2.0 ([`4ec09f4`](https://github.com/Byron/gitoxide/commit/4ec09f4d2239ea1d44f7145027e64191bf2c158c))
    - Remove locate(…) -> Option<Result<…>> in favor of Result<Option<…>> ([`40ee743`](https://github.com/Byron/gitoxide/commit/40ee7438a98c4094c0fd04977cd4904668087512))
    - A trial for Result<Option<Object>>  for loose object databases ([`3842859`](https://github.com/Byron/gitoxide/commit/3842859c5bddb8b4583443685c26dcae3f8db558))
    - Added [directory] argument to init. ([`62f8dc6`](https://github.com/Byron/gitoxide/commit/62f8dc62ec3e76efd7311ced32094035856dbcbb))
    - (cargo-release) version 0.9.0 ([`efc8983`](https://github.com/Byron/gitoxide/commit/efc898381d830e44487c62e35a665d3ccd0a2d39))
    - (cargo-release) version 0.5.0 ([`3cc4a57`](https://github.com/Byron/gitoxide/commit/3cc4a5799fa1f487452b5c346b57fea97e45b47e))
    - (cargo-release) version 0.3.0 ([`d5c6643`](https://github.com/Byron/gitoxide/commit/d5c6643a41d295eaf7aabb84eab435e42a11dd42))
    - thanks clippy ([`f25598a`](https://github.com/Byron/gitoxide/commit/f25598a82256d2c7d538e9be90437cb5ca8c973f))
    - thanks clippy ([`0fc239c`](https://github.com/Byron/gitoxide/commit/0fc239cf9b773f72928b7c42344b578c6ff5d19f))
    - [gix] Use flate2 by default ([`f1158a1`](https://github.com/Byron/gitoxide/commit/f1158a1a4bc8e13913461db4d4851e32d57816ff))
    - [gix] Add optional zlib feature ([`f1f9665`](https://github.com/Byron/gitoxide/commit/f1f96658a6cd6165ba9c9d7acb809fcaf2c46f9c))
    - make it work with bare and non-bare repositories ([`b85a389`](https://github.com/Byron/gitoxide/commit/b85a3891a08248aaa0d7ec429940c9793a2ddcd1))
    - Make client state meaning explicit ([`0f4265f`](https://github.com/Byron/gitoxide/commit/0f4265fa93060f47637ac7e9bd286d4918b3db62))
    - [gitoxide-core] Fix find_origin_remote location ([`a3c19fc`](https://github.com/Byron/gitoxide/commit/a3c19fcfdf144119caf469c0d18278a1578c483e))
    - [gitoxide-core] Use git-config for remote url parsing ([`c45feed`](https://github.com/Byron/gitoxide/commit/c45feed6124601a8bbef609d5b47c5b8a9d5defa))
    - [gitoxide-core] Use git-config as dependency ([`c567925`](https://github.com/Byron/gitoxide/commit/c567925906c73a00753f4ddb6bcbd64d99d78885))
    - Make 'find' reproducable ([`c5af6eb`](https://github.com/Byron/gitoxide/commit/c5af6eb1f044d1396f23839ecec08bb0e6776fe6))
    - mildly improve performance in case there is nothing to do for 'organize' ([`4f9fdc5`](https://github.com/Byron/gitoxide/commit/4f9fdc5be6eac4ad518469990b3258a40262d337))
    - Fix journey tests by not allowing canonicalization of possibly… ([`532ff2b`](https://github.com/Byron/gitoxide/commit/532ff2b9deb491d870b772d91fad0024790e8f59))
    - Avoid claiming we would move something even though we won't (in 'organize') ([`47c7fb3`](https://github.com/Byron/gitoxide/commit/47c7fb3bb1a24e5d2fc1aa71f2febe8fe87172d4))
    - (cargo-release) version 0.8.0 ([`1ccfdcd`](https://github.com/Byron/gitoxide/commit/1ccfdcdb96b59c6415e7fbc800371d594b2ef7a1))
    - Implement `find` subcommand ([`28d506a`](https://github.com/Byron/gitoxide/commit/28d506a6c0df18fc0c2e4a578707203f8e89577d))
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com/Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
    - Fix tests ([`da94cfc`](https://github.com/Byron/gitoxide/commit/da94cfcfa3e745d1174fd9b065f57f56e9f70efe))
    - thanks clippy ([`de32204`](https://github.com/Byron/gitoxide/commit/de32204cdac809fb20c9fe56d5ea6fa828217038))
    - Avoid moving nested repositories out of their place ([`5d7e6bf`](https://github.com/Byron/gitoxide/commit/5d7e6bf22af432a3a813daaf485b3a72f64bf257))
    - Recurse into directories much less… ([`87561eb`](https://github.com/Byron/gitoxide/commit/87561eb0df8a9da3e0befcdf3d0976cc6a66550d))
    - Better use of jwalk filter capabilities… ([`781ea7f`](https://github.com/Byron/gitoxide/commit/781ea7fe00fd48c68c0bdc5e0bf03d47dfce4f63))
    - optimize number of CPUs for directory walk for M1 chips ([`129a699`](https://github.com/Byron/gitoxide/commit/129a69997fb5c50d28fe9340a9b20bab0a69121e))
    - Remove usage of gitfeatures::fs in organize subcommand ([`b567d37`](https://github.com/Byron/gitoxide/commit/b567d3709a74e9fdafef54b0fe58ca82721cd773))
    - prepare to put 'organize' behind a feature flag ([`9986509`](https://github.com/Byron/gitoxide/commit/9986509af150a90c4c9271b402fcac419090d9d4))
    - refactor; planning ([`5df492c`](https://github.com/Byron/gitoxide/commit/5df492c7d7322bde2b268deaf590f1ba012a6b8e))
    - fix progress ([`1abd761`](https://github.com/Byron/gitoxide/commit/1abd761670f6b6aba3af10ab4a60c86a3f314f6a))
    - Assure basic 'organize' operation is working as expected ([`deb6073`](https://github.com/Byron/gitoxide/commit/deb6073671ae95de674aaef7ca01e03f95b41ca8))
    - A version of organize which works; in theory ([`800a2f4`](https://github.com/Byron/gitoxide/commit/800a2f4a488112fbd31882c734889e4841aaa120))
    - A first stab at finding git repositories ([`e4dc964`](https://github.com/Byron/gitoxide/commit/e4dc96403894f1fe509335905679347ecdf535c7))
    - Fix verbose parsing unit tests ([`ce38ede`](https://github.com/Byron/gitoxide/commit/ce38edee5b8c7f6829fc8050ce3eeffe5943eedf))
    - (cargo-release) version 0.2.0 ([`0c39373`](https://github.com/Byron/gitoxide/commit/0c39373de5aba0acc4aaa330bf51b6abd4f50474))
    - thanks clippy ([`9e93a71`](https://github.com/Byron/gitoxide/commit/9e93a71c6664b3d40c9e76811ada81d6e1180bfe))
    - first sketch of parsing git remotes (from git :D) ([`f8ab261`](https://github.com/Byron/gitoxide/commit/f8ab261fe77a9339c121e6254a523d09fa339e40))
    - first tiny journey test for dry run of organize subcommand ([`7bbba5a`](https://github.com/Byron/gitoxide/commit/7bbba5a76d8cccb527dd6e782b830dbc4ce426bd))
    - refactor ([`64495b0`](https://github.com/Byron/gitoxide/commit/64495b0a6468679d882e5bebda45704891e7bf4e))
    - first sketch of interface for 'organize' subcommand ([`4f64d12`](https://github.com/Byron/gitoxide/commit/4f64d1277308bc2281c065236f2f14d66826d14d))
    - silence so far unknown clippy lints ([`b5f2a4b`](https://github.com/Byron/gitoxide/commit/b5f2a4b079665daa8b9e0228acc59d1eddd603b2))
    - thanks clippy ([`343ab9a`](https://github.com/Byron/gitoxide/commit/343ab9adb62da1dde495fc209c179137bbe59a10))
</details>

## v0.7.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - All crates use git-hash::Kind and its types, sometimes through git-object ([`124c171`](https://github.com/Byron/gitoxide/commit/124c171aaf546d8977e9913ff84e65383a80ee98))
    - use git-hash in git-features ([`5b307e0`](https://github.com/Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
</details>

## v0.6.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 74 calendar days.
 - 88 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`4df97ce`](https://github.com/Byron/gitoxide/commit/4df97ce6869a53a688b9af18405b284d9ff27b24))
    - (cargo-release) version 0.3.0 ([`e60dbe6`](https://github.com/Byron/gitoxide/commit/e60dbe6c21843eab44d6f05fe70927252453cb41))
    - (cargo-release) version 0.6.0 ([`27f5955`](https://github.com/Byron/gitoxide/commit/27f5955e047f35e21a86789eb46bfd89e1c99b44))
    - (cargo-release) version 0.2.0 ([`d61ad88`](https://github.com/Byron/gitoxide/commit/d61ad884021d3c0a61a14ba1df4daadfa1a0b561))
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com/Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
    - (cargo-release) version 0.5.0 ([`fc7d600`](https://github.com/Byron/gitoxide/commit/fc7d600ac2c438c8b6b91f67cb69b0ac5ec37675))
    - (cargo-release) version 0.5.0 ([`ae9c52b`](https://github.com/Byron/gitoxide/commit/ae9c52bdbe43488bb9d5b5448bf07367a1d0a24a))
    - (cargo-release) version 0.2.0 ([`a476a46`](https://github.com/Byron/gitoxide/commit/a476a46b7b933a3c2fa4aa8c285beec1777a3f2d))
    - (cargo-release) version 0.5.0 ([`c767e07`](https://github.com/Byron/gitoxide/commit/c767e07ccfc58a28e3e8ec22b590afdf0d92b9f2))
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com/Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com/Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - finish refactoring git-odb ([`ec282ae`](https://github.com/Byron/gitoxide/commit/ec282ae1a3d9f16eb9c89a44e17259112d097a41))
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com/Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - refactor ([`6b909a2`](https://github.com/Byron/gitoxide/commit/6b909a22cf981b33060cb6f1324ec3231146d159))
    - refactor ([`b511a2b`](https://github.com/Byron/gitoxide/commit/b511a2b1d9b6d55b1937ad3f4bbbb331b5cdd9a3))
    - refactor ([`8c658da`](https://github.com/Byron/gitoxide/commit/8c658da05a4649814eef9f7ab57525aff0605afc))
    - [commitgraph] Implement basic commit-graph file verification. ([`2571113`](https://github.com/Byron/gitoxide/commit/2571113fea516737acedac08d66632ead499b474))
    - [commitgraph] Add `Graph::at` constructor. ([`a783052`](https://github.com/Byron/gitoxide/commit/a783052d0cc2d3c9fa1dda3ea77286a79690d2c1))
    - [commitgraph] Stub out commit-graph-verify plumbing command. ([`aacf0f0`](https://github.com/Byron/gitoxide/commit/aacf0f05a909e5b7d9ffd5623ef9833e0465be93))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
</details>

## v0.4.1 (2020-09-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.1 ([`105c501`](https://github.com/Byron/gitoxide/commit/105c50132c8ad1f15ace0821278a11b06c81103c))
    - Finish removal of rust 2018 idioms ([`0d1699e`](https://github.com/Byron/gitoxide/commit/0d1699e0e0bc9052be0a74b1b3f3d3eeeec39e3e))
    - (cargo-release) version 0.6.0 ([`9ef184e`](https://github.com/Byron/gitoxide/commit/9ef184e35712f938fb4f9f6da7390a8777a9284e))
    - Switch to prodash 10 and safe a lot of trait bounds in the process ([`e2fb1d9`](https://github.com/Byron/gitoxide/commit/e2fb1d944b4d803a11c91f868b831d406fb5e35f))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 33 commits contributed to the release over the course of 28 calendar days.
 - 30 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`92e8b27`](https://github.com/Byron/gitoxide/commit/92e8b273654c3dedce60de244944683c7cf153e7))
    - (cargo-release) version 0.4.0 ([`2b1bca8`](https://github.com/Byron/gitoxide/commit/2b1bca83c453544972e370dc0adff57cb7590b42))
    - (cargo-release) version 0.4.0 ([`2272fa4`](https://github.com/Byron/gitoxide/commit/2272fa4bcacdaf1898e4cd8b791232fc1321227f))
    - (cargo-release) version 0.4.0 ([`0d7b60e`](https://github.com/Byron/gitoxide/commit/0d7b60e856325009431172e1df742a1cd2165575))
    - (cargo-release) version 0.5.0 ([`82b7313`](https://github.com/Byron/gitoxide/commit/82b73131b79ec3c42a712dad1c0766a72209d737))
    - thanks clippy ([`e5d80b1`](https://github.com/Byron/gitoxide/commit/e5d80b19b83dc03d49606b7ccba20ff0c39bc5d9))
    - [clone] make cloning the linux kernel work ([`e780526`](https://github.com/Byron/gitoxide/commit/e78052649c734f16f4d154edcbf54f4cc4484f5e))
    - refactor ([`dc022ce`](https://github.com/Byron/gitoxide/commit/dc022ce94505ce091e52fd64076bba01f0fe0eb0))
    - [clone] refs can now be written into a specified directory ([`fb1f048`](https://github.com/Byron/gitoxide/commit/fb1f04837be994fa5bcb9aa24f25b5f4f72e4ce0))
    - [clone] First version of writing references, but… ([`445be27`](https://github.com/Byron/gitoxide/commit/445be27cf81663ba4fe941c00262448444efbac2))
    - [clone] better JSON output for pack-receive ([`bc6b8e8`](https://github.com/Byron/gitoxide/commit/bc6b8e86f258835b6da60ea7e749fe01243a4010))
    - [clone] initial implementation of Json format for pack-receive ([`9090ac6`](https://github.com/Byron/gitoxide/commit/9090ac6c6acdb5e050c597a279a963b48c08871a))
    - [clone] nicer pack-receive output for humans ([`09c6c57`](https://github.com/Byron/gitoxide/commit/09c6c576ddb4c791b1b5f9b1812485e73a080f93))
    - [clone] Don't hide nested pack-decoding information ([`4d4be97`](https://github.com/Byron/gitoxide/commit/4d4be975707d017a67a0b2c081a07c4092b2801d))
    - [clone] When unpacking peeled refs, use the object that refers to the tag… ([`fe8bb39`](https://github.com/Byron/gitoxide/commit/fe8bb3985bd5529a36c71fa170ca48df91060491))
    - [clone] minor refactor; it's definitely the read() that doesn't work… ([`406829b`](https://github.com/Byron/gitoxide/commit/406829b951164673c0b8152d1e9de76f1318df0a))
    - [clone] This actually works: first MVP of retrieving packs via clone ([`c06d819`](https://github.com/Byron/gitoxide/commit/c06d8194173f9ec468ddd0faf72dd6d8dbf7d35d))
    - [clone] First step towards implementing a working pack receiving… ([`264ec82`](https://github.com/Byron/gitoxide/commit/264ec821ca92a08d1756222abab11ffebb6dc0ff))
    - [clone] Support for reading multi-step negoritaions, but… ([`507d342`](https://github.com/Byron/gitoxide/commit/507d342dfe2a714a4dd0bc100d96ed9e64a58243))
    - [clone] support for progress that can handle writing pack files ([`46e0055`](https://github.com/Byron/gitoxide/commit/46e0055eab47e402807b15c63b6a4577f5c0b7bb))
    - [clone] Actually pass pack file to the delegate ([`94c5e62`](https://github.com/Byron/gitoxide/commit/94c5e62b274b0fc39f64ee5b04273db5ead4a470))
    - refactor ([`61e9812`](https://github.com/Byron/gitoxide/commit/61e98128ddd85cde1a352b70f83870fdea0c6bac))
    - [ref-ls] first step towards supporting negotiation ([`27b6d2d`](https://github.com/Byron/gitoxide/commit/27b6d2d24a92c1ffc1579a116a044cece50d9d20))
    - [ref-ls] usable JSON output ([`735ae50`](https://github.com/Byron/gitoxide/commit/735ae50c1fdf1a7c403782f40b5234ea881da7b1))
    - [ref-ls] Fix progress display ([`2fcb557`](https://github.com/Byron/gitoxide/commit/2fcb557dce941eb94ca60f46ecee86b94e029db7))
    - [ref-ls] Make things compile ([`b6506a4`](https://github.com/Byron/gitoxide/commit/b6506a46ef59d8e25b245fa8caac5b4de4fdaa3d))
    - [ref-ls] And it even doesn't work if it is the very same transport ([`4ba50fe`](https://github.com/Byron/gitoxide/commit/4ba50fe06f7423c31f4cd78079d51ef3ffd51920))
    - [ref-ls] first actual call of ls-remote, but… ([`5fc4330`](https://github.com/Byron/gitoxide/commit/5fc4330eca42a0a3ba6c14fe8c27aeda16e440ec))
    - [ref-ls] Frame for remote-ref-ls command in gitoxide-core ([`161e7df`](https://github.com/Byron/gitoxide/commit/161e7df34a53db40551879c6d2319ee775dfd551))
    - bump git-features to 0.4 to allow publishes after breaking changes ([`9d6b879`](https://github.com/Byron/gitoxide/commit/9d6b8790e2edd7fa01b3239adff86a7cd2393f10))
    - [clone] first sketch of transport layer's connection logic ([`f10cee5`](https://github.com/Byron/gitoxide/commit/f10cee5638a220fff629af274baebbcc0f4f0f61))
    - [clone] link up lean plumbing command with gitoxide-core: pack-receive ([`5ea49c8`](https://github.com/Byron/gitoxide/commit/5ea49c8aa0d449bed98ce0147ad222ff25c27c32))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 67 commits contributed to the release over the course of 30 calendar days.
 - 31 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com/Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - first step towards parallelizing file hashes and traversal! ([`9573836`](https://github.com/Byron/gitoxide/commit/95738369e0d3accf7f6239c8cd966a7f5c36825a))
    - update to quick-error 2.0 ([`4b1b784`](https://github.com/Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
    - better progress for Sha1 of pack and index ([`310a59e`](https://github.com/Byron/gitoxide/commit/310a59ee99ce78a4f14326c0058ea0c543a1d24c))
    - first successful test of moving the streaming iterator into its own thread ([`c9fcb68`](https://github.com/Byron/gitoxide/commit/c9fcb68c644c96a15cb9956a754bec7b65bb5fbd))
    - unify used ranges for line renderer amond pretty and lean interface ([`f59f66e`](https://github.com/Byron/gitoxide/commit/f59f66e189732f567414f68c7463364e510f41c4))
    - Add convenience method to get a new bundle for the index/data just written ([`a6d74ad`](https://github.com/Byron/gitoxide/commit/a6d74ad7b65cdc293c8504dae73ea1c717e5bfca))
    - support for JSON format output ([`1931575`](https://github.com/Byron/gitoxide/commit/19315750f4f409e3f105c3c4054c4afbef91daad))
    - first pieces of the index-from-pack journey tests ([`181d69c`](https://github.com/Byron/gitoxide/commit/181d69c1da46a931c513cbd7d8bca7b2fa53351c))
    - more flexible error types for processors - anything goes ([`be3a947`](https://github.com/Byron/gitoxide/commit/be3a947ba6197319fea0b38e48008850cc971bf6))
    - refactor ([`c7dd581`](https://github.com/Byron/gitoxide/commit/c7dd581348a05146d7a79f7622bf30a08d34f474))
    - interrupt support for pretty plumbing ([`bca7ce2`](https://github.com/Byron/gitoxide/commit/bca7ce2e668a4be2600d2d04d00f46b21c82eee2))
    - count object types as well ([`e04a8d1`](https://github.com/Byron/gitoxide/commit/e04a8d16fda3712663d8d9220f3a017e668b6283))
    - refactor ([`b77d148`](https://github.com/Byron/gitoxide/commit/b77d148ed1c5aec31cb0493b4f1e0f2d82d7e641))
    - remove memory mode entirely (and some complexity with it) ([`8812e91`](https://github.com/Byron/gitoxide/commit/8812e916a21983868a37c4aade10f79a1dc9b926))
    - turns out you never want to keep deltas in memory ([`657aa2c`](https://github.com/Byron/gitoxide/commit/657aa2c38673cf10174f42bcb97039ac37b2926e))
    - Remove support for keeping compressed memory to reduce the index size ([`1e2ec7e`](https://github.com/Byron/gitoxide/commit/1e2ec7e9d0ef2f2a4908860672080e411e945bff))
    - Use call to produce the resolver, allowing to delay opening a file mapping… ([`dd30e8d`](https://github.com/Byron/gitoxide/commit/dd30e8d3c8b6754bd90e2777ec0153e158d4a708))
    - minor fixes after first local tests - it's up to twice as fast!! ([`43c7fd1`](https://github.com/Byron/gitoxide/commit/43c7fd1f81b9b4c938f99c0bf1deabdf121226b9))
    - quick and dirty impl of lean command-line for index-from-pack ([`9660bbf`](https://github.com/Byron/gitoxide/commit/9660bbffd8ace621178b067e22d227ef8c50ba84))
    - quick and dirty impl of gitoxide layer for bundle writing, aka index-pack ([`e78386b`](https://github.com/Byron/gitoxide/commit/e78386b824010c5ca8efca87604c339d40b545ae))
    - first sketch of gitoxide index::from_pack(…) ([`da0eace`](https://github.com/Byron/gitoxide/commit/da0eacea838a0fcdf09e052334f944269a153f42))
    - refactor; better tests ([`12d14bf`](https://github.com/Byron/gitoxide/commit/12d14bfe2aa089723a395287c5100aad6e838935))
    - update tasks ([`45c3520`](https://github.com/Byron/gitoxide/commit/45c352009092dbfd80bcb3e367d848d5b10737d4))
    - it looks like something is wrong with the object stream implementation ([`d187b5a`](https://github.com/Byron/gitoxide/commit/d187b5a769b62ec706c1265e0db8403327d8e92d))
    - Loose object verifycation - but it doesn't seem to work as expected ([`9dd5676`](https://github.com/Byron/gitoxide/commit/9dd56761ae75eac691449cd86a1be04c11c0fecb))
    - prepare full 'verify' implementation ([`ee45c7f`](https://github.com/Byron/gitoxide/commit/ee45c7f47b95fc406cc5922a322c8fd6c0f52775))
    - refactor ([`0a33b24`](https://github.com/Byron/gitoxide/commit/0a33b24f5b61ccdf1358f1e9adcf0f6fd4099c1c))
    - Allow sink-compress configuration; choose best algorithm ([`29b9c23`](https://github.com/Byron/gitoxide/commit/29b9c230e35ba9b4334797b63ab9fa88c2fe59d0))
    - Always compress values when using a sink when exploding packs ([`70562fa`](https://github.com/Byron/gitoxide/commit/70562fa123faf51bd72a4aedb12acb0d3247e4e2))
    - Most tests and clearer error message if object directory is inaccessible ([`1d8f597`](https://github.com/Byron/gitoxide/commit/1d8f5974a5c754750f46697370cb2551f6660666))
    - Nice error message on failure ([`adbc82c`](https://github.com/Byron/gitoxide/commit/adbc82c31450681fcb38233eeb8095efc5e52a18))
    - inform about deleted files using progress ([`a3ee516`](https://github.com/Byron/gitoxide/commit/a3ee5160093c9326006fcedbf1f507d8978a97c2))
    - Don't uncondionally delete packs/indices on explode :D ([`1979715`](https://github.com/Byron/gitoxide/commit/19797156bafbacbaf0a53d01d72bbe86881aea9b))
    - The first 'explode' implementation… ([`0d31ad1`](https://github.com/Byron/gitoxide/commit/0d31ad1b61997fa0d0692c5919fb8032ffaaa35b))
    - Get all pieces ready for action ([`1805d64`](https://github.com/Byron/gitoxide/commit/1805d64b9222d6a05a8718f04b29b789c1f42fea))
    - Pass option for safety checks down to explode(…) ([`0bcb790`](https://github.com/Byron/gitoxide/commit/0bcb790dc8c35097916876afbb68bbfcc894c369))
    - Restore original verification functionality ([`0e3c1b9`](https://github.com/Byron/gitoxide/commit/0e3c1b9bb9841ae4bb0ef1df2e72e950f7a7fd33))
    - nearly there! Interesting that anyhow errors must be sync! ([`eaee77e`](https://github.com/Byron/gitoxide/commit/eaee77ea4ce10f5c85b42a33452eef996adac3bf))
    - refactor ([`bae7781`](https://github.com/Byron/gitoxide/commit/bae7781ab549f0daa73980a29d18d64320601470))
    - refactor ([`f66b116`](https://github.com/Byron/gitoxide/commit/f66b116ddfbee62c3e20a4c5e7cd878fbf064195))
    - basic tests and CLI args for explode pack ([`f932256`](https://github.com/Byron/gitoxide/commit/f932256a62d6fc5d5558446de079fb666ddc27da))
    - refactor ([`d3c00c8`](https://github.com/Byron/gitoxide/commit/d3c00c841ee1aeda6bb0534fe365db13c31f8d3c))
    - (cargo-release) version 0.2.0 ([`76fe0ab`](https://github.com/Byron/gitoxide/commit/76fe0ab5f0b58504a5ea5adb74b349b9d588e51e))
    - (cargo-release) version 0.2.0 ([`0bb8314`](https://github.com/Byron/gitoxide/commit/0bb831480d8657e1bb29ee7009aeac673471403e))
    - Run clippy first; pacify clippy ([`0a5b883`](https://github.com/Byron/gitoxide/commit/0a5b883c22f2df8a6d51f75c5e09bdfdf276fee4))
    - use faster algorithm by default ([`bb45c3d`](https://github.com/Byron/gitoxide/commit/bb45c3d8a2aabf87231981000240f0444abf6fc4))
    - refactor; enable testing of reverse-delta lookup ([`512daf9`](https://github.com/Byron/gitoxide/commit/512daf94038f675353271c930694e0577ac746b4))
    - Fix clippy ([`ec40e09`](https://github.com/Byron/gitoxide/commit/ec40e093d72f93d86168f39ebaca5b122ca0bec3))
    - refactor ([`fdfab40`](https://github.com/Byron/gitoxide/commit/fdfab408c38087c5afcdd028e988089c56311baf))
    - Easy access to sorted offsets in pack index files ([`d93540f`](https://github.com/Byron/gitoxide/commit/d93540fe2a6d4bb70248e82d039d6a2665354ef3))
    - refactor ([`cb8d561`](https://github.com/Byron/gitoxide/commit/cb8d56101bdc4cd7e3fa95ac79f82c1cda99871c))
    - Change course and do pack streaming first ([`bcb275e`](https://github.com/Byron/gitoxide/commit/bcb275e91cfd6f0a71b3cb59a2b706b60608a594))
    - Switch to latest quick-error ([`9760856`](https://github.com/Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - Fully implement --encode and --re-encode flags ([`a7cfac8`](https://github.com/Byron/gitoxide/commit/a7cfac83ddd859d9c2c25e457c0d7043738792dc))
    - prepare for re-encoding each pack object ([`afae684`](https://github.com/Byron/gitoxide/commit/afae684c72e5dc4b718976056dd5d34ed61de72a))
    - move git_object::Id into git_object::owned::Id - much better already! ([`50c7136`](https://github.com/Byron/gitoxide/commit/50c71368a69f57b0a43061df105685e992ed384a))
    - fix naming change, which was introduced accidentally ([`fbb9f98`](https://github.com/Byron/gitoxide/commit/fbb9f98508ec722e192466e28ded47aef2fb78b3))
    - refactor ([`34e85f2`](https://github.com/Byron/gitoxide/commit/34e85f2242b12ec1560b8e50bc9ab447cd1805fc))
    - refactor ([`2888f1b`](https://github.com/Byron/gitoxide/commit/2888f1b10a2baf40155544e667ddd461f3ddc938))
    - refactor ([`dcacd3b`](https://github.com/Byron/gitoxide/commit/dcacd3b06d7a4532c600dfdf62e03561e8ed55ef))
    - refactor ([`b113da9`](https://github.com/Byron/gitoxide/commit/b113da945715f9611eb0fb79925d1239eaf1569c))
    - refactor ([`bed5dc8`](https://github.com/Byron/gitoxide/commit/bed5dc80c5b307c6d35f7b4405693dce1f7f6d71))
    - refactor ([`8b416d4`](https://github.com/Byron/gitoxide/commit/8b416d4b8417c04ea5d3527a88190d867dc8b7c2))
    - Respect thread limit in 'in_parallel' ([`babfd84`](https://github.com/Byron/gitoxide/commit/babfd84cba77ef7a0f541ba921b31ebd3f3c50e3))
    - pass threadlimit down from CLIs ([`f98c5b1`](https://github.com/Byron/gitoxide/commit/f98c5b160db80a7cac530e18b9256562c25be47f))
    - add new Context argument to support more configuration options ([`7c5d8b8`](https://github.com/Byron/gitoxide/commit/7c5d8b8bb318e59a59ad74ad767a1525e2833632))
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 19 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable ([`5688a34`](https://github.com/Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - Add metadata to allow docs.rs build all featueres ([`10f9386`](https://github.com/Byron/gitoxide/commit/10f9386a12decc1f13999aee72be484c8f6d48ce))
    - support for json in pretty-plumbing and gitoxide (on demand) ([`b3780f8`](https://github.com/Byron/gitoxide/commit/b3780f87438d34b372c48b7385199f7ea22b3965))
    - git-odb with serde support ([`0da930c`](https://github.com/Byron/gitoxide/commit/0da930cf23f215cc1e2bda8f7340a5d69370735a))
    - pass serde1 through from gitoxide ([`1991b9f`](https://github.com/Byron/gitoxide/commit/1991b9ffef1a2b9a402d080d0a31e0857c434bc4))
    - don't print 'OK' at the end of verify-pack ([`4956ef2`](https://github.com/Byron/gitoxide/commit/4956ef23783104d64c35983934c69db918f3027a))
    - \#[forbid(unsafe)] for all crates ([`afda803`](https://github.com/Byron/gitoxide/commit/afda8039259b7a30cfed5dbcdd9caf4773b4c234))
    - Allow for more screen space when formatting ([`6794300`](https://github.com/Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - disable LRU cache if we have to get statistics ([`befba3b`](https://github.com/Byron/gitoxide/commit/befba3b769195fb592d714afe12194a61ae4a330))
    - wonderful statistics on compression efficiency! ([`1bb09c5`](https://github.com/Byron/gitoxide/commit/1bb09c509dae4e493ab05022bbf51c0b1786d479))
    - pretty-print objects per delta chain length ([`66553b1`](https://github.com/Byron/gitoxide/commit/66553b1c544a25c9703641ab6ea1a4a2a08b945a))
    - count objects per chain level ([`209d53f`](https://github.com/Byron/gitoxide/commit/209d53f531ec9bcffbb04ba060447bee59ad26f6))
    - Pretty-printing of some statistics ([`125b565`](https://github.com/Byron/gitoxide/commit/125b565f0fb4085c615fdf136f35a2285d69966a))
    - fix pretty build ([`6adf615`](https://github.com/Byron/gitoxide/commit/6adf615ed7d6c488c25589940fc0a55bf0fb3d5c))
    - pass average stats through to the top level ([`5b4979c`](https://github.com/Byron/gitoxide/commit/5b4979c1dfeb9a29974dd4e6529ae5da074d0b1a))
    - first very basic progress implementation ([`b820717`](https://github.com/Byron/gitoxide/commit/b8207177daee8a9ffa23c7c052cf9ca651b15804))
    - Pass progress everywhere, for now just to discard it ([`da3ae1c`](https://github.com/Byron/gitoxide/commit/da3ae1c82cd726b8fae9b8d26069719930e9ba99))
    - Control which hashing crates to use from the top-level as well. ([`dfe9b20`](https://github.com/Byron/gitoxide/commit/dfe9b203b2e877a7e345b4f2942bf5a1582ab43e))
    - Use git-features to toggle 'parallel' mode from the 'gitoxide' level ([`d944fbf`](https://github.com/Byron/gitoxide/commit/d944fbf181acc5fb83a841613174702af1e074d6))
    - first working version of actually parallel `in_parallel` ([`145ee39`](https://github.com/Byron/gitoxide/commit/145ee399e2c057aec3330e26bafb7910ca7dc56d))
    - Support for verifying pack files and index files ([`b09b4e1`](https://github.com/Byron/gitoxide/commit/b09b4e1f35c3802dfd3418bda42b96828acd9ec8))
    - cleanup - don't build and run tests while there is nothing to test ([`4a153da`](https://github.com/Byron/gitoxide/commit/4a153da0d60a30615fc402cfecb977f0d771594a))
    - First basic index file verification ([`994700f`](https://github.com/Byron/gitoxide/commit/994700f96b058a0910e734bdecced44bd0a7ea5d))
    - reorganize crates to make 'gitoxide' the CLI, and 'gitoxide-core' the library ([`0ac9c5a`](https://github.com/Byron/gitoxide/commit/0ac9c5af0cbb562d3cb48a661736afd98dd1a940))
    - rename grit to 'gitoxide', CLI name is 'gio' ([`9d6007f`](https://github.com/Byron/gitoxide/commit/9d6007f83b3b018d736d58aa0722b83b9cffb228))
</details>

