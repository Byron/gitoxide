# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 22 calendar days.
 - 22 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#436](https://github.com/Byron/gitoxide/issues/436)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#436](https://github.com/Byron/gitoxide/issues/436)**
    - Remove outdated examples ([`cb9529e`](https://github.com/Byron/gitoxide/commit/cb9529e18b222b9fd9f8c1bb0dba8038a6ea1d4b))
 * **Uncategorized**
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - remove `pwd` crate dependency in favor of using libc directly ([`4adfa11`](https://github.com/Byron/gitoxide/commit/4adfa11d70cf78bed541fa59707e8a5082dda245))
    - Drop non-existent config paths before parsing ([`475d6fa`](https://github.com/Byron/gitoxide/commit/475d6fab2467ad0499db7df2d4c99f74e43682fc))
    - Merge branch 'main' into davidkna-envopen ([`bc0abc6`](https://github.com/Byron/gitoxide/commit/bc0abc643d3329f885f250b6880560dec861150f))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - Merge branch 'davidkna-discover-x-fs' ([`9abaeda`](https://github.com/Byron/gitoxide/commit/9abaeda2d22e2dbb1db1632c6eb637f1458d06e1))
</details>

## 0.4.0 (2022-05-21)

### Changed (BREAKING)

 - <csr-id-553f87225363903e6acdb3e7eaa8cc66a91110f1/> `File::len()` -> `File::num_values()`
   The same is true for `Section::len()` which now is
   `Section::num_values()`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - fix benchmark compilation ([`53adcfe`](https://github.com/Byron/gitoxide/commit/53adcfea1942e9dd32a7d84d02a83c9a08408fad))
    - Bring init functions back to `File` type ([`f1f69d8`](https://github.com/Byron/gitoxide/commit/f1f69d8f983e2505990e7ee21cbd7f64ac7ba766))
    - disallow Rust 2018 idioms ([`81aca45`](https://github.com/Byron/gitoxide/commit/81aca458f4b7f6768e14da5719ada772f419f1b5))
    - fix most of docs ([`1fe053f`](https://github.com/Byron/gitoxide/commit/1fe053f60fa4843e7da6a6328fc293b4bcd25277))
    - thanks clippy ([`409a95b`](https://github.com/Byron/gitoxide/commit/409a95b6505db8568bfea24bc62c92640a5c3cbf))
    - dissolve git_config module in favor of `file` module ([`2d4a19b`](https://github.com/Byron/gitoxide/commit/2d4a19b0c72c4aab79cd3b18430710909ba1af5f))
    - refactor ([`6cc5c20`](https://github.com/Byron/gitoxide/commit/6cc5c20aba825a5a712b33740ea2c7f44387f3f9))
    - refactor ([`3471f95`](https://github.com/Byron/gitoxide/commit/3471f95b5e490d22bb42b6c4204446c52812e4fc))
    - `File::len()` -> `File::num_values()` ([`553f872`](https://github.com/Byron/gitoxide/commit/553f87225363903e6acdb3e7eaa8cc66a91110f1))
    - refactor ([`2626e0c`](https://github.com/Byron/gitoxide/commit/2626e0ca58947eb846128507ffb254e9ebd91ee1))
    - refactor ([`07e0f5e`](https://github.com/Byron/gitoxide/commit/07e0f5e91b3c41614b9182cf9716120fe41ddf40))
    - Split git_config into modules. ([`a85d864`](https://github.com/Byron/gitoxide/commit/a85d8643cbfbfc4bd4d4c1fb17ae3672b8b36931))
    - Fix linux test. ([`e0d063e`](https://github.com/Byron/gitoxide/commit/e0d063ebdfa8effabd53c6a51818617abe4a0b4e))
    - Fix test. ([`ed5de9e`](https://github.com/Byron/gitoxide/commit/ed5de9e8d2e225313ef8e60003797c5466d81273))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - Temp ignore test. ([`9b70eca`](https://github.com/Byron/gitoxide/commit/9b70eca08aaa36e3f803da1685ac85bab40f0b03))
    - Tryfix windows test. Includes module. ([`b02d147`](https://github.com/Byron/gitoxide/commit/b02d147468f902597d4022c1fce3424213cb9eb8))
    - Tryfix windows test. ([`4098278`](https://github.com/Byron/gitoxide/commit/40982788f88267f0885513fffb112467e2f3b370))
    - Tryfix windows test. ([`17a296f`](https://github.com/Byron/gitoxide/commit/17a296ffc5af08c6c0455b3028d275b9ebe7c18c))
    - Tryfix windows test. ([`a29657a`](https://github.com/Byron/gitoxide/commit/a29657a8118300f11db4e0783800eeadf838532c))
    - Fix merge. ([`07bc9a8`](https://github.com/Byron/gitoxide/commit/07bc9a869d501b78c060e4ed18d4003c287560a8))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
    - Tryfix windows test. ([`300ecbc`](https://github.com/Byron/gitoxide/commit/300ecbc75fbf8d94d7e21a35e12b93f0a954515d))
</details>

## 0.3.0 (2022-05-18)

### Bug Fixes

 - <csr-id-36e2fc0aff4e6aaa35335da90108918882d4cd16/> Use `std::env::var_os()` to avoid potential decode errors

### Changed

 - <csr-id-b04a3465ed20d8f3088e5d3faf11e98e5595f219/> `GitConfig::from_paths(<paths>, …)` accepts more inputs
   `<paths>` is more flexible and is easier to use.

### New Features

 - <csr-id-7c75eac149c6ecb99c3dd7355d76d8d3e8b59cd0/> `GitConfig::path()` for direct access to paths.
   Very similar to `string()`, but as path, whose query can never fail.
 - <csr-id-031bd2f401199a05d6465c0260ceed3cc849c7ac/> add suppport for android
   Do not interpolate `~user/` on Android (Termux).
   There is no meaning of it. It is single user system.
 - <csr-id-dc3dc3b41b5de3ec17429769747bf99bb2bdd03d/> support for `try_value()`, `boolean()` and `string()` access`.
   Support for a convenient way of knowing if a value does or doesn't exist
   via `try_value()`, which can only fail if the conversion fails.
   
   Lastly, `string()` is a special case which doesn't fail as there is
   no conversion, and `boolean()` allows to obtain a plain boolean value
   if it was a valid boolean representation.
 - <csr-id-13554f8d21beb241e0fbdeb56b8414957cbee28a/> new hierarchical errors for value lookup
 - <csr-id-4726bb524c1b0935d35770c907d40a0a16dbb8b5/> `GitConfig::integers()`
   Get multiple fully validated integer values, with their suffix
   interpreted and checked for overflow.
 - <csr-id-ae22a4de486676f11469cec84be403903758b48b/> add `GitConfig::integer()`
   A way to quickly obtain a valid integer with suffixes resolved
   and overflow checked.
 - <csr-id-bfc263797226d027e04daaf6426e57183773d7c3/> `GitConfig::strings()` for multi-value strings.

### Changed (BREAKING)

 - <csr-id-38dfdcf80f9b7368ccaa10f4b78b2129849848d0/> remove `values::*Error` in favor of `value::parse::Error`.
   This makes it easier to work with errors in practice, we are either
   interested in the value that failed to parse to try something else
   or want a nice user message.
   
   Having one decode error type facilitates that.
 - <csr-id-a98a7a7af69482e9ef63f106184049049939459d/> switch from quickerror to thiserror.
   This allows for generic types for sources of errors and allows to
   workaround a limitation with associated type constraints in the MSRV
   of 1.54.
   
   Using thiserror makes this work and brings the crate more closely
   to the rest of the gitoxide crates (which now prefer thiserror over
   quickerror).
 - <csr-id-a86b2541561674df5dbef4120d3e03483cb80117/> remove all `get_` prefixes from methods
   That way the API is more idiomatic and fits better into the
   existing `gitoxide` crates.
 - <csr-id-f9aaac11f0734afbd791132369eb5601bfc7efe9/> use `lookup::Error` and `lookup::existing::Error`
   Use the newly introduced structured error to replace the 'catch-all'
   `GitConfigError` while getting closer to naming conventions in other
   `gitoxide` crates.
 - <csr-id-c7fcb5e1db225aefc3eeab4f29f3fb85c670894a/> `GitConfig::from_paths(…, <option>)` is now owned.
   The type is `Copy`, so no need to pass it by reference.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 86 commits contributed to the release over the course of 40 calendar days.
 - 43 days passed between releases.
 - 14 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#386](https://github.com/Byron/gitoxide/issues/386), [#404](https://github.com/Byron/gitoxide/issues/404)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade dependencies ([`b039d39`](https://github.com/Byron/gitoxide/commit/b039d39613bb14d49670c4d8b586f76ffb420d03))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - finished refactoring ([`4163c7f`](https://github.com/Byron/gitoxide/commit/4163c7fe0a98b77998fa263458d06bdeb435996d))
    - refactor ([`a359cfd`](https://github.com/Byron/gitoxide/commit/a359cfd86ffae9feab11b45e3167fe28f22dbac8))
    - `GitConfig::from_paths(…, <option>)` is now owned. ([`c7fcb5e`](https://github.com/Byron/gitoxide/commit/c7fcb5e1db225aefc3eeab4f29f3fb85c670894a))
    - Use `std::env::var_os()` to avoid potential decode errors ([`36e2fc0`](https://github.com/Byron/gitoxide/commit/36e2fc0aff4e6aaa35335da90108918882d4cd16))
    - `GitConfig::from_paths(<paths>, …)` accepts more inputs ([`b04a346`](https://github.com/Byron/gitoxide/commit/b04a3465ed20d8f3088e5d3faf11e98e5595f219))
    - refactor unconditional include tests and stabilize them ([`72a5a02`](https://github.com/Byron/gitoxide/commit/72a5a027dd8120b27909efea339dfc7919a865be))
    - `GitConfig::integers()` ([`4726bb5`](https://github.com/Byron/gitoxide/commit/4726bb524c1b0935d35770c907d40a0a16dbb8b5))
    - add `GitConfig::integer()` ([`ae22a4d`](https://github.com/Byron/gitoxide/commit/ae22a4de486676f11469cec84be403903758b48b))
    - refactor ([`c139479`](https://github.com/Byron/gitoxide/commit/c13947977205828dcda177686362e25867fdfe44))
    - refactor ([`4408f17`](https://github.com/Byron/gitoxide/commit/4408f17736052c899a9c98af41485d7dde9a297f))
    - `GitConfig::strings()` for multi-value strings. ([`bfc2637`](https://github.com/Byron/gitoxide/commit/bfc263797226d027e04daaf6426e57183773d7c3))
    - refactor ([`7ea17e1`](https://github.com/Byron/gitoxide/commit/7ea17e1e16346239032844b8f4be9e9c22c6be8e))
    - initial refactoring ([`43a34a5`](https://github.com/Byron/gitoxide/commit/43a34a5bdae53fbb53d3ae095f03c9456115a013))
    - Some notes about of 'path' will soon have to be amended with more safety ([`97e53f6`](https://github.com/Byron/gitoxide/commit/97e53f63df2c0262f23af3d7d997f148d23474be))
    - `GitConfig::path()` for direct access to paths. ([`7c75eac`](https://github.com/Byron/gitoxide/commit/7c75eac149c6ecb99c3dd7355d76d8d3e8b59cd0))
    - remove `values::*Error` in favor of `value::parse::Error`. ([`38dfdcf`](https://github.com/Byron/gitoxide/commit/38dfdcf80f9b7368ccaa10f4b78b2129849848d0))
    - A sketch of what can be a general value decode error ([`4612fca`](https://github.com/Byron/gitoxide/commit/4612fca79446c6f92f0e6a4163bc895fc346b30d))
    - Remove IntegerSuffix error which wasn't ever used ([`732c0fa`](https://github.com/Byron/gitoxide/commit/732c0fa6e1832efcc0de4adc894e820b3bd27b8f))
    - support for `try_value()`, `boolean()` and `string()` access`. ([`dc3dc3b`](https://github.com/Byron/gitoxide/commit/dc3dc3b41b5de3ec17429769747bf99bb2bdd03d))
    - fix build warnings ([`4496b5a`](https://github.com/Byron/gitoxide/commit/4496b5a26abaf91fd4844e0494aaa1b4cce73628))
    - switch from quickerror to thiserror. ([`a98a7a7`](https://github.com/Byron/gitoxide/commit/a98a7a7af69482e9ef63f106184049049939459d))
    - remove all #[inline] attributes ([`8aef1d3`](https://github.com/Byron/gitoxide/commit/8aef1d313dc9d3ac0004e790b6f91ad0c7ac99b0))
    - remove all `get_` prefixes from methods ([`a86b254`](https://github.com/Byron/gitoxide/commit/a86b2541561674df5dbef4120d3e03483cb80117))
    - use `lookup::Error` and `lookup::existing::Error` ([`f9aaac1`](https://github.com/Byron/gitoxide/commit/f9aaac11f0734afbd791132369eb5601bfc7efe9))
    - new hierarchical errors for value lookup ([`13554f8`](https://github.com/Byron/gitoxide/commit/13554f8d21beb241e0fbdeb56b8414957cbee28a))
    - adapt to changes in git-path ([`cc2d810`](https://github.com/Byron/gitoxide/commit/cc2d81012d107da7a61bf4de5b28342dea5083b7))
    - adapt to all changes in git-path with bstr support ([`f158648`](https://github.com/Byron/gitoxide/commit/f158648aef8ad94d86550ceb2eeb20efb3df7596))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - make fmt ([`5fc5459`](https://github.com/Byron/gitoxide/commit/5fc5459b17b623726f99846c432a70106464e970))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - refactor ([`92fe564`](https://github.com/Byron/gitoxide/commit/92fe56486c349a4b08bcefa3e3355c591e281afb))
    - Remove untested error case in integger parsing ([`2b21a35`](https://github.com/Byron/gitoxide/commit/2b21a35e1ba31caea227515ddebc7608cdcca245))
    - validate underflow as well ([`83eda34`](https://github.com/Byron/gitoxide/commit/83eda3443a1b64ff7bc672fbfe16e3a69def1c6d))
    - Case-insensitive integer suffix handling ([`9034bd4`](https://github.com/Byron/gitoxide/commit/9034bd45bba0aa7c6c5691c2e592c389949dd5d6))
    - refactor tests ([`f943d2a`](https://github.com/Byron/gitoxide/commit/f943d2aeb0773752adbb68d731305586ab2ce686))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - Sketch `Permissions` for git-config ([`8443330`](https://github.com/Byron/gitoxide/commit/8443330b051c109742fe55928e2afd36fc0172fd))
 * **[#404](https://github.com/Byron/gitoxide/issues/404)**
    - Add test to clarify underscores in sections headers aren't allowed ([`47079d4`](https://github.com/Byron/gitoxide/commit/47079d470e44b1adf515ae4df2bed945b7e91108))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Temp ignore test ([`6f35866`](https://github.com/Byron/gitoxide/commit/6f35866725f52eee368042a4293ac74a82752331))
    - update lifetime ([`fd24e2c`](https://github.com/Byron/gitoxide/commit/fd24e2cc1c10ef5c65a5b923ad30806e91427117))
    - Add includeIf test with symlink. ([`5d74404`](https://github.com/Byron/gitoxide/commit/5d744049286632f3141ec07fa3f128093480d1c0))
    - Refactor condition match. ([`15ac22a`](https://github.com/Byron/gitoxide/commit/15ac22a9b28577d2c4175bc752eb7099a3b128fa))
    - Fix realpath tests. ([`0426f4d`](https://github.com/Byron/gitoxide/commit/0426f4deb5d73fd88529530f9a6d01ba55eeadc4))
    - thanks clippy ([`da13aff`](https://github.com/Byron/gitoxide/commit/da13affabe34c3d691b18a70ce61eb00319668c5))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - vec -> Option in tests ([`538de54`](https://github.com/Byron/gitoxide/commit/538de54aab0ab0352fbff95e0334c89c415627e9))
    - Tests use `options_with_git_dir()`. ([`9abbac1`](https://github.com/Byron/gitoxide/commit/9abbac1b1a1e7af7c5219f84a9edc1594deda55a))
    - thanks clippy ([`60da03c`](https://github.com/Byron/gitoxide/commit/60da03c3edc38d14601ac2dfbeb3b3045958f860))
    - thanks clippy ([`4d0e29c`](https://github.com/Byron/gitoxide/commit/4d0e29c25fd53421487a624b90072c8553509d45))
    - Merge branch 'git_includeif' of https://github.com/svetli-n/gitoxide into svetli-n-git_includeif ([`0e01da7`](https://github.com/Byron/gitoxide/commit/0e01da74dffedaa46190db6a7b60a2aaff190d81))
    - thanks clippy ([`5bf6b52`](https://github.com/Byron/gitoxide/commit/5bf6b52cd51bef19079e87230e5ac463f8f881c0))
    - thanks clippy ([`53f27e0`](https://github.com/Byron/gitoxide/commit/53f27e04dd186c32eaa8c03615a58a10938cab8d))
    - thanks clippy ([`1e2b239`](https://github.com/Byron/gitoxide/commit/1e2b239abee7e8889fe2060c79c00f2e506023e1))
    - onbranch uses wildmatch ([`8382df2`](https://github.com/Byron/gitoxide/commit/8382df2a7cb9cb12113085b9310560f63c51447f))
    - gitdir/i support. ([`5dd3f92`](https://github.com/Byron/gitoxide/commit/5dd3f92964cd530f435f2dfb81a10fb236dd5334))
    - Fail fast include condition if it's raw value contains backslash. ([`2c78e48`](https://github.com/Byron/gitoxide/commit/2c78e4866e10141b23980aa2db6405644ad92f34))
    - Replace \\ in pattern after expanding relative paths. ([`a955449`](https://github.com/Byron/gitoxide/commit/a9554497f81b2e2ecdd2ea7a14c347b1f136a688))
    - Replace \\ in pattern after interpolation. ([`d774485`](https://github.com/Byron/gitoxide/commit/d77448510bd276db83802bd7b183a757d6a48db3))
    - Handle relative path patterns. Update tests. ([`546ec2c`](https://github.com/Byron/gitoxide/commit/546ec2c30fd6d1ed1eea1f1497251513940e82ac))
    - Use `std::path::MAIN_SEPARATOR` when adding ** globbing. ([`b85e706`](https://github.com/Byron/gitoxide/commit/b85e7066fe86d822b6912101c7eb499998d2c4cd))
    - Use `std::path::MAIN_SEPARATOR` when adding ** globbing. ([`cc42edf`](https://github.com/Byron/gitoxide/commit/cc42edf1e5c11ad25fe3ffc9dbc170868748cf66))
    - Use `git-glob` for pattern matching. ([`6066701`](https://github.com/Byron/gitoxide/commit/6066701f1c852b61203aa46399bd7731834c79bf))
    - Refact. ([`35f955a`](https://github.com/Byron/gitoxide/commit/35f955a3cd881359b573f4abd92239e18701aa34))
    - Fix out of order whne reading includeIf sections. ([`e6ef931`](https://github.com/Byron/gitoxide/commit/e6ef931567888e2794d17f2e0fa598a04ac1ef49))
    - Fix out of order whne reading includeIf sections. ([`293e86e`](https://github.com/Byron/gitoxide/commit/293e86ec96864fcdf5f42ba0d5b4d3892574e7ec))
    - Test WIP. ([`7a59791`](https://github.com/Byron/gitoxide/commit/7a59791181c21927340137340b17cd9715755722))
    - PR feedback. ([`fd2b085`](https://github.com/Byron/gitoxide/commit/fd2b085a856ea4665976e5f662a4fad3d7cb5090))
    - Use new git-ref API. ([`32c5729`](https://github.com/Byron/gitoxide/commit/32c5729c5a42ade2e881de5d5575e670b5808f33))
    - IncludeIf condition and gitdir tests. ([`892b77a`](https://github.com/Byron/gitoxide/commit/892b77a8ae09c61391cb637051ea4576b66cf450))
    - Nop includeIf. ([`5d86a02`](https://github.com/Byron/gitoxide/commit/5d86a02dd7617488285b6d0bd43d13ebfa3fb67a))
    - Get values for a section across all subsections. ([`aff2777`](https://github.com/Byron/gitoxide/commit/aff2777baaffa08d1d8ad2e1da34f47e0fe01f7f))
    - Merge branch 'main' into git-sec ([`2fe70f9`](https://github.com/Byron/gitoxide/commit/2fe70f96cfb68e108637ce78f8edda2eb4e2e61a))
    - add suppport for android ([`031bd2f`](https://github.com/Byron/gitoxide/commit/031bd2f401199a05d6465c0260ceed3cc849c7ac))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - thanks clippy ([`273895a`](https://github.com/Byron/gitoxide/commit/273895a06ddfff33c6197799d7e63e8382b4b5e3))
    - Update doc comment. ([`322f825`](https://github.com/Byron/gitoxide/commit/322f82529c1b5fb22406a1392217af5d53dcdac4))
    - Handle overflow. ([`61c5285`](https://github.com/Byron/gitoxide/commit/61c52853e61a4cbb356cc607f970e150c827d679))
    - Add doc comment. ([`001862a`](https://github.com/Byron/gitoxide/commit/001862abde9cbb717c83fdf49a6ddf89a4db16e2))
    - Canonicalize ´git_config::values::Integer` values as simple decimal numbers. ([`03f360a`](https://github.com/Byron/gitoxide/commit/03f360a19d365f614e71948df7e8b0c62d13cff4))
    - Refactor values tests. ([`ee4ad7e`](https://github.com/Byron/gitoxide/commit/ee4ad7eadd6675959c9759bb43a08159e3e0daa9))
    - do not treat empty values in sections like multi-line values ([`8b9432c`](https://github.com/Byron/gitoxide/commit/8b9432c22186a290fd05b6272490dad2bccb7f63))
</details>

## 0.2.1 (2022-04-05)

### Features

- New `values::String` data type which makes it easier to obtain string values to work with as
  binary string.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#298](https://github.com/Byron/gitoxide/issues/298)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - prepare changelog prior to release ([`fc8f52d`](https://github.com/Byron/gitoxide/commit/fc8f52d91c89fdc1130990e4392f151a30d1899c))
    - Support for simple BString powered string values ([`2381c5d`](https://github.com/Byron/gitoxide/commit/2381c5d3b91e3a071c887d9e1e166625977d5830))
 * **Uncategorized**
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - thanks clippy ([`7887d8b`](https://github.com/Byron/gitoxide/commit/7887d8b5bedc49890bd73beb058a9828aa734729))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
</details>

## 0.2.0 (2022-04-02)

<csr-id-55c00d880535a1f8c37cb7d4405d39ff5a7654a0/>

### New Features

 - <csr-id-e4d6685064ad2b433f8acd3a74b320bf0169a994/> Add `git_config::values::Path` for a typesafe git path
   Add a `Path` type to the `git_config::values` which
   can be interpolated according to gits own path interpolation
   rules.
 - <csr-id-61af06b905926849abce19677ff4b9ac05d625a3/> compatibility with Rust <1.53
 - Respect `include.path` when reading configuration files
 - Support for path interpolation

### Refactor

 - <csr-id-55c00d880535a1f8c37cb7d4405d39ff5a7654a0/> remove `git_config::values::Value`; use `Bytes` in its place.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 44 commits contributed to the release over the course of 60 calendar days.
 - 60 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#331](https://github.com/Byron/gitoxide/issues/331)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - minor refactor ([`2f0234c`](https://github.com/Byron/gitoxide/commit/2f0234c05d1a3e1e3b96dff9680189c67cb6c9ff))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Update changelog prior to release ([`1d07934`](https://github.com/Byron/gitoxide/commit/1d079346e789b0acc9a4bdf7577b21c1c37b6106))
    - minor refactor of tests ([`ebe551f`](https://github.com/Byron/gitoxide/commit/ebe551fc16a98d4101799b0d657b42f445f5b16e))
    - refactor ([`845fe37`](https://github.com/Byron/gitoxide/commit/845fe373bf25de497f01209f4bed5132cc8eae65))
    - refactor include path recursion logic ([`2862a07`](https://github.com/Byron/gitoxide/commit/2862a0718e3bc03e4edda60f1b359dd6068e8d1d))
    - Ignore subsections when resolving include.path keys ([`baa300f`](https://github.com/Byron/gitoxide/commit/baa300f844eadf8db6ca5503a0b426235d4ee6fb))
    - assure `from_env()` include paths only use paths of the correct key ([`0d84ce8`](https://github.com/Byron/gitoxide/commit/0d84ce8b229bcbbaa7ee1b3682bd3f374e803dff))
    - remove unnecessary doc comments; remove unused field in Options ([`e94ded4`](https://github.com/Byron/gitoxide/commit/e94ded49d8913bd74aff556ee745e83c5dffc3ac))
    - add TODO to not forget reworking the 'fs' module ([`0b032e4`](https://github.com/Byron/gitoxide/commit/0b032e44c1c124c80039e48cdf539e2bda68607c))
    - use the same BOM bytes as in git-attributes ([`7204755`](https://github.com/Byron/gitoxide/commit/7204755a4e800dfc58cc667f4e751359badf548b))
    - refactor ([`85be984`](https://github.com/Byron/gitoxide/commit/85be98437be80d8f79fbfbc032972e4395f19ef0))
    - add from_paths::Options::default(); minor refactor ([`bcd038c`](https://github.com/Byron/gitoxide/commit/bcd038cccc197cca9012db268dd7502d05c88369))
    - implement include.path support ([`a392988`](https://github.com/Byron/gitoxide/commit/a3929880e1639eba448aec15333dfaf08ac2dd28))
    - fix docs ([`3e7ef3e`](https://github.com/Byron/gitoxide/commit/3e7ef3e6bb5915126da5486ef627e4edf6a727ff))
    - more descriptive test names ([`049b243`](https://github.com/Byron/gitoxide/commit/049b2434dfbc97fa5734d852ebc8d07b18265e8a))
    - turn PathError into path::interpolate::Error; refactor ([`27085e0`](https://github.com/Byron/gitoxide/commit/27085e0e7a1d5067cbc5a8083953446bc6926c5d))
    - Work with std::path::* during interpolation ([`f0ff687`](https://github.com/Byron/gitoxide/commit/f0ff6879d0453be2fa2700f5a2432c3a5c830c31))
    - Fix build ([`f6d9693`](https://github.com/Byron/gitoxide/commit/f6d969370b8ef05b3b29983dcd9f6fa11d6225f2))
    - Make `Path::interpolate()` more useful by returning an actual `PathBuf` ([`86aa7b3`](https://github.com/Byron/gitoxide/commit/86aa7b3a98f933d9eff377fc426f37a22bf473be))
    - Don't interpolate on Path creation due to missing context ([`a071ce8`](https://github.com/Byron/gitoxide/commit/a071ce8f49cd70802776effbd25777a4e65d036c))
    - Add AsRef and Deref for values::Path; additional assertions ([`0666a35`](https://github.com/Byron/gitoxide/commit/0666a358b3b7aadda504979e543cc2058b478bfe))
    - Add `git_config::values::Path` for a typesafe git path ([`e4d6685`](https://github.com/Byron/gitoxide/commit/e4d6685064ad2b433f8acd3a74b320bf0169a994))
 * **Uncategorized**
    - Release git-config v0.2.0 ([`ddfe833`](https://github.com/Byron/gitoxide/commit/ddfe833c13a9fd46aa96283bc3bb372e3f7d82ce))
    - Release git-features v0.20.0, git-config v0.2.0 ([`a6460db`](https://github.com/Byron/gitoxide/commit/a6460db80ba3c49ea37c712465c7cbdefa5c32b6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - derive PartialEq, Eq and Debug on ResolvedGitConfig ([`b2a88a5`](https://github.com/Byron/gitoxide/commit/b2a88a5af259ec07c51d873cac172bb60d7575aa))
    - Refactor git_config tests. ([`714ef5c`](https://github.com/Byron/gitoxide/commit/714ef5c2cdea2af4026dba91119845ff68298d8d))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Merge branch 'describe-rev' ([`77b7cd9`](https://github.com/Byron/gitoxide/commit/77b7cd9a7813aaa1a15d035ea42c1e3fe4eef8dd))
    - thanks clippy ([`a87844a`](https://github.com/Byron/gitoxide/commit/a87844ab8b03357a52bea6a36002c8f1f1c3a5bb))
    - Revert "Move tests out of git_config." ([`3cbe072`](https://github.com/Byron/gitoxide/commit/3cbe072b024848c0133b6800dc84e68f58eee621))
    - Move tests out of git_config. ([`7fd8369`](https://github.com/Byron/gitoxide/commit/7fd83692411d8c5d392875c877f3f25985123f00))
    - Relative include path from env is error. ([`e303466`](https://github.com/Byron/gitoxide/commit/e303466857484eab5110a11d90f482f32943f74a))
    - Refactor and add skip bom when reading config. ([`2d5768d`](https://github.com/Byron/gitoxide/commit/2d5768dc9b40e0b830e0cc5aefb77f5e030bb8f8))
    - Replace `GitConfigFromEnvError` with `from_env::Error`. ([`e1f8b52`](https://github.com/Byron/gitoxide/commit/e1f8b527067cb3dcc74a33c238ba4edfafa95789))
    - Add path include to `from_env`. Follow duplicate include paths until max include depth is exceeded. ([`2295dc5`](https://github.com/Byron/gitoxide/commit/2295dc5c8fbbd6e27292dfd7a489ad0567421155))
    - Return error when max allowed nested includes depth is passed. ([`9692694`](https://github.com/Byron/gitoxide/commit/969269475d76a3bad323a9cc6e5b9d0f436ddc37))
    - remove `git_config::values::Value`; use `Bytes` in its place. ([`55c00d8`](https://github.com/Byron/gitoxide/commit/55c00d880535a1f8c37cb7d4405d39ff5a7654a0))
    - Minor fixes ([`c72ca00`](https://github.com/Byron/gitoxide/commit/c72ca0098e4daa153186789143a192ed38e9656c))
    - make fmt; fix build ([`ae4f122`](https://github.com/Byron/gitoxide/commit/ae4f122d191f1e4ee63bd11971fd61dfdd60bc8f))
    - Use context in PathError. ([`3b55f25`](https://github.com/Byron/gitoxide/commit/3b55f257a7ff1b89eea6616d61dfd51d409c797b))
    - Small refactoring and documentation. ([`fefb01b`](https://github.com/Byron/gitoxide/commit/fefb01b84f954700b19e010282c4b413de8e03d2))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Merge branch 'index-verification' ([`ad3c803`](https://github.com/Byron/gitoxide/commit/ad3c8032cee02052ef3940d1d7c950270a0a299a))
</details>

## 0.1.11 (2022-01-31)

### New Features

 - <csr-id-e822f566dcff3f6c784c206dff2fbc5f82d543be/> subsection iteration.
   
   introduce method `sections_by_name_with_header` to allow iterating over tuples of
   section header and section body.

### Bug Fixes

 - <csr-id-469406dc0d9fece4a06230ef0d8018846202f0ad/> fix usage example in README.md

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 7 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#319](https://github.com/Byron/gitoxide/issues/319)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#319](https://github.com/Byron/gitoxide/issues/319)**
    - update changelog prior to release ([`858ec8c`](https://github.com/Byron/gitoxide/commit/858ec8cc25f18c435465baee762def3013743f0b))
    - Adjust docs ([`38c201c`](https://github.com/Byron/gitoxide/commit/38c201c505ce2ea4257cdd0255713154745a330c))
    - An example to illustrate the problem ([`c47e8f8`](https://github.com/Byron/gitoxide/commit/c47e8f8ee8ea79f8f654f6c28e54e0b0b1fff1b6))
 * **Uncategorized**
    - Release git-config v0.1.11 ([`a605b67`](https://github.com/Byron/gitoxide/commit/a605b67294773628590220600f5017c63911f620))
    - fix usage example in README.md ([`469406d`](https://github.com/Byron/gitoxide/commit/469406dc0d9fece4a06230ef0d8018846202f0ad))
    - implement a draft for subsection fetching ([`e822f56`](https://github.com/Byron/gitoxide/commit/e822f566dcff3f6c784c206dff2fbc5f82d543be))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
</details>

## 0.1.10 (2022-01-23)

### New Features

 - <csr-id-61af06b905926849abce19677ff4b9ac05d625a3/> compatibility with Rust <1.53

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 51 calendar days.
 - 55 days passed between releases.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#266](https://github.com/Byron/gitoxide/issues/266)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - upgrade dependencies ([`8adf0d8`](https://github.com/Byron/gitoxide/commit/8adf0d80bbd5c4e81ccd0b5363dbce6609a6c90a))
 * **Uncategorized**
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - compatibility with Rust <1.53 ([`61af06b`](https://github.com/Byron/gitoxide/commit/61af06b905926849abce19677ff4b9ac05d625a3))
    - thanks clippy ([`7dd2313`](https://github.com/Byron/gitoxide/commit/7dd2313d980fe7c058319ae66d313b3097e3ae5f))
</details>

## 0.1.9 (2021-11-29)

A maintenance release.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 11 calendar days.
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
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - Fix build warnings related to pin-project lite ([`126aeec`](https://github.com/Byron/gitoxide/commit/126aeec1f4cb358c7d24fec4fb0a92e7ff9319e8))
    - thanks clippy ([`db1bb99`](https://github.com/Byron/gitoxide/commit/db1bb99101a9248b464b0df9f526067b8f2a184e))
    - Add `GitConfig::from_env_paths` with git-like sequence resolution ([`aec51a2`](https://github.com/Byron/gitoxide/commit/aec51a2240c548a0737e61aeaebc2997945af197))
    - Merge branch 'git-loose-objects' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-git-loose-objects ([`ee737cd`](https://github.com/Byron/gitoxide/commit/ee737cd237ad70bf9f2c5e0d3e4557909e495bca))
</details>

## 0.1.8 (2021-11-16)

A maintenance release triggered by changes to git-pack and changelog rewrites.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 25 calendar days.
 - 31 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#241](https://github.com/Byron/gitoxide/issues/241), [#254](https://github.com/Byron/gitoxide/issues/254)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#241](https://github.com/Byron/gitoxide/issues/241)**
    - Improve usability of the pack-cache environment variable ([`47d8162`](https://github.com/Byron/gitoxide/commit/47d81629a0bfa2eccf75cbe081de55d80d0abd59))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **Uncategorized**
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - better changelog descriptions. ([`f69b2d6`](https://github.com/Byron/gitoxide/commit/f69b2d627099639bc144fd94fde678d84a10d6f7))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Remove stale clippy allow ([`8441e92`](https://github.com/Byron/gitoxide/commit/8441e9217def0c77cfb69a75d98644ec6a9b46d9))
    - Note, not zero-copy nor alloc ([`75879b0`](https://github.com/Byron/gitoxide/commit/75879b0997afe87af96ccdff44b2c1a696aa223e))
    - Comment ([`4b00d68`](https://github.com/Byron/gitoxide/commit/4b00d6898bd21a7bd924b39c0ddb90f7c36e014b))
    - Lint ([`e700284`](https://github.com/Byron/gitoxide/commit/e7002844fbab0d415b9656395450402f2de7539b))
    - Format ([`960dcdc`](https://github.com/Byron/gitoxide/commit/960dcdc6752685e19b97e56f3fae9bc45a9ced4c))
    - Add multi value test ([`f3bcefb`](https://github.com/Byron/gitoxide/commit/f3bcefbd83d5c6f78a710b031c93342658b4a3a1))
    - Assert error kind ([`763266d`](https://github.com/Byron/gitoxide/commit/763266d24746247dc333916761561a12a210a767))
    - Assert io error ([`03541c5`](https://github.com/Byron/gitoxide/commit/03541c579027dc4b00745f573bb41c043cea087a))
    - Not mutable ([`0cfe8a4`](https://github.com/Byron/gitoxide/commit/0cfe8a40c7087fa744cd0b51878c7369f89a3801))
    - Rename test ([`bfcad07`](https://github.com/Byron/gitoxide/commit/bfcad07650398e83bb27201b25e61342ad20a03e))
    - Assert invalid paths ([`be4a4ea`](https://github.com/Byron/gitoxide/commit/be4a4ea2735060aee59f23cc742b6d97a324cb79))
    - Assert config len ([`bbce210`](https://github.com/Byron/gitoxide/commit/bbce210326311ef1d10b12d19c35b39a7606412b))
    - Remove debug print ([`cdf88e6`](https://github.com/Byron/gitoxide/commit/cdf88e6f8a1dab3c7dbd24314232908ae4a7b8ad))
    - First pass ([`a424d5a`](https://github.com/Byron/gitoxide/commit/a424d5adff97adf421aa9b1a3da9c39148c12144))
</details>

## v0.1.7 (2021-10-15)

This is a maintenance release without functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 3 calendar days.
 - 38 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#213](https://github.com/Byron/gitoxide/issues/213)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Maintenance release note to avoid being fully generated ([`56ef363`](https://github.com/Byron/gitoxide/commit/56ef363f0e7a8b9106765d96d0636e38b2bed550))
    - Changlog for git-config ([`abdfe58`](https://github.com/Byron/gitoxide/commit/abdfe588030b0fbdd4d69a73c5739ef4a83e3616))
    - Use correct title for github release to match name of tag ([`90f39ad`](https://github.com/Byron/gitoxide/commit/90f39ad693e0998bc3307bf553fccdc37c8dc0c8))
 * **[#213](https://github.com/Byron/gitoxide/issues/213)**
    - refactor ([`e906d37`](https://github.com/Byron/gitoxide/commit/e906d37e0b4e088b7973728db386a23ea7645fc9))
    - Remove environment variable after test passed ([`7a3ff29`](https://github.com/Byron/gitoxide/commit/7a3ff293048dd6bebec492bd79b12d7889fee3a1))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Remove after ([`cb72bef`](https://github.com/Byron/gitoxide/commit/cb72befbe08590f29489bde1b85c1582d729e0c4))
    - Mark tests as serial ([`0456142`](https://github.com/Byron/gitoxide/commit/0456142c2ee79c39fd738b5e0ef5a258e56d524f))
    - Add a test for multiple sections ([`0ad6438`](https://github.com/Byron/gitoxide/commit/0ad6438b0c19ef6cd7db469cb3f45f3f820665fd))
    - Format ([`d743ef8`](https://github.com/Byron/gitoxide/commit/d743ef8a4fb08511100650b7cbd027491ecb54de))
    - Add a test for a single key value pair ([`a64d312`](https://github.com/Byron/gitoxide/commit/a64d312313ae9f268747e400ba78cd6254d91426))
    - Add a test case for GIT_CONFIG_COUNT parse error ([`a864812`](https://github.com/Byron/gitoxide/commit/a86481207c592eef9abf3b382fe658370657d296))
    - Add test case for GIT_CONFIG_COUNT=0 ([`c33b498`](https://github.com/Byron/gitoxide/commit/c33b498cac29f04d260e361622a4ee86c035a9c1))
</details>

## v0.1.6 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 7 calendar days.
 - 8 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.6 ([`b1b6fe0`](https://github.com/Byron/gitoxide/commit/b1b6fe0af52d0ec133cd9ec9ffd5a173ba14a5d2))
    - [repository #185] rustfmt ([`dfbb015`](https://github.com/Byron/gitoxide/commit/dfbb015a89db47c79015135870013ecc384c4aea))
    - [config #185] refactor ([`509c938`](https://github.com/Byron/gitoxide/commit/509c938dd061060141756ee791cdcb6017934fe2))
    - [config #185] Count lines correctly on windows… ([`57203ce`](https://github.com/Byron/gitoxide/commit/57203ce5d5e3c481b69c3ca173e4b00f11aaf7d7))
    - [config #185] add test for handling windows formatted files… ([`2a2a89f`](https://github.com/Byron/gitoxide/commit/2a2a89f68cc45e27a1cf0d33fc644ebabc762302))
    - [config #185] flyby refactor ([`9b9ffa3`](https://github.com/Byron/gitoxide/commit/9b9ffa3c1d5ccbea22aa38b740daa8a349494395))
</details>

## v0.1.5 (2021-08-29)

- maintenance release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 10 calendar days.
 - 12 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.5 ([`150ed76`](https://github.com/Byron/gitoxide/commit/150ed760c8b357e5c40ec0bd8d0cd849b39c34c0))
    - [various #184] configure docs.rs build features ([`cc50249`](https://github.com/Byron/gitoxide/commit/cc502492c512293e93e95610ca80a71896076ded))
    - [object #177] fix docs ([`2fd23ed`](https://github.com/Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com/Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Upgrade to nom-7 ([`f0aa3e1`](https://github.com/Byron/gitoxide/commit/f0aa3e1b5b407b2afd187c9cb622676fcddaf706))
    - [smart-release #162] format everything ([`8ff83e5`](https://github.com/Byron/gitoxide/commit/8ff83e5c511ae29979348789bd6e7a2f72b16f1c))
</details>

## v0.1.4 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.4 ([`535ff79`](https://github.com/Byron/gitoxide/commit/535ff79d6d28d3f08572f4353a8db4da2b658473))
    - [git-config] Resolved config construction ([`1ab44c0`](https://github.com/Byron/gitoxide/commit/1ab44c06b30b745711bda3711b5ce92dfae306be))
    - [config] Allow certain warnings during development, fix docs ([`1a2f408`](https://github.com/Byron/gitoxide/commit/1a2f408d045b48925062646bf014d419bd753086))
    - Don't enable resolved module yet ([`0bd05b2`](https://github.com/Byron/gitoxide/commit/0bd05b22c86b366bdd01be747ffd5207434ece0d))
    - disable all git-config lints ([`05687b4`](https://github.com/Byron/gitoxide/commit/05687b471cb1cbaa8785ec09177c949773dac05a))
    - disable lint ([`b4302cd`](https://github.com/Byron/gitoxide/commit/b4302cd257e6c76cd85c3af5f28457a1ed91f098))
    - rustfmt git-config for consistency ([`b559dd0`](https://github.com/Byron/gitoxide/commit/b559dd0eda1b210eb996b3e9518d6264e614035f))
    - Add todos ([`dbcd79a`](https://github.com/Byron/gitoxide/commit/dbcd79a0b9776ad2e9f5ca0ff2ed965d3d52c104))
    - Fix contains_key ([`50f9122`](https://github.com/Byron/gitoxide/commit/50f91225b903c8d45a7f3c4a3754b03bc80ccc45))
    - Add IntoIterator for SectionBody ([`d37b17c`](https://github.com/Byron/gitoxide/commit/d37b17c55d35b76cd831e51f18c3b0942bc53724))
    - Document GitConfigFromEnvError ([`eb44cf6`](https://github.com/Byron/gitoxide/commit/eb44cf675cb49a313220377b05f0eded422f7e09))
    - More git-config docs ([`f05a669`](https://github.com/Byron/gitoxide/commit/f05a66905c12844515860d0d5e5e113e05df54cb))
</details>

## v0.1.3 (2021-08-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 6 calendar days.
 - 8 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.1.3 ([`319a4ae`](https://github.com/Byron/gitoxide/commit/319a4ae2b71f4e847757aa46f1d9fcc4b4ee12ca))
    - [config] pacify clippy ([`ad41ba6`](https://github.com/Byron/gitoxide/commit/ad41ba6c96da28d704163a455c3185aec7050db5))
    - Fix bench path ([`70f9403`](https://github.com/Byron/gitoxide/commit/70f94032ca7fcad5eaa9cd0064720d72569f9c17))
</details>

## v0.1.2 (2021-08-06)

### Added

 - Added the following methods to `GitConfig`:
   - `is_empty`
   - `len`
   - `from_env`
   - `open`
- `len`
- `from_env`
- `open`

### Changed

 - `parse_from_path` now accepts a `AsRef<Path>` instead of a `&Path`.
 - `parse_from_path` now returns an `ParserOrIoError<'static>` instead, from
   `ParserFromIoError`

### Fixed

 - _None._

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 86 calendar days.
 - 89 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix bench path ([`bf0004e`](https://github.com/Byron/gitoxide/commit/bf0004e05ede404921073755aadd8ab8f75273c9))
    - Bump git-config to 0.1.2 ([`9c275dc`](https://github.com/Byron/gitoxide/commit/9c275dc6f1a07ebd6c4cc8ae0edae382bd13c0cf))
    - Use newtyped Index and Size ([`15ae2d7`](https://github.com/Byron/gitoxide/commit/15ae2d76bb1b4bd64d3ee50021a359a777e95538))
    - Re-export everything in git-config::file::mod ([`392c131`](https://github.com/Byron/gitoxide/commit/392c13175892ecf6e543ee6a1fd47c62a38f09cb))
    - Fix rustdoc links ([`042eaf4`](https://github.com/Byron/gitoxide/commit/042eaf4b4625ed47b417a9012556ef6fc69aa2d6))
    - Use AsRef<Path> when opening from path ([`515d256`](https://github.com/Byron/gitoxide/commit/515d2564e430da77c092ceb9414a3b3e7071c158))
    - Add GitConfig::from_env ([`17e30a1`](https://github.com/Byron/gitoxide/commit/17e30a1ede39326cda6c64989ab37d979c9c4a29))
    - Add GitConfig::from_path ([`27df3d1`](https://github.com/Byron/gitoxide/commit/27df3d1d5de1e7660beaf599e4931c3cf7c1f99a))
    - Add is_empty and len to GitConfig ([`aa86594`](https://github.com/Byron/gitoxide/commit/aa865942559ee48d7998adb211a6a8f4e0760375))
    - split file.rs into module ([`da40593`](https://github.com/Byron/gitoxide/commit/da40593b3e4d35dcdf8003123cefc0e367367734))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Implement Parser::into_iter without extra allocation ([`aa79924`](https://github.com/Byron/gitoxide/commit/aa79924b36c0d717cc65d7471fedd27eb41e83a5))
    - clippy cleanup; fix CI build ([`3e943f2`](https://github.com/Byron/gitoxide/commit/3e943f2afd5f0cfe7294a21cca8e0344c7dd0216))
    - thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - [git-config] Annotate more functions with inline ([`2006acb`](https://github.com/Byron/gitoxide/commit/2006acb381a3a9e807575991a8eeab1ea010af60))
</details>

<csr-unknown>
lenfrom_envopen<csr-unknown/>

## v0.1.1 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release over the course of 56 calendar days.
 - 58 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.1.1 ([`e583f70`](https://github.com/Byron/gitoxide/commit/e583f70947803b5b6885a4eb22cd515263177b5b))
    - thanks clippy ([`17258cc`](https://github.com/Byron/gitoxide/commit/17258cc58767caa6e71227898decd160ad0cdf13))
    - TODO's about 'Iterator::reduce()' are probably not applicable ([`ac1a433`](https://github.com/Byron/gitoxide/commit/ac1a4333612c7ef238b84d15a194d4bc4685cd3a))
    - Thank cargo-doc ([`ca96be1`](https://github.com/Byron/gitoxide/commit/ca96be1654a175606a4af6032b2ace4875334231))
    - [git-config] Finish cleaning up 1.51 clippy lints ([`aec7240`](https://github.com/Byron/gitoxide/commit/aec7240036750c98796b8ef4075758f6b825d293))
    - [git-config] Fix various 1.51 clippy lints; inline ([`d899df0`](https://github.com/Byron/gitoxide/commit/d899df0d9feec1f38b60be73af80113958dfa7d1))
    - Merge pull request #50 from Byron/edward-shen/odb-zlib-ng ([`acb90d7`](https://github.com/Byron/gitoxide/commit/acb90d755fb02c37f8a5a431778abcbe143fb5e5))
    - [git-config] Fix must_use lints ([`71aff75`](https://github.com/Byron/gitoxide/commit/71aff75d02329caf78c61d3c1dd8ab3c33b8597d))
    - Slim down git-config with cargo-diet ([`1c555e0`](https://github.com/Byron/gitoxide/commit/1c555e04d395eadb6b22639afd41c0892d48fa0d))
    - [git-config] add parse test from git remote ([`63bee9c`](https://github.com/Byron/gitoxide/commit/63bee9c3217689df5fbe36d79857db7cdd349d84))
    - [git-config] Add sections_by_name ([`1f7a533`](https://github.com/Byron/gitoxide/commit/1f7a53357d0f1f2f8164b59e8b276ae61fff552f))
    - [git-config] Add to_owned for parser::Error ([`e316c8c`](https://github.com/Byron/gitoxide/commit/e316c8c7a8864daf2ade0ec8fdf42aa20694805f))
    - [git-config] Add coercion into owned variants ([`6387aea`](https://github.com/Byron/gitoxide/commit/6387aeaefccb2c80f9a276f3a8978be28f23bdfb))
</details>

## v0.1.0 (2021-03-12)

<csr-id-949622e461eb2116393ec6f4633ec0cb2e1695b5/>
<csr-id-bcacfc9bcf19a0339541b24e84de68d95291c62b/>
<csr-id-41f118d2aa560188fd3399d2390aa43794b0af75/>

### Other

 - <csr-id-949622e461eb2116393ec6f4633ec0cb2e1695b5/> Include benches in crate to allow publishing to work
 - <csr-id-bcacfc9bcf19a0339541b24e84de68d95291c62b/> remove clippy-cargo lint until there are no warnings
   Please feel free to re-add once all other git-* crates have been
   adjusted. It's interesting to see how a crate lint spills into the
   workspace.
   
   Personally I am surprised that the keywords value is supposed
   to repeat the crate name as it seems redundant.
 - <csr-id-41f118d2aa560188fd3399d2390aa43794b0af75/> remove redundant lines from git-ignore file

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 125 commits contributed to the release over the course of 157 calendar days.
 - 158 days passed between releases.
 - 3 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Include benches in crate to allow publishing to work ([`949622e`](https://github.com/Byron/gitoxide/commit/949622e461eb2116393ec6f4633ec0cb2e1695b5))
    - remove clippy-cargo lint until there are no warnings ([`bcacfc9`](https://github.com/Byron/gitoxide/commit/bcacfc9bcf19a0339541b24e84de68d95291c62b))
    - remove redundant lines from git-ignore file ([`41f118d`](https://github.com/Byron/gitoxide/commit/41f118d2aa560188fd3399d2390aa43794b0af75))
    - fix format ([`1655b56`](https://github.com/Byron/gitoxide/commit/1655b56b34618d28f67b1ef08b41c598eaf3530e))
    - remove release-profile ([`3d62449`](https://github.com/Byron/gitoxide/commit/3d62449feed68cef213df31268dfbb9fc54f2d62))
    - rename_section ([`4975fff`](https://github.com/Byron/gitoxide/commit/4975fff3edc67a39bd0046870ac8c572c09d0f78))
    - return key iterator ([`adfa460`](https://github.com/Byron/gitoxide/commit/adfa460f8ae0cdbbe8a3b91e0d0c55e46bb9b9ce))
    - add push_section ([`81271e2`](https://github.com/Byron/gitoxide/commit/81271e24bcf9fc9f7241bbd70d11e0cbbab789a1))
    - more work on sections ([`84e959d`](https://github.com/Byron/gitoxide/commit/84e959da3a19abb451be82d290fa8329ee1df015))
    - remove offset newtype ([`41da7ed`](https://github.com/Byron/gitoxide/commit/41da7edc1b8661e2e3f53b4b061f7e52c9604a06))
    - remove section for owned section type ([`11cf526`](https://github.com/Byron/gitoxide/commit/11cf526769998915e542a93d01023f67b3329fa6))
    - more functionality to mutablesection ([`b0cf849`](https://github.com/Byron/gitoxide/commit/b0cf849f32fb6b0e612efa12f279a6e0bb40d49d))
    - optimize section pushing ([`f409931`](https://github.com/Byron/gitoxide/commit/f4099310dfad85c087bae45a9af2a441569c58fa))
    - section API ([`0aad0f1`](https://github.com/Byron/gitoxide/commit/0aad0f12ed82caa784abde36a9dd4fe2f0cc83bf))
    - section stuct ([`21b4fe1`](https://github.com/Byron/gitoxide/commit/21b4fe11001fa4f09718bc5cacc140a0a97e8ab3))
    - fix example ([`357a761`](https://github.com/Byron/gitoxide/commit/357a76137c8c309b6c16809af32641d1f52a1222))
    - update readme ([`c2fa869`](https://github.com/Byron/gitoxide/commit/c2fa869ffb37180f67c37f2c229b7b26390d6957))
    - fix macro comment gen ([`a19c17f`](https://github.com/Byron/gitoxide/commit/a19c17f6d64bd87ec42ec975a9b4a8e641288816))
    - implement case insensitivity for names ([`c39ff33`](https://github.com/Byron/gitoxide/commit/c39ff332415a7c546af14bf925c9cc5c60b36622))
    - test MutableMultiValue ([`8cfe67d`](https://github.com/Byron/gitoxide/commit/8cfe67df4f322d7ebfee9f5c7de206cacf08f5ed))
    - more tests, fix mutablevalue ([`377532c`](https://github.com/Byron/gitoxide/commit/377532c4b6a3bf2ac8cc3ca7c3c661cfc954a16c))
    - fix lints ([`bb7a544`](https://github.com/Byron/gitoxide/commit/bb7a5445272e8abd825751212762a76e7876e9d1))
    - enable requiring docs ([`68320ca`](https://github.com/Byron/gitoxide/commit/68320ca5ba2d3665bc64580ff9ab12d86b719e56))
    - clippy fix ([`e7bad2e`](https://github.com/Byron/gitoxide/commit/e7bad2eb2633b60f1bf12b7f53dcda1f431172d4))
    - docs ([`a1f833c`](https://github.com/Byron/gitoxide/commit/a1f833ccc7c07b37b5285c109aefaabf9a97f202))
    - add into bytes for gitconfig ([`9b54a5b`](https://github.com/Byron/gitoxide/commit/9b54a5b9df699f2508cf47411293f74252a02473))
    - docs ([`4d7da4e`](https://github.com/Byron/gitoxide/commit/4d7da4e9ef60a90360e51d5425580d9b309db151))
    - mutableevent interface ([`014776f`](https://github.com/Byron/gitoxide/commit/014776f8be482d2d61ef532d1cea84e26a42d893))
    - remove serde code for now ([`fc4ee85`](https://github.com/Byron/gitoxide/commit/fc4ee8585372c023b47c0f28d0c746ff9f8eac5b))
    - disable serde ([`7c01808`](https://github.com/Byron/gitoxide/commit/7c01808c4fdfee35e339c2d939ef6b013b430f38))
    - multablemultivalue ([`faa1b93`](https://github.com/Byron/gitoxide/commit/faa1b9368b8dbcfbef10a3bbb027bdef81a377a0))
    - better test formatting ([`635e5c1`](https://github.com/Byron/gitoxide/commit/635e5c15e39b24767e793e6ab4f230b8913a0760))
    - Add get_multi_value ([`79eeca1`](https://github.com/Byron/gitoxide/commit/79eeca128a3b146e2900cc90f34877cef4fa6e52))
    - check all sections for lookup before failing ([`01b617d`](https://github.com/Byron/gitoxide/commit/01b617d74f10eab8f87e1032aebeeb6f56a2ae10))
    - misc improvements ([`87057c9`](https://github.com/Byron/gitoxide/commit/87057c9f03516d6659cb2cc54f330ba4072d6563))
    - benchmarks ([`b0ff69e`](https://github.com/Byron/gitoxide/commit/b0ff69e9f5260c30ecfd2879b3346437b38cec83))
    - crate level docs ([`a909bcf`](https://github.com/Byron/gitoxide/commit/a909bcf90fa7ad3995fa15f00417257f66ccb6d0))
    - integration tests for value extraction ([`d45af63`](https://github.com/Byron/gitoxide/commit/d45af630ab25cfc3fc9b4b10c5038132f23b5c95))
    - fix drain ([`4f425a7`](https://github.com/Byron/gitoxide/commit/4f425a72cad6b0159e3747ce6a42c4d4343b8e61))
    - use memrchr ([`ccadf89`](https://github.com/Byron/gitoxide/commit/ccadf89aa13051c55debd6aa7c709b138bab6167))
    - use drain instead ([`6e5b67b`](https://github.com/Byron/gitoxide/commit/6e5b67b0d5dec90f22cf05b1597201c8a8b9ab80))
    - use mutablevalue for mut entries ([`13fdda5`](https://github.com/Byron/gitoxide/commit/13fdda5e01cb494f5992be0bb82464392016d4ed))
    - normalize get_raw_value ([`5952cab`](https://github.com/Byron/gitoxide/commit/5952cab59c3aab7f35683f959737a5c8bef29fb1))
    - fix get_raw_value, fix returning refs to cows ([`ba982b9`](https://github.com/Byron/gitoxide/commit/ba982b971aa97eaa17d58d7b00f20923171d89eb))
    - cleanup docs ([`e0a8b8d`](https://github.com/Byron/gitoxide/commit/e0a8b8d808cbed2d7ca902304e6819bfba8f6715))
    - add tests and docs ([`7caf012`](https://github.com/Byron/gitoxide/commit/7caf012b05c6819f5a897918fc50eb61d76517de))
    - rename config mod to file ([`a965ebc`](https://github.com/Byron/gitoxide/commit/a965ebcfc08c71255389c62fe43ff479960e7921))
    - pendantic clippy lints ([`18c9dff`](https://github.com/Byron/gitoxide/commit/18c9dff7c0bde99d4c1d4a7263f86fd4a656d1c0))
    - remove unnecessarily lifetimes ([`7d0e6b4`](https://github.com/Byron/gitoxide/commit/7d0e6b4fe1cb8e3d752cd5b42fa9167b552d6320))
    - use str in most cases ([`9fc8993`](https://github.com/Byron/gitoxide/commit/9fc8993a54950e88aa05b0fe85962ee124a86891))
    - fully comment values ([`8e32d56`](https://github.com/Byron/gitoxide/commit/8e32d5609d81087e17a5dcd15dbe7ed22594aa50))
    - more normalize docs ([`9767b5b`](https://github.com/Byron/gitoxide/commit/9767b5be5a4d45c44e2aba6c2164c800e17ec437))
    - collaspe if block ([`1cf1f3b`](https://github.com/Byron/gitoxide/commit/1cf1f3be284afb1b0ed3d8f06439a5effc553be0))
    - better doc ([`ec63ce6`](https://github.com/Byron/gitoxide/commit/ec63ce633578af6adf6239d76f5802aaa842941c))
    - implement unquoting in normalize ([`7e8ae93`](https://github.com/Byron/gitoxide/commit/7e8ae932f888707fe7466e93ef6c8289749d04d2))
    - add normalize ([`6c245dc`](https://github.com/Byron/gitoxide/commit/6c245dc654bfd7bc6b82226abd900ba2e8a312cd))
    - dedup multivar docs ([`236d37b`](https://github.com/Byron/gitoxide/commit/236d37b608b8a912fc005c956c87d251172fff0f))
    - add todo ([`bc63005`](https://github.com/Byron/gitoxide/commit/bc630057c6f138647f2f8a2d93f8a09e4f4494c3))
    - Implement get_value for GitConfig ([`ca7c1dc`](https://github.com/Byron/gitoxide/commit/ca7c1dca5539bf71e524ba0ee4b40c60bd80f0ad))
    - Use traits instead of from_str ([`ce9b7bf`](https://github.com/Byron/gitoxide/commit/ce9b7bfbb9abe5e74dbef1d3c637876e7d996e52))
    - Use traits instead of shadowing from_str ([`a4ce9b0`](https://github.com/Byron/gitoxide/commit/a4ce9b04012060ab06dabd6658f3e518f994831b))
    - remove falsevariant ([`e10a4a2`](https://github.com/Byron/gitoxide/commit/e10a4a298371b6641e6184fa8d61bebe8c783923))
    - more tests ([`24a2dfd`](https://github.com/Byron/gitoxide/commit/24a2dfd2d7dd0c1b28b62b7dbebeb539e086016e))
    - remove unreachable variants ([`93b85e3`](https://github.com/Byron/gitoxide/commit/93b85e38ff46f0f8fdaabcf2c7210bb9e0421254))
    - use mut vec reference ([`8b68fdb`](https://github.com/Byron/gitoxide/commit/8b68fdb2aeac6dd7211f31333489012647e65f55))
    - Don't use mutex ([`4027daf`](https://github.com/Byron/gitoxide/commit/4027daf93bb931d9b839057b88afcfa849a4ed8c))
    - documented parsererror ([`0c226ad`](https://github.com/Byron/gitoxide/commit/0c226ad112f7ff70dee20669419ba8f7eae3f0c7))
    - clippy fixes ([`8618c22`](https://github.com/Byron/gitoxide/commit/8618c2233abada5fb101258cffc8c046b155134b))
    - don't use stack for error handling ([`819a1d3`](https://github.com/Byron/gitoxide/commit/819a1d3a0bf47c95dc469c7bbf80b3452ef9918d))
    - very rough error handling ([`45d5250`](https://github.com/Byron/gitoxide/commit/45d52502fb91635f3db3d4b09f69f5d7b2a29e09))
    - add error trait impl for ParserError ([`d173b4b`](https://github.com/Byron/gitoxide/commit/d173b4bcc855365640d8931460d0bed748264817))
    - Basic error reporting ([`f293334`](https://github.com/Byron/gitoxide/commit/f293334d984755053b532706134df88a5c57a43f))
    - move fully_sumed to test_util ([`41245eb`](https://github.com/Byron/gitoxide/commit/41245ebd74a0ee40bbc91a2e2f9bca670da02b16))
    - Don't immediately drop fuzzer values ([`568d360`](https://github.com/Byron/gitoxide/commit/568d36084f2c4f94a4757588ca17078ce523a6ac))
    - Add more fields to cargo.toml ([`89791fd`](https://github.com/Byron/gitoxide/commit/89791fd0f3bba9d22418302fe862896a6ccacdc4))
    - Add basic fuzzer ([`cf41bb3`](https://github.com/Byron/gitoxide/commit/cf41bb300fe8213a7b4b02329cd987fd55d2ac9c))
    - add from_bytes variants for parser ([`954f433`](https://github.com/Byron/gitoxide/commit/954f4338e7496208b1e8b13a105eec9f5c07ba76))
    - exclude fuzz folder from cargo ([`afe4ac7`](https://github.com/Byron/gitoxide/commit/afe4ac754f0843d8350504200a2a833bf158c335))
    - add tests for boolean ([`0353033`](https://github.com/Byron/gitoxide/commit/03530334a956e4337d3e92d2bb1a2be5c0278014))
    - Use lto and single codegen unit for release ([`53077bd`](https://github.com/Byron/gitoxide/commit/53077bda06aeed84b3985f941e7f4660b5e0d5e8))
    - select nom features ([`79dc19f`](https://github.com/Byron/gitoxide/commit/79dc19f7a7517f0953a1b14b180e54cfe110bab8))
    - make serde optional, clippy lints ([`5defc4a`](https://github.com/Byron/gitoxide/commit/5defc4a64a9ee37910c2caa9f23253adead1ab6f))
    - Add ColorValue tests ([`5b7cc13`](https://github.com/Byron/gitoxide/commit/5b7cc13e62aff1014369aa7a8d64dc9eaad1f0cd))
    - Add tests for ColorAttribute ([`0cc9cd6`](https://github.com/Byron/gitoxide/commit/0cc9cd6bd1d383bbca97610f6a1a67119a90ab56))
    - Fix docs ([`b0fc08b`](https://github.com/Byron/gitoxide/commit/b0fc08b940dbca8d6b78f29f5d4e391c8b535121))
    - Use BStr instead ([`ec2602c`](https://github.com/Byron/gitoxide/commit/ec2602cfce2867ed341a1bff5e26ca82785a4434))
    - Add key-value delimination event ([`df0da82`](https://github.com/Byron/gitoxide/commit/df0da822d23708cc488027c0830895f0274ad9ce))
    - gitconfig writing to string ([`b59a51a`](https://github.com/Byron/gitoxide/commit/b59a51af3896ba510c30db1044a11472df7d3998))
    - document multivar behavior better ([`e43518e`](https://github.com/Byron/gitoxide/commit/e43518ebdadd739bd4edc90cd4ef279000b4f94e))
    - Use Cow instead of strs ([`ff8ee4a`](https://github.com/Byron/gitoxide/commit/ff8ee4a0352ad7fb8a2c93c0e09f5b3b2c15d3a9))
    - Implement get_mut for gitconfig ([`1d8e58b`](https://github.com/Byron/gitoxide/commit/1d8e58b6008c36141ba38fd37bfbbbdb458b35ef))
    - remove meme comment ([`897450c`](https://github.com/Byron/gitoxide/commit/897450c2e7d89388072d806d4646e1bbac4df422))
    - test get_raw_values ([`0ea6210`](https://github.com/Byron/gitoxide/commit/0ea62105abd84b3b73996f8809772b6856fab6fa))
    - finish raw value queries for gitconfig ([`4b7f218`](https://github.com/Byron/gitoxide/commit/4b7f21875ee12a86c56c2942f2981a79993a3a2a))
    - Handle empty git-config file for parser ([`a516885`](https://github.com/Byron/gitoxide/commit/a5168857071db604f3b2e7191ed749bcdb0354af))
    - fully document parser ([`f66e0be`](https://github.com/Byron/gitoxide/commit/f66e0bedff4c7ba9d455bd7e4d24d299b21fb109))
    - completely refactor config ([`b820d6c`](https://github.com/Byron/gitoxide/commit/b820d6c987263251575844547298e69ddb52d8c4))
    - Booleans now retain original value ([`8ea467e`](https://github.com/Byron/gitoxide/commit/8ea467e01e7e231827041ada0c531f7a3e66715f))
    - parser is now perfect ([`3f708ec`](https://github.com/Byron/gitoxide/commit/3f708ecadee910aebc007fdba1aae004e3344104))
    - more work on parser ([`2691756`](https://github.com/Byron/gitoxide/commit/26917564cf007949ee7f71ee48e10be1efb6f6a0))
    - more work ([`eb07890`](https://github.com/Byron/gitoxide/commit/eb07890347fda7c24721ea91fca5a2eaa519d5b3))
    - Complete initial parser ([`d721625`](https://github.com/Byron/gitoxide/commit/d72162555cc677c4ff143d01f338fc4508a7b11e))
    - Deny rust-2018-idioms ([`a4d2a4b`](https://github.com/Byron/gitoxide/commit/a4d2a4b248f0f22ffd6c8c567d780a152831347a))
    - Add remaining docs for all types in 'git-config' crate ([`b7790b4`](https://github.com/Byron/gitoxide/commit/b7790b4ce3884daaff198890f4a8fb36c38f2230))
    - more planning for config parser implementation ([`9676db9`](https://github.com/Byron/gitoxide/commit/9676db9f58b5776986cfd7185a0ade93f89cb080))
    - Add missing '.' at end of doc comments ([`7136854`](https://github.com/Byron/gitoxide/commit/71368544f97369a4d371d43513607c4805bd0fd0))
    - Signal the compiler that configuration edits must be used ([`14b17e4`](https://github.com/Byron/gitoxide/commit/14b17e4d202fae2eeabfe46552a1c0b17e30ac9c))
    - better docs for git-config; name method for Entry ([`5ab4bdb`](https://github.com/Byron/gitoxide/commit/5ab4bdbc3f1760bcb667d9a1b26eb069084581eb))
    - refactor; more comments ([`8d933cb`](https://github.com/Byron/gitoxide/commit/8d933cbd08977ad0dc70ed18b37e7e06ab24c4fb))
    - time-constrained write-down of some high-level concepts of git-config structures ([`157fa2a`](https://github.com/Byron/gitoxide/commit/157fa2a31e0382ee2c8524ff7862873787f5f648))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com/Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - a path towards making config Files editable ([`bc008c3`](https://github.com/Byron/gitoxide/commit/bc008c32a16849a212eced783aa14727765004c3))
    - additional setters for more fluid edits ([`5a54dae`](https://github.com/Byron/gitoxide/commit/5a54dae6470c5dcf48bf96c16c5bbe2a8951be6a))
    - sketch out editing lossless of Files ([`8f00063`](https://github.com/Byron/gitoxide/commit/8f00063bc9b6a63ffe44e58945be55acca40a714))
    - Skip comments as well ([`32cc684`](https://github.com/Byron/gitoxide/commit/32cc6849444c16a3d2917c6de62e47597c9979da))
    - Stop entry iteration when next section is encountered ([`83a1b83`](https://github.com/Byron/gitoxide/commit/83a1b83a1f7a0ff22850efc7b5b460f0c1ed8230))
    - sketch of iteration over sections and entries ([`acb8947`](https://github.com/Byron/gitoxide/commit/acb894762b38f77d21e6d70936727cf0daeaff6f))
    - sketch out section and entries access ([`06679d9`](https://github.com/Byron/gitoxide/commit/06679d9b69575183231ddb22edd89ab29357632d))
    - refactor ([`b5fa727`](https://github.com/Byron/gitoxide/commit/b5fa727403a78e5f9238dd36d8b071eec425d731))
    - Turn off 'unused' warnings for experimental git-config crate ([`0b52eb0`](https://github.com/Byron/gitoxide/commit/0b52eb0e75a268c5c7b6475677fd20acace3435b))
    - Revert "remove git-config from workspace while it's so fresh…" ([`99214f4`](https://github.com/Byron/gitoxide/commit/99214f4c1097fa8da8f14f1279caf00db78fa822))
    - remove git-config from workspace while it's so fresh… ([`84e0d19`](https://github.com/Byron/gitoxide/commit/84e0d19ab2285916cb6a6b941ec2206aef485d56))
    - Plan how to deal with whitespace and comments to be lossless ([`eb5a534`](https://github.com/Byron/gitoxide/commit/eb5a534340396429d7c2c95e71b0a23457d954f4))
    - refactor ([`3846bab`](https://github.com/Byron/gitoxide/commit/3846bab8c7ae53e5528388522bf4571260ec4ae6))
    - very first sketch of types for read-only git config ([`e2a39c9`](https://github.com/Byron/gitoxide/commit/e2a39c96a96b3ec9de519c685fe9caddeb89342c))
</details>

## v0.0.0 (2020-10-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - stub for git-config crate ([`3539531`](https://github.com/Byron/gitoxide/commit/3539531adb06e8f59609f0a83e8ed94d0864c0a1))
</details>

