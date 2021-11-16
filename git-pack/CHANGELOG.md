# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.14.0 (2021-11-16)

<csr-id-e8b091943f0c9a26317da0003f7fcdf5a56ef21a/>

An important bugfix to prevent assertion failures when writing thin packs.

### Bug Fixes

 - <csr-id-20b3994206aa5bc5e35cbbc9c8f8f99187077f79/> Adjust size-hints of resolving entries iterator and use the upper bound in delta tree.
   
   The delta-tree is a data structure that actually heavily relies on
   favorable allocation and a known amount of objects in order to
   provide front and back buffers. However, this is an implementation
   detail and they don't have to stay consistent at all especially
   after growing the buffer by pushing to it.
   
   Interestingly, the VecDeque internally over-allocates as well which
   definitely helps the example of `as_mut_slices()`, otherwise
   it could also suffer from the assertions that trigger here.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 18 calendar days.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#247](https://github.com/Byron/gitoxide/issues/247), [#254](https://github.com/Byron/gitoxide/issues/254)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#247](https://github.com/Byron/gitoxide/issues/247)**
    - Rename gix->ein and gixp->gix ([`e8b0919`](https://github.com/Byron/gitoxide/commit/e8b091943f0c9a26317da0003f7fcdf5a56ef21a))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
    - minor refactor ([`227c8b1`](https://github.com/Byron/gitoxide/commit/227c8b1859a6cbf96d48fd8564e575ef7e201db1))
    - Adjust size-hints of resolving entries iterator and use the upper bound in delta tree ([`20b3994`](https://github.com/Byron/gitoxide/commit/20b3994206aa5bc5e35cbbc9c8f8f99187077f79))
 * **Uncategorized**
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Adjust changelogs prior to git-pack release ([`ac8015d`](https://github.com/Byron/gitoxide/commit/ac8015de710142c2bedd0e4188e872e0cf1ceccc))
    - Merge branch 'header-field-multi-improve' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-header-field-multi-improve ([`d88e377`](https://github.com/Byron/gitoxide/commit/d88e377c21e566bf86c274d5e87eff06100698b9))
</details>

## v0.13.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
    - stabilize changelogs ([`920e832`](https://github.com/Byron/gitoxide/commit/920e83219911df1c440d3fe42fd5ec3a295b0bb8))
    - Update changelogs prior to release ([`b3e2252`](https://github.com/Byron/gitoxide/commit/b3e2252f7461a003d9a4612da60ba931dd8c0bef))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.12.0 (2021-10-15)

<csr-id-d8fe8141e80a9e9a433b5e1a072b850325c806c8/>
<csr-id-faf6f813927720c5adf62102f9ce46606ff2617c/>
<csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/>

This release contains bugfixes and features, but is considered breaking as `git-traverse`
signalled a breaking change which is one of our dependencies.

### New Features

 - <csr-id-d6c44e6ab8f436020d4fb235e423b018fd1e7a9f/> dynamically sized full-object speeds up diff-based object counting
   which is what happens when counting objects for fetches where only changed objects should be sent.
 - <csr-id-50cf610e8939812c3d2268c48835e2dac67d0c31/> `cache::Object` trait for caching and retrieving whole objects
 - <csr-id-60c9fad8002b4e3f6b9607bba6361871752f4d3d/> control pack and object cache size in megabytes
 - <csr-id-5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc/> object cache size is configurable

### Bug Fixes

 - <csr-id-d8fe8141e80a9e9a433b5e1a072b850325c806c8/> don't put more objects into the pack cache than needed.
   
   Previously when accessing a packed object, it would store the base
   object into the pack cache (if it wasn't retrieved from there)
   which is great if that operation is free.
   
   Since it isn't, it's better not to stress the cache with puts
   and trash more objects than necessary.
   
   Now only the last decompressed object will be put into the LRU cache.
 - <csr-id-faf6f813927720c5adf62102f9ce46606ff2617c/> don't include submodules in count,
   which avoids dealing with missing objects entirely. Those ominous missing objects where just git submodules after all.
   
   It's still a good idea to handle these gracefully though, git itself
   seems to ignore them, too, and so do we at least for now.

### Performance

 - <csr-id-f9232acf8e52f8cd95520d122469e136eb07b39f/> ObjectID specific hashers, using the fact that object ids are hashes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 34 commits contributed to the release over the course of 32 calendar days.
 - 10 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#164](https://github.com/Byron/gitoxide/issues/164), [#198](https://github.com/Byron/gitoxide/issues/198), [#67](https://github.com/Byron/gitoxide/issues/67)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#164](https://github.com/Byron/gitoxide/issues/164)**
    - Avoid duplicate module paths in 'tree' and 'commit' ([`2f2d856`](https://github.com/Byron/gitoxide/commit/2f2d856efe733d3cf81110c0e0607d2e7c40d968))
 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Fix stop-release-for-changelog logic and fix all affected changelogs ([`52b38bc`](https://github.com/Byron/gitoxide/commit/52b38bc4856be5ba8b5372a3dd20f5d06504e7ed))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - don't put more objects into the pack cache than needed ([`d8fe814`](https://github.com/Byron/gitoxide/commit/d8fe8141e80a9e9a433b5e1a072b850325c806c8))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`2f43a54`](https://github.com/Byron/gitoxide/commit/2f43a54298e7ecfff2334627df149fe0882b5d1d))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - ObjectID specific hashers, using the fact that object ids are hashes ([`f9232ac`](https://github.com/Byron/gitoxide/commit/f9232acf8e52f8cd95520d122469e136eb07b39f))
    - Use a custom hasher for 'seen' objects hashset… ([`70179e2`](https://github.com/Byron/gitoxide/commit/70179e2cf8d15ba4e1cf8e94a9915bf5b02cf755))
    - don't include submodules in count… ([`faf6f81`](https://github.com/Byron/gitoxide/commit/faf6f813927720c5adf62102f9ce46606ff2617c))
    - control pack and object cache size in megabytes ([`60c9fad`](https://github.com/Byron/gitoxide/commit/60c9fad8002b4e3f6b9607bba6361871752f4d3d))
    - Use 'cache::Object' trait where it matters ([`71c628d`](https://github.com/Byron/gitoxide/commit/71c628d46088ab455b54eb2330d24dcff96c911d))
    - split data::output::count::objects into files ([`8fe4612`](https://github.com/Byron/gitoxide/commit/8fe461281842b58aa11437445637c6e587bedd63))
    - cache::Object trait for caching and retrieving whole objects ([`50cf610`](https://github.com/Byron/gitoxide/commit/50cf610e8939812c3d2268c48835e2dac67d0c31))
    - object cache size is configurable ([`5a8c2da`](https://github.com/Byron/gitoxide/commit/5a8c2da6cb1e2accf7cfdccc16bc3a1d0b2a7dbc))
    - dynamically sized full-object speeds up diff-based object counting… ([`d6c44e6`](https://github.com/Byron/gitoxide/commit/d6c44e6ab8f436020d4fb235e423b018fd1e7a9f))
    - Count ref-deltas in thin packs as well ([`80c6994`](https://github.com/Byron/gitoxide/commit/80c6994149d19917c25e36e1bdf0dc8c9678365e))
    - Add '--thin' flag to pack-create and pass it on ([`2664d73`](https://github.com/Byron/gitoxide/commit/2664d73f531a4b1f4bc784c1fe3a991711c86475))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.11.0 (2021-09-08)

- manual bump for safety as its dependencies have breaking changes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.11.0 ([`5ae6ff5`](https://github.com/Byron/gitoxide/commit/5ae6ff52cd2cd1ccd1e26bb987c154eb19603696))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com/Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
    - [repository #164] generic write_object() ([`c569f83`](https://github.com/Byron/gitoxide/commit/c569f83363489dde03c8b9cd01e75d35f5e04dbc))
</details>

## v0.10.0 (2021-09-07)

- **renames**
   - `data::Object::into_commit_iter()` -> `data::Object::try_into_commit_iter()`
   - `data::Object::into_tree_iter()` -> `data::Object::try_into_tree_iter()`
   - `data::Object::into_tag_iter()` -> `data::Object::try_into_tag_iter()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.10.0 ([`e5e3c80`](https://github.com/Byron/gitoxide/commit/e5e3c8024e1c2e5e90cee83abbdae41d58eee156))
    - [repository #190] first shot at ancestor iteration… ([`85f1a48`](https://github.com/Byron/gitoxide/commit/85f1a48ea39f3b224e8d0ba3728dd75e03a6edc3))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #185] refactor ([`7604935`](https://github.com/Byron/gitoxide/commit/7604935b12eacb26a98bedc5f77636b5583629a5))
</details>

## v0.9.0 (2021-08-27)

- **renames / moves / visibility**
   - `find::Find`  and `find::FindExt` only in `Find` and `FindExt` (not in `find` anymore)
   - `data::output::count::Count` -> `data::output::Count`
   - `data::output::entry::Entry` -> `data::output::Entry`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find()-> `Find::try_find()`
   - `bundle::Bundle` -> `Bundle`
   - `bundle::Error` -> `bundle::init::Error`
   - `pub tree::` -> `pub(crate) cache::delta::`
   - `data::object::Object` -> `data::Object`
   - `data::entry::Entry` -> `data::Entry`

* **new methods**
   - `Find::find_tag_iter()`
 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 56 commits contributed to the release over the course of 5 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] adjust various changelogs ([`081faf5`](https://github.com/Byron/gitoxide/commit/081faf5c3a21b34b7068b44d8206fb5770c392f5))
    - [pack #179] refactor ([`76e66d1`](https://github.com/Byron/gitoxide/commit/76e66d1b9d24bb25a9f681d9612e52c8ccd60e2c))
    - [pack #179] move Tree traversal cache private ([`34e45d7`](https://github.com/Byron/gitoxide/commit/34e45d745cb8756831c56dc441695a25cd0069a9))
    - [pack #179] refactor ([`5a3677d`](https://github.com/Byron/gitoxide/commit/5a3677dd3f3dcab26a3d9270b6184fd0fe18c54e))
    - [pack #179] refactor bundle ([`420dca2`](https://github.com/Byron/gitoxide/commit/420dca29bccca6e7d759880d8342f23b33eead0d))
    - [pack #179] fix docs ([`7ad7a44`](https://github.com/Byron/gitoxide/commit/7ad7a4428d0e38f2ff776f7efab6996505d2bba2))
    - [pack #179] refactor ([`ab6554b`](https://github.com/Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [pack #179] refactor ([`620d8a5`](https://github.com/Byron/gitoxide/commit/620d8a54db5cd8367ec85c8b837cab710c509e3e))
    - [pack #179] add changelog ([`2102569`](https://github.com/Byron/gitoxide/commit/210256932a338038adb55c5475d8f90560aa4c12))
    - Bump git-traverse v0.8.0 ([`54f3541`](https://github.com/Byron/gitoxide/commit/54f3541f1448a8afa044d3958fa1be5b074e4445))
    - Bump git-diff v0.9.0 ([`2e2e798`](https://github.com/Byron/gitoxide/commit/2e2e7983178b3af7e5684995de68ed5d020927ec))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com/Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177] dissolve 'immutable' module ([`70e11c2`](https://github.com/Byron/gitoxide/commit/70e11c21b0637cd250f54381d5490e9976880ad9))
    - [object #177]  commit::RefIter -> CommitRefIter ([`e603306`](https://github.com/Byron/gitoxide/commit/e603306e81f392af97aa5afd232653de56bf3ce9))
    - [object #177] migrate immutable::commit into crate::commit ([`45d3934`](https://github.com/Byron/gitoxide/commit/45d393438eac2c7ecd47670922437dd0de4cd69b))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com/Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [object #177] into_mutable() -> into_owned() ([`7e701ce`](https://github.com/Byron/gitoxide/commit/7e701ce49efe5d40327770a988aae88692d88219))
    - [object #177] fix docs ([`25d8e7b`](https://github.com/Byron/gitoxide/commit/25d8e7b1862bd05489359b162a32c6ad45ecdf9a))
    - [object #177] move mutable objects to crate::* ([`c551c02`](https://github.com/Byron/gitoxide/commit/c551c0236c64f3237cb9be7f35159f753d4b871f))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] fix docs ([`07be661`](https://github.com/Byron/gitoxide/commit/07be6611d1742633815566443f71eef8b85ad5c0))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com/Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - Merge branch 'git-ref-refactor' ([`5dbf753`](https://github.com/Byron/gitoxide/commit/5dbf753ce2035ffd07e4bce7ceb3bcd4e309c16e))
    - [pack #172] A note about empty packs in Bundle writer ([`09a777f`](https://github.com/Byron/gitoxide/commit/09a777f1da5e792c5eb4c8ff9e83504ad8d19c5c))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com/Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [actor #173] fix docs ([`2d7956a`](https://github.com/Byron/gitoxide/commit/2d7956a22511d73b767e443dac21b60e93f286dd))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - cleanup imports ([`e669303`](https://github.com/Byron/gitoxide/commit/e6693032f1391416fd704c21617051ddfb862a3a))
    - Merge branch 'Byron:main' into main ([`dc58eca`](https://github.com/Byron/gitoxide/commit/dc58eca510e5a067acdeaad4b595a34b4598a0cd))
    - Allow creation of empty indices ([`d122fc7`](https://github.com/Byron/gitoxide/commit/d122fc79cc9b9a52a2817bdd46d3215c10e61129))
    - [pack #170] there can only be one ([`dce4f97`](https://github.com/Byron/gitoxide/commit/dce4f97a84aa6a73e31e7397501cfce27241c5b8))
    - [pack #170] clru allows for free lists, reducing allocation pressure... ([`4d820d2`](https://github.com/Byron/gitoxide/commit/4d820d2f94dc3afc062bbd25e969c87410212c3a))
    - [pack #170] basic progress for resolution ([`ada0b96`](https://github.com/Byron/gitoxide/commit/ada0b96e3707c06d7d6f7e4002907e12b45f7419))
    - [pack #170] Basic entry resolution without progress ([`7461f31`](https://github.com/Byron/gitoxide/commit/7461f31f03d67ecc9fdf398adf3cb6d4eb365412))
    - [pack #170] first step towards resolving in multi-threaded mode… ([`f3c21f9`](https://github.com/Byron/gitoxide/commit/f3c21f99594ab4080b8aa1ffed9ea8a33e18fabd))
    - [pack #170] Don't double-lookup trees during traversal… ([`7b06829`](https://github.com/Byron/gitoxide/commit/7b068296fe5ca10af212d8fe2662940188b7359c))
    - Revert "[pack #67] Don't pre-fetch packed objects during counting" ([`811bb54`](https://github.com/Byron/gitoxide/commit/811bb54991636f7e517087b62cf0c8c8cc2ad9e6))
    - [pack #67] Don't pre-fetch packed objects during counting ([`d08b673`](https://github.com/Byron/gitoxide/commit/d08b6739d8e9294b795aba75e9c7f9f20645af2b))
    - Release git-pack v0.9.0 ([`7fbc961`](https://github.com/Byron/gitoxide/commit/7fbc9617da97d4ba4bb3784f41d4163c0839c03c))
    - [pack #67] refactor ([`14717f6`](https://github.com/Byron/gitoxide/commit/14717f6132672a5d271832a68de0b323b73abb2a))
    - [pack #67] Add cache debugging capabilities to git-features ([`8776c98`](https://github.com/Byron/gitoxide/commit/8776c9834ac4622b3057f5db464a9817ed9acdb0))
    - [pack #167] Use custom uluru version to avoid a lot of allocations… ([`7bd3671`](https://github.com/Byron/gitoxide/commit/7bd3671ad949d62f84147ef7ff3fde59937fee54))
    - [pack #164] fix docs ([`08ee674`](https://github.com/Byron/gitoxide/commit/08ee674c55cef6ab76520de2f836b246c907888c))
    - Revert "[pack #167] Use custom uluru version to avoid a lot of allocations…" ([`4c2ea21`](https://github.com/Byron/gitoxide/commit/4c2ea212bbffb0ba3c21ba388dfc79cc7a1c4734))
    - [pack #167] Use custom uluru version to avoid a lot of allocations… ([`8d49976`](https://github.com/Byron/gitoxide/commit/8d499762b74c08437d901bb98806e0a1fc6f93bb))
    - [pack #167] a single-threaded special case for counting… ([`65e29de`](https://github.com/Byron/gitoxide/commit/65e29de45a92c82cebd832634ab194db19a1b590))
    - [pack #167] generalize over immutable insertions… ([`169f000`](https://github.com/Byron/gitoxide/commit/169f000087aab18f0257fb0c61dc3b3901e97505))
    - [pack #167] refactor ([`6bf0f7e`](https://github.com/Byron/gitoxide/commit/6bf0f7e86312b2a4d262c80979c61c94519bd4b0))
    - [pack #167] progress is handled by reducer… ([`a22f8e1`](https://github.com/Byron/gitoxide/commit/a22f8e171e705bc42fcf290789e8e05423bd72d1))
    - [pack #167] Error handling for object input ([`0aac40c`](https://github.com/Byron/gitoxide/commit/0aac40c88a5c26f7c295db8433b510b168f15ca3))
    - thanks clippy ([`d689599`](https://github.com/Byron/gitoxide/commit/d689599d1b819c18a3be60075170dbe00462e216))
    - [pack #167] remove iterator based count objects impl… ([`7ec2f2b`](https://github.com/Byron/gitoxide/commit/7ec2f2b40e83aaa218360a8b5989792cd67de2ed))
    - [pack] A non-iterator version of parallel object counting… ([`04fe855`](https://github.com/Byron/gitoxide/commit/04fe855a37577d3da5bbd619807b44e449947893))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.8.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.2 ([`39a3f71`](https://github.com/Byron/gitoxide/commit/39a3f71ba5997ac26d9994cdc7c2145af3220f64))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.8.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-pack v0.8.1 ([`045eb09`](https://github.com/Byron/gitoxide/commit/045eb094691324a398120f6039bbfa34b4fda1af))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
</details>

## v0.8.0 (2021-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com/Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com/Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com/Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com/Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
</details>

## v0.6.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`d704bca`](https://github.com/Byron/gitoxide/commit/d704bca7de0a6591f35345c842d6418b36ecd206))
    - (cargo-release) version 0.6.0 ([`4b71e15`](https://github.com/Byron/gitoxide/commit/4b71e15c3ba4a17ff2da5a5ef79986a2832fa3f2))
    - (cargo-release) version 0.5.0 ([`e21142b`](https://github.com/Byron/gitoxide/commit/e21142ba1a113b2afc4725d4d4225dff519c513a))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com/Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
</details>

## v0.5.0 (2021-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`c2f94a5`](https://github.com/Byron/gitoxide/commit/c2f94a51bce287be301090450cb00cde57e92f76))
    - (cargo-release) version 0.4.0 ([`d69d0ac`](https://github.com/Byron/gitoxide/commit/d69d0ac21989243fdafa514fa41579fd51bc2558))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.5.0 ([`1687e59`](https://github.com/Byron/gitoxide/commit/1687e599be98d97925fbab594f31cf5558e9d2b1))
    - (cargo-release) version 0.4.0 ([`28e58f6`](https://github.com/Byron/gitoxide/commit/28e58f6b43a44e010da749a5618df02441f0d2e8))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com/Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
    - [utils #154] refactor: bool.then(||this) - neat ([`1dec1c4`](https://github.com/Byron/gitoxide/commit/1dec1c49032c8acb449e463fde41f403cb640e45))
    - Revert "break more dev-depedency cycles up to git-odb" ([`22337ce`](https://github.com/Byron/gitoxide/commit/22337ce23995eee474e7dfb2e37fb56814522942))
</details>

## v0.3.1 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.1 ([`8b24197`](https://github.com/Byron/gitoxide/commit/8b241977b31720e7f08809bca0b277267b29102e))
    - break more dev-depedency cycles up to git-odb ([`7ee278b`](https://github.com/Byron/gitoxide/commit/7ee278bf5b04adc5e4ab82cb83a3519f93587176))
</details>

## v0.3.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 136 commits contributed to the release over the course of 76 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`0e9c73a`](https://github.com/Byron/gitoxide/commit/0e9c73abd17e0dd21952275077ae53ad7e7aa1af))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - [pack #153] finish transitioning to git-tempfile ([`38173fc`](https://github.com/Byron/gitoxide/commit/38173fcf62c04b485c4b309bdf7e6b7afacfcd58))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref #139] add missing docs ([`5422ec8`](https://github.com/Byron/gitoxide/commit/5422ec8923a5f3c284f7094894a952a392812e63))
    - [pack] refactor ([`581fb51`](https://github.com/Byron/gitoxide/commit/581fb51a84567e341d315e6bacee8e681718f7a7))
    - [pack] refactor ([`b19f6b9`](https://github.com/Byron/gitoxide/commit/b19f6b9b1fcd5ebbc5b1f2a4bef0543b1c693bd1))
    - [pack] fix docs ([`e7b9d96`](https://github.com/Byron/gitoxide/commit/e7b9d9613874cd1ebaf740dc08db467c461a4751))
    - [pack] fix build ([`98dd557`](https://github.com/Byron/gitoxide/commit/98dd557b963acfe1c4e717451d222c187c46a5da))
    - [pack] update CRC values when changing entries to satisfy all consistency checks ([`990ea48`](https://github.com/Byron/gitoxide/commit/990ea4866be2d22ae2043da2dcd9577b748de255))
    - [pack] fix trailer of last entry to match expected recomputed pack hash… ([`8d0ec7d`](https://github.com/Byron/gitoxide/commit/8d0ec7d7c0afb6112e66518a2987907d2e4d29e3))
    - [pack] refactor ([`1852e3e`](https://github.com/Byron/gitoxide/commit/1852e3ea98a462958862ab05f110649e3b06e2b5))
    - [pack] all tests running for now, but… ([`aec8439`](https://github.com/Byron/gitoxide/commit/aec8439683c639f7b6e344cb76bf1dd9fc769d17))
    - [pack] hacky proof of concept that this actually works… ([`6085a92`](https://github.com/Byron/gitoxide/commit/6085a9201ecbd9285547c1d17c9834f09e22fef9))
    - [pack] on the way to 'quickly' get a proof of concept ([`cdc7582`](https://github.com/Byron/gitoxide/commit/cdc7582ab7e35ec1daac44401bf7cb62e0b592a2))
    - [pack] refactor ([`685cce6`](https://github.com/Byron/gitoxide/commit/685cce612eec99ed9f15d86d5ce2a7e6c270ae0d))
    - [pack] refactor ([`f822ebb`](https://github.com/Byron/gitoxide/commit/f822ebb9e899bd52d5baec8179a843c47d073e44))
    - thanks clippy ([`96ef0b0`](https://github.com/Byron/gitoxide/commit/96ef0b036c3c94a45f3ab882a8b32bfcc1250653))
    - [pack] a quickly made iterator that writes input::Entries ([`116bdc4`](https://github.com/Byron/gitoxide/commit/116bdc4ba879da9785877ebca56ab3c57b9cfd98))
    - [pack] prepare a custom writing iterator for input::Entries… ([`a4d2764`](https://github.com/Byron/gitoxide/commit/a4d27648b4021bcf65c95dc5bcfa2b3d11f538fd))
    - thanks clippy ([`bd517d6`](https://github.com/Byron/gitoxide/commit/bd517d6374f20670086eedce2776a8ecf7d0d22b))
    - [pack] prepare bundle writer for yet another iterator wrapper… ([`33be1a1`](https://github.com/Byron/gitoxide/commit/33be1a1ffba34a64eeb04b4479790fec2f50bcba))
    - [pack] refactor ([`50861e6`](https://github.com/Byron/gitoxide/commit/50861e6266a6e1800607eb19288e040846325c06))
    - [pack] refactor ([`dc07225`](https://github.com/Byron/gitoxide/commit/dc07225d7eea04e0cfe61c87b56009e06491726c))
    - [pack] another todo down, the last one ([`3fc8c8f`](https://github.com/Byron/gitoxide/commit/3fc8c8ff5ab1c49b55e3b9e1af3fa2f0aee68b94))
    - [pack] one more todo down, it should work now, right?… ([`69a9ff1`](https://github.com/Byron/gitoxide/commit/69a9ff17b3fe16de782ffabb76b87510e8a5b74e))
    - [pack] fix thin pack support test… ([`4bdebdd`](https://github.com/Byron/gitoxide/commit/4bdebddd3791ba71f3f6b4182229a1c48c5a4a95))
    - [pack] definitely not working yet ([`690d9b7`](https://github.com/Byron/gitoxide/commit/690d9b7fbc34b7d2393649d39290071f81cb8bb1))
    - [pack] a step closer, new cases show up ([`75eaba3`](https://github.com/Byron/gitoxide/commit/75eaba36072cf29e76a97fbbd425f0861eb657e2))
    - [pack] refactor ([`a8512f8`](https://github.com/Byron/gitoxide/commit/a8512f89a4e0dd7492fa208c1da41eed9d6a208f))
    - [pack] improved test to validate a fix ([`e3eeeb1`](https://github.com/Byron/gitoxide/commit/e3eeeb146a0ba3dbe701b2e4da560309ff181753))
    - [pack] attempt to get a more realistic test, but… ([`2890737`](https://github.com/Byron/gitoxide/commit/2890737c7e074d31f3bb55acb63664a2da93faaa))
    - [pack] refactor ([`cabc1e5`](https://github.com/Byron/gitoxide/commit/cabc1e5858d52806542ee8d9266bac36e5d39c96))
    - [pack] first succeeding test ([`f5da439`](https://github.com/Byron/gitoxide/commit/f5da439dce93cc203dacb4a5e9d0ae68a87b9be4))
    - [pack] first reasonably failing test showing that offset computation is indeed wrong ([`df1bc2f`](https://github.com/Byron/gitoxide/commit/df1bc2f66ff9e7046898b6937c5ad239313a70dc))
    - [pack] the first test for the lookup ref deltas iter ([`b162f9e`](https://github.com/Byron/gitoxide/commit/b162f9eb37f09f49e363376dc3f0c6c126442bbf))
    - [pack] Make use of thin-pack resolver when writing bundles… ([`9f43bf0`](https://github.com/Byron/gitoxide/commit/9f43bf029624f7c94346646465e366609b89e2e1))
    - [pack] handle the same ref-base correctly ([`2f94854`](https://github.com/Byron/gitoxide/commit/2f948545a935d2cb7c5a252ec74764440a9ff595))
    - [pack] thin pack resolver which might actually work ([`54f055a`](https://github.com/Byron/gitoxide/commit/54f055a53e888156459340e8ab160650a198ab13))
    - [pack] first sketch of resolver for thin pack entries ([`ee428e0`](https://github.com/Byron/gitoxide/commit/ee428e07bcc3df9bc795d06068a444beed71f2d0))
    - [pack] refactor ([`a8fd70f`](https://github.com/Byron/gitoxide/commit/a8fd70fdbff871779ad5a9ba491162ae49605c9f))
    - [pack] thanks clippy ([`7c2fc89`](https://github.com/Byron/gitoxide/commit/7c2fc89c70aa6de9cb0707799918e623267326a8))
    - [pack] actually, this is how it works, so this code should be unreachable ([`8f359e1`](https://github.com/Byron/gitoxide/commit/8f359e1fc8cb99fcf0003eaab1d97cdeaac20876))
    - [pack] first step towards fixing bad-objects properly ([`3c96507`](https://github.com/Byron/gitoxide/commit/3c965070a7c799f0507f9e7faae2896346bc9e65))
    - [pack] discard bad-object tracking in favor of delayed handling ([`31ce008`](https://github.com/Byron/gitoxide/commit/31ce008208cdd3bc4f093abab6fabf4c8074c130))
    - Revert "[pack] fix race to finally make pack-gen missing objects…" ([`ad0d2a8`](https://github.com/Byron/gitoxide/commit/ad0d2a8e4e92d11351225db0115de0ed1210f9e3))
    - [pack] fix race to finally make pack-gen missing objects… ([`73394db`](https://github.com/Byron/gitoxide/commit/73394db1b048d3dc87b8b4934737f27b6a8a0d3c))
    - [pack] it seems git is just skipping bad objects during pack-gen ([`0f29b82`](https://github.com/Byron/gitoxide/commit/0f29b82b48f45f509016eb16ea92af7f6dbf65a6))
    - Revert "[pack] FAIL: See if not looking up the pack location speeds up counting…" ([`d03fe97`](https://github.com/Byron/gitoxide/commit/d03fe9732b69c6ca3b7a6df96097233661e53a05))
    - [pack] FAIL: See if not looking up the pack location speeds up counting… ([`48c4930`](https://github.com/Byron/gitoxide/commit/48c49300a55e6443d5e4d94632979b6d07f2bc5a))
    - Revert "[pack] FAIL: speedup with Mutex<HashSet>" ([`df98edf`](https://github.com/Byron/gitoxide/commit/df98edf48c49717136a6e8e5d9b1f64aeda17db2))
    - [pack] FAIL: speedup with Mutex<HashSet> ([`f8aca03`](https://github.com/Byron/gitoxide/commit/f8aca03c2d126574541c136019df4e51b52a5b10))
    - [pack] In single-threaded mode, use a huge cache for some speedup ([`aec8a9b`](https://github.com/Byron/gitoxide/commit/aec8a9b4b9deb102b06390a19727eab7660621f9))
    - [pack] fix offset index properly by using chunk-absolute offsets ([`461c1ee`](https://github.com/Byron/gitoxide/commit/461c1eefe9214b07cd80a37292b23744846383d3))
    - [pack] forcefully fix issue with incorrect partition point ([`290bd65`](https://github.com/Byron/gitoxide/commit/290bd65f10f5a64de6735b09119b7bbffc44254b))
    - [pack] test for parital pack without thin pack allowance… ([`1f48d3b`](https://github.com/Byron/gitoxide/commit/1f48d3b58a1151a1fefce9bf4af5649837309a37))
    - [pack] pack-create with immediate counting and traversing… ([`b74a98f`](https://github.com/Byron/gitoxide/commit/b74a98fc87a92a8ccbaec59aeea5284731e2fe49))
    - [pack] entry writer now supports deltas and it seems to work even ([`fcda6f0`](https://github.com/Byron/gitoxide/commit/fcda6f096f95a6322122229ac364a2dd5ea0ce6b))
    - thanks clippy ([`cc61f82`](https://github.com/Byron/gitoxide/commit/cc61f82f597d9a0ab43efaaccc2cb568b9aa746f))
    - [pack] on-demand cache for pack-offset to id lookup ([`0bfdea8`](https://github.com/Byron/gitoxide/commit/0bfdea843606673005ecab6a482a9fce89a4cb69))
    - [pack] refactor ([`4bb3ce4`](https://github.com/Byron/gitoxide/commit/4bb3ce4f2e89dd817c284ed8ae9e2559ed60f9a2))
    - [pack] thin pack offset to index lookup ([`121aca4`](https://github.com/Byron/gitoxide/commit/121aca45ecb1acce3496b1b2ac003aa95851f247))
    - [pack] refactor ([`372b9ce`](https://github.com/Byron/gitoxide/commit/372b9cee78a6b49eb7ebb5cf452a324e07775d98))
    - [pack] a way to obtain whole bundles for offset-to-index lookup ([`15fcbe2`](https://github.com/Byron/gitoxide/commit/15fcbe254b75e8f74652711cc339ae5ade74d24c))
    - [pack] refactor ([`64b1dcd`](https://github.com/Byron/gitoxide/commit/64b1dcdb0fb53749ce73017d0dc1e053689d17d4))
    - [pack] refactor ([`1d713b4`](https://github.com/Byron/gitoxide/commit/1d713b482264ddb0aba6a98e3918f8236ce12c80))
    - [pack] refactor ([`cdf020a`](https://github.com/Byron/gitoxide/commit/cdf020a3b29bc59062d3ccf56672e9c18201c67c))
    - [pack] refactor ([`2ccefb2`](https://github.com/Byron/gitoxide/commit/2ccefb2832b326966a24d0cbcfd79ca5309f91aa))
    - [pack] refactor; entry-iterator now produces delta-objects ([`5dc370b`](https://github.com/Byron/gitoxide/commit/5dc370ba01d25a6e8b7f4bfa03259c83e6b1d758))
    - [pack] rough version of obtaining object indices for deltas ([`a58e270`](https://github.com/Byron/gitoxide/commit/a58e270ef96011ffd2434539e3099cbe27aed3f3))
    - [pack] refactor ([`8cfa414`](https://github.com/Byron/gitoxide/commit/8cfa414482a4318ed385f42582ec885fb73134e3))
    - [pack] pass all data to where it belongs to… ([`af5cb1f`](https://github.com/Byron/gitoxide/commit/af5cb1f4b809ac268ca3d878896854c966dcea97))
    - [pack] add the notion of thin-packs to the pack generator ([`a289bba`](https://github.com/Byron/gitoxide/commit/a289bbaa36546109d3371a8fcd7a6dc3c363861f))
    - [pack] build an index of pack ranges as well ([`4d6ab7b`](https://github.com/Byron/gitoxide/commit/4d6ab7b74c325820a3760361faace380f958572f))
    - [pack] bundle::Location with pack offset; order counts by that… ([`f92f285`](https://github.com/Byron/gitoxide/commit/f92f285167c6b5bc4d86f255e360c4534e38bb29))
    - [pack] better identify the currently implemented pack generation mode. ([`f9e3b3c`](https://github.com/Byron/gitoxide/commit/f9e3b3ca3bbf063e8d71c62fe607b812c745a969))
    - [pack] refactor ([`f3dc3da`](https://github.com/Byron/gitoxide/commit/f3dc3da492e1dda5dd9e43fddc57da6a118081b3))
    - [pack] refactor ([`9ee1e22`](https://github.com/Byron/gitoxide/commit/9ee1e22fa5c5d97ff626f0dfc44706272433bfef))
    - [pack] refactor ([`78d46c1`](https://github.com/Byron/gitoxide/commit/78d46c13d0510ee3e2e2f33cd60d624d63e85900))
    - [pack] refactor ([`69af352`](https://github.com/Byron/gitoxide/commit/69af3526b0fcfa8a270238f3e2cf59d332bd187e))
    - change wording ([`6c82a16`](https://github.com/Byron/gitoxide/commit/6c82a16d340acb9b11c5cf56c917c9fe6f2cdf0e))
    - Bump uluru from 2.1.1 to 2.2.0 ([`52e274f`](https://github.com/Byron/gitoxide/commit/52e274fe985948b6b742ff7066fcb9831e427ba3))
    - Don't use ASM on windows for Sha1 as it fails to build there. ([`ba1fb7a`](https://github.com/Byron/gitoxide/commit/ba1fb7ab5bc03f5a23ece32ff1e144544e1eaeae))
    - Merge branch 'remove-unnecessary-unsafe' ([`7a3c5c1`](https://github.com/Byron/gitoxide/commit/7a3c5c14dc56d8711548d1b219a969836693cbaa))
    - Remove unnecessary unsafe code ([`83e207a`](https://github.com/Byron/gitoxide/commit/83e207a44aece0ff4870e57990bd5aaf43f38e22))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - Bump thiserror from 1.0.25 to 1.0.26 ([`9682590`](https://github.com/Byron/gitoxide/commit/9682590095dc3a502b0c84ccd206ca4797635092))
    - thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - fix build ([`dbfa49a`](https://github.com/Byron/gitoxide/commit/dbfa49acf58b2c0763c5e98e5276860b43dfb27b))
    - Fix everything up so that… ([`5930563`](https://github.com/Byron/gitoxide/commit/5930563601d6c2148cf39e109f69f8b7c7dfcb36))
    - A first attempt to make intrerupt tools work, but… ([`8fb8d37`](https://github.com/Byron/gitoxide/commit/8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5))
    - fix pack tests ([`7968467`](https://github.com/Byron/gitoxide/commit/7968467cc0d392e3d223811ed36ae777531a5a36))
    - The last occurrence of the global git-features::interrupt usage gone ([`6820724`](https://github.com/Byron/gitoxide/commit/6820724be83ebf48c7ccf6a65a3d6383f766c9de))
    - another one ([`0a8ed0e`](https://github.com/Byron/gitoxide/commit/0a8ed0ecc078d76dc3a5fe13518cf43bfbb121f0))
    - And another one down ([`abce75e`](https://github.com/Byron/gitoxide/commit/abce75eefff44b9538c112b60ad5e0596482e89c))
    - refactor ([`7f9be36`](https://github.com/Byron/gitoxide/commit/7f9be36ea909ee67555591287bcb140fdc54c801))
    - And one less usage of the global interrupt handler… ([`5da57a3`](https://github.com/Byron/gitoxide/commit/5da57a3b0efef75ad82cb4d1cd496fc7fc0f1c23))
    - thanks clippy ([`3b2e765`](https://github.com/Byron/gitoxide/commit/3b2e7650d8afe2c0e246e005ab1c321a157cbd44))
    - Make most interrupts local to the method or function ([`4588993`](https://github.com/Byron/gitoxide/commit/458899306a3f3c8578f185d7ecbf1ade2a7142dd))
    - [features] sketch of iterator to auto-check for interruptions ([`61d3a15`](https://github.com/Byron/gitoxide/commit/61d3a15c66b4c1be1d98715b8a60705a3a314455))
    - [pack] refactor ([`25f04ba`](https://github.com/Byron/gitoxide/commit/25f04baa100bd1996f48fbeb4c87e40ff1b27d90))
    - [pack] refactor ([`18cabb8`](https://github.com/Byron/gitoxide/commit/18cabb8618ffc324412302bfda208948abffb61f))
    - [pack] also put counts in order for stable packs ([`f299160`](https://github.com/Byron/gitoxide/commit/f299160cafd00f0fea00a2402901570f5ddf27d5))
    - [pack] fix run of 'cargo test --all' ([`e7ecdc1`](https://github.com/Byron/gitoxide/commit/e7ecdc195d03fa9a29ad1e44464b42e3ca6fb6a4))
    - [pack] a working in-order iterator ([`5fea926`](https://github.com/Byron/gitoxide/commit/5fea926803bcc7b2ef7d8f156e3d31a503831091))
    - [pack] tests for error handling of in-order iterator ([`44892cc`](https://github.com/Byron/gitoxide/commit/44892cca9309c4cca0eaa30dbedc65422a2699d1))
    - [pack] ground work for ordering in produced chunks ([`9680649`](https://github.com/Byron/gitoxide/commit/96806494d32243bd1798a89c094e220dbe050d68))
    - [pack] also run multi-threaded tests as part of unit-tests ([`5d3006a`](https://github.com/Byron/gitoxide/commit/5d3006a5d075bce9011b20920a84404952624c45))
    - Bump uluru from 2.0.0 to 2.1.1 ([`b6ac506`](https://github.com/Byron/gitoxide/commit/b6ac506ba2df0f82eaae64eaf023cc0c0376ddff))
    - [pack] hopefully fix tests on CI; verify determinism of pack ([`51dec8b`](https://github.com/Byron/gitoxide/commit/51dec8b3c661ba9071306ab89796aa93d9a25b65))
    - [pack] deterministic single-threaded pack generation ([`ddb6442`](https://github.com/Byron/gitoxide/commit/ddb6442fd6681a2dd3890a8a415003ec770c7d64))
    - [pack] refactor ([`cfdf802`](https://github.com/Byron/gitoxide/commit/cfdf8021ea1448ac4844b1f3bf252fefde2572fa))
    - [pack] basic statistics for entries ([`37229a6`](https://github.com/Byron/gitoxide/commit/37229a650ceb0155aa7ca87b499fe188ac4bb565))
    - thanks clippy ([`18b2113`](https://github.com/Byron/gitoxide/commit/18b2113b1e3c372145bc9037ee6a9de7efe4e506))
    - [pack] write packs to a directory with the proper name ([`3fbca7d`](https://github.com/Byron/gitoxide/commit/3fbca7dd62752a7dd752b83a39ec8dfd7b2f2ea8))
    - [pack] refactor ([`f10adea`](https://github.com/Byron/gitoxide/commit/f10adea76d92eada3ca204fe69e7b5f81a06d8cc))
    - [pack] fix docs ([`6ba471d`](https://github.com/Byron/gitoxide/commit/6ba471d228c45a3821b4984905a4b4ecaff5b0b0))
    - [pack] fix build ([`81ee633`](https://github.com/Byron/gitoxide/commit/81ee633c7f482746bc28a2a43d74ebbaded7af5f))
    - [pack] statistics for counting objects seemingly work… ([`4e3deb1`](https://github.com/Byron/gitoxide/commit/4e3deb1364dd1bef0af79d6aa97086a95b4983bc))
    - [pack] actual counts statistics ([`3a9f6d8`](https://github.com/Byron/gitoxide/commit/3a9f6d8a53da3235bde4a3f32859381d4843cb7e))
    - [pack] aggregate the count outcome ([`c7ac0e6`](https://github.com/Byron/gitoxide/commit/c7ac0e60a5d69f3a948d47c3acc3060cddbafb98))
    - [pack] use statistics reducer ([`0974ab1`](https://github.com/Byron/gitoxide/commit/0974ab176777bfa02ac0ea32915f6d9c46e3ddeb))
    - [pack] count object reducer sketch ([`ea45692`](https://github.com/Byron/gitoxide/commit/ea4569282e2f63042869dd47205874c161bfecfe))
    - [pack] refactor ([`fdf485a`](https://github.com/Byron/gitoxide/commit/fdf485afa66af20abca586b04f588a33c167310f))
    - [pack] refactor ([`0514f1d`](https://github.com/Byron/gitoxide/commit/0514f1df113c5f6bf1c934b15741ca8ea47316ae))
    - [pack] refactor ([`37922d1`](https://github.com/Byron/gitoxide/commit/37922d12765c221e747fad4ca813597490525279))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com/Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
    - refactor ([`a25a774`](https://github.com/Byron/gitoxide/commit/a25a774675e2e9db1c891351077d3af2fd5c72ed))
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports ([`de2ba3c`](https://github.com/Byron/gitoxide/commit/de2ba3c4919d454894911c54fd4bb0e0a4665723))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com/Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - [git-ref] the first failing test ([`7e802a0`](https://github.com/Byron/gitoxide/commit/7e802a0576230dfc666c253d484ea255f265f92f))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

## v0.2.0 (2021-05-25)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`b213628`](https://github.com/Byron/gitoxide/commit/b213628feeb8dfa87dab489c7d3155a60e6a236d))
    - [git-odb] prep release ([`4984ce3`](https://github.com/Byron/gitoxide/commit/4984ce3e19b60b89a4337f90ac4b9c44c42558a0))
    - [git-odb] refactor ([`2958145`](https://github.com/Byron/gitoxide/commit/2958145a0ae1ef582bbf88352f5567d5c2b5eaf0))
    - [git-pack] fix docs ([`efd20d4`](https://github.com/Byron/gitoxide/commit/efd20d4e1afbfbe573d620dea4761c06f948a296))
    - [git-pack] refactor ([`ea2b3de`](https://github.com/Byron/gitoxide/commit/ea2b3deab78882943e11270e4166ca7c340b03e1))
    - [git-pack] refactor ([`bc4b7b1`](https://github.com/Byron/gitoxide/commit/bc4b7b18a04506a3d08d66d1222d706b82a2f6e7))
    - [git-pack] refactor ([`157b6ff`](https://github.com/Byron/gitoxide/commit/157b6ff7b55ba2b7f8f90f66864212906426f8d7))
    - [git-pack] refactor ([`49c1c3e`](https://github.com/Byron/gitoxide/commit/49c1c3ea67379c5a122a8c3921d8ff713e14d371))
    - (cargo-release) version 0.16.0 ([`769c649`](https://github.com/Byron/gitoxide/commit/769c649c00c009bf5a3f7c0611a7b999618f2938))
    - [git-pack] refactor ([`be6ddaa`](https://github.com/Byron/gitoxide/commit/be6ddaa98fc1dcaf77dc0fd9c9d67754e74927e4))
    - [git-pack] used by git-odb ([`5d6ee07`](https://github.com/Byron/gitoxide/commit/5d6ee07a8dec64fe5f68c14c418d922077fad3df))
    - [git-pack] refactor ([`1b2a245`](https://github.com/Byron/gitoxide/commit/1b2a245aa494c0f9cacc2ad6b8ca02e9891fdb4c))
    - [git-pack] move hash-writer to git-features as it's quite general purpose ([`80e5640`](https://github.com/Byron/gitoxide/commit/80e5640169363910b4189fda58bb495c6677eaaa))
</details>

## v0.1.0 (2021-05-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [git-pack] the very first version… ([`8c06cdb`](https://github.com/Byron/gitoxide/commit/8c06cdb14269e798b7ff771ea3864f85fa673ed7))
</details>

