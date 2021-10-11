# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### New Features

 - <csr-id-11b64fce4630371633b6415f227eecdc6b42b20b/> Make `git_url::Url` available under `git_repository::Url`

### refactor (BREAKING)

 - <csr-id-1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7/> Use 'to_*' when converting `easy::Object` to specific object kind
   This also makes the API more consistent while being more idiomatic.

### other (BREAKING)

 - <csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/> Avoid duplicate module paths in 'tree' and 'commit'
 - <csr-id-a19567eceab0dd7f5478b83c2ff9ce79754db308/> rename ObjectIdExt::ancestors_iter() to *::ancestors()
 - <csr-id-61793ff42f5c2f9ddf302901adea2dac6149eac8/> rename `easy::Object::to_(commit|tag)_iter()`…
   …to  `easy::Object::try_to_(commit|tag)_iter()` for consistency.
 - <csr-id-0cd585e20a5abd323a34ec32d92fbd48531b3b18/> rename `*::State` into `*::Platform`
 - <csr-id-89f15051763a03627f332c46beedfc53b8b9b15b/> various small API changes
 - <csr-id-f644d0ede7a2e8d344a81c7003c3877eed64a6b0/> move easy::head::peel::Error -> easy::head::peel::to_id::Error
 - <csr-id-ac3b9efb7b90958274ce55800959d930f8641115/> rename path::is_git to path::is
 - <csr-id-03fe8a7ebd34608d725d4585da5c1630123762ec/> rename easy::reference::log::State to easy::reference::Logs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 84 commits contributed to the release over the course of 29 calendar days.
 - 35 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#164](https://github.com//Byron/gitoxide/issues/164), [#198](https://github.com//Byron/gitoxide/issues/198), [#200](https://github.com//Byron/gitoxide/issues/200), [#67](https://github.com//Byron/gitoxide/issues/67)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#164**
    - path::is ([`1f4e45a`](https://github.com//Byron/gitoxide/commit/1f4e45a26a3d2727f00c3f248452dd41fc8a95be))
    - rename path::is_git to path::is ([`ac3b9ef`](https://github.com//Byron/gitoxide/commit/ac3b9efb7b90958274ce55800959d930f8641115))
    - path::discover ([`1958e8a`](https://github.com//Byron/gitoxide/commit/1958e8aa65eb97f9755f065d713f0a48c5e41b1b))
    - Avoid duplicate module paths in 'tree' and 'commit' ([`2f2d856`](https://github.com//Byron/gitoxide/commit/2f2d856efe733d3cf81110c0e0607d2e7c40d968))
    - top-level of 'path' module ([`066f59b`](https://github.com//Byron/gitoxide/commit/066f59b23a125b1ce9a015437a3f4468e5791da0))
    - object_id ([`329d183`](https://github.com//Byron/gitoxide/commit/329d183ad4e256a4f9cdeb34589b5f3432495f79))
    - rename ObjectIdExt::ancestors_iter() to *::ancestors() ([`a19567e`](https://github.com//Byron/gitoxide/commit/a19567eceab0dd7f5478b83c2ff9ce79754db308))
    - repository ([`1a1959f`](https://github.com//Byron/gitoxide/commit/1a1959f487d69ffdd5394775b707139c44dbd11d))
    - ext::tree ([`5e091fb`](https://github.com//Byron/gitoxide/commit/5e091fb2b4fd33879c176e6dadd3c9805d99af50))
    - easy::object::peel ([`e376067`](https://github.com//Byron/gitoxide/commit/e3760679547e0dc1bf31761acdb6e63b04a50919))
    - easy::object::errors ([`de004b3`](https://github.com//Byron/gitoxide/commit/de004b318fdc6923711dd001bff5f4bcbba4270e))
    - rename `easy::Object::to_(commit|tag)_iter()`… ([`61793ff`](https://github.com//Byron/gitoxide/commit/61793ff42f5c2f9ddf302901adea2dac6149eac8))
    - easy::object, sans a few child-modules ([`f582439`](https://github.com//Byron/gitoxide/commit/f582439a3efe5c234f54c488792395e9de09a032))
    - update 'platform' information to reflect the current usage ([`42080ae`](https://github.com//Byron/gitoxide/commit/42080aefe3b286afb58235c1c22491579ab73919))
    - rename easy::reference::log::State to easy::reference::Logs ([`03fe8a7`](https://github.com//Byron/gitoxide/commit/03fe8a7ebd34608d725d4585da5c1630123762ec))
    - rename `*::State` into `*::Platform` ([`0cd585e`](https://github.com//Byron/gitoxide/commit/0cd585e20a5abd323a34ec32d92fbd48531b3b18))
 * **#198**
    - regenerate all changelogs to get links ([`d654788`](https://github.com//Byron/gitoxide/commit/d65478880a170235e4f838156862ed035894fd5b))
    - pass actual repository url down from commands ([`a10f51d`](https://github.com//Byron/gitoxide/commit/a10f51d2da0a4291bfd907ff6a963dac2e7cdc8e))
    - Make `git_url::Url` available under `git_repository::Url` ([`11b64fc`](https://github.com//Byron/gitoxide/commit/11b64fce4630371633b6415f227eecdc6b42b20b))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com//Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com//Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - merge doesn't consider user generated sections, only the ones it would want to add ([`ebbebdd`](https://github.com//Byron/gitoxide/commit/ebbebdd70aeec9aa3ad453d61375429a7f555bbc))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com//Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com//Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com//Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Use hashmap based lookup for trees… ([`48a0c76`](https://github.com//Byron/gitoxide/commit/48a0c76ab163b6e35b19dd2a9efc2e101a721633))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com//Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com//Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com//Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com//Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com//Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com//Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com//Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Use 'to_*' when converting `easy::Object` to specific object kind ([`1cb41f8`](https://github.com//Byron/gitoxide/commit/1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7))
    - Fix panic related to incorrect handling of character boundaries ([`9e92cff`](https://github.com//Byron/gitoxide/commit/9e92cff33f4f53d3b2d6b55a722d577c2dd6a4f2))
    - Fix build ([`d0a956f`](https://github.com//Byron/gitoxide/commit/d0a956fdb5a822dbd116792bfbe70d1532a95ec9))
    - refactor!: Use git_object::commit::MessageRef::summary()… ([`13e7c3a`](https://github.com//Byron/gitoxide/commit/13e7c3ad5e079fe778d07d115c9e41c4c6eb038f))
    - Sketch data for parsed messages ([`32dd280`](https://github.com//Byron/gitoxide/commit/32dd280eaada635994e11b4f2722a4efc59faa8f))
    - smart-release: a seemingly slow version of path lookup, but… ([`41afad3`](https://github.com//Byron/gitoxide/commit/41afad3386461b658ee859225785b6de86d13cfb))
    - configure caches with env vars using `apply_environment()` ([`d422b9a`](https://github.com//Byron/gitoxide/commit/d422b9a31a37a03551bec4382039aaf3a7e49902))
    - refactor ([`e7c061b`](https://github.com//Byron/gitoxide/commit/e7c061b10c263001eb4abf03098d6694b770f828))
    - set package cache via RepositoryAccessExt ([`66292fd`](https://github.com//Byron/gitoxide/commit/66292fd1076c2c9db4694c5ded09799a0be11a03))
    - smart-release(feat): Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size… ([`5aadf75`](https://github.com//Byron/gitoxide/commit/5aadf75a0d93d1a990ad0305c38366c5c22bdcb2))
    - allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE ([`d79a1b7`](https://github.com//Byron/gitoxide/commit/d79a1b75304e397c16b5af7055906591a187ddfd))
    - prepare for configurable pack cache ([`7d2b6b6`](https://github.com//Byron/gitoxide/commit/7d2b6b66e09ff39727fccd68d190679b52d90126))
    - object-cache to allow for a speed boost… ([`06996e0`](https://github.com//Byron/gitoxide/commit/06996e032b1e451a674395ebaca94434fac46f05))
    - smart-release: build commit history for later use in changelog generation ([`daec716`](https://github.com//Byron/gitoxide/commit/daec7167df524b329daad7dabb1b9920b6ef8936))
    - Allow object access during commit ancestor traversal… ([`4fe4786`](https://github.com//Byron/gitoxide/commit/4fe4786797d240a59d29dbf2c6310490a381c8b6))
    - smart-release: sketch history acquisition ([`debe009`](https://github.com//Byron/gitoxide/commit/debe0094826f83839f907523715def929133fd58))
    - various small API changes ([`89f1505`](https://github.com//Byron/gitoxide/commit/89f15051763a03627f332c46beedfc53b8b9b15b))
    - add 'Head::peeled()' method ([`56e39fa`](https://github.com//Byron/gitoxide/commit/56e39fac54bfa3871c42bbf76a9f7c49486b85be))
    - move easy::head::peel::Error -> easy::head::peel::to_id::Error ([`f644d0e`](https://github.com//Byron/gitoxide/commit/f644d0ede7a2e8d344a81c7003c3877eed64a6b0))
    - loose reference iteration with non-dir prefixes… ([`293bfc0`](https://github.com//Byron/gitoxide/commit/293bfc0278c5983c0beaec93253fb51f00d81156))
    - Add 'references().all().peeled().'… ([`6502412`](https://github.com//Byron/gitoxide/commit/650241251a420602f74037babfc24c9f64df78d8))
    - smart-release: filter refs correctly, but… ([`2b4a615`](https://github.com//Byron/gitoxide/commit/2b4a61589a7cba3f7600710e21304e731ae3b36a))
 * **#200**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… ([`f293b63`](https://github.com//Byron/gitoxide/commit/f293b633d16c0f7393d0ede64e12f14e47d0296b))
 * **#67**
    - split data::output::count::objects into files ([`8fe4612`](https://github.com//Byron/gitoxide/commit/8fe461281842b58aa11437445637c6e587bedd63))
    - use new git_pack::cache::Object trait ([`b209da2`](https://github.com//Byron/gitoxide/commit/b209da29f361512ba757febf56bc1aca039f2a41))
    - remove object cache impl which now lives in git-pack ([`741558d`](https://github.com//Byron/gitoxide/commit/741558dd8194590c5cc8566aa22f96e73df38edf))
    - Use Easy in the one spot where it is possible… ([`6a97bfa`](https://github.com//Byron/gitoxide/commit/6a97bfabcec6597efe9282e6d5c9f0ac3ada61dc))
    - try to create persistent Easy iterator, but can't make it Send… ([`54a64a5`](https://github.com//Byron/gitoxide/commit/54a64a588ff72515451a3d0343306ac4abe1cb35))
 * **Uncategorized**
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com//Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com//Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com//Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - thanks clippy ([`ae7826e`](https://github.com//Byron/gitoxide/commit/ae7826e1cf79fce6ad12f407162f58b3bfb02b16))
    - thanks clippy ([`b02edb5`](https://github.com//Byron/gitoxide/commit/b02edb5b1e9b7c8f8bd1b4a8e2d60667da629839))
    - thanks clippy ([`68ea77d`](https://github.com//Byron/gitoxide/commit/68ea77dcdd5eb8033618e7af2e3eb0989007b89b))
    - improved changelog… ([`8b82f7d`](https://github.com//Byron/gitoxide/commit/8b82f7d44c7eb63b7922ddc31ada9cefdce776b0))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com//Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
    - Bump git-repository v0.10.0 ([`5a10dde`](https://github.com//Byron/gitoxide/commit/5a10dde1bcbc03157f3ba45104638a8b5b296cb9))
    - [repository #164] docs for easy::reference::log ([`7de7c7e`](https://github.com//Byron/gitoxide/commit/7de7c7eb51b7d709fd140dbf789e31e97161bfa7))
    - [repository #164] docs for easy::reference::iter ([`d86c713`](https://github.com//Byron/gitoxide/commit/d86c71363a5a73dd8986566a9687e2b4756972cb))
    - [repository #164] refactor ([`437e63b`](https://github.com//Byron/gitoxide/commit/437e63b4e841ef478c12a91bf3e2dce63d5b1041))
    - [repository #164] docs for top-level of easy::reference ([`9e465e0`](https://github.com//Byron/gitoxide/commit/9e465e03dc636c360128c93864749c4a3f8a99e5))
    - [repository #164] docs for easy::oid ([`b66b6fe`](https://github.com//Byron/gitoxide/commit/b66b6fe759eeb55cb875fcb65aa58b62c6963ca8))
    - [repository #164] docs for easy::commit and easy::odb ([`abf37e5`](https://github.com//Byron/gitoxide/commit/abf37e54e5a4584f521988e27dd02f6d6badc4ef))
    - [repository #164] Documentation for `easy::borrow` ([`3e612f4`](https://github.com//Byron/gitoxide/commit/3e612f441e1e837d7ba3d3ddd40b4a8c2ba05c61))
    - [repository #164] docs for easy::head::* ([`516fde7`](https://github.com//Byron/gitoxide/commit/516fde7ffb505603479b4de2a78200da480b66ed))
    - [repository #164] refactor ([`65b0e0f`](https://github.com//Byron/gitoxide/commit/65b0e0fbe7ab7cb405fd267802e7ad3de36d98f7))
    - [repository #164] docs for `easy::ext::ReferenceAccessExt` ([`ab4910f`](https://github.com//Byron/gitoxide/commit/ab4910f1b4bf98569a04596b43aba862caca029b))
    - [repository #164] docs for easy::ext::RepositoryAccessExt ([`9041d47`](https://github.com//Byron/gitoxide/commit/9041d474f178f45c86d628a7140c64810365b97d))
    - [repository #164] another test and fix for `commit()` ([`8d676d7`](https://github.com//Byron/gitoxide/commit/8d676d77cb69df203d3fcbf8c1a34f212035605f))
    - [repository #164] easy::ext::ObjectAccessExt docs ([`c4984af`](https://github.com//Byron/gitoxide/commit/c4984af4f6343a17290f6c85f8385e77354875bb))
    - [repository #164] ([`4111d22`](https://github.com//Byron/gitoxide/commit/4111d22ebe4cc9ddd726cce566e5872708067440))
</details>

## v0.9.1 (2021-09-10)

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.9.1 ([`262c122`](https://github.com//Byron/gitoxide/commit/262c1229d6d2d55c70fe0e199ab15d10954d967b))
    - Release git-ref v0.7.3 ([`b0a9815`](https://github.com//Byron/gitoxide/commit/b0a98157ab3b240af027acb9965c981a543e55fa))
    - [repository] don't enforce feature flags that may fail on windows by default ([`afdec2e`](https://github.com//Byron/gitoxide/commit/afdec2e89eee0397b16602fdff16d3997ef370d0))
    - Release git-ref v0.7.2 ([`e940e9a`](https://github.com//Byron/gitoxide/commit/e940e9a21938035eb8791bba19cc16814a0fb4e7))
    - Release git-protocol v0.10.4 ([`898ee08`](https://github.com//Byron/gitoxide/commit/898ee08befa1eb7dd22980063c7633f83d0a8958))
    - Release git-odb v0.21.3 ([`223f930`](https://github.com//Byron/gitoxide/commit/223f93075a28dd49f44505c039cfeae5a7296914))
</details>

## v0.9.0 (2021-09-08)

- rename `prelude::ConfigAccessExt` to `prelude::RepositoryAccessExt`
- `prelude::ObjectAccessExt::commit()` signature change
- cargo feature changed in incompatible ways. `network` was replaced by more finegrained options for _blocking_ and _async_ networking, as well as optional http transport

### New

- `init()`
- `init_bare()`
- `Repository::init(Kind)`
- `open()`
- `Repository::open()`
- `easy::Head`
- `easy::ext::ReferenceAccessExt::head()`
- `ext::ReferenceExt` trait

### Breaking
- **renames / moves / Signature Changes**
    - `path::Path` to `Path`
    - `init::repository(dir)` -> `path::create::into(dir, **Kind**)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.11.0 ([`5ae6ff5`](https://github.com//Byron/gitoxide/commit/5ae6ff52cd2cd1ccd1e26bb987c154eb19603696))
    - Bump git-repository v0.9.0 ([`b797fc1`](https://github.com//Byron/gitoxide/commit/b797fc10f3f3d1fbc23916a4ff6e5e860e2dd4ed))
    - [repository #193] Add feature flags for async/blocking ([`57f482c`](https://github.com//Byron/gitoxide/commit/57f482c59ac47b7a5f1abf01b4a3e25364e061c2))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com//Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
    - [repository #164] Prepare `commit()` for a possible less-allocating future ([`0fd01f7`](https://github.com//Byron/gitoxide/commit/0fd01f7071c785c27c56d2c034aac8dcdf690677))
    - [repository #164] Support for refreshing the object database ([`46e10f8`](https://github.com//Byron/gitoxide/commit/46e10f863e1fea419483a7b086022c16cd0ca226))
    - [odb #164] Add refresh() functionality ([`ee16d04`](https://github.com//Byron/gitoxide/commit/ee16d041941a5777c8f6495a28f7633c327cbd6b))
</details>

## v0.8.2 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 66 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.2 ([`3fc23be`](https://github.com//Byron/gitoxide/commit/3fc23beaf103c037253ace727c87ec457be5dedd))
    - [repository #190] test for oid.ancestors().all() ([`fdc3678`](https://github.com//Byron/gitoxide/commit/fdc3678c63fa128ac754b3fa9ae3d88a4a221d0d))
    - [repository #190] fix build, lets just make traversal available by default ([`6da3599`](https://github.com//Byron/gitoxide/commit/6da35994cf2a3c9ab741733af53761c9a2cebeed))
    - Bump git-pack v0.10.0 ([`e5e3c80`](https://github.com//Byron/gitoxide/commit/e5e3c8024e1c2e5e90cee83abbdae41d58eee156))
    - [repository #190] access to repository directories ([`f4d1ec4`](https://github.com//Byron/gitoxide/commit/f4d1ec4ac0be8aa46d97eb92fb8a8f3fb8da94fb))
    - [repository #190] first shot at ancestor iteration… ([`85f1a48`](https://github.com//Byron/gitoxide/commit/85f1a48ea39f3b224e8d0ba3728dd75e03a6edc3))
    - [repository #190] refactor ([`e7188e0`](https://github.com//Byron/gitoxide/commit/e7188e047529cb0f4b20b3876f36b4592e9d2dc4))
    - [ref #190] fix tests ([`e426e15`](https://github.com//Byron/gitoxide/commit/e426e15188d8ec38ee0029f1d080dbab9afd8642))
    - [repository #190] fix tests; needs inbound transaction handling… ([`e5a5c09`](https://github.com//Byron/gitoxide/commit/e5a5c09bb108741fff416672566e381f50f02b38))
    - [repository #190] leverage git-ref namespace support ([`1aa9c11`](https://github.com//Byron/gitoxide/commit/1aa9c113488175f03758f8a64338a33b3417dd87))
    - [repository #190] refactor ([`609c249`](https://github.com//Byron/gitoxide/commit/609c249916ca64f4beecdab86eb4562adbd1ca4f))
    - [repository #190] fix build ([`f5e118c`](https://github.com//Byron/gitoxide/commit/f5e118c8871e45ed3db9da9cd6bc63a5ea99621e))
    - [repository #190] note a known limitation about finding references in namespaces… ([`d335731`](https://github.com//Byron/gitoxide/commit/d3357318cf100fc3e0751e5b6de3922b1c209ddb))
    - [repository #190] transparent namespace support ([`d14f073`](https://github.com//Byron/gitoxide/commit/d14f073707c2f4641a271ba7965ec8281638e8df))
    - [repository #190] turns out we need bstr with unicode support ([`3d8796e`](https://github.com//Byron/gitoxide/commit/3d8796e670f9bb5d2ed22fb3b75130a599737341))
    - [repository #190] public bstr re-export ([`3b7ffde`](https://github.com//Byron/gitoxide/commit/3b7ffde385b1984393ee65a7505ad7221fecd0dc))
    - [repository #190] cleanup usage of bstr… ([`e4411ff`](https://github.com//Byron/gitoxide/commit/e4411ff43b24af79fefeaa4411e004dc504a4e2a))
    - [repository #190] prefixed reference iteration ([`a6e19c9`](https://github.com//Byron/gitoxide/commit/a6e19c9a49bdc6a7c5cabef0a8d93bfd48a74fcd))
    - [repository #190] implementation of reference iteration (all() for now)… ([`2c0939a`](https://github.com//Byron/gitoxide/commit/2c0939a146b5973de26bd03987e075a34a84bc88))
    - [repository #190] refactor ([`8c532a4`](https://github.com//Byron/gitoxide/commit/8c532a4c78452dd11115cf36a906a27741858774))
    - [repository #190] prepare reference iteration ([`427f146`](https://github.com//Byron/gitoxide/commit/427f14622fb98e0397de2cae4d36a29f5915d375))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com//Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #190] obtain the kind fo hash used in a repo ([`a985491`](https://github.com//Byron/gitoxide/commit/a985491bcea5f76942b863de8a9a89dd235dd0c9))
    - [repository #190] refactor ([`7a111b1`](https://github.com//Byron/gitoxide/commit/7a111b126cfb318acb2d09d119315150a38b7cd3))
    - [repository #190] shortcut to create references ([`28afd8e`](https://github.com//Byron/gitoxide/commit/28afd8e7cf09a17410c4a6ad57cddda608371364))
    - [ref #190] add forward log iter and localize iter types… ([`c3e240d`](https://github.com//Byron/gitoxide/commit/c3e240da47021226311681f3bcd48983f354243f))
    - [repository #190] refactor ([`e751688`](https://github.com//Byron/gitoxide/commit/e751688a5378552b73cfddd07f38a0d0bb491b83))
    - thanks clippy ([`023dedc`](https://github.com//Byron/gitoxide/commit/023dedc41aa859cd49d208392a586deaf77bd1bd))
    - [ref #190] reverse reflog ergonomics ([`2de86f9`](https://github.com//Byron/gitoxide/commit/2de86f904f6ee63e292f9c701cc3524e8bfe87e4))
    - [repository #190] ref log for HEAD specifically ([`946bbf1`](https://github.com//Byron/gitoxide/commit/946bbf19ed3f793b0eb1c5c90a655140e12d7e21))
    - [repository #190] reflog tests ([`641edde`](https://github.com//Byron/gitoxide/commit/641edde5608ff22bf18cea845ba1925b84a7b9f2))
    - [ref #190] First working sketch of reverse log iter access ([`4a36ded`](https://github.com//Byron/gitoxide/commit/4a36dedc17ce3124802d1b72330abc524fd98c6f))
    - [ref #190] move remaining file store functions to extension trait ([`60fc215`](https://github.com//Byron/gitoxide/commit/60fc215ccac529b4a14cb9d8260ab9ddec86758a))
    - thanks clippy ([`376c045`](https://github.com//Byron/gitoxide/commit/376c045cf589e51b639cf6c3633c4a8fcae7b6aa))
    - [repository #190] refactor ([`15d4ac8`](https://github.com//Byron/gitoxide/commit/15d4ac8f4b08716f6b06938f01396fb8ba8e7086))
    - [repository #190] a major step forward with `head()` access ([`43ac4f5`](https://github.com//Byron/gitoxide/commit/43ac4f5acbe3ace5d43ed3ed1bc394d721f0e273))
    - [ref #190] cache peeled objects properly ([`2cb511e`](https://github.com//Byron/gitoxide/commit/2cb511efe5833f860f3c17b8e5f5b4cd643baddb))
    - Bump git-ref v0.7.0 ([`ac4413c`](https://github.com//Byron/gitoxide/commit/ac4413ce4e45703d5fe722e7220d039217f0bdef))
    - [repository #190] experiment with 'HEAD' API… ([`c55ce4d`](https://github.com//Byron/gitoxide/commit/c55ce4d8453c1ab4a107f5c6fb01521b422ee5c4))
    - thanks clippy ([`14dff63`](https://github.com//Byron/gitoxide/commit/14dff63fbc0d318bbc8a2618e0d72aaa98948acf))
    - [ref #190] Use Raw Reference everywhere for great simplification… ([`7aeea9c`](https://github.com//Byron/gitoxide/commit/7aeea9c36d4da04a806e68968356f8cc0dc11475))
    - [repository #190] refactor ([`d6bef3a`](https://github.com//Byron/gitoxide/commit/d6bef3afe7168659a75e26fb3ae2aa722fecf853))
    - [ref #190] introduce Raw reference type that simplifies everything… ([`8634341`](https://github.com//Byron/gitoxide/commit/86343416dec8026f32c57d164dec4bf9b75b6536))
    - [ref #190] refactor ([`07126d6`](https://github.com//Byron/gitoxide/commit/07126d65946e981b339b6535986597cb328a1c9e))
    - [ref #190] Allow for explicit expected previous values ([`1a4786f`](https://github.com//Byron/gitoxide/commit/1a4786fb3bdb3d3a86b026dbf04e6baef6d3c695))
    - [repository #190] show that unconditional creation of references doesn't is lacking… ([`06b9270`](https://github.com//Byron/gitoxide/commit/06b9270e67823e9e911a9fa9d6eeeedcd93e62cb))
    - [repository #190] another commit() test… ([`4ec631c`](https://github.com//Byron/gitoxide/commit/4ec631c92349bbffa69c786838d2127b0c51970e))
    - [repository #190] produce nice reflog messages ([`e7a8b62`](https://github.com//Byron/gitoxide/commit/e7a8b62eb24f840f639aa436b4e79a4a567d3d05))
    - [repository #190] commit::summary() ([`43f7568`](https://github.com//Byron/gitoxide/commit/43f7568bd11fc310bac8350991ff3d4183dcd17b))
    - [repository #190] thanks clippy ([`0763ac2`](https://github.com//Byron/gitoxide/commit/0763ac260450b53b42f3c139deae5736fef056ce))
    - [repository #190] first version of 'commit(…)' without reflog message handling ([`bfcf8f1`](https://github.com//Byron/gitoxide/commit/bfcf8f17c7a89027e5bbcb5f85e3d0ba4036e8a0))
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly ([`63bedc7`](https://github.com//Byron/gitoxide/commit/63bedc7647bb584353289e19972adf351765a526))
    - [repository #190] put git-lock into ST1… ([`26a6637`](https://github.com//Byron/gitoxide/commit/26a6637222081997ad7c08f4dc8d8facfb9cf94e))
    - [repository #190] refactor ([`1e029b4`](https://github.com//Byron/gitoxide/commit/1e029b4beb6266853d5035c52b3d85bf98469556))
    - [repository #190] A way to write objects and the empty tree specifically ([`7c559d6`](https://github.com//Byron/gitoxide/commit/7c559d6e1b68bc89220bca426257f383bce586ae))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com//Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [repository #190] Make local-offset available on demand only… ([`1927be7`](https://github.com//Byron/gitoxide/commit/1927be7764f6af04ecc715dd52c631a3c8e16577))
    - [repository #185] rustfmt ([`dfbb015`](https://github.com//Byron/gitoxide/commit/dfbb015a89db47c79015135870013ecc384c4aea))
    - [repository #185] remove quick-error infavor of thiserror ([`212c44c`](https://github.com//Byron/gitoxide/commit/212c44c84b903681f6d35d934ee5f7ad6e1da791))
    - [repository #185] on the way to removing quick-error ([`6ecd431`](https://github.com//Byron/gitoxide/commit/6ecd431661e7ddc2f97e5a78a7932d2a7f1f27f0))
    - [repository #185] support for initializing bare repositories ([`9e8a39e`](https://github.com//Byron/gitoxide/commit/9e8a39e3cbd620bd48f379743df0d5783c33a86f))
    - [repository #185] use git-config to handle bare repos more properly ([`8a5aac5`](https://github.com//Byron/gitoxide/commit/8a5aac55cf62bdd7287a363fa29f12aa39d4c583))
    - [repository #185] sketch of how to open a repository… ([`48207b5`](https://github.com//Byron/gitoxide/commit/48207b54b97ac1b6354f6b53c13ccc4d1d8ea98f))
    - [repository #185] refactor ([`63089ff`](https://github.com//Byron/gitoxide/commit/63089ff356ea0f62963ae213ea0dbb09f891ada6))
    - [repository #185] refactor ([`7604935`](https://github.com//Byron/gitoxide/commit/7604935b12eacb26a98bedc5f77636b5583629a5))
    - [repository #185] refactor repository initialization… ([`5ff7eaa`](https://github.com//Byron/gitoxide/commit/5ff7eaa86bddfa94aec97355a5d6adb117045693))
</details>

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.1 ([`b269a12`](https://github.com//Byron/gitoxide/commit/b269a1264f830bafcfe74f0f3ce01448c894146e))
    - [repository #164] make EasyArcExclusive available ([`2fa3dcb`](https://github.com//Byron/gitoxide/commit/2fa3dcb40a34a7ec19382e5f6a71348ecf7a7c36))
</details>

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies
### Commit Statistics

<csr-read-only-do-not-edit/>

 - 117 commits contributed to the release over the course of 10 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] keep assets ([`e0fca77`](https://github.com//Byron/gitoxide/commit/e0fca771f5ee068b0a9a0975930317d0883701cc))
    - [repository #174] remove arc_lock code entirely ([`dcbe742`](https://github.com//Byron/gitoxide/commit/dcbe742eb5244f0b5c6563cf59962183b708f54f))
    - [repository #174] conditionally compile future parking_lot version… ([`5375fc8`](https://github.com//Byron/gitoxide/commit/5375fc872b9af2526683326f58e9c3d7f20ef166))
    - Bump git-repository v0.8.0 ([`cdb45ff`](https://github.com//Byron/gitoxide/commit/cdb45ffa0810e9fcc9fd25bff7b696c2d27eeef5))
    - [repository #174] adjust various changelogs ([`081faf5`](https://github.com//Byron/gitoxide/commit/081faf5c3a21b34b7068b44d8206fb5770c392f5))
    - Bump git-protocol v0.10.0 ([`82d5a0b`](https://github.com//Byron/gitoxide/commit/82d5a0bb38903a8389e43cd5416e02e5496e661a))
    - Bump git-odb v0.21.0 ([`7b9854f`](https://github.com//Byron/gitoxide/commit/7b9854fb35e86958a5ca827ec9a55b1168f38395))
    - [pack #179] refactor ([`ab6554b`](https://github.com//Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [packetline #178] fix compile warnings ([`c8d2e72`](https://github.com//Byron/gitoxide/commit/c8d2e72d272243da7d853f78463552bfc58ed9d6))
    - Bump git-traverse v0.8.0 ([`54f3541`](https://github.com//Byron/gitoxide/commit/54f3541f1448a8afa044d3958fa1be5b074e4445))
    - Bump git-diff v0.9.0 ([`2e2e798`](https://github.com//Byron/gitoxide/commit/2e2e7983178b3af7e5684995de68ed5d020927ec))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com//Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177] fix docs ([`2fd23ed`](https://github.com//Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - [object #177] migrate immutable::commit into crate::commit ([`45d3934`](https://github.com//Byron/gitoxide/commit/45d393438eac2c7ecd47670922437dd0de4cd69b))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com//Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [object #177] move mutable objects to crate::* ([`c551c02`](https://github.com//Byron/gitoxide/commit/c551c0236c64f3237cb9be7f35159f753d4b871f))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com//Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com//Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com//Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com//Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - [ref #175] follow (try_)find(_what) naming convention ([`679895c`](https://github.com//Byron/gitoxide/commit/679895cf866d643e768e353af614a55aeed2ba5c))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com//Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [ref #175] make 'mutable' module private ([`a80dbcf`](https://github.com//Byron/gitoxide/commit/a80dbcf083bfcf2e291013f7b13bba9e787c5cb4))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com//Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [ref #175] refactor ([`292e567`](https://github.com//Byron/gitoxide/commit/292e567eaa04a121fb4d7262bb316d37dd8ad11f))
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com//Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com//Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com//Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com//Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [smart-release #171] it's about time we get some tests ([`48a489b`](https://github.com//Byron/gitoxide/commit/48a489b4247ed6feff222924bdcdb53ce45c6ce6))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com//Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
    - [stability #171] mark git-hash and git-actor as ST1 as well ([`32caae1`](https://github.com//Byron/gitoxide/commit/32caae1c32aae38bde59756e52848bef1cef049b))
    - [stability #171] git-ref is now ST1 and available through git-repository ([`50154cd`](https://github.com//Byron/gitoxide/commit/50154cd02fdd90930a1d7c5a4406d53c8067cb4b))
    - [smart-release #171] Try to avoid unstable git-repository features… ([`c8f325b`](https://github.com//Byron/gitoxide/commit/c8f325bed5d644eded035109702098f9fed3fba3))
    - Merge branch 'main' into stability ([`11bae43`](https://github.com//Byron/gitoxide/commit/11bae437e473fef6ed09c178d54ad11eee001b1d))
    - [stability #171] Don't provide access to less stable crates in `Respository` ([`e4c5b58`](https://github.com//Byron/gitoxide/commit/e4c5b58ad935c907dfbd0d61049453dcb64a7e19))
    - cleanup imports ([`e669303`](https://github.com//Byron/gitoxide/commit/e6693032f1391416fd704c21617051ddfb862a3a))
    - [stability #171] Don't leak unstable plumbing crates in git-repository… ([`71eb30f`](https://github.com//Byron/gitoxide/commit/71eb30f1caa41c1f9fe5d2785b71c9d77922c2af))
    - Release git-pack v0.9.0 ([`7fbc961`](https://github.com//Byron/gitoxide/commit/7fbc9617da97d4ba4bb3784f41d4163c0839c03c))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com//Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [repository #164] top-level easy docs ([`6b71c51`](https://github.com//Byron/gitoxide/commit/6b71c51f703aa3b6a7d5a110d04294dd7ea4e8b0))
    - [repository #165] see if `git-config` can already be placed… ([`d287a4a`](https://github.com//Byron/gitoxide/commit/d287a4aec70e5dd33976a25d9a849c44d62d77c9))
    - [repository #165] add limitations along with possible workarouds ([`7578f1e`](https://github.com//Byron/gitoxide/commit/7578f1e2e578010eee087a9176d53a5862ec8862))
    - [repository #165] assure packed-refs are always uptodate ([`a5605df`](https://github.com//Byron/gitoxide/commit/a5605df9b83a25f1726b181b78d751987d71a32b))
    - [repository #165] Allow cloning packed-refs and try to see how it differs… ([`7ec32b7`](https://github.com//Byron/gitoxide/commit/7ec32b7662995b5a60aba1bd932830e68ab1dbdc))
    - Release git-ref v0.6.0 ([`0bb4c13`](https://github.com//Byron/gitoxide/commit/0bb4c133da96f6a96d9f1767848ada792a27c2be))
    - [ref #165] refactor ([`66624c3`](https://github.com//Byron/gitoxide/commit/66624c3ef1faf7048ee86ed73cf5f622802c061e))
    - Revert "[repository #165] PROOF: GATs will work as expected!" ([`853f072`](https://github.com//Byron/gitoxide/commit/853f0723d3d202b1cc2e653109ae92aa14d4d437))
    - [repository #165] PROOF: GATs will work as expected! ([`7f56dbd`](https://github.com//Byron/gitoxide/commit/7f56dbd82db2abc18b8e6d228c8a5f54b3dbf32a))
    - [repository #165] refactor ([`1547d0b`](https://github.com//Byron/gitoxide/commit/1547d0b062e35bad2229dac532e6f30bf105db73))
    - [repository #165] refactor; fine grained allow(missing_docs)… ([`aa0511f`](https://github.com//Byron/gitoxide/commit/aa0511f80f11de8e83fc333e78db369ceb9b2794))
    - [repository #165] prepare for writing light docs for Easy ([`f8834c9`](https://github.com//Byron/gitoxide/commit/f8834c9c8d2ab2ce87857c6773c6204f60df240e))
    - [repository #165] refactor ([`3a0160e`](https://github.com//Byron/gitoxide/commit/3a0160ed1c5bc33d330ad4e9189c4937d194e98d))
    - [repository #165] fmt ([`a02d5aa`](https://github.com//Byron/gitoxide/commit/a02d5aa8ef0e4a1118a9d8523c3f34b836461952))
    - [repository #165] Don't panic on repo borrow error… ([`b2f644a`](https://github.com//Byron/gitoxide/commit/b2f644a73c2b1945ab71c5f5719c9b2b32c01b07))
    - thanks clippy ([`b496d99`](https://github.com//Byron/gitoxide/commit/b496d9952924afdb67e9ba8ea0b9b61c8c8fb1f2))
    - [repository #165] Write about the GAT plan to make this better one day ([`d793ecd`](https://github.com//Byron/gitoxide/commit/d793ecd00f55b5bf7c6dcaee8772975e97bd5e30))
    - [repository #165] quick test to see if Access2 can become Access… ([`45acc7a`](https://github.com//Byron/gitoxide/commit/45acc7a9d6a89977563872c2eac389a2b78b9e27))
    - [repository #165] Generalizing over mutable Repos is possible too… ([`0f7efe3`](https://github.com//Byron/gitoxide/commit/0f7efe3f2e2608213ad5c75b52db876dd4214908))
    - [repository #165] show that Access2 works for all Easy* types… ([`b8ceefe`](https://github.com//Byron/gitoxide/commit/b8ceefed275953aa36d823d51b466cd100729905))
    - [repository #165] First success with creating a shared borrow to the repo ([`f2a38b2`](https://github.com//Byron/gitoxide/commit/f2a38b20aee484e0354d3e2e3db9cc880ae95310))
    - Revert "[repository #165] FAIL Look into `owned_ref` crate" ([`a1443e4`](https://github.com//Byron/gitoxide/commit/a1443e4982fa4d1a1615554a37294d56fd9026eb))
    - [repository #165] FAIL Look into `owned_ref` crate ([`09aa714`](https://github.com//Byron/gitoxide/commit/09aa714f2db5ad220b0e76a65e01e394663f08b4))
    - [repository #165] FAIL AsRef works for basic refs but… ([`02979b6`](https://github.com//Byron/gitoxide/commit/02979b61e6bc4e1de3b3badc784a950477b31cad))
    - [repository #165] FAIL try to generalize with Borrow… ([`295ba95`](https://github.com//Byron/gitoxide/commit/295ba95a341775b566c18e897a2d58a94e6d98f9))
    - [repository #165] FAIL See if EasyExclusive can work… ([`016debb`](https://github.com//Byron/gitoxide/commit/016debbfce7a29502742408da304c80405063230))
    - [repository #165] introduce EasyShared ([`a119ad9`](https://github.com//Byron/gitoxide/commit/a119ad94096a3464b98f6a6bc26c92ba6efa9474))
    - [repository #165] First thoughts about stale caches ([`7f8b63e`](https://github.com//Byron/gitoxide/commit/7f8b63e23ef3561117249668d14507cec1508ad3))
    - [repository #165] hide all easy::State fields behind result-enforcing methods ([`000c537`](https://github.com//Byron/gitoxide/commit/000c537ab766a50679764118af50731b3bab39e5))
    - [repository #165] pack cache access only with errors ([`2353e50`](https://github.com//Byron/gitoxide/commit/2353e5092599228f147ef58c0f0cd45c63c126e2))
    - [repository #165] assure packed-refs is only used non-panicking ([`a355d94`](https://github.com//Byron/gitoxide/commit/a355d943b986307216161bad38e5bb89f8608b49))
    - [repository #165] refactor ([`16fce63`](https://github.com//Byron/gitoxide/commit/16fce637561af29727a8fa025f6ddece853fcc20))
    - [repository #165] a sample of a simpler way to create a tag ([`fb8f584`](https://github.com//Byron/gitoxide/commit/fb8f58412cdd32991a182a41cbc0d463127a4e0e))
    - [smart-release #165] Use generic edit-reference functionality ([`be3e57f`](https://github.com//Byron/gitoxide/commit/be3e57f6221dc87505ba1aad1166e28c328c3b54))
    - [repository #165] sketch generic ref file editing ([`3a026ae`](https://github.com//Byron/gitoxide/commit/3a026aea2a98648a6b624bca9661555f5a147494))
    - [repository #165] refactor ([`00ec15d`](https://github.com//Byron/gitoxide/commit/00ec15dcfdb839095e508139d238df384ea418eb))
    - [repository #165] refactor ([`0f13104`](https://github.com//Byron/gitoxide/commit/0f13104375216ccf099ebc2fcf0d180ed0de5237))
    - [repository #165] An experiment on transforming panics into errors… ([`1f52226`](https://github.com//Byron/gitoxide/commit/1f5222660970e24eb2d82fed3917f234dce7e0eb))
    - [repository #165] offer panicking type conversions for objects ([`f802f8c`](https://github.com//Byron/gitoxide/commit/f802f8c8c382f8063fa615fda022857a740a974a))
    - [repository #165] try a more common naming convention for fallbile things… ([`fc70393`](https://github.com//Byron/gitoxide/commit/fc703937a078937840ea1c254f11e64aaf31de90))
    - [repository #165] refactor ([`6207735`](https://github.com//Byron/gitoxide/commit/6207735f7d955e8a1676c8ad549ce6c1137da760))
    - thanks clippy ([`41d7a44`](https://github.com//Byron/gitoxide/commit/41d7a443aa63b6ee997fd38ceee05b9b1be3e577))
    - [repository #162] cleanup imports ([`983d11a`](https://github.com//Byron/gitoxide/commit/983d11a1f46c1ad21dbf2d57b63ecf979fab48b9))
    - [smart-release #162] use TreeRef capabilities to lookup path ([`51d1943`](https://github.com//Byron/gitoxide/commit/51d19433e6704fabb6547a0ba1b5c32afce43d8b))
    - [repository #162] what could be a correct implementation of a tree path lookup ([`1f638ee`](https://github.com//Byron/gitoxide/commit/1f638eee0aa5f6e1cc34c5bc59a18b5f22af4cbc))
    - [repository #162] detachable ObjectRefs and a few conversions ([`ec123bb`](https://github.com//Byron/gitoxide/commit/ec123bb615035684e52f2d786dfb41d0449823d2))
    - [repository #162] finally let smart-release use the correct abstraction for peeling ([`ba243a3`](https://github.com//Byron/gitoxide/commit/ba243a35ff6f059e5581c6f7ff80e1253ceca6f8))
    - [repository #162] Add id field to ObjectRef… ([`f5ba98e`](https://github.com//Byron/gitoxide/commit/f5ba98ebd0e1d7d0491871be58476cb6882b8436))
    - [repository #162] Make clear that Objects are actually references… ([`d1e6843`](https://github.com//Byron/gitoxide/commit/d1e68435d0b7d9dcc9e0099be3c0c5723dc08e93))
    - [repository #162] another attempt to find a decent peeling abstraction… ([`716d623`](https://github.com//Byron/gitoxide/commit/716d623fb189eb3002d2137827dbfeb143f6ed12))
    - [repository #162] attach the Object to 'Access' ([`9a12564`](https://github.com//Byron/gitoxide/commit/9a125640da19d5633e51df40dee5332eb9600462))
    - [repository #162] refactor ([`a32d361`](https://github.com//Byron/gitoxide/commit/a32d361fd5cb0eb1a4112d834b53c1625372a7bc))
    - [repository #162] trying new names ([`b3f453b`](https://github.com//Byron/gitoxide/commit/b3f453b33f8cda04526110a82f0e0a46a3bb2e34))
    - [repository #162] put impl for finding object data into the extension trait ([`91b9446`](https://github.com//Byron/gitoxide/commit/91b9446fc7035047ebefaa7907e6a8224b56cf27))
    - [repository #162] experiment with finding objects… ([`312a692`](https://github.com//Byron/gitoxide/commit/312a69256a67a0f9d3f3f5c5f9eaf51b50971c5e))
    - thanks clippy ([`f2fb026`](https://github.com//Byron/gitoxide/commit/f2fb0266ba64d002a9913699bcf5843647843beb))
    - [repository #162] Cannot ever store a RefCell Ref in an object… ([`5c17199`](https://github.com//Byron/gitoxide/commit/5c171995383fa9a3698b6aaf3fbd9537110c0299))
    - [repository #162] experiemnt with optionally keeping data in Object ([`b8a8e08`](https://github.com//Byron/gitoxide/commit/b8a8e08e1d972e5069b136c30407c079825b7e1d))
    - [smart-release #162] Object can be used like a git_hash::ObjectId ([`c7bc730`](https://github.com//Byron/gitoxide/commit/c7bc730836f05fe9d967320a6858443a649a59ce))
    - [smart-release #162] format everything ([`8ff83e5`](https://github.com//Byron/gitoxide/commit/8ff83e5c511ae29979348789bd6e7a2f72b16f1c))
    - [smart-release #162] don't throw away work… ([`b43b780`](https://github.com//Byron/gitoxide/commit/b43b780c0382683edc859e3fbd27739716a47141))
    - [smart-release #162] a demo of attaching and detaching objects… ([`ff2927c`](https://github.com//Byron/gitoxide/commit/ff2927ce3fede654d491559fde1c7b07be6a6979))
    - [smart-release #162] an actual Data type… ([`7fd996f`](https://github.com//Byron/gitoxide/commit/7fd996f5f631f83665e81c0f89c34cc47f270d2b))
    - [smart-release #162] unify 'ext' visibility ([`ca082a7`](https://github.com//Byron/gitoxide/commit/ca082a75ff29de2a471cec4331a80f84477cca56))
    - thanks clippy ([`1f2d458`](https://github.com//Byron/gitoxide/commit/1f2d4584f8b650f7e751c8d2df9a5d27725f4f2f))
    - [smart-release #162] a sketch for accessing objects data… ([`ba27101`](https://github.com//Byron/gitoxide/commit/ba27101e08b2bab5d33b53fedcc0c6aa13b8f35e))
    - [smart-release #162] peeling objects to a certain target kind… ([`5785136`](https://github.com//Byron/gitoxide/commit/57851361f3fc729b964fd0ca5dca9f084fe20f5e))
    - [smart-release #162] a single import path for ReferenceExt ([`7060797`](https://github.com//Byron/gitoxide/commit/7060797031e5bdbb8d635cc2da3269996bdfc4cc))
    - [smart-release #162] rename git-repository::object -> objs ([`ac70d81`](https://github.com//Byron/gitoxide/commit/ac70d81791cad04ffdeb04916d7a2a6b533eee6c))
    - [smart-release #162] replace reference peeling with git_easy ([`7cfd5f9`](https://github.com//Byron/gitoxide/commit/7cfd5f9e0a7f828152594f0393a919617c60a9d6))
    - [smart-release #162] smart-release uses Easy repository in 'plumbing' mode ([`4fb672a`](https://github.com//Byron/gitoxide/commit/4fb672a6e7116722577cbbeeee67887871f583bf))
    - [smart-release #162] refactor ([`ef623a6`](https://github.com//Byron/gitoxide/commit/ef623a6835ab86225ac65b933b0df62c00baa1af))
    - [smart-release #162] reduce visibility of Cache ([`397fbfe`](https://github.com//Byron/gitoxide/commit/397fbfe6bde7e03c23b66aa60f70d2e6649f5eef))
    - [smart-release #162] more granular cache control WORKS ([`25dce2a`](https://github.com//Byron/gitoxide/commit/25dce2a4b4522fb9f51fab506dddd8c6ebfb0f54))
    - Revert "[smart-release #162] FAIL: definitely need better granularity" ([`499993f`](https://github.com//Byron/gitoxide/commit/499993fe0b71ac08b3940119bc682533223a3ddb))
    - [smart-release #162] FAIL: definitely need better granularity ([`5f27871`](https://github.com//Byron/gitoxide/commit/5f27871b773c18a9f065a0c8e86101382d23c71f))
    - [smart-release #162] FAIL: promising at first, but not really working… ([`fa01f76`](https://github.com//Byron/gitoxide/commit/fa01f7684c0b7d3b90ec7bde651684a84014a576))
</details>

## v0.7.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.2 ([`c5791b1`](https://github.com//Byron/gitoxide/commit/c5791b1903e91987f2684eaa8d5d8d21255ae40f))
    - [smart-release #162] separate mutable state more cleanly… ([`f00de95`](https://github.com//Byron/gitoxide/commit/f00de9575358dec477667e2e7b5090fb75b46ad6))
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… ([`65db010`](https://github.com//Byron/gitoxide/commit/65db0104146248b273081fc6616a6ed484aa948e))
    - [smart-release #162] a promising lead, this might just work ([`0c4f77b`](https://github.com//Byron/gitoxide/commit/0c4f77b27815d708be4fa6ed26414231f0d51a38))
    - bump git-protocol to v0.9.0 as there are breaking changes ([`b4e3340`](https://github.com//Byron/gitoxide/commit/b4e33408b8eb12c9418704f663322385fd1dfb25))
    - [smart-release #162] a barely working version of refs handling… ([`3e01025`](https://github.com//Byron/gitoxide/commit/3e0102565f0ecdac61e83ed9fb06cc7d788638c7))
    - [smart-release #162] a sign - can't store references, but… ([`7862652`](https://github.com//Byron/gitoxide/commit/7862652fad734a51ead99d6c3988c1bfe92ad2ad))
    - Revert "[smart-release #162] FAIL try to use Rc<RefCell<_>>…" ([`58529a1`](https://github.com//Byron/gitoxide/commit/58529a1e67b77ba1cfe0b794b6ce513162a65139))
    - [smart-release #162] FAIL try to use Rc<RefCell<_>>… ([`180be72`](https://github.com//Byron/gitoxide/commit/180be72d8fd37f326484ebdf99a1e1fc8843958d))
    - [smart-release #162] refactor ([`8f558af`](https://github.com//Byron/gitoxide/commit/8f558afc88276a66c42004e0ac66d89382d83426))
    - thanks clippy ([`b63cd40`](https://github.com//Byron/gitoxide/commit/b63cd40909d02af85f10b77bc40e1630caf355cf))
    - [smart-release #162] refactor ([`35ff637`](https://github.com//Byron/gitoxide/commit/35ff637ab8deaef23a29cfb9bd91f5ea07da7a0c))
    - [smart-release #162] First compiling version, non-threadsafe… ([`d2b2ce9`](https://github.com//Byron/gitoxide/commit/d2b2ce9c1fd78fa63ad24d40eac62f5cbd4f4682))
    - [smart-release #162] FAIL: RefCell as self param also doesn't work :D… ([`ec0c863`](https://github.com//Byron/gitoxide/commit/ec0c8632360e7c4c7ecf02d0915202d616730644))
    - [smart-release #162] back to a more humble, hard-coded approach… ([`bdceb7c`](https://github.com//Byron/gitoxide/commit/bdceb7cf6a3c83536c0a3b0cd5f392040d25bb00))
    - Revert "[smart-release #162] FAIL: cannot use extension traits…" ([`2878a14`](https://github.com//Byron/gitoxide/commit/2878a14613ed1083dd4ff7bc11b09820bade9058))
    - [smart-release #162] FAIL: cannot use extension traits… ([`e115631`](https://github.com//Byron/gitoxide/commit/e1156314f38e998f1b15a49a126382aa2c10022a))
    - [smart-release #162] FAIL: try to do things borrowck doesn't like… ([`853ae9c`](https://github.com//Byron/gitoxide/commit/853ae9cfb12f9ce981d1fa20b9d73d7e3d371f77))
    - [smart-release #162] a sketch of an API that seems to satisfy the constraints… ([`bec8473`](https://github.com//Byron/gitoxide/commit/bec847386a198b4ca5b70bd2a8bf337c007d0501))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com//Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.7.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.1 ([`4369697`](https://github.com//Byron/gitoxide/commit/4369697e6c5f80a899a5e38fa9fe8be44c6504f1))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com//Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com//Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com//Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com//Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - Release git-actor-0.3.1 ([`727087d`](https://github.com//Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com//Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
    - (cargo-release) version 0.6.0 ([`d704bca`](https://github.com//Byron/gitoxide/commit/d704bca7de0a6591f35345c842d6418b36ecd206))
    - (cargo-release) version 0.6.0 ([`4b71e15`](https://github.com//Byron/gitoxide/commit/4b71e15c3ba4a17ff2da5a5ef79986a2832fa3f2))
    - (cargo-release) version 0.5.0 ([`e21142b`](https://github.com//Byron/gitoxide/commit/e21142ba1a113b2afc4725d4d4225dff519c513a))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com//Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
    - (cargo-release) version 0.5.0 ([`c2f94a5`](https://github.com//Byron/gitoxide/commit/c2f94a51bce287be301090450cb00cde57e92f76))
    - (cargo-release) version 0.4.0 ([`d69d0ac`](https://github.com//Byron/gitoxide/commit/d69d0ac21989243fdafa514fa41579fd51bc2558))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com//Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.5.0 ([`1687e59`](https://github.com//Byron/gitoxide/commit/1687e599be98d97925fbab594f31cf5558e9d2b1))
    - (cargo-release) version 0.4.0 ([`28e58f6`](https://github.com//Byron/gitoxide/commit/28e58f6b43a44e010da749a5618df02441f0d2e8))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com//Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com//Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com//Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
</details>

## v0.7.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 63 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 ([`1c5dfb8`](https://github.com//Byron/gitoxide/commit/1c5dfb86028f266435475ca8bdddc57f95002330))
    - (cargo-release) version 0.3.0 ([`0e9c73a`](https://github.com//Byron/gitoxide/commit/0e9c73abd17e0dd21952275077ae53ad7e7aa1af))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com//Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com//Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com//Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com//Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - [repository #149] pre-emptively fix windows ([`b4d3934`](https://github.com//Byron/gitoxide/commit/b4d39345d723981bba1db8d313ef7ec4cd83cc82))
    - [repository #149] only canonicalize if absolutely required ([`d537fac`](https://github.com//Byron/gitoxide/commit/d537fac34e3fb18bd02281f7c74535b59510cff9))
    - [repository #149] canonicalize only when needed ([`57f42bd`](https://github.com//Byron/gitoxide/commit/57f42bdeda1895ca6aba84b58ad44762a17480c2))
    - [repository #149] prepare for canonicalizing only when needed ([`cac9d70`](https://github.com//Byron/gitoxide/commit/cac9d702f62cb2527b9c6357bfcbc9d31da69b01))
    - [repository #149] refactor ([`3c368ec`](https://github.com//Byron/gitoxide/commit/3c368ecb7a07aaff73f0db4432a6184479eb3929))
    - [repository] Fix TreeExt trait name - it's actually for TreeIters ([`f8e0747`](https://github.com//Byron/gitoxide/commit/f8e07475f8867fc98a9264b1270977b48283a94e))
    - Canonicalize path when discovering repositories ([`7bfaa14`](https://github.com//Byron/gitoxide/commit/7bfaa14aca1e96c1998e464971808f67c1c4077f))
    - thanks clippy ([`e1964e4`](https://github.com//Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref] fix build ([`1dcc590`](https://github.com//Byron/gitoxide/commit/1dcc590133ff36e2b2c892b3f51df737a46ccc4c))
    - [ref] refactor ([`e26c72f`](https://github.com//Byron/gitoxide/commit/e26c72fb1bf9392932ffe42843f3dec52c7bbd7d))
    - [ref] and it compiles again, may todos left ([`16618b9`](https://github.com//Byron/gitoxide/commit/16618b916ff67316717d95575fc1344d956d2c49))
    - [ref] fix build ([`83002df`](https://github.com//Byron/gitoxide/commit/83002df0296a431de839ebb3522f57d42a17515f))
    - [ref] rename find_one to 'find' in git-ref… ([`ae7746a`](https://github.com//Byron/gitoxide/commit/ae7746a0815bb94659de67383ba372ac522d53b8))
    - [ref] refactor ([`758c090`](https://github.com//Byron/gitoxide/commit/758c0907df8dc6987f374e326304e0f9fad29812))
    - Revert "[ref] parameterize all uses of hash length…" ([`21f187e`](https://github.com//Byron/gitoxide/commit/21f187e6b7011bb59ed935fc1a2d0a5557890ffe))
    - [ref] parameterize all uses of hash length… ([`5c7285e`](https://github.com//Byron/gitoxide/commit/5c7285e7233390fd7589188084fcd05febcbbac2))
    - [ref] another deletion test succeeds ([`6037900`](https://github.com//Byron/gitoxide/commit/60379001d2729627c042f304217d6459f99f01bf))
    - [ref] file store can ignore all writes; sketch transaction API ([`52a81e9`](https://github.com//Byron/gitoxide/commit/52a81e98f38657023d3eb384fd6db69917dd57ca))
    - [actor] fix gix hours ([`b4e95fd`](https://github.com//Byron/gitoxide/commit/b4e95fdbb6664adcb2603d9cb6e6a69182de050f))
    - (cargo-release) version 0.4.0 ([`4512798`](https://github.com//Byron/gitoxide/commit/45127986daba0a409f5b405d463fa23f5c4a053b))
    - [lock] cleanup signal handling even more… ([`9fb13d2`](https://github.com//Byron/gitoxide/commit/9fb13d27ccce5b0742ee9289fca891dbeb8a65de))
    - (cargo-release) version 0.3.0 ([`92f3a83`](https://github.com//Byron/gitoxide/commit/92f3a830457766c88c68f8424828bfd9b5145f86))
    - (cargo-release) version 0.2.0 ([`7c2eb36`](https://github.com//Byron/gitoxide/commit/7c2eb36274d13646956ac850bee90abbbac91c5b))
    - fix docs ([`e68d460`](https://github.com//Byron/gitoxide/commit/e68d460716dc51c7f4757c11f3c8af6c3881e2cf))
    - fix build ([`dbfa49a`](https://github.com//Byron/gitoxide/commit/dbfa49acf58b2c0763c5e98e5276860b43dfb27b))
    - Remove mentions of interrupt handling feature toggles ([`833ac04`](https://github.com//Byron/gitoxide/commit/833ac0464b42bd3ecc76c6263b4b06e8ab4ff182))
    - Fix everything up so that… ([`5930563`](https://github.com//Byron/gitoxide/commit/5930563601d6c2148cf39e109f69f8b7c7dfcb36))
    - A first attempt to make intrerupt tools work, but… ([`8fb8d37`](https://github.com//Byron/gitoxide/commit/8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5))
    - First step towards moving git-features::interrupt… ([`8a741d0`](https://github.com//Byron/gitoxide/commit/8a741d0c5423ed7c35d9382307c760a6b9460ccd))
    - [pack] add --statistics flag to pack-create ([`51a3077`](https://github.com//Byron/gitoxide/commit/51a307730b8514acffa75c78ecca3f02b1eb467b))
    - [async-client] frame for async connect ([`9ada080`](https://github.com//Byron/gitoxide/commit/9ada0805fc5896f8ef1a31dc821b789b7f0438a6))
    - Separate networking via feature toggles and pass that through in the main crate ([`2c749f1`](https://github.com//Byron/gitoxide/commit/2c749f10dd03ea0b027fb046e8c40c77869fb2e9))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com//Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com//Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com//Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
</details>

## v0.6.0 (2021-05-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`d35c55d`](https://github.com//Byron/gitoxide/commit/d35c55d8ff4b52e25befb8bff839d805b9f3caf4))
    - [git-repository] better docs ([`f60a7c5`](https://github.com//Byron/gitoxide/commit/f60a7c567a2ae856840b276479582b87bb0530f5))
    - [git-repository] gitoxide-core uses more of git-repository ([`bb5b074`](https://github.com//Byron/gitoxide/commit/bb5b0747dfd3a3985a904b7748f296a591fcb26e))
    - [git-repository] replaces git-features and git-protocol in gitoxide-core ([`081d20f`](https://github.com//Byron/gitoxide/commit/081d20f927f222daa69f2a1a492957fd3146bfc1))
    - [git-repository] used by gix-hours ([`24e0258`](https://github.com//Byron/gitoxide/commit/24e0258b9691b82df5c35a35111d19df56087cdc))
    - [git-repository] refactor ([`b5ebcfa`](https://github.com//Byron/gitoxide/commit/b5ebcfa278a0be85ea10893fd40a8b3e2e28efd5))
    - [git-repository] now used by gixp-organize ([`aa91fad`](https://github.com//Byron/gitoxide/commit/aa91fad3cf237f6d6f9d588ed390baa6e55f6540))
    - [git-repository] make it easy to get maximum performance in apps using this crate ([`dc150a5`](https://github.com//Byron/gitoxide/commit/dc150a5913ac5db6211c5881873254bc8377aad2))
    - [git-repository] prevent other implementations of extension traits; refactor ([`e14df75`](https://github.com//Byron/gitoxide/commit/e14df75fa999508a1e3102add4829ba55ec3aa50))
    - [git-repository] finish 'diffing' program upgrade ([`7eea39a`](https://github.com//Byron/gitoxide/commit/7eea39a8d945f28b376698af9b1a0f67ffaa7e6f))
    - [git-repository] more details on how this crate is intended ([`cd85050`](https://github.com//Byron/gitoxide/commit/cd85050a506ef99192909db6d8373a99282df53d))
    - [git-repository] refactor ([`b9f4d25`](https://github.com//Byron/gitoxide/commit/b9f4d25ae80c3dc6e03b734202eae44d444cb442))
    - [git-repository] try out an API for ancestor iteration ([`de0b5bb`](https://github.com//Byron/gitoxide/commit/de0b5bbe71ce8cfb49665b4f7e429d719dcb08dd))
    - [git-repository] the first extension trait for more convenience ([`63a1fee`](https://github.com//Byron/gitoxide/commit/63a1fee9195c9d3c23001e09cccece2b2af43324))
    - [git-repository] now with a prelude for traits ([`7f7a5ea`](https://github.com//Byron/gitoxide/commit/7f7a5eaf080217628b3645af3ff5f1872d5ce11c))
    - [git-repository] more re-exports for convenience ([`6a5c00e`](https://github.com//Byron/gitoxide/commit/6a5c00e2e1fb7ca911d1f8ce3534a74316478149))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com//Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com//Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - [git-repository] repo-init sketch ([`5855c95`](https://github.com//Byron/gitoxide/commit/5855c952e2703412a5f7c1ffbfe57b85f339bab1))
    - [git-repository] refactor ([`63c22af`](https://github.com//Byron/gitoxide/commit/63c22afe153b08453c3c12c3bb81626d2381f472))
    - [git-repository] refactor ([`996944a`](https://github.com//Byron/gitoxide/commit/996944a75160538588d34385b6a6717b05ee9c47))
    - [git-repository] refactor ([`a2d58c1`](https://github.com//Byron/gitoxide/commit/a2d58c100ca696bceaaa0788347bba41f29ab0b8))
    - [git-repository] a sketch of how the repository could look like ([`3854cef`](https://github.com//Byron/gitoxide/commit/3854cef47205e449bfc638255eefe303a99897d8))
    - [git-repository] traversal uses git-repository ([`db564c5`](https://github.com//Byron/gitoxide/commit/db564c5016272ff6d2038fd2b554cb6dacb0a6c5))
    - [git-repository] an actual repository abstraction ([`3f20b26`](https://github.com//Byron/gitoxide/commit/3f20b267b97f0855d958a37b36984da288263cc2))
    - [git-repository] refactor ([`c8323e4`](https://github.com//Byron/gitoxide/commit/c8323e484f08d5ea59400636cb26334d6976e4c0))
    - [git-repository] traversal program uses new facilities, and it's cumbersome ([`29ea2de`](https://github.com//Byron/gitoxide/commit/29ea2de9ad48036f78d3776d8526d959f68bf287))
    - [git-repository] bare repository handling ([`3a8e6ff`](https://github.com//Byron/gitoxide/commit/3a8e6ff041efc57482252458acf379b43ef6b523))
    - [git-repository] tests pass, bare repo tests missing ([`a5ed9ea`](https://github.com//Byron/gitoxide/commit/a5ed9ea3004f81c2132b86fe26ad34abf620c765))
    - [git-repository] most of the git repository discovery ([`72a49c8`](https://github.com//Byron/gitoxide/commit/72a49c816253520230a04290619f469df608be19))
    - [git-repository] frame for repository testing; sketch of discovery API ([`467e340`](https://github.com//Byron/gitoxide/commit/467e340b6c36cad299d35546a60cdb308e29b289))
</details>

## v0.5.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 204 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`02df134`](https://github.com//Byron/gitoxide/commit/02df1345a22889a573adfc1be80bda271b2dc9a5))
    - refactor ([`170215d`](https://github.com//Byron/gitoxide/commit/170215dc941af9b6a8f19c1fef91f3b5802e1cc7))
    - Ensured linter checks pass ([`51f2183`](https://github.com//Byron/gitoxide/commit/51f2183357573f9ea30dffbf61af73d5e845f5aa))
    - Ensured output of directory-less git init unchanged ([`539a573`](https://github.com//Byron/gitoxide/commit/539a5737459de10404b6ba6f06a20224b6d534af))
    - Added [directory] argument to init. ([`62f8dc6`](https://github.com//Byron/gitoxide/commit/62f8dc62ec3e76efd7311ced32094035856dbcbb))
    - Spelling fix in error message ([`944d0f4`](https://github.com//Byron/gitoxide/commit/944d0f4ae830c8f2e7eabe3bd58cd023f5674ce1))
    - remove dash in all repository links ([`98c1360`](https://github.com//Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - refactor ([`ba1d883`](https://github.com//Byron/gitoxide/commit/ba1d88364424eb60a0874a5726b62740dc348592))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 28 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`2b1bca8`](https://github.com//Byron/gitoxide/commit/2b1bca83c453544972e370dc0adff57cb7590b42))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com//Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com//Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - update to quick-error 2.0 ([`4b1b784`](https://github.com//Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
    - Switch to latest quick-error ([`9760856`](https://github.com//Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - refactor ([`2888f1b`](https://github.com//Byron/gitoxide/commit/2888f1b10a2baf40155544e667ddd461f3ddc938))
    - explicitly include assets in git-repository crate ([`9da6071`](https://github.com//Byron/gitoxide/commit/9da6071c97d668e5af4eedb554ca8f91d184ee7e))
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 17 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable ([`5688a34`](https://github.com//Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - Fix tests ([`59ed51d`](https://github.com//Byron/gitoxide/commit/59ed51d0c84bf067ef0a921730260f2c444e5409))
    - Use 'main' branches instead of the previous default when initializing a repository ([`da77cc8`](https://github.com//Byron/gitoxide/commit/da77cc807f34d23da76e4d94e4220ed638713f59))
    - Allow for more screen space when formatting ([`6794300`](https://github.com//Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - goodbye git-core, hello git-repository ([`7cec2b6`](https://github.com//Byron/gitoxide/commit/7cec2b648f86fc665b4fc5bfe269e9ca16679a55))
</details>

