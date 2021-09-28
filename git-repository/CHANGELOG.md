# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Breaking

- Change return value of `prelude::RepositoryAccessExt::committer()` from `git_actor::Signature` to `Result<git_actor::Signature, easy::borrow:repo::Error>`
- Change return value of `prelude::ReferenceAccessExt` from `Result<Vec<RefEdit>>, _>` to `Result<easy::Reference, _>`.
- Rename `State` structs that serve as platform for iterators or other dependent types into `Platform`. These are usually intermediate objects only.
- Rename `easy::Reference::log()` into `easy::Reference::logs()`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 74 commits contributed to the release over the course of 16 calendar days.
 - 34 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **#164**
    - path::is (1f4e45a)
    - rename path::is_git to path::is (ac3b9ef)
    - path::discover (1958e8a)
    - Avoid duplicate module paths in 'tree' and 'commit' (2f2d856)
    - top-level of 'path' module (066f59b)
    - object_id (329d183)
    - rename ObjectIdExt::ancestors_iter() to *::ancestors() (a19567e)
    - repository (1a1959f)
    - ext::tree (5e091fb)
    - easy::object::peel (e376067)
    - easy::object::errors (de004b3)
    - rename `easy::Object::to_(commit|tag)_iter()`… (61793ff)
    - easy::object, sans a few child-modules (f582439)
    - update 'platform' information to reflect the current usage (42080ae)
    - rename easy::reference::log::State to easy::reference::Logs (03fe8a7)
    - rename `*::State` into `*::Platform` (0cd585e)
 * **#198**
    - greatly reduce changelog size now that the traversal fix is applied (3924c03)
    - Use hashmap based lookup for trees… (55d2d17)
    - Fixup remaining changelogs… (0ac488a)
    - Generate changelogs with details (fd0f3bd)
    - Update all changelogs with details (0732699)
    - Update changelogs (b30db3b)
    - Avoid adding newlines which make writing unstable (6b5c394)
    - Fix section headline level (9d6f263)
    - Write first version of changlogs thus far… (719b6bd)
    - Use 'to_*' when converting `easy::Object` to specific object kind (1cb41f8)
    - Fix panic related to incorrect handling of character boundaries (9e92cff)
    - Fix build (d0a956f)
    - refactor!: Use git_object::commit::MessageRef::summary()… (13e7c3a)
    - Sketch data for parsed messages (32dd280)
    - smart-release: a seemingly slow version of path lookup, but… (41afad3)
    - configure caches with env vars using `apply_environment()` (d422b9a)
    - refactor (e7c061b)
    - set package cache via RepositoryAccessExt (66292fd)
    - smart-release(feat): Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size… (5aadf75)
    - allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE (d79a1b7)
    - prepare for configurable pack cache (7d2b6b6)
    - object-cache to allow for a speed boost… (06996e0)
    - smart-release: build commit history for later use in changelog generation (daec716)
    - Allow object access during commit ancestor traversal… (4fe4786)
    - smart-release: sketch history acquisition (debe009)
    - various small API changes (89f1505)
    - add 'Head::peeled()' method (56e39fa)
    - move easy::head::peel::Error -> easy::head::peel::to_id::Error (f644d0e)
    - loose reference iteration with non-dir prefixes… (293bfc0)
    - Add 'references().all().peeled().'… (6502412)
    - smart-release: filter refs correctly, but… (2b4a615)
 * **#200**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… (f293b63)
 * **#67**
    - split data::output::count::objects into files (8fe4612)
    - use new git_pack::cache::Object trait (b209da2)
    - remove object cache impl which now lives in git-pack (741558d)
    - Use Easy in the one spot where it is possible… (6a97bfa)
    - try to create persistent Easy iterator, but can't make it Send… (54a64a5)
 * **Uncategorized**
    - Merge branch 'main' into changelog-generation (c956f33)
    - thanks clippy (ae7826e)
    - thanks clippy (b02edb5)
    - thanks clippy (68ea77d)
    - improved changelog… (8b82f7d)
    - Bump git-traverse v0.9.0, safety bump 8 crates (d39fabb)
    - Bump git-repository v0.10.0 (5a10dde)
    - [repository #164] docs for easy::reference::log (7de7c7e)
    - [repository #164] docs for easy::reference::iter (d86c713)
    - [repository #164] refactor (437e63b)
    - [repository #164] docs for top-level of easy::reference (9e465e0)
    - [repository #164] docs for easy::oid (b66b6fe)
    - [repository #164] docs for easy::commit and easy::odb (abf37e5)
    - [repository #164] Documentation for `easy::borrow` (3e612f4)
    - [repository #164] docs for easy::head::* (516fde7)
    - [repository #164] refactor (65b0e0f)
    - [repository #164] docs for `easy::ext::ReferenceAccessExt` (ab4910f)
    - [repository #164] docs for easy::ext::RepositoryAccessExt (9041d47)
    - [repository #164] another test and fix for `commit()` (8d676d7)
    - [repository #164] easy::ext::ObjectAccessExt docs (c4984af)
    - [repository #164] (4111d22)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.9.1 (2021-09-10)

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.9.1 (262c122)
    - Release git-ref v0.7.3 (b0a9815)
    - [repository] don't enforce feature flags that may fail on windows by default (afdec2e)
    - Release git-ref v0.7.2 (e940e9a)
    - Release git-protocol v0.10.4 (898ee08)
    - Release git-odb v0.21.3 (223f930)
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
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.11.0 (5ae6ff5)
    - Bump git-repository v0.9.0 (b797fc1)
    - [repository #193] Add feature flags for async/blocking (57f482c)
    - Bump git-object v0.14.0 (d4fc81f)
    - [repository #164] Prepare `commit()` for a possible less-allocating future (0fd01f7)
    - [repository #164] Support for refreshing the object database (46e10f8)
    - [odb #164] Add refresh() functionality (ee16d04)
</details>

## v0.8.2 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 66 commits contributed to the release over the course of 8 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.2 (3fc23be)
    - [repository #190] test for oid.ancestors().all() (fdc3678)
    - [repository #190] fix build, lets just make traversal available by default (6da3599)
    - Bump git-pack v0.10.0 (e5e3c80)
    - [repository #190] access to repository directories (f4d1ec4)
    - [repository #190] first shot at ancestor iteration… (85f1a48)
    - [repository #190] refactor (e7188e0)
    - [ref #190] fix tests (e426e15)
    - [repository #190] fix tests; needs inbound transaction handling… (e5a5c09)
    - [repository #190] leverage git-ref namespace support (1aa9c11)
    - [repository #190] refactor (609c249)
    - [repository #190] fix build (f5e118c)
    - [repository #190] note a known limitation about finding references in namespaces… (d335731)
    - [repository #190] transparent namespace support (d14f073)
    - [repository #190] turns out we need bstr with unicode support (3d8796e)
    - [repository #190] public bstr re-export (3b7ffde)
    - [repository #190] cleanup usage of bstr… (e4411ff)
    - [repository #190] prefixed reference iteration (a6e19c9)
    - [repository #190] implementation of reference iteration (all() for now)… (2c0939a)
    - [repository #190] refactor (8c532a4)
    - [repository #190] prepare reference iteration (427f146)
    - Bump git-hash v0.6.0 (6efd90d)
    - [repository #190] obtain the kind fo hash used in a repo (a985491)
    - [repository #190] refactor (7a111b1)
    - [repository #190] shortcut to create references (28afd8e)
    - [ref #190] add forward log iter and localize iter types… (c3e240d)
    - [repository #190] refactor (e751688)
    - thanks clippy (023dedc)
    - [ref #190] reverse reflog ergonomics (2de86f9)
    - [repository #190] ref log for HEAD specifically (946bbf1)
    - [repository #190] reflog tests (641edde)
    - [ref #190] First working sketch of reverse log iter access (4a36ded)
    - [ref #190] move remaining file store functions to extension trait (60fc215)
    - thanks clippy (376c045)
    - [repository #190] refactor (15d4ac8)
    - [repository #190] a major step forward with `head()` access (43ac4f5)
    - [ref #190] cache peeled objects properly (2cb511e)
    - Bump git-ref v0.7.0 (ac4413c)
    - [repository #190] experiment with 'HEAD' API… (c55ce4d)
    - thanks clippy (14dff63)
    - [ref #190] Use Raw Reference everywhere for great simplification… (7aeea9c)
    - [repository #190] refactor (d6bef3a)
    - [ref #190] introduce Raw reference type that simplifies everything… (8634341)
    - [ref #190] refactor (07126d6)
    - [ref #190] Allow for explicit expected previous values (1a4786f)
    - [repository #190] show that unconditional creation of references doesn't is lacking… (06b9270)
    - [repository #190] another commit() test… (4ec631c)
    - [repository #190] produce nice reflog messages (e7a8b62)
    - [repository #190] commit::summary() (43f7568)
    - [repository #190] thanks clippy (0763ac2)
    - [repository #190] first version of 'commit(…)' without reflog message handling (bfcf8f1)
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly (63bedc7)
    - [repository #190] put git-lock into ST1… (26a6637)
    - [repository #190] refactor (1e029b4)
    - [repository #190] A way to write objects and the empty tree specifically (7c559d6)
    - [various #190] rename 'local-offset' to 'local-time-support' (3a7d379)
    - [repository #190] Make local-offset available on demand only… (1927be7)
    - [repository #185] rustfmt (dfbb015)
    - [repository #185] remove quick-error infavor of thiserror (212c44c)
    - [repository #185] on the way to removing quick-error (6ecd431)
    - [repository #185] support for initializing bare repositories (9e8a39e)
    - [repository #185] use git-config to handle bare repos more properly (8a5aac5)
    - [repository #185] sketch of how to open a repository… (48207b5)
    - [repository #185] refactor (63089ff)
    - [repository #185] refactor (7604935)
    - [repository #185] refactor repository initialization… (5ff7eaa)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

## v0.7.2 (2021-08-17)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.2 (c5791b1)
    - [smart-release #162] separate mutable state more cleanly… (f00de95)
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… (65db010)
    - [smart-release #162] a promising lead, this might just work (0c4f77b)
    - bump git-protocol to v0.9.0 as there are breaking changes (b4e3340)
    - [smart-release #162] a barely working version of refs handling… (3e01025)
    - [smart-release #162] a sign - can't store references, but… (7862652)
    - Revert "[smart-release #162] FAIL try to use Rc<RefCell<_>>…" (58529a1)
    - [smart-release #162] FAIL try to use Rc<RefCell<_>>… (180be72)
    - [smart-release #162] refactor (8f558af)
    - thanks clippy (b63cd40)
    - [smart-release #162] refactor (35ff637)
    - [smart-release #162] First compiling version, non-threadsafe… (d2b2ce9)
    - [smart-release #162] FAIL: RefCell as self param also doesn't work :D… (ec0c863)
    - [smart-release #162] back to a more humble, hard-coded approach… (bdceb7c)
    - Revert "[smart-release #162] FAIL: cannot use extension traits…" (2878a14)
    - [smart-release #162] FAIL: cannot use extension traits… (e115631)
    - [smart-release #162] FAIL: try to do things borrowck doesn't like… (853ae9c)
    - [smart-release #162] a sketch of an API that seems to satisfy the constraints… (bec8473)
    - Apply nightly rustfmt rules. (5e0edba)
</details>

## v0.7.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.1 (4369697)
    - remove dev-dependency cycles by removing their version (c40faca)
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 (f123f69)
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 (c67291f)
    - Release git-object v0.12.0 (7006150)
    - Release git-actor-0.3.1 (727087d)
    - (cargo-release) version 0.18.0 (b327590)
    - (cargo-release) version 0.6.0 (d704bca)
    - (cargo-release) version 0.6.0 (4b71e15)
    - (cargo-release) version 0.5.0 (e21142b)
    - (cargo-release) version 0.17.0 (c52a491)
    - (cargo-release) version 0.5.0 (c2f94a5)
    - (cargo-release) version 0.4.0 (d69d0ac)
    - (cargo-release) version 0.6.0 (d58f37e)
    - (cargo-release) version 0.5.0 (1687e59)
    - (cargo-release) version 0.4.0 (28e58f6)
    - (cargo-release) version 0.11.0 (a5be31c)
    - (cargo-release) version 0.3.0 (64efc05)
    - (cargo-release) version 0.4.0 (70ef344)
</details>

## v0.7.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 63 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 (1c5dfb8)
    - (cargo-release) version 0.3.0 (0e9c73a)
    - (cargo-release) version 0.5.0 (ae02dab)
    - (cargo-release) version 0.16.0 (1231dbd)
    - (cargo-release) version 0.5.0 (0e11e98)
    - (cargo-release) version 0.2.0 (8ff5115)
    - [repository #149] pre-emptively fix windows (b4d3934)
    - [repository #149] only canonicalize if absolutely required (d537fac)
    - [repository #149] canonicalize only when needed (57f42bd)
    - [repository #149] prepare for canonicalizing only when needed (cac9d70)
    - [repository #149] refactor (3c368ec)
    - [repository] Fix TreeExt trait name - it's actually for TreeIters (f8e0747)
    - Canonicalize path when discovering repositories (7bfaa14)
    - thanks clippy (e1964e4)
    - [ref] fix build (1dcc590)
    - [ref] refactor (e26c72f)
    - [ref] and it compiles again, may todos left (16618b9)
    - [ref] fix build (83002df)
    - [ref] rename find_one to 'find' in git-ref… (ae7746a)
    - [ref] refactor (758c090)
    - Revert "[ref] parameterize all uses of hash length…" (21f187e)
    - [ref] parameterize all uses of hash length… (5c7285e)
    - [ref] another deletion test succeeds (6037900)
    - [ref] file store can ignore all writes; sketch transaction API (52a81e9)
    - [actor] fix gix hours (b4e95fd)
    - (cargo-release) version 0.4.0 (4512798)
    - [lock] cleanup signal handling even more… (9fb13d2)
    - (cargo-release) version 0.3.0 (92f3a83)
    - (cargo-release) version 0.2.0 (7c2eb36)
    - fix docs (e68d460)
    - fix build (dbfa49a)
    - Remove mentions of interrupt handling feature toggles (833ac04)
    - Fix everything up so that… (5930563)
    - A first attempt to make intrerupt tools work, but… (8fb8d37)
    - First step towards moving git-features::interrupt… (8a741d0)
    - [pack] add --statistics flag to pack-create (51a3077)
    - [async-client] frame for async connect (9ada080)
    - Separate networking via feature toggles and pass that through in the main crate (2c749f1)
    - (cargo-release) version 0.3.0 (6b33678)
    - Merge branch 'dependabot/cargo/crc-2.0.0' (683c44d)
    - (cargo-release) version 0.2.0 (3286e42)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.6.0 (2021-05-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 (d35c55d)
    - [git-repository] better docs (f60a7c5)
    - [git-repository] gitoxide-core uses more of git-repository (bb5b074)
    - [git-repository] replaces git-features and git-protocol in gitoxide-core (081d20f)
    - [git-repository] used by gix-hours (24e0258)
    - [git-repository] refactor (b5ebcfa)
    - [git-repository] now used by gixp-organize (aa91fad)
    - [git-repository] make it easy to get maximum performance in apps using this crate (dc150a5)
    - [git-repository] prevent other implementations of extension traits; refactor (e14df75)
    - [git-repository] finish 'diffing' program upgrade (7eea39a)
    - [git-repository] more details on how this crate is intended (cd85050)
    - [git-repository] refactor (b9f4d25)
    - [git-repository] try out an API for ancestor iteration (de0b5bb)
    - [git-repository] the first extension trait for more convenience (63a1fee)
    - [git-repository] now with a prelude for traits (7f7a5ea)
    - [git-repository] more re-exports for convenience (6a5c00e)
    - (cargo-release) version 0.4.0 (866f86f)
    - [git-repository] towards git-repository as one stop shop (aea6cc5)
    - [git-repository] repo-init sketch (5855c95)
    - [git-repository] refactor (63c22af)
    - [git-repository] refactor (996944a)
    - [git-repository] refactor (a2d58c1)
    - [git-repository] a sketch of how the repository could look like (3854cef)
    - [git-repository] traversal uses git-repository (db564c5)
    - [git-repository] an actual repository abstraction (3f20b26)
    - [git-repository] refactor (c8323e4)
    - [git-repository] traversal program uses new facilities, and it's cumbersome (29ea2de)
    - [git-repository] bare repository handling (3a8e6ff)
    - [git-repository] tests pass, bare repo tests missing (a5ed9ea)
    - [git-repository] most of the git repository discovery (72a49c8)
    - [git-repository] frame for repository testing; sketch of discovery API (467e340)
</details>

## v0.5.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 204 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 (02df134)
    - refactor (170215d)
    - Ensured linter checks pass (51f2183)
    - Ensured output of directory-less git init unchanged (539a573)
    - Added [directory] argument to init. (62f8dc6)
    - Spelling fix in error message (944d0f4)
    - remove dash in all repository links (98c1360)
    - refactor (ba1d883)
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 28 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 (2b1bca8)
    - Allow dual-licensing with Apache 2.0 (ea353eb)
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 31 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 (4351e28)
    - update to quick-error 2.0 (4b1b784)
    - Switch to latest quick-error (9760856)
    - refactor (2888f1b)
    - explicitly include assets in git-repository crate (9da6071)
</details>

## v0.1.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 17 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Make crates publishable (5688a34)
    - Fix tests (59ed51d)
    - Use 'main' branches instead of the previous default when initializing a repository (da77cc8)
    - Allow for more screen space when formatting (6794300)
    - goodbye git-core, hello git-repository (7cec2b6)
</details>

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.1 (b269a12)
    - [repository #164] make EasyArcExclusive available (2fa3dcb)
</details>

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies
### Commit Statistics

<csr-read-only-do-not-edit/>

 - 117 commits contributed to the release over the course of 10 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] keep assets (e0fca77)
    - [repository #174] remove arc_lock code entirely (dcbe742)
    - [repository #174] conditionally compile future parking_lot version… (5375fc8)
    - Bump git-repository v0.8.0 (cdb45ff)
    - [repository #174] adjust various changelogs (081faf5)
    - Bump git-protocol v0.10.0 (82d5a0b)
    - Bump git-odb v0.21.0 (7b9854f)
    - [pack #179] refactor (ab6554b)
    - [packetline #178] fix compile warnings (c8d2e72)
    - Bump git-traverse v0.8.0 (54f3541)
    - Bump git-diff v0.9.0 (2e2e798)
    - [object #177] cleanup CommitRefIter imports and git_object::Error (058f68a)
    - [object #177] fix docs (2fd23ed)
    - [object #177] migrate immutable::commit into crate::commit (45d3934)
    - [object #177] tag::RefIter -> TagRefIter (28587c6)
    - [object #177] move mutable objects to crate::* (c551c02)
    - [object #177] migrate immutable::tree to crate::tree (fa5cd06)
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments (461dc53)
    - [object #177] rename immutable::* to immutable::*Ref (6deb012)
    - Release git-object v0.13.0 (708fc5a)
    - [ref #175] follow (try_)find(_what) naming convention (679895c)
    - Merge pull request #172 from mellowagain/main (61aebbf)
    - [ref #175] make 'mutable' module private (a80dbcf)
    - Release git-actor v0.5.0 (a684b0f)
    - [ref #175] refactor (292e567)
    - Release git-actor v0.4.0 (16358c9)
    - [actor #173] rename immutable::Signature to SignatureRef! (96461ac)
    - Release git-lock v1.0.0 (f38f72c)
    - Release git-tempfile v1.0.0 (1238535)
    - [smart-release #171] it's about time we get some tests (48a489b)
    - [stability #171] Prime git-tempfile and git-lock for release (01278fe)
    - [stability #171] mark git-hash and git-actor as ST1 as well (32caae1)
    - [stability #171] git-ref is now ST1 and available through git-repository (50154cd)
    - [smart-release #171] Try to avoid unstable git-repository features… (c8f325b)
    - Merge branch 'main' into stability (11bae43)
    - [stability #171] Don't provide access to less stable crates in `Respository` (e4c5b58)
    - cleanup imports (e669303)
    - [stability #171] Don't leak unstable plumbing crates in git-repository… (71eb30f)
    - Release git-pack v0.9.0 (7fbc961)
    - Merge branch 'main' into 162-repo-design-sketch (e63b634)
    - [repository #164] top-level easy docs (6b71c51)
    - [repository #165] see if `git-config` can already be placed… (d287a4a)
    - [repository #165] add limitations along with possible workarouds (7578f1e)
    - [repository #165] assure packed-refs are always uptodate (a5605df)
    - [repository #165] Allow cloning packed-refs and try to see how it differs… (7ec32b7)
    - Release git-ref v0.6.0 (0bb4c13)
    - [ref #165] refactor (66624c3)
    - Revert "[repository #165] PROOF: GATs will work as expected!" (853f072)
    - [repository #165] PROOF: GATs will work as expected! (7f56dbd)
    - [repository #165] refactor (1547d0b)
    - [repository #165] refactor; fine grained allow(missing_docs)… (aa0511f)
    - [repository #165] prepare for writing light docs for Easy (f8834c9)
    - [repository #165] refactor (3a0160e)
    - [repository #165] fmt (a02d5aa)
    - [repository #165] Don't panic on repo borrow error… (b2f644a)
    - thanks clippy (b496d99)
    - [repository #165] Write about the GAT plan to make this better one day (d793ecd)
    - [repository #165] quick test to see if Access2 can become Access… (45acc7a)
    - [repository #165] Generalizing over mutable Repos is possible too… (0f7efe3)
    - [repository #165] show that Access2 works for all Easy* types… (b8ceefe)
    - [repository #165] First success with creating a shared borrow to the repo (f2a38b2)
    - Revert "[repository #165] FAIL Look into `owned_ref` crate" (a1443e4)
    - [repository #165] FAIL Look into `owned_ref` crate (09aa714)
    - [repository #165] FAIL AsRef works for basic refs but… (02979b6)
    - [repository #165] FAIL try to generalize with Borrow… (295ba95)
    - [repository #165] FAIL See if EasyExclusive can work… (016debb)
    - [repository #165] introduce EasyShared (a119ad9)
    - [repository #165] First thoughts about stale caches (7f8b63e)
    - [repository #165] hide all easy::State fields behind result-enforcing methods (000c537)
    - [repository #165] pack cache access only with errors (2353e50)
    - [repository #165] assure packed-refs is only used non-panicking (a355d94)
    - [repository #165] refactor (16fce63)
    - [repository #165] a sample of a simpler way to create a tag (fb8f584)
    - [smart-release #165] Use generic edit-reference functionality (be3e57f)
    - [repository #165] sketch generic ref file editing (3a026ae)
    - [repository #165] refactor (00ec15d)
    - [repository #165] refactor (0f13104)
    - [repository #165] An experiment on transforming panics into errors… (1f52226)
    - [repository #165] offer panicking type conversions for objects (f802f8c)
    - [repository #165] try a more common naming convention for fallbile things… (fc70393)
    - [repository #165] refactor (6207735)
    - thanks clippy (41d7a44)
    - [repository #162] cleanup imports (983d11a)
    - [smart-release #162] use TreeRef capabilities to lookup path (51d1943)
    - [repository #162] what could be a correct implementation of a tree path lookup (1f638ee)
    - [repository #162] detachable ObjectRefs and a few conversions (ec123bb)
    - [repository #162] finally let smart-release use the correct abstraction for peeling (ba243a3)
    - [repository #162] Add id field to ObjectRef… (f5ba98e)
    - [repository #162] Make clear that Objects are actually references… (d1e6843)
    - [repository #162] another attempt to find a decent peeling abstraction… (716d623)
    - [repository #162] attach the Object to 'Access' (9a12564)
    - [repository #162] refactor (a32d361)
    - [repository #162] trying new names (b3f453b)
    - [repository #162] put impl for finding object data into the extension trait (91b9446)
    - [repository #162] experiment with finding objects… (312a692)
    - thanks clippy (f2fb026)
    - [repository #162] Cannot ever store a RefCell Ref in an object… (5c17199)
    - [repository #162] experiemnt with optionally keeping data in Object (b8a8e08)
    - [smart-release #162] Object can be used like a git_hash::ObjectId (c7bc730)
    - [smart-release #162] format everything (8ff83e5)
    - [smart-release #162] don't throw away work… (b43b780)
    - [smart-release #162] a demo of attaching and detaching objects… (ff2927c)
    - [smart-release #162] an actual Data type… (7fd996f)
    - [smart-release #162] unify 'ext' visibility (ca082a7)
    - thanks clippy (1f2d458)
    - [smart-release #162] a sketch for accessing objects data… (ba27101)
    - [smart-release #162] peeling objects to a certain target kind… (5785136)
    - [smart-release #162] a single import path for ReferenceExt (7060797)
    - [smart-release #162] rename git-repository::object -> objs (ac70d81)
    - [smart-release #162] replace reference peeling with git_easy (7cfd5f9)
    - [smart-release #162] smart-release uses Easy repository in 'plumbing' mode (4fb672a)
    - [smart-release #162] refactor (ef623a6)
    - [smart-release #162] reduce visibility of Cache (397fbfe)
    - [smart-release #162] more granular cache control WORKS (25dce2a)
    - Revert "[smart-release #162] FAIL: definitely need better granularity" (499993f)
    - [smart-release #162] FAIL: definitely need better granularity (5f27871)
    - [smart-release #162] FAIL: promising at first, but not really working… (fa01f76)
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

