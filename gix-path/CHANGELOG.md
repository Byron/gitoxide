# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.2 (2023-02-20)

### Bug Fixes

 - <csr-id-e14dc7d475373d2c266e84ff8f1826c68a34ab92/> note that crates have been renamed from `git-*` to `gix-*`.
   This also means that the `git-*` prefixed crates of the `gitoxide` project
   are effectively unmaintained.
   Use the crates with the `gix-*` prefix instead.
   
   If you were using `git-repository`, then `gix` is its substitute.
 - <csr-id-135d317065aae87af302beb6c26bb6ca8e30b6aa/> compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`.
   `as_ref()` relies on a known target type which isn't always present. However, once
   there is only one implementation, that's no problem, but when that changes compilation
   fails due to ambiguity.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - compatibility with `bstr` v1.3, use `*.as_bytes()` instead of `.as_ref()`. ([`135d317`](https://github.com/Byron/gitoxide/commit/135d317065aae87af302beb6c26bb6ca8e30b6aa))
</details>

## 0.7.1 (2023-02-17)

<csr-id-37cab07f283a368f323604372c84475d73d6c258/>
<csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/>
<csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/>
<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Refactor (BREAKING)

 - <csr-id-37cab07f283a368f323604372c84475d73d6c258/> Rename absolutize() to normalize()
   The name absolutize implies strongly that the returned path will be
   absolute, but the function only converts relative paths to absolute under a
   few specific circumstances.
   
   The new name, normalize(), is inspired by Python's os.path.normpath(),
   Java's java.nio.file.Path.normalize(), Node's Path.normalize(), and maybe
   some others which have similar semantics to this function.
 - <csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/> various name changes for more convenient API

### Bug Fixes (BREAKING)

 - <csr-id-7dbab1c62c49822983c59be0443478f7b4fecbca/> `absolutize()` now takes a mandatory `current_dir()` parameter and returns `Option<path>`
   Previously the function was willing to return an empty path despite it
   being invalid. With the `current_dir` being required, this won't be the
   case anymore and will yield logically consistent results in all cases.
   
   This forces the caller to deal with the relative path being invalid
   or crafted to produce some other path, maybe to bypass sanity checks.
 - <csr-id-c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce/> Remove `git-config` test utilities from `git-path`.

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
 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-dfa1e05d3c983f1e8b1cb3b80d03608341187883/> `realpath()` handles `cwd` internally
   This makes for more convenient usage in the common case.

### Refactor

 - <csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/> rename tests/convert/normalize.rs
   This renames the test module to match the new function name.

### Other

 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> :discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absoluteize' the path later (std::path::absolute()) is
   on the way for that.

### Bug Fixes

 - <csr-id-745d92636f8a3436ded0c9da21beb92182341998/> `.` substitution is only done if the input was relative.
   Previously it was possible to have `/a/b/../b` and a CWD of `/a/b`
   replaced with `.` even though that clearly isn't what the user provided.
   
   Now the `.` resubstitution only happens when it's in the interest
   of the caller.
 - <csr-id-92d5d133e17c6b79400ec57b55ccd5337f3796b7/> `normalize()` would fail to interpret `../` correctly and end up in an invalid path.
   This is now fixed and should never happen again thanks to the addition
   of a missing test.
 - <csr-id-9171adb796b38b08cae9bdd375b16a59a8166a1c/> Handle `.` specifically in `absolutize()`.
   Previously, absolutizing `./../../` would lead to one path component
   of the `../` to be ignored as `.` was popped successfully, not realizing
   that it is a no-op.
   
   This could lead to problems with repository discovery if `.` was passed.

### New Features

 - <csr-id-25e795f4fe858d646ae7a3c4706e14a3837c3e66/> Add `os_string_into_bstring()` as sibling of `os_str_into_bstr()`.
 - <csr-id-523418f69030faa0add6472b14333e9aafc69f56/> add support for `wasi`
   This allows path conversions there to be just as efficient as on unix.
   
   This was adopted from [a PR in the
   hexlix-editor](https://github.com/helix-editor/helix/pull/3890/files#diff-504515b66023120e75a921cd56a932aed76c0ff62593fbb69d92e0ef65089501R47).
 - <csr-id-f58a043273b8e15afd01aac71f33652783baf462/> add `is_absolute()` for git-style absolute checks
   This essentially means that starting slashes are always absolute, even
   on windows.
 - <csr-id-35f146a8573dcc9a1de3230373c0cf0794c6b897/> Add `absolutize_components()`
   It helps to cleanup paths a little which comes in handy when dealing
   with `commondir` appended paths.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 189 commits contributed to the release over the course of 296 calendar days.
 - 20 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#422](https://github.com/Byron/gitoxide/issues/422), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#691](https://github.com/Byron/gitoxide/issues/691)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - `absolutize_*(dir)` is now `absolutize(dir, Option<cwd>)` ([`de87657`](https://github.com/Byron/gitoxide/commit/de87657194ad976cc73ebcc13c231537b35b4195))
    - More robust absolutize-paths implementation ([`4800ebe`](https://github.com/Byron/gitoxide/commit/4800ebec42f9bb6298cb5b2efdab71d6baf3b1ba))
    - Add `absolutize_components()` ([`35f146a`](https://github.com/Byron/gitoxide/commit/35f146a8573dcc9a1de3230373c0cf0794c6b897))
    - Allow reading patterns from stdin ([`0c597fe`](https://github.com/Byron/gitoxide/commit/0c597fe78acdd5672b4535a7d82620c5f7f93649))
    - :discover()` now returns the shortest path. ([`e4f4c4b`](https://github.com/Byron/gitoxide/commit/e4f4c4b2c75a63a40a174e3a006ea64ef8d78809))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - frame for `gix repo exclude query` ([`a331314`](https://github.com/Byron/gitoxide/commit/a331314758629a93ba036245a5dd03cf4109dc52))
    - refactor ([`21d4076`](https://github.com/Byron/gitoxide/commit/21d407638285b728d0c64fabf2abe0e1948e9bec))
    - The first indication that directory-based excludes work ([`e868acc`](https://github.com/Byron/gitoxide/commit/e868acce2e7c3e2501497bf630e3a54f349ad38e))
    - various name changes for more convenient API ([`5480159`](https://github.com/Byron/gitoxide/commit/54801592488416ef2bb0f34c5061b62189c35c5e))
    - Use bstr intead of [u8] ([`9380e99`](https://github.com/Byron/gitoxide/commit/9380e9990065897e318b040f49b3c9a6de8bebb1))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - Copy all existing functions from git-features::path to git-path:: ([`725e198`](https://github.com/Byron/gitoxide/commit/725e1985dc521d01ff9e1e89b6468ef62fc09656))
    - add empty git-path crate ([`8d13f81`](https://github.com/Byron/gitoxide/commit/8d13f81068b4663d322002a9617d39b307b63469))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - `realpath()` handles `cwd` internally ([`dfa1e05`](https://github.com/Byron/gitoxide/commit/dfa1e05d3c983f1e8b1cb3b80d03608341187883))
 * **[#422](https://github.com/Byron/gitoxide/issues/422)**
    - prepare changelog ([`de2d587`](https://github.com/Byron/gitoxide/commit/de2d5874b8d75c53165a9fc3ed35e2b37142bf52))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - add `is_absolute()` for git-style absolute checks ([`f58a043`](https://github.com/Byron/gitoxide/commit/f58a043273b8e15afd01aac71f33652783baf462))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
    - add support for `wasi` ([`523418f`](https://github.com/Byron/gitoxide/commit/523418f69030faa0add6472b14333e9aafc69f56))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **Uncategorized**
    - Release gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6efd0d3`](https://github.com/Byron/gitoxide/commit/6efd0d31fbeca31ab7319aa2ac97bb31dc4ce055))
    - Release gix-date v0.4.2, gix-hash v0.10.2, gix-features v0.26.4, gix-actor v0.17.1, gix-glob v0.5.3, gix-path v0.7.1, gix-quote v0.4.1, gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`6ccc88a`](https://github.com/Byron/gitoxide/commit/6ccc88a8e4a56973b1a358cf72dc012ee3c75d56))
    - Merge branch 'rename-crates' into inform-about-gix-rename ([`c9275b9`](https://github.com/Byron/gitoxide/commit/c9275b99ea43949306d93775d9d78c98fb86cfb1))
    - rename `git-testtools` to `gix-testtools` ([`b65c33d`](https://github.com/Byron/gitoxide/commit/b65c33d256cfed65d11adeff41132e3e58754089))
    - adjust to renaming of `git-pack` to `gix-pack` ([`1ee81ad`](https://github.com/Byron/gitoxide/commit/1ee81ad310285ee4aa118118a2be3810dbace574))
    - adjust to renaming of `git-odb` to `gix-odb` ([`476e2ad`](https://github.com/Byron/gitoxide/commit/476e2ad1a64e9e3f0d7c8651d5bcbee36cd78241))
    - adjust to renaming of `git-index` to `gix-index` ([`86db5e0`](https://github.com/Byron/gitoxide/commit/86db5e09fc58ce66b252dc13b8d7e2c48e4d5062))
    - adjust to renaming of `git-diff` to `gix-diff` ([`49a163e`](https://github.com/Byron/gitoxide/commit/49a163ec8b18f0e5fcd05a315de16d5d8be7650e))
    - adjust to renaming of `git-commitgraph` to `gix-commitgraph` ([`f1dd0a3`](https://github.com/Byron/gitoxide/commit/f1dd0a3366e31259af029da73228e8af2f414244))
    - adjust to renaming of `git-mailmap` to `gix-mailmap` ([`2e28c56`](https://github.com/Byron/gitoxide/commit/2e28c56bb9f70de6f97439818118d3a25859698f))
    - adjust to renaming of `git-discover` to `gix-discover` ([`53adfe1`](https://github.com/Byron/gitoxide/commit/53adfe1c34e9ea3b27067a97b5e7ac80b351c441))
    - adjust to renaming of `git-chunk` to `gix-chunk` ([`59194e3`](https://github.com/Byron/gitoxide/commit/59194e3a07853eae0624ebc4907478d1de4f7599))
    - adjust to renaming of `git-bitmap` to `gix-bitmap` ([`75f2a07`](https://github.com/Byron/gitoxide/commit/75f2a079b17489f62bc43e1f1d932307375c4f9d))
    - adjust to renaming for `git-protocol` to `gix-protocol` ([`823795a`](https://github.com/Byron/gitoxide/commit/823795addea3810243cab7936cd8ec0137cbc224))
    - adjust to renaming of `git-refspec` to `gix-refspec` ([`c958802`](https://github.com/Byron/gitoxide/commit/c9588020561577736faa065e7e5b5bb486ca8fe1))
    - adjust to renaming of `git-revision` to `gix-revision` ([`ee0ee84`](https://github.com/Byron/gitoxide/commit/ee0ee84607c2ffe11ee75f27a31903db68afed02))
    - adjust to renaming of `git-transport` to `gix-transport` ([`b2ccf71`](https://github.com/Byron/gitoxide/commit/b2ccf716dc4425bb96651d4d58806a3cc2da219e))
    - adjust to renaming of `git-credentials` to `gix-credentials` ([`6b18abc`](https://github.com/Byron/gitoxide/commit/6b18abcf2856f02ab938d535a65e51ac282bf94a))
    - adjust to renaming of `git-prompt` to `gix-prompt` ([`6a4654e`](https://github.com/Byron/gitoxide/commit/6a4654e0d10ab773dd219cb4b731c0fc1471c36d))
    - adjust to renaming of `git-command` to `gix-command` ([`d26b8e0`](https://github.com/Byron/gitoxide/commit/d26b8e046496894ae06b0bbfdba77196976cd975))
    - adjust to renaming of `git-packetline` to `gix-packetline` ([`5cbd22c`](https://github.com/Byron/gitoxide/commit/5cbd22cf42efb760058561c6c3bbcd4dab8c8be1))
    - adjust to renaming of `git-worktree` to `gix-worktree` ([`73a1282`](https://github.com/Byron/gitoxide/commit/73a12821b3d9b66ec1714d07dd27eb7a73e3a544))
    - adjust to renamining of `git-worktree` to `gix-worktree` ([`108bb1a`](https://github.com/Byron/gitoxide/commit/108bb1a634f4828853fb590e9fc125f79441dd38))
    - adjust to renaming of `git-url` to `gix-url` ([`b50817a`](https://github.com/Byron/gitoxide/commit/b50817aadb143e19f61f64e19b19ec1107d980c6))
    - adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
    - adjust to renaming of `git-pathspec` to `gix-pathspec` ([`37f7c6b`](https://github.com/Byron/gitoxide/commit/37f7c6b9070e118604aa3fc0b38530699dcfec6e))
    - adjust to renamining of `git-attributes` to `gix-attributes` ([`4a8b3b8`](https://github.com/Byron/gitoxide/commit/4a8b3b812ac26f2a2aee8ce8ca81591273383c84))
    - adjust to renaminig of `git-quote` to `gix-quote` ([`648025b`](https://github.com/Byron/gitoxide/commit/648025b7ca94411fdd0d90c53e5faede5fde6c8d))
    - adjust to renaming of `git-config` to `gix-config` ([`3a861c8`](https://github.com/Byron/gitoxide/commit/3a861c8f049f6502d3bcbdac752659aa1aeda46a))
    - adjust to renaming of `git-ref` to `gix-ref` ([`1f5f695`](https://github.com/Byron/gitoxide/commit/1f5f695407b034377d94b172465ff573562b3fc3))
    - adjust to renaming of `git-lock` to `gix-lock` ([`2028e78`](https://github.com/Byron/gitoxide/commit/2028e7884ae1821edeec81612f501e88e4722b17))
    - adjust to renaming of `git-tempfile` to `gix-tempfile` ([`b6cc3eb`](https://github.com/Byron/gitoxide/commit/b6cc3ebb5137084a6327af16a7d9364d8f092cc9))
    - adjust to renaming of `git-object` to `gix-object` ([`fc86a1e`](https://github.com/Byron/gitoxide/commit/fc86a1e710ad7bf076c25cc6f028ddcf1a5a4311))
    - adjust to renaming of `git-actor` to `gix-actor` ([`4dc9b44`](https://github.com/Byron/gitoxide/commit/4dc9b44dc52f2486ffa2040585c6897c1bf55df4))
    - adjust to renaming of `git-validate` to `gix-validate` ([`5e40ad0`](https://github.com/Byron/gitoxide/commit/5e40ad078af3d08cbc2ca81ce755c0ed8a065b4f))
    - adjust to renaming of `git-hash` to `gix-hash` ([`4a9d025`](https://github.com/Byron/gitoxide/commit/4a9d0257110c3efa61d08c8457c4545b200226d1))
    - adjust to renaming of `git-features` to `gix-features` ([`e2dd68a`](https://github.com/Byron/gitoxide/commit/e2dd68a417aad229e194ff20dbbfd77668096ec6))
    - adjust to renaming of `git-glob` to `gix-glob` ([`35b2a3a`](https://github.com/Byron/gitoxide/commit/35b2a3acbc8f2a03f151bc0a3863163844e0ca86))
    - adjust to renaming of `git-sec` to `gix-sec` ([`eabbb92`](https://github.com/Byron/gitoxide/commit/eabbb923bd5a32fc80fa80f96cfdc2ab7bb2ed17))
    - adapt to renaming of `git-path` to `gix-path` ([`d3bbcfc`](https://github.com/Byron/gitoxide/commit/d3bbcfccad80fc44ea8e7bf819f23adaca06ba2d))
    - rename `git-path` to `gix-path` ([`9fe8e83`](https://github.com/Byron/gitoxide/commit/9fe8e8389c0ba677f31356c26a375e694e4d1f64))
    - adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Merge branch 'git-pack-wasm' ([`4bc19d1`](https://github.com/Byron/gitoxide/commit/4bc19d104233a3e3d3d2768c0e9b9ad027cc34c0))
    - CI validates WASM support ([`0d4b804`](https://github.com/Byron/gitoxide/commit/0d4b804171acd307bdac6eecd3b49bd8b2fb2968))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'main' into read-split-index ([`c57bdde`](https://github.com/Byron/gitoxide/commit/c57bdde6de37eca9672ea715962bbd02aa3eb055))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - make fmt ([`747008d`](https://github.com/Byron/gitoxide/commit/747008d9d370844574dda94e5bec1648c4deb57e))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Merge branch 'discovery-fix' ([`689752e`](https://github.com/Byron/gitoxide/commit/689752e67e8895b3d40d335e5778d8a90ec89c4c))
    - `.` substitution is only done if the input was relative. ([`745d926`](https://github.com/Byron/gitoxide/commit/745d92636f8a3436ded0c9da21beb92182341998))
    - `normalize()` would fail to interpret `../` correctly and end up in an invalid path. ([`92d5d13`](https://github.com/Byron/gitoxide/commit/92d5d133e17c6b79400ec57b55ccd5337f3796b7))
    - Merge branch 'path-normalize' ([`805329a`](https://github.com/Byron/gitoxide/commit/805329a0a5f6543bbc1d5885977b47bf7baa7f08))
    - rename tests/convert/normalize.rs ([`8ab47bb`](https://github.com/Byron/gitoxide/commit/8ab47bbdac44c0fa738215d3cc457eb3b6f30504))
    - Rename absolutize() to normalize() ([`37cab07`](https://github.com/Byron/gitoxide/commit/37cab07f283a368f323604372c84475d73d6c258))
    - Add `os_string_into_bstring()` as sibling of `os_str_into_bstr()`. ([`25e795f`](https://github.com/Byron/gitoxide/commit/25e795f4fe858d646ae7a3c4706e14a3837c3e66))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'discover-rel-path' ([`5f908fb`](https://github.com/Byron/gitoxide/commit/5f908fb86857d565715b9b0b8b453b29273fb022))
    - improve documentation to clarify intent ([`b8f73aa`](https://github.com/Byron/gitoxide/commit/b8f73aa5afe3f7aefa5627d7708e4c7e7da950a2))
    - Merge branch 'cwd-consistency' ([`ea7c6a3`](https://github.com/Byron/gitoxide/commit/ea7c6a3b069c9e13905b51b87538c57ba9182dca))
    - Adapt to changes in `git-discover` and `git-path` and `git-odb` ([`98c2501`](https://github.com/Byron/gitoxide/commit/98c250175a39598b9d37613c43dda2299da8eff3))
    - `absolutize()` now takes a mandatory `current_dir()` parameter and returns `Option<path>` ([`7dbab1c`](https://github.com/Byron/gitoxide/commit/7dbab1c62c49822983c59be0443478f7b4fecbca))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Handle `.` specifically in `absolutize()`. ([`9171adb`](https://github.com/Byron/gitoxide/commit/9171adb796b38b08cae9bdd375b16a59a8166a1c))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - fix docs ([`4f8e3b1`](https://github.com/Byron/gitoxide/commit/4f8e3b169e57d599439c7abc861c82c08bcd92e3))
    - thanks clippy ([`7a2a31e`](https://github.com/Byron/gitoxide/commit/7a2a31e5758a2be8434f22cd9401ac00539f2bd9))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`0e9df36`](https://github.com/Byron/gitoxide/commit/0e9df364c4cddf006b1de18b8d167319b7cc1186))
    - generally avoid using `target_os = "windows"` in favor of `cfg(windows)` and negations ([`91d5402`](https://github.com/Byron/gitoxide/commit/91d54026a61c2aae5e3e1341d271acf16478cd83))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - avoid unwraps in tests as they are now stable ([`efa1423`](https://github.com/Byron/gitoxide/commit/efa14234c352b6b8417f0a42fc946e88f2eb52d3))
    - remove canonicalized-path abstraction ([`9496e55`](https://github.com/Byron/gitoxide/commit/9496e5512975825efebe0db86335d0d2dc8c9095))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Fix git-paths tests; improve error handling. ([`9c00504`](https://github.com/Byron/gitoxide/commit/9c0050451f634a54e610c86199b5d7d393378878))
    - docs for git-path ([`a520092`](https://github.com/Byron/gitoxide/commit/a52009244c9b1059ebb3d5dd472c25f9c49691f3))
    - Remove `git-config` test utilities from `git-path`. ([`c9933c0`](https://github.com/Byron/gitoxide/commit/c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - prepare for smart-release release ([`2f74cb0`](https://github.com/Byron/gitoxide/commit/2f74cb05e9b2399355af07517fe3c14e4e8724c5))
    - adjust git-path size limits ([`5ac8a3b`](https://github.com/Byron/gitoxide/commit/5ac8a3b58e0f61d4801a6f4dbd011f757208dbac))
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - Merge branch 'svetli-n-git_includeif' ([`cf24fbe`](https://github.com/Byron/gitoxide/commit/cf24fbe4b62d67b06138243d470dcc1805ebd55b))
    - Remove forbid missing_docs ([`23acebb`](https://github.com/Byron/gitoxide/commit/23acebb8e9e53d89e7f629ab690253610358b0bb))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - declare `git-path` usable ([`496594d`](https://github.com/Byron/gitoxide/commit/496594d2d8b4216b51cfbd97805834c71c030c75))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Fix create_symlink ([`714db70`](https://github.com/Byron/gitoxide/commit/714db70f02134c7f53dc7ba0461f43a0d6b659e9))
    - Add includeIf test with symlink. ([`5d74404`](https://github.com/Byron/gitoxide/commit/5d744049286632f3141ec07fa3f128093480d1c0))
    - Fix realpath tests. ([`0426f4d`](https://github.com/Byron/gitoxide/commit/0426f4deb5d73fd88529530f9a6d01ba55eeadc4))
    - Refactor real_path tests. ([`b696849`](https://github.com/Byron/gitoxide/commit/b696849e5fd210da397b0e7a3b26a63314d87607))
    - Refactor real_path tests. ([`8ade69f`](https://github.com/Byron/gitoxide/commit/8ade69fbddfa5d0be3bbe761210e49be647c3356))
    - Fix windows (probably) ([`c980014`](https://github.com/Byron/gitoxide/commit/c980014206ff071bc4f351416bb14995ac739e1b))
    - thanks clippy ([`da13aff`](https://github.com/Byron/gitoxide/commit/da13affabe34c3d691b18a70ce61eb00319668c5))
    - refactor ([`6bba054`](https://github.com/Byron/gitoxide/commit/6bba054a9a87219a7f94c155058fda5a3e6dffa6))
    - turn recursion into loop ([`9b83c2c`](https://github.com/Byron/gitoxide/commit/9b83c2c233d41034796694d000bed10d45f40c92))
    - refactor ([`1ca0540`](https://github.com/Byron/gitoxide/commit/1ca0540d170dcb8066a9141ce97631fcb9f2d5ae))
    - refactor ([`1f6ecd2`](https://github.com/Byron/gitoxide/commit/1f6ecd2ba91a34171d708ab7cb9414e853face95))
    - refactor ([`5efb972`](https://github.com/Byron/gitoxide/commit/5efb97251a9bf9e342d28bcbde27b0e69b0b7849))
    - refactor ([`353c245`](https://github.com/Byron/gitoxide/commit/353c2455dc01cf342b1186f0be263a87952b70be))
    - put `realpath` into its own module ([`d142e01`](https://github.com/Byron/gitoxide/commit/d142e01445ef545bd8284d3899d7e68f578943e9))
    - refactor ([`50583f0`](https://github.com/Byron/gitoxide/commit/50583f083be7ba890f7727a6491cbacf8b87ebe4))
    - rename `real_path()` to `realpath()` ([`478ff6c`](https://github.com/Byron/gitoxide/commit/478ff6caa630970847094fc11af10a6b69d72c34))
    - refactor ([`8f1daf5`](https://github.com/Byron/gitoxide/commit/8f1daf55c0027ec124bc6672ec545275065af9a7))
    - Fix linux test ([`8a36810`](https://github.com/Byron/gitoxide/commit/8a368102c89161006cad343839105d3a5ff284e2))
    - Fix windows test. ([`1afb2da`](https://github.com/Byron/gitoxide/commit/1afb2daa6704cc0c2efd9437dff5518ea3e64429))
    - Temp ignore real_path_tests. ([`c2f5db9`](https://github.com/Byron/gitoxide/commit/c2f5db9a3fcc7bdcdd84cdda30d970bdcedaff2a))
    - Windows absolute path. ([`8dc33cc`](https://github.com/Byron/gitoxide/commit/8dc33ccd1f5886b5e3f23eac5d6381473c386c2f))
    - Windows absolute path. ([`070f8c7`](https://github.com/Byron/gitoxide/commit/070f8c79a54141d3b3064622ac7b528a24875d4f))
    - Windows absolute path. ([`cefc8fb`](https://github.com/Byron/gitoxide/commit/cefc8fbfbb591fe714ffd87f39d0a7ca00e4a754))
    - Windows absolute path. ([`31a71f3`](https://github.com/Byron/gitoxide/commit/31a71f37d596a3a7a7279d4b6e2508c32383b2b6))
    - Fix merge. ([`f2b46df`](https://github.com/Byron/gitoxide/commit/f2b46dfbf73387d4501a7bf5039cb80ac4cb8d9c))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Add custom tempdir in. ([`8bfd52a`](https://github.com/Byron/gitoxide/commit/8bfd52a65fcecb33ae69917a67c48027f8fb3dff))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - thanks clippy ([`a084951`](https://github.com/Byron/gitoxide/commit/a084951c72818d7cb2061053078793213890c899))
    - Temp ignore real_path_tests. ([`27f4bfc`](https://github.com/Byron/gitoxide/commit/27f4bfcb2fba45bd02d1977094acb31b7b989cac))
    - Windows fix. ([`ce0b408`](https://github.com/Byron/gitoxide/commit/ce0b408fcdeae80d6c9263955f70a00ead3841e1))
    - Windows fix. ([`25dd319`](https://github.com/Byron/gitoxide/commit/25dd319a2b46327fb553f824619311484726c742))
    - Windows fix. ([`61bc0e7`](https://github.com/Byron/gitoxide/commit/61bc0e776b9b02fdd36df6c0f54aecae63bf5895))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Handle windows path prefix. ([`1723236`](https://github.com/Byron/gitoxide/commit/1723236377db483b09f123a5c24c949afa285b8a))
    - Max symlinks exceeded test. ([`cfff300`](https://github.com/Byron/gitoxide/commit/cfff30075d87045bf9def697c417a3eb46b4b215))
    - Use thiserror in `real_path()` ([`2bd7a44`](https://github.com/Byron/gitoxide/commit/2bd7a441beb7e0a86169ec89ca56a8ba448fbf2b))
    - input_path is Iterator. ([`c993d78`](https://github.com/Byron/gitoxide/commit/c993d7826fcf76ddaddffca619b4d35555b6636c))
    - real_path wip ([`3890a61`](https://github.com/Byron/gitoxide/commit/3890a6149683663b16dccdc3b50e2aab7eb4e048))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Release git-path v0.1.0 ([`ca019fc`](https://github.com/Byron/gitoxide/commit/ca019fca03c4ea0d70fabbf09808732925b58077))
</details>

## 0.7.0 (2022-12-19)

<csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/>
<csr-id-37cab07f283a368f323604372c84475d73d6c258/>

### New Features

 - <csr-id-25e795f4fe858d646ae7a3c4706e14a3837c3e66/> Add `os_string_into_bstring()` as sibling of `os_str_into_bstr()`.

### Bug Fixes

 - <csr-id-745d92636f8a3436ded0c9da21beb92182341998/> `.` substitution is only done if the input was relative.
   Previously it was possible to have `/a/b/../b` and a CWD of `/a/b`
   replaced with `.` even though that clearly isn't what the user provided.
   
   Now the `.` resubstitution only happens when it's in the interest
   of the caller.
 - <csr-id-92d5d133e17c6b79400ec57b55ccd5337f3796b7/> `normalize()` would fail to interpret `../` correctly and end up in an invalid path.
   This is now fixed and should never happen again thanks to the addition
   of a missing test.

### Refactor

 - <csr-id-8ab47bbdac44c0fa738215d3cc457eb3b6f30504/> rename tests/convert/normalize.rs
   This renames the test module to match the new function name.

### Refactor (BREAKING)

 - <csr-id-37cab07f283a368f323604372c84475d73d6c258/> Rename absolutize() to normalize()
   The name absolutize implies strongly that the returned path will be
   absolute, but the function only converts relative paths to absolute under a
   few specific circumstances.
   
   The new name, normalize(), is inspired by Python's os.path.normpath(),
   Java's java.nio.file.Path.normalize(), Node's Path.normalize(), and maybe
   some others which have similar semantics to this function.

## 0.6.0 (2022-11-21)

### New Features (BREAKING)

 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `gix-features` and `gix-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Bug Fixes (BREAKING)

 - <csr-id-7dbab1c62c49822983c59be0443478f7b4fecbca/> `absolutize()` now takes a mandatory `current_dir()` parameter and returns `Option<path>`
   Previously the function was willing to return an empty path despite it
   being invalid. With the `current_dir` being required, this won't be the
   case anymore and will yield logically consistent results in all cases.
   
   This forces the caller to deal with the relative path being invalid
   or crafted to produce some other path, maybe to bypass sanity checks.

## 0.5.0 (2022-09-20)

### New Features

 - <csr-id-523418f69030faa0add6472b14333e9aafc69f56/> add support for `wasi`
   This allows path conversions there to be just as efficient as on unix.
   
   This was adopted from [a PR in the
   hexlix-editor](https://github.com/helix-editor/helix/pull/3890/files#diff-504515b66023120e75a921cd56a932aed76c0ff62593fbb69d92e0ef65089501R47).

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.4.2 (2022-08-29)

### New Features

 - <csr-id-f58a043273b8e15afd01aac71f33652783baf462/> add `is_absolute()` for git-style absolute checks
   This essentially means that starting slashes are always absolute, even
   on windows.

## 0.4.1 (2022-08-23)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Bug Fixes

 - <csr-id-9171adb796b38b08cae9bdd375b16a59a8166a1c/> Handle `.` specifically in `absolutize()`.
   Previously, absolutizing `./../../` would lead to one path component
   of the `../` to be ignored as `.` was popped successfully, not realizing
   that it is a no-op.
   
   This could lead to problems with repository discovery if `.` was passed.

## 0.4.0 (2022-07-22)

### Changed (BREAKING)

 - <csr-id-dfa1e05d3c983f1e8b1cb3b80d03608341187883/> `realpath()` handles `cwd` internally
   This makes for more convenient usage in the common case.

## 0.3.0 (2022-06-19)

### Bug Fixes (BREAKING)

 - <csr-id-c9933c0b0f51d21dc8244b2acc33d7dc8a33f6ce/> Remove `gix-config` test utilities from `gix-path`.

## 0.2.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

## 0.1.3 (2022-05-23)

A maintenance release without user-facing changes.

## 0.1.2 (2022-05-21)

A maintenance release without user-facing changes.

## 0.1.1 (2022-05-18)

<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>

### New Features

 - <csr-id-35f146a8573dcc9a1de3230373c0cf0794c6b897/> Add `absolutize_components()`
   It helps to cleanup paths a little which comes in handy when dealing
   with `commondir` appended paths.

### Other

 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> :discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absolutize' the path later (std::path::absolute()) is
   on the way for that.

## 0.1.0 (2022-04-28)

<csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/>

### Refactor (BREAKING)

 - <csr-id-54801592488416ef2bb0f34c5061b62189c35c5e/> various name changes for more convenient API

## 0.0.0 (2022-03-31)

An empty crate without any content to reserve the name for the gitoxide project.

