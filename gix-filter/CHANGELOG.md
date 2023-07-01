# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.0.0 (2023-07-01)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

An empty crate without any content to reserve the name for the gitoxide project.

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

### New Features

 - <csr-id-198ce27e05fd07d4e511dff1ac50c3f4ed163cdf/> API support for receiving delayed entries
 - <csr-id-5670bbba73c7d48ba6220c2e3773633c234fb21c/> Ability to steer long running filter processes.
 - <csr-id-c538c6eba466f22a3000a76a0d37902ac5256e7a/> ability to run define and run simple filters.
   Simple filters run in real-time and are piped their content to stdin
   while we read it from stdout.
 - <csr-id-d1fed3e9907d0a9e3fe45dbfe2ff27bd10b3e1f4/> `worktree::encode_to_worktree()` to turn UTf-8 into the worktree encoding.
 - <csr-id-1b8f2b7f51e7d17b9b0839f42b75781ae6f940ec/> `worktree::encode_to_git()` to turn worktree encoding to UTF-8.
 - <csr-id-1517cbc42c43b253046b7359c79731771fd7b941/> add `eol::convert_to_worktree()`.
   It's the inverse of `eol::convert_to_git()` to re-add CRLF where there were LF only.
 - <csr-id-e45fec9663f87b7ba4162a9517677f6278c20a98/> Add `eol::convert_to_git()`.
   This function supports all the logic that git executes to determine
   if a converion should actually be done.
 - <csr-id-b79ffeb9ed584c47f2609eea261e1ada557a744c/> `eol::Stats::from_bytes()` to obtain stats about a buffer.
   It can help to determine if it is binary and if line conversions should be performed at all.
 - <csr-id-306c8eabcffe80da1d627283c4b188a1b979f692/> add `ident::apply()` to substitute `$Id$` with `$Id: <hex>$`
 - <csr-id-496445ca97687a38ecb80e871a1cbdc7ecd6b313/> `ident::undo()` to replace `$Id: XXX$` with `$Id$`

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 40 commits contributed to the release over the course of 438 calendar days.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#691](https://github.com/Byron/gitoxide/issues/691)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - Add git-filter crate for name generation ([`5a3c628`](https://github.com/Byron/gitoxide/commit/5a3c628c757a7eb4ecfc5fd7265fa36c8362797b))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - Set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Use latest version of `gix-path` to allow `gix-filter` release. ([`bb9f308`](https://github.com/Byron/gitoxide/commit/bb9f3088aee60f140e960602429cc7eb46121b75))
    - Release gix-packetline-blocking v0.16.3, gix-filter v0.0.0 ([`fb3ad29`](https://github.com/Byron/gitoxide/commit/fb3ad29967d08558e42cbe8e80de5dd0b38f12c5))
    - Merge branch 'filter-programs' ([`97f8e96`](https://github.com/Byron/gitoxide/commit/97f8e960ed52538bb55b72f9dfc5f9d144d72885))
    - API support for receiving delayed entries ([`198ce27`](https://github.com/Byron/gitoxide/commit/198ce27e05fd07d4e511dff1ac50c3f4ed163cdf))
    - Handle process error codes similarly to how git does it ([`d00e6c5`](https://github.com/Byron/gitoxide/commit/d00e6c569a4320e334ef15a82867433497dc235e))
    - Make it possible to not run a process at all and test graceful shutdowns ([`cb74130`](https://github.com/Byron/gitoxide/commit/cb7413062e3ef46c896f552c889bd7984a35bd03))
    - Ability to steer long running filter processes. ([`5670bbb`](https://github.com/Byron/gitoxide/commit/5670bbba73c7d48ba6220c2e3773633c234fb21c))
    - Ability to run define and run simple filters. ([`c538c6e`](https://github.com/Byron/gitoxide/commit/c538c6eba466f22a3000a76a0d37902ac5256e7a))
    - Release gix-date v0.7.0, gix-trace v0.1.2, gix-actor v0.23.0, gix-commitgraph v0.17.1, gix-utils v0.1.4, gix-object v0.32.0, gix-ref v0.32.0, gix-config v0.25.0, gix-diff v0.32.0, gix-discover v0.21.0, gix-hashtable v0.2.3, gix-revwalk v0.3.0, gix-traverse v0.29.0, gix-index v0.20.0, gix-mailmap v0.15.0, gix-negotiate v0.4.0, gix-pack v0.39.0, gix-odb v0.49.0, gix-protocol v0.35.0, gix-revision v0.17.0, gix-refspec v0.13.0, gix-worktree v0.21.0, gix v0.48.0, safety bump 20 crates ([`27e8c18`](https://github.com/Byron/gitoxide/commit/27e8c18db5a9a21843381c116a8ed6d9f681b3f8))
    - Merge branch 'worktree-encoding' ([`5af2cf3`](https://github.com/Byron/gitoxide/commit/5af2cf368dcd05fe4dffbd675cffe6bafec127e7))
    - `worktree::encode_to_worktree()` to turn UTf-8 into the worktree encoding. ([`d1fed3e`](https://github.com/Byron/gitoxide/commit/d1fed3e9907d0a9e3fe45dbfe2ff27bd10b3e1f4))
    - `worktree::encode_to_git()` to turn worktree encoding to UTF-8. ([`1b8f2b7`](https://github.com/Byron/gitoxide/commit/1b8f2b7f51e7d17b9b0839f42b75781ae6f940ec))
    - Refactor ([`7ae7ebd`](https://github.com/Byron/gitoxide/commit/7ae7ebd673b7062f7e4116e9ae4fc51a1451e34b))
    - Merge branch 'basic-filtering' ([`3fd5e16`](https://github.com/Byron/gitoxide/commit/3fd5e16e205db18edc21341fb4c2a75d0726f5a5))
    - Add `eol::convert_to_worktree()`. ([`1517cbc`](https://github.com/Byron/gitoxide/commit/1517cbc42c43b253046b7359c79731771fd7b941))
    - Add `eol::convert_to_git()`. ([`e45fec9`](https://github.com/Byron/gitoxide/commit/e45fec9663f87b7ba4162a9517677f6278c20a98))
    - `eol::Stats::from_bytes()` to obtain stats about a buffer. ([`b79ffeb`](https://github.com/Byron/gitoxide/commit/b79ffeb9ed584c47f2609eea261e1ada557a744c))
    - Refactor ([`9bb9c48`](https://github.com/Byron/gitoxide/commit/9bb9c48e0c935179885b774cd685bcaf1008c043))
    - Add `ident::apply()` to substitute `$Id$` with `$Id: <hex>$` ([`306c8ea`](https://github.com/Byron/gitoxide/commit/306c8eabcffe80da1d627283c4b188a1b979f692))
    - `ident::undo()` to replace `$Id: XXX$` with `$Id$` ([`496445c`](https://github.com/Byron/gitoxide/commit/496445ca97687a38ecb80e871a1cbdc7ecd6b313))
    - Merge branch 'corpus' ([`aa16c8c`](https://github.com/Byron/gitoxide/commit/aa16c8ce91452a3e3063cf1cf0240b6014c4743f))
    - Change MSRV to 1.65 ([`4f635fc`](https://github.com/Byron/gitoxide/commit/4f635fc4429350bae2582d25de86429969d28f30))
    - Merge branch 'main' into auto-clippy ([`3ef5c90`](https://github.com/Byron/gitoxide/commit/3ef5c90aebce23385815f1df674c1d28d58b4b0d))
    - Merge branch 'blinxen/main' ([`9375cd7`](https://github.com/Byron/gitoxide/commit/9375cd75b01aa22a0e2eed6305fe45fabfd6c1ac))
    - Include license files in all crates ([`facaaf6`](https://github.com/Byron/gitoxide/commit/facaaf633f01c857dcf2572c6dbe0a92b7105c1c))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - Rename `git-filter` to `gix-filter` ([`0875ae6`](https://github.com/Byron/gitoxide/commit/0875ae61d9e2e7553ea990ea1c391a26f0eb1a59))
    - Rename `git-filter` to `gix-filter` ([`4aa193f`](https://github.com/Byron/gitoxide/commit/4aa193f359ba31fc6ca7c3c28654e08b12ace6b3))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - Upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - Uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - Remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Release git-filter v0.0.0 ([`2465381`](https://github.com/Byron/gitoxide/commit/2465381626a1f6de58f45df2e68f36c6b585b68f))
</details>

