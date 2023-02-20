# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.13.3 (2023-02-20)

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

## 0.13.2 (2023-02-17)

<csr-id-098f802e6dc9f55632791ddf8d046563f75cba7a/>
<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Bug Fixes (BREAKING)

 - <csr-id-2da6c862e184ac37d59147e9cf809017b65db966/> make Scheme work with serde, removing `Copy` in the process.  e#450)
   This wasn't supposed to happen but a requirement to get `serde` support
   back.
 - <csr-id-2bcfdee6a3af758a0b70e2af9c4b6f8cc09d8da0/> Prohibit invalid state by making parts the url's data private
   This fix is meant to improve serialization support which can now happen
   `to_bstring()` without possibility for error.
   
   Empty paths can still be set which won't be valid for all URLs.

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
 - <csr-id-96a265cc67ea787ed28adde2c5d0a07babf64c9e/> generalize extension schemes.
   Previously this was hard-coded to `radicle`, now it's just an extension
   scheme along with a statically known string. This means we have to
   explicitly support new formats which should be fine.

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`
 - <csr-id-653ebc52f97116e9c72e985eda0d76f566e8c74d/> Introduce `parse(&BStr)` (previously it took `&[u8]`)
   A `&BStr` better indicates that we are expecting human-readable input
   with ascii-compatible or UTF-8 endcoding.
 - <csr-id-f6506e0c463bdccbcfd9324bc312da9cc957d8e6/> Use `&BStr` as input instead of `[u8]`
   This signals that it's indeed intended to be human readable while
   allowing it to be a path as well without loss, at least theoretically.
   After all we currently don't have a way to parse invalid UTF-8.
 - <csr-id-79ab4aeb8206a5f32735891336d7745e046bbea1/> remove `impl std::fmt::Display for Url` as it's not lossless.
 - <csr-id-ffc4a85b9a914b685d7ab528b30f2a3eefb44094/> `From<&[u8]>` is now `From<&BStr>`
   This better represents the meaning of the input, and simplifies
   interactions with `git-config`.

### Other

 - <csr-id-098f802e6dc9f55632791ddf8d046563f75cba7a/> try for leaner tests, but it does the opposite kind of :D

### New Features

 - <csr-id-61d89f586a0ad913fc2f502520282520a5e1fd15/> collect ssh-specific options to control how the ssh program is invoked.
   These are passed through when creating the ssh transport.
 - <csr-id-01f25744bba45a5f8a8615734a5beeacd29d1c4e/> add `Url::canonicalized()` and `Url::canonicalize()`.
   These methods allow to assure file urls are absolute, useful when
   cloning from any url.
 - <csr-id-22d3b37ea6239170a478b859361a7d1d7ba01a9a/> `Url::try_from(path: &std::path::Path)` for more convenient instantiation.
 - <csr-id-39ce98ba9a427b8cea1b843f333c2e7de300499c/> (mostly) lossless roundtripping of scp-like urls.
   Previously `git@host:path` would turn into `ssh://git@host/path`,
   which now remains exactly as is.
 - <csr-id-58a6000d669acd33bad91509eaa469f041f119e5/> lossless serialization of file urls.
   Previously a url like `/path/to/repo` would serialize to
   `file:///path/to/repo`, preventing round-trips.
   
   Now it serializes like it was parsed. This also means that
   `file://path` still serializes as `file://path`.
 - <csr-id-7484db5d36383de450de31b4c94c01bc4c237ce4/> `Url::port_or_default()` to fill in default numbers for ports if possible.
 - <csr-id-fbe75c9457708b95dd833e00afa2dcc1db677167/> `Url::path_is_root()` to determine if the path is `/`.
   This could also be considered an empty path depending on the context
   which is what makes it useful.
 - <csr-id-d40f6e1f34eb3f4664caec36727bf0aa3a396a33/> `Scheme::try_from(&str)`
 - <csr-id-7a1769009d68d14a134f368f93245abab0fb41dd/> `TryFrom<&OsStr>` to allow direct usage of `Url` in `clap`.
   In `clap_derive`, this needs
   `parse(try_from_os_str = std::convert::TryFrom::try_from)`.
 - <csr-id-b7a5f7a3b5cf058f503cc18d18fc75356ab98955/> `TryFrom<PathBuf>` which is useful if urls are obtained from the command-line.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-a67fc26b80e5d1183ddc5c6598396214f3e19945/> more conversions for `TryFrom`: `String` and `&str`
 - <csr-id-833899dce120d26a2bbe04d07fc4c71455eb3afe/> `Url::write_to(out)` to write itself more flexibly.
 - <csr-id-5f707c7e99c70ab9683d55c396e8dc11e1d3b0ea/> Add `Url::to_bstring()` for lossless but fallible bstring conversion.

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Documentation

 - <csr-id-39ed9eda62b7718d5109135e5ad406fb1fe2978c/> fix typos

### Bug Fixes

 - <csr-id-372cea13ad8abe1c47ed7bc806b42292a8ebfae0/> correctly parse scp-like ssh urls with alias as such.
   Previously it would falsely consider it a file path, leading to
   the inability to use the URL at all.
   
   Now the initial scheme detection has been improved to not rely on
   the '.' in host names exclusively anymore.
 - <csr-id-1058330adcc3262c59d30a0b8854fade20ffc3d5/> properly set default SSH port to 22
 - <csr-id-f20f2728ee78d90510e27769a61ead405c4018c1/> scp-like URLs should preserve relative and home-relative paths
 - <csr-id-302a2d866692a541e01d112b6870aa22fcdbe32b/> reject empty paths where needed, add `Url::from_parts_as_alternative_form()`.
   The new constructor allows to create URLs that represent paths which otherwise couldn't
   be valid URLs.
 - <csr-id-3e3aff9f2f427d030a38fe147c5252d7bfd45109/> make sure that `file:..` isn't considered a valid file url.
 - <csr-id-d6f90beac37866f992a1714d38e5b320eea6f1bb/> handle `file:///C:/foo/bar` urls correctly on windows, as paths now are `C:\\foo\bar`.
   These paths are created when using the `url::Url::from_file_path()`
   family of methods, which adds an extra slash at the beginning of a
   windows path which makes it invalid there unless there is further
   processing.
   
   This is now applied by using `url` features, making this case work
   specifically. Note that all other attributes are still the same
   and `git-url` generally tries to keep paths in tact to be a hybrid
   of type that can handle any file system paths as well as actual urls.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 277 commits contributed to the release over the course of 918 calendar days.
 - 34 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 12 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#331](https://github.com/Byron/gitoxide/issues/331), [#333](https://github.com/Byron/gitoxide/issues/333), [#364](https://github.com/Byron/gitoxide/issues/364), [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470), [#524](https://github.com/Byron/gitoxide/issues/524), [#691](https://github.com/Byron/gitoxide/issues/691), [#725](https://github.com/Byron/gitoxide/issues/725), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Only write headlines that we can parse back… ([`d44369a`](https://github.com/Byron/gitoxide/commit/d44369ab5d849720dda9a9c0edc1ba1a3c1a78b5))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Adapt to changes in git_features::path to deal with Result ([`bba4c68`](https://github.com/Byron/gitoxide/commit/bba4c680c627a418efbd25f14bd168df19b8dedd))
 * **[#333](https://github.com/Byron/gitoxide/issues/333)**
    - Use git_features::path everywhere where there is a path conversion ([`2e1437c`](https://github.com/Byron/gitoxide/commit/2e1437cb0b5dc77f2317881767f71eaf9b009ebf))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - update changelogs prior to release ([`746a676`](https://github.com/Byron/gitoxide/commit/746a676056cd4907da7137a00798344b5bdb4419))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - add `Url::canonicalized()` and `Url::canonicalize()`. ([`01f2574`](https://github.com/Byron/gitoxide/commit/01f25744bba45a5f8a8615734a5beeacd29d1c4e))
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - refactor ([`02e5775`](https://github.com/Byron/gitoxide/commit/02e5775f86ea414112ea9f66daf24ead8be31f73))
    - `Url::port_or_default()` to fill in default numbers for ports if possible. ([`7484db5`](https://github.com/Byron/gitoxide/commit/7484db5d36383de450de31b4c94c01bc4c237ce4))
    - `Url::path_is_root()` to determine if the path is `/`. ([`fbe75c9`](https://github.com/Byron/gitoxide/commit/fbe75c9457708b95dd833e00afa2dcc1db677167))
    - make note about why we don't support passwords in URLs ([`970ec9b`](https://github.com/Byron/gitoxide/commit/970ec9b2e51a32cae070e561a13ca3d76b9d22f9))
    - url-preprocessing for scripts ([`c00cc35`](https://github.com/Byron/gitoxide/commit/c00cc35493cec8f0b2673248caf1f0d83590dd54))
    - `Scheme::try_from(&str)` ([`d40f6e1`](https://github.com/Byron/gitoxide/commit/d40f6e1f34eb3f4664caec36727bf0aa3a396a33))
    - generalize extension schemes. ([`96a265c`](https://github.com/Byron/gitoxide/commit/96a265cc67ea787ed28adde2c5d0a07babf64c9e))
    - `TryFrom<&OsStr>` to allow direct usage of `Url` in `clap`. ([`7a17690`](https://github.com/Byron/gitoxide/commit/7a1769009d68d14a134f368f93245abab0fb41dd))
    - `TryFrom<PathBuf>` which is useful if urls are obtained from the command-line. ([`b7a5f7a`](https://github.com/Byron/gitoxide/commit/b7a5f7a3b5cf058f503cc18d18fc75356ab98955))
    - A first sketch on how connections could be working ([`e55b43e`](https://github.com/Byron/gitoxide/commit/e55b43ef72bb3f23655c7e0884b8efcf2496f944))
    - Use `&BStr` as input instead of `[u8]` ([`f6506e0`](https://github.com/Byron/gitoxide/commit/f6506e0c463bdccbcfd9324bc312da9cc957d8e6))
    - Prohibit invalid state by making parts the url's data private ([`2bcfdee`](https://github.com/Byron/gitoxide/commit/2bcfdee6a3af758a0b70e2af9c4b6f8cc09d8da0))
    - remove invalid test as it looks like it parses hosts from paths and that is fine ([`224c605`](https://github.com/Byron/gitoxide/commit/224c605d11a823bdaad6eb2bae1149bc671fb92d))
    - switch to `thiserror` ([`cfd7c0a`](https://github.com/Byron/gitoxide/commit/cfd7c0a29f10010841b310e0eb8b000083381a58))
    - prepare for better error handling around ssh urls ([`6d8d9b8`](https://github.com/Byron/gitoxide/commit/6d8d9b87db3b41a45343c14ad1b50f742d084f11))
    - more conversions for `TryFrom`: `String` and `&str` ([`a67fc26`](https://github.com/Byron/gitoxide/commit/a67fc26b80e5d1183ddc5c6598396214f3e19945))
    - remove `impl std::fmt::Display for Url` as it's not lossless. ([`79ab4ae`](https://github.com/Byron/gitoxide/commit/79ab4aeb8206a5f32735891336d7745e046bbea1))
    - `Url::write_to(out)` to write itself more flexibly. ([`833899d`](https://github.com/Byron/gitoxide/commit/833899dce120d26a2bbe04d07fc4c71455eb3afe))
    - Add `Url::to_bstring()` for lossless but fallible bstring conversion. ([`5f707c7`](https://github.com/Byron/gitoxide/commit/5f707c7e99c70ab9683d55c396e8dc11e1d3b0ea))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **[#524](https://github.com/Byron/gitoxide/issues/524)**
    - prepare changelogs prior to release ([`6446b39`](https://github.com/Byron/gitoxide/commit/6446b395d5926565ef899b0c923f35468ccf1921))
    - Introduce `parse(&BStr)` (previously it took `&[u8]`) ([`653ebc5`](https://github.com/Byron/gitoxide/commit/653ebc52f97116e9c72e985eda0d76f566e8c74d))
 * **[#691](https://github.com/Byron/gitoxide/issues/691)**
    - set `rust-version` to 1.64 ([`55066ce`](https://github.com/Byron/gitoxide/commit/55066ce5fd71209abb5d84da2998b903504584bb))
 * **[#725](https://github.com/Byron/gitoxide/issues/725)**
    - correctly parse scp-like ssh urls with alias as such. ([`372cea1`](https://github.com/Byron/gitoxide/commit/372cea13ad8abe1c47ed7bc806b42292a8ebfae0))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`41d57b9`](https://github.com/Byron/gitoxide/commit/41d57b98964094fc1528adb09f69ca824229bf25))
    - Release gix-attributes v0.8.2, gix-config-value v0.10.1, gix-tempfile v3.0.2, gix-lock v3.0.2, gix-validate v0.7.2, gix-object v0.26.1, gix-ref v0.24.0, gix-sec v0.6.2, gix-config v0.16.1, gix-command v0.2.3, gix-prompt v0.3.2, gix-url v0.13.2, gix-credentials v0.9.1, gix-diff v0.26.1, gix-discover v0.13.0, gix-hashtable v0.1.1, gix-bitmap v0.2.1, gix-traverse v0.22.1, gix-index v0.12.3, gix-mailmap v0.9.2, gix-chunk v0.4.1, gix-pack v0.30.2, gix-odb v0.40.2, gix-packetline v0.14.2, gix-transport v0.25.4, gix-protocol v0.26.3, gix-revision v0.10.3, gix-refspec v0.7.2, gix-worktree v0.12.2, gix v0.36.0 ([`e313112`](https://github.com/Byron/gitoxide/commit/e31311257bd138b52042dea5fc40c3abab7f269b))
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
    - rename `git-url` to `gix-url` ([`e95d72e`](https://github.com/Byron/gitoxide/commit/e95d72ed5b12b94a45f5ebfdea70a352b842cbec))
    - adjust to renaming of `git-date` to `gix-date` ([`9a79ff2`](https://github.com/Byron/gitoxide/commit/9a79ff2d5cc74c1efad9f41e21095ae498cce00b))
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
    - adjust to rename of `git-config-value` to `gix-config-value` ([`622b3e1`](https://github.com/Byron/gitoxide/commit/622b3e1d0bffa0f8db73697960f9712024fac430))
    - Release git-features v0.26.4 ([`109f434`](https://github.com/Byron/gitoxide/commit/109f434e66559a791d541f86876ded8df10766f1))
    - Release git-features v0.26.3 ([`1ecfb7f`](https://github.com/Byron/gitoxide/commit/1ecfb7f8bfb24432690d8f31367488f2e59a642a))
    - Release git-date v0.4.2, git-hash v0.10.2, git-features v0.26.2, git-actor v0.17.1, git-glob v0.5.3, git-path v0.7.1, git-quote v0.4.1, git-attributes v0.8.2, git-config-value v0.10.1, git-tempfile v3.0.2, git-lock v3.0.2, git-validate v0.7.2, git-object v0.26.1, git-ref v0.24.0, git-sec v0.6.2, git-config v0.16.0, git-command v0.2.3, git-prompt v0.3.2, git-url v0.13.2, git-credentials v0.9.1, git-diff v0.26.1, git-discover v0.13.0, git-hashtable v0.1.1, git-bitmap v0.2.1, git-traverse v0.22.1, git-index v0.12.3, git-mailmap v0.9.2, git-chunk v0.4.1, git-pack v0.30.2, git-odb v0.40.2, git-packetline v0.14.2, git-transport v0.25.4, git-protocol v0.26.3, git-revision v0.10.2, git-refspec v0.7.2, git-worktree v0.12.2, git-repository v0.34.0, safety bump 3 crates ([`c196d20`](https://github.com/Byron/gitoxide/commit/c196d206d57a310b1ce974a1cf0e7e6d6db5c4d6))
    - prepare changelogs prior to release ([`7c846d2`](https://github.com/Byron/gitoxide/commit/7c846d2102dc767366771925212712ef8cc9bf07))
    - Merge branch 'Lioness100/main' ([`1e544e8`](https://github.com/Byron/gitoxide/commit/1e544e82455bf9ecb5e3c2146280eaf7ecd81f16))
    - fix typos ([`39ed9ed`](https://github.com/Byron/gitoxide/commit/39ed9eda62b7718d5109135e5ad406fb1fe2978c))
    - thanks clippy ([`bac57dd`](https://github.com/Byron/gitoxide/commit/bac57dd05ea2d5a4ee45ef9350fa3f2e19474bc0))
    - Release git-date v0.4.1, git-features v0.26.1, git-glob v0.5.2, git-attributes v0.8.1, git-tempfile v3.0.1, git-ref v0.23.1, git-sec v0.6.1, git-config v0.15.1, git-prompt v0.3.1, git-url v0.13.1, git-discover v0.12.1, git-index v0.12.2, git-mailmap v0.9.1, git-pack v0.30.1, git-odb v0.40.1, git-transport v0.25.3, git-protocol v0.26.2, git-revision v0.10.1, git-refspec v0.7.1, git-worktree v0.12.1, git-repository v0.33.0 ([`5b5b380`](https://github.com/Byron/gitoxide/commit/5b5b3809faa71c658db38b40dfc410224d08a367))
    - prepare changelogs prior to release ([`93bef97`](https://github.com/Byron/gitoxide/commit/93bef97b3c0c75d4bf7119fdd787516e1efc77bf))
    - Merge branch 'patch-1' ([`b93f0c4`](https://github.com/Byron/gitoxide/commit/b93f0c49fc677b6c19aea332cbfc1445ce475375))
    - thanks clippy ([`9e04685`](https://github.com/Byron/gitoxide/commit/9e04685dd3f109bfb27663f9dc7c04102e660bf2))
    - Release git-features v0.26.0, git-actor v0.16.0, git-attributes v0.8.0, git-object v0.25.0, git-ref v0.22.0, git-config v0.14.0, git-command v0.2.1, git-url v0.13.0, git-credentials v0.9.0, git-diff v0.25.0, git-discover v0.11.0, git-traverse v0.21.0, git-index v0.11.0, git-mailmap v0.8.0, git-pack v0.29.0, git-odb v0.39.0, git-transport v0.25.0, git-protocol v0.26.0, git-revision v0.9.0, git-refspec v0.6.0, git-worktree v0.11.0, git-repository v0.31.0, safety bump 24 crates ([`5ac9fbe`](https://github.com/Byron/gitoxide/commit/5ac9fbe265a5b61c533a2a6b3abfed2bdf7f89ad))
    - prepare changelogs prior to release ([`30d8ca1`](https://github.com/Byron/gitoxide/commit/30d8ca19284049dcfbb0de2698cafae1d1a16b0c))
    - make fmt ([`511ed00`](https://github.com/Byron/gitoxide/commit/511ed0000397a5b268530c8f5362e7d25b7c1594))
    - Merge branch 'adjustments-for-cargo' ([`f8c562a`](https://github.com/Byron/gitoxide/commit/f8c562a559e6dc3377583cc7200585dad7c3d481))
    - collect ssh-specific options to control how the ssh program is invoked. ([`61d89f5`](https://github.com/Byron/gitoxide/commit/61d89f586a0ad913fc2f502520282520a5e1fd15))
    - Release git-features v0.25.1, git-url v0.12.2, git-odb v0.38.1, git-transport v0.24.2, git-repository v0.30.2 ([`bb0a07b`](https://github.com/Byron/gitoxide/commit/bb0a07b5edd5f980989d1a92e74df7f183febe87))
    - Merge branch 'fix/ssh-clone' ([`3678a6a`](https://github.com/Byron/gitoxide/commit/3678a6abab6f59ff7008ccfe02bb8d61da47e166))
    - properly set default SSH port to 22 ([`1058330`](https://github.com/Byron/gitoxide/commit/1058330adcc3262c59d30a0b8854fade20ffc3d5))
    - Release git-url v0.12.1, git-transport v0.24.1, git-protocol v0.25.1, git-repository v0.30.1, git-commitgraph v0.12.0, gitoxide-core v0.22.0, gitoxide v0.20.0 ([`08ec3a9`](https://github.com/Byron/gitoxide/commit/08ec3a93d77a1018439a5c41c23729ffed27c5a5))
    - prepare changelogs prior to release ([`68ce15d`](https://github.com/Byron/gitoxide/commit/68ce15d07b50cfacdac0d1e42fe7f5e6330ba523))
    - Merge branch 'fix/relative-scplike-urls' ([`b688592`](https://github.com/Byron/gitoxide/commit/b68859254a02b93e7ea90f4881323357cfd080a4))
    - don't sanitize SCP-like paths ([`d21f9eb`](https://github.com/Byron/gitoxide/commit/d21f9eb3d6f295ed25da0b55541f9535f144b3b4))
    - refactor ([`c70bf74`](https://github.com/Byron/gitoxide/commit/c70bf74e6625179d4555a2468d3b2492179d86bf))
    - Release git-date v0.3.1, git-features v0.25.0, git-actor v0.15.0, git-glob v0.5.1, git-path v0.7.0, git-attributes v0.7.0, git-config-value v0.10.0, git-lock v3.0.1, git-validate v0.7.1, git-object v0.24.0, git-ref v0.21.0, git-sec v0.6.0, git-config v0.13.0, git-prompt v0.3.0, git-url v0.12.0, git-credentials v0.8.0, git-diff v0.24.0, git-discover v0.10.0, git-traverse v0.20.0, git-index v0.10.0, git-mailmap v0.7.0, git-pack v0.28.0, git-odb v0.38.0, git-packetline v0.14.1, git-transport v0.24.0, git-protocol v0.25.0, git-revision v0.8.0, git-refspec v0.5.0, git-worktree v0.10.0, git-repository v0.30.0, safety bump 26 crates ([`e6b9906`](https://github.com/Byron/gitoxide/commit/e6b9906c486b11057936da16ed6e0ec450a0fb83))
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - scp-like URLs should preserve relative and home-relative paths ([`f20f272`](https://github.com/Byron/gitoxide/commit/f20f2728ee78d90510e27769a61ead405c4018c1))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - Merge branch 'main' into adjustments-for-cargo ([`bb60d3d`](https://github.com/Byron/gitoxide/commit/bb60d3d5cb9dbd7abe61accded6d21e320c624db))
    - reject empty paths where needed, add `Url::from_parts_as_alternative_form()`. ([`302a2d8`](https://github.com/Byron/gitoxide/commit/302a2d866692a541e01d112b6870aa22fcdbe32b))
    - make sure that `file:..` isn't considered a valid file url. ([`3e3aff9`](https://github.com/Byron/gitoxide/commit/3e3aff9f2f427d030a38fe147c5252d7bfd45109))
    - Merge branch 'paulyoung/scheme-ext' ([`3e27550`](https://github.com/Byron/gitoxide/commit/3e27550577ea942427a57c902570f0416f540753))
    - realign test expectations ([`93e6d71`](https://github.com/Byron/gitoxide/commit/93e6d7199408e492574c43fcfb81faccea2b6fd4))
    - improve documentation ([`db7577f`](https://github.com/Byron/gitoxide/commit/db7577ff348bbe9ffffcb1d5951c9dd579e111e3))
    - cargo fmt ([`3b61a47`](https://github.com/Byron/gitoxide/commit/3b61a47266abfb2145f64e8233eca12fa1d9cb65))
    - Allow parsing arbitrary URL schemes ([`4753e64`](https://github.com/Byron/gitoxide/commit/4753e641eada72f4e944811ea85390481444b210))
    - handle `file:///C:/foo/bar` urls correctly on windows, as paths now are `C:\\foo\bar`. ([`d6f90be`](https://github.com/Byron/gitoxide/commit/d6f90beac37866f992a1714d38e5b320eea6f1bb))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - Make a clearer note of the obvious deviation due to lack of storing passwords ([`d91bbcc`](https://github.com/Byron/gitoxide/commit/d91bbcc14b34166c79bba6faafd4395d6a571477))
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'clone' ([`507dc7e`](https://github.com/Byron/gitoxide/commit/507dc7e706cb3c9d89d048b3aff5df239a9b6788))
    - `Url::try_from(path: &std::path::Path)` for more convenient instantiation. ([`22d3b37`](https://github.com/Byron/gitoxide/commit/22d3b37ea6239170a478b859361a7d1d7ba01a9a))
    - more assurance we understand how relative paths in scp-like urls work ([`5926322`](https://github.com/Byron/gitoxide/commit/5926322c7dc9ef45c0f8c7dc50551d0bf1800ada))
    - (mostly) lossless roundtripping of scp-like urls. ([`39ce98b`](https://github.com/Byron/gitoxide/commit/39ce98ba9a427b8cea1b843f333c2e7de300499c))
    - lossless serialization of file urls. ([`58a6000`](https://github.com/Byron/gitoxide/commit/58a6000d669acd33bad91509eaa469f041f119e5))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - thanks clippy ([`dc74fbd`](https://github.com/Byron/gitoxide/commit/dc74fbd9a58e1d424713fc5f2442cedcc09c1200))
    - make Scheme work with serde, removing `Copy` in the process.  e#450) ([`2da6c86`](https://github.com/Byron/gitoxide/commit/2da6c862e184ac37d59147e9cf809017b65db966))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - assure document-features are available in all 'usable' and 'early' crates ([`238581c`](https://github.com/Byron/gitoxide/commit/238581cc46c7288691eed37dc7de5069e3d86721))
    - `From<&[u8]>` is now `From<&BStr>` ([`ffc4a85`](https://github.com/Byron/gitoxide/commit/ffc4a85b9a914b685d7ab528b30f2a3eefb44094))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'svetli-n-path_value' ([`e8383ca`](https://github.com/Byron/gitoxide/commit/e8383caf6db211beb57d70019fe4ad13ce9066ee))
    - Merge branch 'unify-path-encoding' ([`566ff8a`](https://github.com/Byron/gitoxide/commit/566ff8a3597b889899d41ca15e5b9af7e05f1a4b))
    - Release git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`42ebb53`](https://github.com/Byron/gitoxide/commit/42ebb536cd6086f096b8422291776c9720fa0948))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepare changelogs for release ([`674ec73`](https://github.com/Byron/gitoxide/commit/674ec73b0816baa2c63b4ef1b40b7a41849c5e95))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - thanks clippy ([`4ca9e07`](https://github.com/Byron/gitoxide/commit/4ca9e07c7ac062d48d64ad7b516274e32dbc51c6))
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Release git-url v0.3.3 ([`fdd5bdb`](https://github.com/Byron/gitoxide/commit/fdd5bdb1bedc9a5d10ee69d315c11860d3f2468b))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
    - (cargo-release) version 0.3.2 ([`03de99e`](https://github.com/Byron/gitoxide/commit/03de99e31fae18cabab19baafc78b2bef8b6a493))
    - (cargo-release) version 0.3.1 ([`4deef67`](https://github.com/Byron/gitoxide/commit/4deef67a2259a0bf0e2cfa7d027e082240c67733))
    - Fix compile warnings ([`42fd77b`](https://github.com/Byron/gitoxide/commit/42fd77b790eade874c559ed0bed14530ecda66d1))
    - (cargo-release) version 0.3.0 ([`d5c6643`](https://github.com/Byron/gitoxide/commit/d5c6643a41d295eaf7aabb84eab435e42a11dd42))
    - thanks clippy ([`e13adb2`](https://github.com/Byron/gitoxide/commit/e13adb2b51e634ee5085038c3b1eaecfd6c43715))
    - [gitoxide-core] Use git-config for remote url parsing ([`c45feed`](https://github.com/Byron/gitoxide/commit/c45feed6124601a8bbef609d5b47c5b8a9d5defa))
    - (cargo-release) version 0.2.0 ([`0c39373`](https://github.com/Byron/gitoxide/commit/0c39373de5aba0acc4aaa330bf51b6abd4f50474))
    - support for radicle urls ([`2c5b955`](https://github.com/Byron/gitoxide/commit/2c5b955b07073c5ef0e7bbe3ab20f0047770440b))
    - (cargo-release) version 0.1.1 ([`e94fefa`](https://github.com/Byron/gitoxide/commit/e94fefaf5e7a10605fa7ca46b2ce84a60b149aa0))
    - finish git-url docs ([`4099508`](https://github.com/Byron/gitoxide/commit/4099508ae32a4cce1a110d68c094d8c9002d8835))
    - begin of documenting git-url crate ([`c891901`](https://github.com/Byron/gitoxide/commit/c891901f1e7b2e0300bb7ae243c3579ced76c5e0))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - Finish removal of rust 2018 idioms ([`0d1699e`](https://github.com/Byron/gitoxide/commit/0d1699e0e0bc9052be0a74b1b3f3d3eeeec39e3e))
    - refactor ([`e07fbd6`](https://github.com/Byron/gitoxide/commit/e07fbd63db297cd9f70f8b86b1f1f56b15e270a8))
    - [clone] encode message for git credentials helper ([`143549e`](https://github.com/Byron/gitoxide/commit/143549e0757d4fa7a8347aa1b8b4734e9b62bf04))
    - [clone] make URL available in transport layer ([`6778447`](https://github.com/Byron/gitoxide/commit/67784478b96f8afd142e52982e2161a1f05d2ec9))
    - [clone] Finish round-trip testing ([`df617fd`](https://github.com/Byron/gitoxide/commit/df617fd8685e2efb9e897bc94a2dad163f0c9f2e))
    - refactor ([`aea52fe`](https://github.com/Byron/gitoxide/commit/aea52fe24168e20cd2949b7c4dd70abc88082429))
    - [clone] first sketch of roundtripping URLs ([`23678f8`](https://github.com/Byron/gitoxide/commit/23678f8d91dd88cc4b797821cdc16af494044c0f))
    - [clone] first steps towards launching git-upload-pack while… ([`41f05f1`](https://github.com/Byron/gitoxide/commit/41f05f13a1fac078b694e6f4a9c8f52eeaff4191))
    - [clone] Better error handling for generalized `connect(…)` ([`713808c`](https://github.com/Byron/gitoxide/commit/713808cd8bd326b632c2b8f0cfbe7f147b1fa0aa))
    - [clone] expand-path should be server-side ([`8a38856`](https://github.com/Byron/gitoxide/commit/8a38856a811078d1d453db9c0e0ad7b6baaaed3c))
    - thanks clippy ([`0506fd9`](https://github.com/Byron/gitoxide/commit/0506fd92aadec7c92747fb80c0aa6fe68908bc5c))
    - [url] more specific 'missing user home' error ([`ec5721a`](https://github.com/Byron/gitoxide/commit/ec5721a7d153da1cc628de2bb20de8f723140a54))
    - refactor ([`e54681a`](https://github.com/Byron/gitoxide/commit/e54681aef693bfd4b0d5dfd385b6fb8cc150376b))
    - [url] Actually the is_relative() case should never be triggered ([`ac89d38`](https://github.com/Byron/gitoxide/commit/ac89d38c6af96b2ae834df00451ad22a8947d43b))
    - [url] try again, maybe this works on windows… ([`f14fdd1`](https://github.com/Byron/gitoxide/commit/f14fdd12fafec6b12feb2ae6ab965793f20ee2c5))
    - [url] Once more with feeling ([`2ea4a8c`](https://github.com/Byron/gitoxide/commit/2ea4a8cb515c3cb8b8273648ebf367324cfec6ae))
    - [url] all debug output there is… ([`3df5b41`](https://github.com/Byron/gitoxide/commit/3df5b41d33b54c87bdda663723253b66179148fe))
    - [url] yikes, more debugging for windows on CI ([`9a430e7`](https://github.com/Byron/gitoxide/commit/9a430e77a428be5b5e499a7fc28ed88860cafe68))
    - [url] Another try to make this work on windows - tests probably ([`a51647f`](https://github.com/Byron/gitoxide/commit/a51647fc8b2297e54ac2ac37f15a7c603ff92d1b))
    - [url] See if this fixes the windows tests ([`534c6a6`](https://github.com/Byron/gitoxide/commit/534c6a67cd98944215e17a5f21490aa06a9f2113))
    - [url]  add standard conversions ([`27e3bdc`](https://github.com/Byron/gitoxide/commit/27e3bdcfc1fe4ceabfe5aca2d55d68a005756cca))
    - refactor ([`73e2b1b`](https://github.com/Byron/gitoxide/commit/73e2b1b16ed5ba584a67488cb481ea13f54c0488))
    - [url] BString in public interface ([`745662d`](https://github.com/Byron/gitoxide/commit/745662da413a0d5379d40a1e26b131477393d26f))
    - [url] Commit to 'bstr' ([`3d26ae1`](https://github.com/Byron/gitoxide/commit/3d26ae1dfaac44054705a3ab3ae5e00ce98298dd))
    - [url] remove feature toggle, 'home' dependency is small enough ([`a5a6f0f`](https://github.com/Byron/gitoxide/commit/a5a6f0fc7f193a3eed0992f90a6f37348fd47830))
    - [url] Add user expansion support (behind feature toggle) ([`a684cfe`](https://github.com/Byron/gitoxide/commit/a684cfe05f6fa33e674ce7a521179e7f65f84705))
    - [url] first stab at expanding paths with user names ([`37459dc`](https://github.com/Byron/gitoxide/commit/37459dcac513b6123157eefe4942a9610a1192ed))
    - thanks clippy ([`50acab7`](https://github.com/Byron/gitoxide/commit/50acab74e57911b1a10dda4a8c2823db5ae1fa2b))
    - [url] Support for git and http urls, as well as user expansion parsing ([`5ef201d`](https://github.com/Byron/gitoxide/commit/5ef201db248d60656b949f59d10b539499459cff))
    - refactor ([`6ab7cc6`](https://github.com/Byron/gitoxide/commit/6ab7cc6c330b3d32cb85cfa3fba63c0e145104b7))
    - [url] first stab at implementing username expansion reasonably ([`86d17a3`](https://github.com/Byron/gitoxide/commit/86d17a3da3330c495b7ec7e53aca50bf864723f7))
    - [url] fix serde ([`569014d`](https://github.com/Byron/gitoxide/commit/569014d49514c744947b84e47be4dda46d2bcca3))
    - [url] Now with support for non-utf8 byte strings ([`81f01fd`](https://github.com/Byron/gitoxide/commit/81f01fde78cb173d7bcdcfa8f22800e69e7981dd))
    - [url] more tests and additional limitations ([`3c2811f`](https://github.com/Byron/gitoxide/commit/3c2811f8fedc9b0018e3bb01e81b365543d65505))
    - [url] handle trivial file protocol URLs better ([`18eb512`](https://github.com/Byron/gitoxide/commit/18eb51286e12608f030cde10646d6502e0dbf427))
    - [url] Disable URL parsing for things that look like paths ([`03b0de9`](https://github.com/Byron/gitoxide/commit/03b0de94c2d85484f474f6a780d564b28de98c8a))
    - [url] turns out that relative URLs and windows paths are killing it ([`0bee58e`](https://github.com/Byron/gitoxide/commit/0bee58e66e24ce6002d4a7eeee86f92146bbee16))
    - [url] Switch to 'url' crate, as correctness certainly is more important than compile times ([`da6ad48`](https://github.com/Byron/gitoxide/commit/da6ad48e48dbc619e1195d0ac10059c8a04e993e))
    - thanks clippy ([`a37c7a3`](https://github.com/Byron/gitoxide/commit/a37c7a37524b2a3a5ef853765832ac9a30ae8f2d))
    - [url] user and IPv4 parsing/simple validation ([`d1929ac`](https://github.com/Byron/gitoxide/commit/d1929ac319767e7846f595dffb3a886abdafa87f))
    - [url] parse port number ([`bc8bd99`](https://github.com/Byron/gitoxide/commit/bc8bd99335ba2502dd5ad7a1d54005c2093156cf))
    - try for leaner tests, but it does the opposite kind of :D ([`098f802`](https://github.com/Byron/gitoxide/commit/098f802e6dc9f55632791ddf8d046563f75cba7a))
    - refactor ([`4499a08`](https://github.com/Byron/gitoxide/commit/4499a08d1c54daaab643fce054141bc8fcc754be))
    - refactor ([`42a1b51`](https://github.com/Byron/gitoxide/commit/42a1b5150fee8d06e17f33fb04b56dac630f7b69))
    - [url] the first green tests ([`a501bc1`](https://github.com/Byron/gitoxide/commit/a501bc19c0a2ad1b4ee576379841fea2af6db6cc))
    - refactor ([`9c5fb91`](https://github.com/Byron/gitoxide/commit/9c5fb91bde4f3562566c0528307cd74f056fe5ce))
    - [url] infrastructure for nom errors, taken from git-object ([`0ae38ed`](https://github.com/Byron/gitoxide/commit/0ae38edcfd2c7b9c6793c0bc21e88e9d4d19a6b1))
    - [url] basic frame and first failing test ([`60aacf0`](https://github.com/Byron/gitoxide/commit/60aacf0c279d277c4abf13e62697a51feeee26fd))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
    - add git-url crate ([`fd2e5ba`](https://github.com/Byron/gitoxide/commit/fd2e5bab97f09666c983634fa89947a4bed1c92d))
</details>

## 0.13.1 (2023-01-10)

A maintenance release without user-facing changes.

## 0.13.0 (2022-12-30)

### New Features

 - <csr-id-61d89f586a0ad913fc2f502520282520a5e1fd15/> collect ssh-specific options to control how the ssh program is invoked.
   These are passed through when creating the ssh transport.

## 0.12.2 (2022-12-26)

### Bug Fixes

 - <csr-id-1058330adcc3262c59d30a0b8854fade20ffc3d5/> properly set default SSH port to 22

## 0.12.1 (2022-12-22)

A maintenance release without user-facing changes.

## 0.12.0 (2022-12-19)

### Bug Fixes

 - <csr-id-302a2d866692a541e01d112b6870aa22fcdbe32b/> reject empty paths where needed, add `Url::from_parts_as_alternative_form()`.
   The new constructor allows to create URLs that represent paths which otherwise couldn't
   be valid URLs.
 - <csr-id-3e3aff9f2f427d030a38fe147c5252d7bfd45109/> make sure that `file:..` isn't considered a valid file url.
 - <csr-id-d6f90beac37866f992a1714d38e5b320eea6f1bb/> handle `file:///C:/foo/bar` urls correctly on windows, as paths now are `C:\\foo\bar`.
   These paths are created when using the `url::Url::from_file_path()`
   family of methods, which adds an extra slash at the beginning of a
   windows path which makes it invalid there unless there is further
   processing.
   
   This is now applied by using `url` features, making this case work
   specifically. Note that all other attributes are still the same
   and `gix-url` generally tries to keep paths in tact to be a hybrid
   of type that can handle any file system paths as well as actual urls.
 - <csr-id-f20f2728ee78d90510e27769a61ead405c4018c1/> scp-like URLs should preserve relative and home-relative paths

## 0.11.0 (2022-11-21)

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

## 0.10.1 (2022-11-06)

### New Features

 - <csr-id-01f25744bba45a5f8a8615734a5beeacd29d1c4e/> add `Url::canonicalized()` and `Url::canonicalize()`.
   These methods allow to assure file urls are absolute, useful when
   cloning from any url.

## 0.10.0 (2022-10-10)

### New Features

 - <csr-id-22d3b37ea6239170a478b859361a7d1d7ba01a9a/> `Url::try_from(path: &std::path::Path)` for more convenient instantiation.
 - <csr-id-39ce98ba9a427b8cea1b843f333c2e7de300499c/> (mostly) lossless roundtripping of scp-like urls.
   Previously `git@host:path` would turn into `ssh://git@host/path`,
   which now remains exactly as is.
 - <csr-id-58a6000d669acd33bad91509eaa469f041f119e5/> lossless serialization of file urls.
   Previously a url like `/path/to/repo` would serialize to
   `file:///path/to/repo`, preventing round-trips.
   
   Now it serializes like it was parsed. This also means that
   `file://path` still serializes as `file://path`.

## 0.9.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

## 0.8.0 (2022-09-04)

A major release to properly introduce the signature change that happened in 0.7.2, which effectively
broke compilation for users of `parse()` in 0.7.1.

### New Features

 - <csr-id-7484db5d36383de450de31b4c94c01bc4c237ce4/> `Url::port_or_default()` to fill in default numbers for ports if possible.
 - <csr-id-fbe75c9457708b95dd833e00afa2dcc1db677167/> `Url::path_is_root()` to determine if the path is `/`.
   This could also be considered an empty path depending on the context
   which is what makes it useful.

### Changed (BREAKING)

 - <csr-id-653ebc52f97116e9c72e985eda0d76f566e8c74d/> Introduce `parse(&BStr)` (previously it took `&[u8]`)
   A `&BStr` better indicates that we are expecting human-readable input
   with ascii-compatible or UTF-8 endcoding.

## 0.7.3 (2022-08-28)

Maintenance release without user-facing changes.

## 0.7.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### Bug Fixes (BREAKING)

 - <csr-id-2da6c862e184ac37d59147e9cf809017b65db966/> make Scheme work with serde, removing `Copy` in the process.  e#450)
   This wasn't supposed to happen but a requirement to get `serde` support
   back.

### New Features (BREAKING)

 - <csr-id-96a265cc67ea787ed28adde2c5d0a07babf64c9e/> generalize extension schemes.
   Previously this was hard-coded to `radicle`, now it's just an extension
   scheme along with a statically known string. This means we have to
   explicitly support new formats which should be fine.

### New Features

 - <csr-id-7a1769009d68d14a134f368f93245abab0fb41dd/> `TryFrom<&OsStr>` to allow direct usage of `Url` in `clap`.
   In `clap_derive`, this needs
   `parse(try_from_os_str = std::convert::TryFrom::try_from)`.
 - <csr-id-b7a5f7a3b5cf058f503cc18d18fc75356ab98955/> `TryFrom<PathBuf>` which is useful if urls are obtained from the command-line.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs
 - <csr-id-d40f6e1f34eb3f4664caec36727bf0aa3a396a33/> `Scheme::try_from(&str)`

## 0.7.1 (2022-08-17)

A maintenance release without user facing changes.

### Bug Fixes (BREAKING)

 - <csr-id-2bcfdee6a3af758a0b70e2af9c4b6f8cc09d8da0/> Prohibit invalid state by making parts the url's data private
   This fix is meant to improve serialization support which can now happen
   `to_bstring()` without possibility for error.
   
   Empty paths can still be set which won't be valid for all URLs.

### Changed (BREAKING)

 - <csr-id-f6506e0c463bdccbcfd9324bc312da9cc957d8e6/> Use `&BStr` as input instead of `[u8]`
   This signals that it's indeed intended to be human readable while
   allowing it to be a path as well without loss, at least theoretically.
   After all we currently don't have a way to parse invalid UTF-8.
 - <csr-id-79ab4aeb8206a5f32735891336d7745e046bbea1/> remove `impl std::fmt::Display for Url` as it's not lossless.

### New Features

 - <csr-id-a67fc26b80e5d1183ddc5c6598396214f3e19945/> more conversions for `TryFrom`: `String` and `&str`
 - <csr-id-833899dce120d26a2bbe04d07fc4c71455eb3afe/> `Url::write_to(out)` to write itself more flexibly.
 - <csr-id-5f707c7e99c70ab9683d55c396e8dc11e1d3b0ea/> Add `Url::to_bstring()` for lossless but fallible bstring conversion.

## 0.7.0 (2022-07-22)

### Changed (BREAKING)

 - <csr-id-ffc4a85b9a914b685d7ab528b30f2a3eefb44094/> `From<&[u8]>` is now `From<&BStr>`
   This better represents the meaning of the input, and simplifies
   interactions with `gix-config`.

## 0.6.0 (2022-06-13)

A maintenance release without user-facing changes.

## 0.5.0 (2022-05-18)

A maintenance release without user-facing changes.

## 0.4.0 (2022-04-03)

A maintenance release without surfacing changes.

## 0.3.5 (2022-01-23)

A maintenance release with no relevant changes.

## v0.3.4 (2021-10-15)

This release contains no functional change, but a more useful changelog.

## v0.3.3 (2021-08-17)

## v0.3.2 (2021-08-10)

## v0.3.0 (2021-03-26)

## v0.2.0 (2021-01-14)

## v0.1.1 (2020-12-18)

## v0.1.0 (2020-09-12)

<csr-id-098f802e6dc9f55632791ddf8d046563f75cba7a/>

### Other

 - <csr-id-098f802e6dc9f55632791ddf8d046563f75cba7a/> try for leaner tests, but it does the opposite kind of :D

## v0.0.0 (2020-08-13)

