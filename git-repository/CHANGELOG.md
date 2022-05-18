# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.17.0 (2022-05-18)

<csr-id-53c06c7e6a3003b34edaab10db1f158e2fb57403/>
<csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/>
<csr-id-da8059ce26343c8cd275f43c879d98c92f77fa51/>

### New Features

 - <csr-id-45920da7c8c5618c6e7258de08dbd633a637d017/> Add `Repository::head_name()`.
   A convenient way to obtain the name of a head, if not detached.

### Bug Fixes

 - <csr-id-a1680b44ef568317465d2971da6e0930f9885530/> `Commit::describe()` now returns annnotated tags before lighweight ones and prefers more recent ones as well
 - <csr-id-99365f221065ebc315ac80940ad72cae253743bc/> Support for in truncated history in git-describe
   This allows `describe()` to work on shallow clones.

### Other

 - <csr-id-53c06c7e6a3003b34edaab10db1f158e2fb57403/> allow reading information about remote branch
 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> :discover()` now returns the shortest path.
   If and only if it canonicalized the source path. That way, users will
   still get a familiar path. This is due to `parent()` not operating
   in the file system, which otherwise would be equivalent to `..`,
   but that's not how we work.
   
   Maybe we should overhaul the way this works to use `../` instead
   and just 'absoluteize' the path later (std::path::absolute()) is
   on the way for that.
 - <csr-id-da8059ce26343c8cd275f43c879d98c92f77fa51/> remove unused variant

### Changed (BREAKING)

 - <csr-id-80e8fd4a5944890f43f3d888b7a73bb26351b195/> integrate trust model into repository discovery
   That way it's possible to ignore repositories which effectively
   aren't owned by the current user, or to not ignore them (default)
   but assign tigher permissions to the repository.
 - <csr-id-2e39b0ede98826e6f85c56fef77ac65a5b7e7ac2/> `path::discover::existing()` -> `path::discover()`
 - <csr-id-38dfdcf80f9b7368ccaa10f4b78b2129849848d0/> remove `values::*Error` in favor of `value::parse::Error`.
   This makes it easier to work with errors in practice, we are either
   interested in the value that failed to parse to try something else
   or want a nice user message.
   
   Having one decode error type facilitates that.

### New Features (BREAKING)

 - <csr-id-32dc1829a5661f66396d109c8d0a8eaae6b1f532/> use `git-credentials` in `git-protocol`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 104 commits contributed to the release over the course of 11 calendar days.
 - 43 days passed between releases.
 - 10 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 9 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#382](https://github.com/Byron/gitoxide/issues/382), [#383](https://github.com/Byron/gitoxide/issues/383), [#384](https://github.com/Byron/gitoxide/issues/384), [#386](https://github.com/Byron/gitoxide/issues/386), [#389](https://github.com/Byron/gitoxide/issues/389), [#393](https://github.com/Byron/gitoxide/issues/393)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - upgrade dependencies ([`b039d39`](https://github.com/Byron/gitoxide/commit/b039d39613bb14d49670c4d8b586f76ffb420d03))
    - adjust msrv to the one required by `windows` crates ([`0f141ca`](https://github.com/Byron/gitoxide/commit/0f141ca5f29ea3f75372a8d030fd8ecfa4f72d10))
    - Support for in truncated history in git-describe ([`99365f2`](https://github.com/Byron/gitoxide/commit/99365f221065ebc315ac80940ad72cae253743bc))
    - fix compile warnings ([`9a06fe1`](https://github.com/Byron/gitoxide/commit/9a06fe1b900c2fb9b4466251224a61e26d637271))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
    - adapt to changes in git-discover ([`bd281b8`](https://github.com/Byron/gitoxide/commit/bd281b80b2d01088a97bbca96ff1401ae06a70d1))
    - Adjust permissions mildly to make a little more sense ([`9c05629`](https://github.com/Byron/gitoxide/commit/9c05629d1ca1ad779f1e07f3a3be102f049874df))
    - Don't have expectations on the path, rather deal with it gracefully ([`3a41d5c`](https://github.com/Byron/gitoxide/commit/3a41d5cd7a6eb9f21c3461d499af4399b8f6e5be))
    - REMOVE ME: debug info for failing CI test ([`b0b3df4`](https://github.com/Byron/gitoxide/commit/b0b3df4e7fa93dba7f03003160f38036cbb6d80f))
    - see if this fixes the CI test issue on windows ([`7697f51`](https://github.com/Byron/gitoxide/commit/7697f517ec7c39a15076b1190056882812fe6a12))
    - rely on `absolutize_components()` ([`e844006`](https://github.com/Byron/gitoxide/commit/e84400660dad6281fe3869ad649470f2adf31979))
    - :discover()` now returns the shortest path. ([`e4f4c4b`](https://github.com/Byron/gitoxide/commit/e4f4c4b2c75a63a40a174e3a006ea64ef8d78809))
    - brutally fix path handling of common dirs ([`e120232`](https://github.com/Byron/gitoxide/commit/e120232252875cd3fdacb9b7df90c3db58e7e24e))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - Assure 'commondir' is actually resolved and not kept relative ([`3de63ff`](https://github.com/Byron/gitoxide/commit/3de63ff789e518cf07a8e1f02c385ae4ce857615))
    - fix build ([`cb1c80f`](https://github.com/Byron/gitoxide/commit/cb1c80f8343691600797b61c61cba9cef82a59fc))
    - first working worktree tests for git-repository ([`508cdd2`](https://github.com/Byron/gitoxide/commit/508cdd213ec48aec879ee4ddc3b76b6c851dd3c7))
    - refactor ([`a89a667`](https://github.com/Byron/gitoxide/commit/a89a66792855fea7d695ec72899da954b8c16f3d))
    - Permission controlled access to xdg config ([`42a6c8c`](https://github.com/Byron/gitoxide/commit/42a6c8c9d19f9aab0b33537156e2774c61621864))
    - refactor ([`6d9b2d9`](https://github.com/Byron/gitoxide/commit/6d9b2d9e7a32c825848a1df1fdec2e53b8705662))
    - A new version of opening a repository while accepting environment overrides ([`9d73424`](https://github.com/Byron/gitoxide/commit/9d7342429d2c4d7e5ef98a51a47d5caaa11297e0))
    - preliminary access to a fully configured exclusion cache ([`259d015`](https://github.com/Byron/gitoxide/commit/259d015c4c0195fb77d372545d790ea4c4d01b8a))
    - adjusts to changes in git-ref ([`b362fb5`](https://github.com/Byron/gitoxide/commit/b362fb594546400e6c42688103df438954df7eeb))
    - basic parsing for git-dir files ([`e11c677`](https://github.com/Byron/gitoxide/commit/e11c67770c301942188f204dbb2cd61880087959))
    - prepare for refactoring the overrides code ([`238e1b0`](https://github.com/Byron/gitoxide/commit/238e1b013d3f4d67b6384c6123c5ab6ea9f236fa))
    - refactor ([`a86ed7b`](https://github.com/Byron/gitoxide/commit/a86ed7bc0e10ebed2918f19d2fc3304fbed87df3))
    - fix build ([`d7dac11`](https://github.com/Byron/gitoxide/commit/d7dac11be455ee99299a8d7dfd412853f0d709f3))
    - Avoid using `Cow` at all in favor of a simple `&PartialNameref` ([`1bc9a87`](https://github.com/Byron/gitoxide/commit/1bc9a875d2b09b906f40db9e2c031c99e4fd9928))
    - remove `values::*Error` in favor of `value::parse::Error`. ([`38dfdcf`](https://github.com/Byron/gitoxide/commit/38dfdcf80f9b7368ccaa10f4b78b2129849848d0))
    - See what it means to use `Cow<PartialNameRef>` ([`2ae129a`](https://github.com/Byron/gitoxide/commit/2ae129ad6183f36179031ea905d8974705e70da8))
    - Adjust to `git-discovery` to support linked worktrees in `Kind` type ([`2a99b7d`](https://github.com/Byron/gitoxide/commit/2a99b7d32374b4863dee0a0cdf55711686c94001))
    - refactor ([`807b7f8`](https://github.com/Byron/gitoxide/commit/807b7f826b4e614478aadd36d6361e9970e5d746))
    - refactor `repositoryKind` adjusted to handle linked worktrees ([`84677cb`](https://github.com/Byron/gitoxide/commit/84677cb09634e1d18ce20850bb7c6c9d63a13818))
    - A first version of opening index files with proper configuration ([`f11cc44`](https://github.com/Byron/gitoxide/commit/f11cc441f10e4a7c2c09e7aa9f9435c837c5e77a))
    - adapt to changes in git-ref ([`21109ca`](https://github.com/Byron/gitoxide/commit/21109ca9ab21df0ab45f3be552e83114817e98d0))
    - Remove IntegerSuffix error which wasn't ever used ([`732c0fa`](https://github.com/Byron/gitoxide/commit/732c0fa6e1832efcc0de4adc894e820b3bd27b8f))
    - Adjust to improvements to the `git-config` API ([`ffc5dec`](https://github.com/Byron/gitoxide/commit/ffc5dec6b9ed2b2d19d927848006053f73741a27))
    - Handle potential issue with overrides using documentation ([`feb4eb2`](https://github.com/Byron/gitoxide/commit/feb4eb26b33a5d3824fe98259193bed7961f6fef))
    - refactor how environment overrides work ([`99d98ec`](https://github.com/Byron/gitoxide/commit/99d98ece1688880f3b0b35bc4f7ab7ddd9289f1f))
    - docs for git-glob ([`8f4969f`](https://github.com/Byron/gitoxide/commit/8f4969fe7c2e3f3bb38275d5e4ccb08d0bde02bb))
    - fix build warnings ([`4496b5a`](https://github.com/Byron/gitoxide/commit/4496b5a26abaf91fd4844e0494aaa1b4cce73628))
    - fix docs ([`3366696`](https://github.com/Byron/gitoxide/commit/3366696af9ec58ebb43ed7d4dde9d2c79ca71d3d))
    - adjust to changes in git-discover ([`3271979`](https://github.com/Byron/gitoxide/commit/3271979f86bd5fb009a946cb06cd4ce8ea03119c))
    - fix build ([`cb56f12`](https://github.com/Byron/gitoxide/commit/cb56f12ad83cf2932a068ef4fa0ca5ce4aa73e84))
    - Adapt to changes in git-config ([`61ea4c4`](https://github.com/Byron/gitoxide/commit/61ea4c4a254bafd3d1f0c18cf1c10cbd66c15a4d))
    - refactor; add worktree id and determine main status of worktree ([`54be8e3`](https://github.com/Byron/gitoxide/commit/54be8e3da14b92f0c2dad32a969a651ad9ba9eec))
    - properly handle common-dir on repo open ([`de0cc1b`](https://github.com/Byron/gitoxide/commit/de0cc1bd1a1ccb26fa4fc5f7d8aedb422226b4a1))
    - first step towards getting repos from worktree proxies ([`60d0433`](https://github.com/Byron/gitoxide/commit/60d0433d8f6dda1e3556a73f85edff1c04d46dff))
    - refactor ([`7b5fe1d`](https://github.com/Byron/gitoxide/commit/7b5fe1de5332ca8a85741c7c0872130b5ebd31f2))
    - Keep instantiation options in Repository for worktrees ([`d25c938`](https://github.com/Byron/gitoxide/commit/d25c938e01e2fc8e9dd44724ea5017997d38e945))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Use `Integer::to_decimal()` in git-repository ([`8fb95bf`](https://github.com/Byron/gitoxide/commit/8fb95bf62a33ccef3b037162f49e9a72abb0e3d9))
 * **[#382](https://github.com/Byron/gitoxide/issues/382)**
    - refactor ([`0010675`](https://github.com/Byron/gitoxide/commit/00106757a2c86e841bcf03ae233d4ff7bfc710dd))
    - match test structure with crate structure ([`b91e4bd`](https://github.com/Byron/gitoxide/commit/b91e4bd335024d8d4404d263e5f761eced2d15e9))
    - Simplify state tests ([`fc61c0d`](https://github.com/Byron/gitoxide/commit/fc61c0d4f7cb3cd9073418e4d8edc55cd14f5fb3))
 * **[#383](https://github.com/Byron/gitoxide/issues/383)**
    - Use previously unused variable in fixture script ([`cfaf31f`](https://github.com/Byron/gitoxide/commit/cfaf31fc654472acf1aacacb516b58a3295cffcd))
 * **[#384](https://github.com/Byron/gitoxide/issues/384)**
    - No need to isolate archives by crate name ([`19d46f3`](https://github.com/Byron/gitoxide/commit/19d46f35440419b9911b6e2bca2cfc975865dce9))
    - add archive files via git-lfs ([`7202a1c`](https://github.com/Byron/gitoxide/commit/7202a1c4734ad904c026ee3e4e2143c0461d51a2))
    - auto-set commit.gpgsign=false when executing git ([`c23feb6`](https://github.com/Byron/gitoxide/commit/c23feb64ad157180cfba8a11c882b829733ea8f6))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - integrate trust model into repository discovery ([`80e8fd4`](https://github.com/Byron/gitoxide/commit/80e8fd4a5944890f43f3d888b7a73bb26351b195))
    - `path::discover::existing()` -> `path::discover()` ([`2e39b0e`](https://github.com/Byron/gitoxide/commit/2e39b0ede98826e6f85c56fef77ac65a5b7e7ac2))
    - more expressive and fuiture-proof handling of git dir access controls ([`b1d319b`](https://github.com/Byron/gitoxide/commit/b1d319b249fb6c6d4d5197734938836824789053))
    - A first PoC to show how the permissions model works in practice ([`67d5837`](https://github.com/Byron/gitoxide/commit/67d58372a8352da0197ec2992f120bd000ffe5de))
    - don't assume repos with work-trees are non-bare; make git-sec manadatory ([`9c4516d`](https://github.com/Byron/gitoxide/commit/9c4516d309fef0c6fa5396e2bc366475182e0690))
    - use `git-credentials` in `git-protocol` ([`32dc182`](https://github.com/Byron/gitoxide/commit/32dc1829a5661f66396d109c8d0a8eaae6b1f532))
 * **[#389](https://github.com/Byron/gitoxide/issues/389)**
    - `Commit::describe()` now returns annnotated tags before lighweight ones and prefers more recent ones as well ([`a1680b4`](https://github.com/Byron/gitoxide/commit/a1680b44ef568317465d2971da6e0930f9885530))
    - test all cases for the names filter in describe ([`0d9f6c6`](https://github.com/Byron/gitoxide/commit/0d9f6c6687d7b2a4c473daa1115c100ef40369e7))
    - first crude fix ([`35019f2`](https://github.com/Byron/gitoxide/commit/35019f282ca7f91bef11cacd03117a756a1bd9f2))
    - reproduce commit-describe name ordering issue ([`6d023e3`](https://github.com/Byron/gitoxide/commit/6d023e3cbed6a24821ab8a1d36084a350a39415b))
 * **[#393](https://github.com/Byron/gitoxide/issues/393)**
    - refactor ([`5044576`](https://github.com/Byron/gitoxide/commit/50445760d180d89501516fc7ed780f0d09edb2d9))
    - Use the name `state()` instead of `in_progress_operation()` ([`e9b92f0`](https://github.com/Byron/gitoxide/commit/e9b92f002eec51e5ccec74d0dbc641aabf6eda9d))
    - refactor ([`6540869`](https://github.com/Byron/gitoxide/commit/6540869ec1a1492e3338ff2d33074be33890ee8f))
    - Add `Repository::head_name()`. ([`45920da`](https://github.com/Byron/gitoxide/commit/45920da7c8c5618c6e7258de08dbd633a637d017))
 * **Uncategorized**
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - refactor ([`5ab5842`](https://github.com/Byron/gitoxide/commit/5ab58428358938bced45cc348ec76b527bca9be3))
    - Use a Cow for remote name handling ([`633a30d`](https://github.com/Byron/gitoxide/commit/633a30dc919ef4a16e4382f6f81825ff2deb7f6b))
    - adjust to changes in git-ref ([`0671586`](https://github.com/Byron/gitoxide/commit/06715861d3a1d236c310d71737ec1d1a5ca6c770))
    - allow reading information about remote branch ([`53c06c7`](https://github.com/Byron/gitoxide/commit/53c06c7e6a3003b34edaab10db1f158e2fb57403))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - fix docs ([`5ee2307`](https://github.com/Byron/gitoxide/commit/5ee23070ecfbf73e5897344421a1f1ec2917a3bd))
    - Merge branch 'main' into repo-status ([`9679d6b`](https://github.com/Byron/gitoxide/commit/9679d6b0e68c28438e22cb65c554d0b31dfaf159))
    - remove unused variant ([`da8059c`](https://github.com/Byron/gitoxide/commit/da8059ce26343c8cd275f43c879d98c92f77fa51))
    - Merge branch 'main' into repo-status ([`8122c5c`](https://github.com/Byron/gitoxide/commit/8122c5c8a122e5297832f96c07a6fb1a5df02620))
    - Merge branch 'main' into git-sec ([`2fe70f9`](https://github.com/Byron/gitoxide/commit/2fe70f96cfb68e108637ce78f8edda2eb4e2e61a))
    - Clean up the error message and comments. ([`463a705`](https://github.com/Byron/gitoxide/commit/463a705dc23cddf0ba0ec2dc578a618c793b1d9d))
    - thanks clippy ([`f802a03`](https://github.com/Byron/gitoxide/commit/f802a03dc0b04d12fa360fb570d460ad4e1eb53a))
    - Print out some human readable text if GNU sed cannot be found. ([`cf19a18`](https://github.com/Byron/gitoxide/commit/cf19a1854091dc5c709dc367ca5f9568dd7e6da8))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - repo_path -> git_dir ([`53c22ee`](https://github.com/Byron/gitoxide/commit/53c22ee00834ce5912ec28d20026032b063fd2ec))
    - Let's try not parsing the git version. ([`475e7d1`](https://github.com/Byron/gitoxide/commit/475e7d1ebdd27e3efd4bd7de6e0a1ee9447feb4b))
    - Take a couple more steps to appease the CI gods. ([`ac3c8c7`](https://github.com/Byron/gitoxide/commit/ac3c8c7397bf5294cbce97e0718bac23588b2ca5))
    - Fix the GNU sed detection so it works where /usr/bin/sed is GNU. ([`5c162e0`](https://github.com/Byron/gitoxide/commit/5c162e05299256e99abe84213b078652b5c637a0))
    - Make clippy happier. ([`a5406b5`](https://github.com/Byron/gitoxide/commit/a5406b5c06a9ecb147f5850db001de2782dd283d))
    - Pass version appropriate rebase flags to git. ([`bb18a13`](https://github.com/Byron/gitoxide/commit/bb18a13cd05ddce3e850760814e4bdc6e35e0f0e))
    - Add test coverage for RepositoryState::CherryPickSequence… ([`fc281d8`](https://github.com/Byron/gitoxide/commit/fc281d820d130b74c80d8fc139188a4c4b7b7331))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Add a few tests for good measure. ([`499c811`](https://github.com/Byron/gitoxide/commit/499c81106d520e3c8ae1aa02e905c8048a054f79))
    - in_progress_operation now returns an Option ([`172b464`](https://github.com/Byron/gitoxide/commit/172b4640984d23d7adafacd96cf9d88569d29769))
    - Tweak the naming and comments a bit ([`56038ed`](https://github.com/Byron/gitoxide/commit/56038ed075d6774043651f14abb61550539b5c26))
    - First pass at Repository::in_progress_state() ([`c2f66e4`](https://github.com/Byron/gitoxide/commit/c2f66e4ea26fb28bde80dc44ea3ea7278c2fd967))
    - Release git-glob v0.2.0, safety bump 3 crates ([`ab6bed7`](https://github.com/Byron/gitoxide/commit/ab6bed7e2aa19eeb9990441741008c430f373708))
    - Merge branch 'main' into refs-and-worktrees ([`9cf0c7b`](https://github.com/Byron/gitoxide/commit/9cf0c7bd0cc5419137db5796f3a5b91bdf3dcc94))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
</details>

## 0.16.0 (2022-04-05)

### New Features

 - <csr-id-47556f6815148ed960a727fd122f7162345544c3/> auto-calculation of a good hex-len, like what git does
   If the `core.abbrev` value isn't set or is set to `auto`.
 - <csr-id-654f4afb794a370b7cd9d9502ff6d0c3378ec417/> `Commit::describe()`
   A way to fluidly configure a `git describe` operation and run it.
   
   Along that, a new `Tag` top-level object was added as well to provide
   convenient access to otherwise lower-level objects. It's not strictly
   required for our implementation here but it's needed for a symmetric
   API.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release.
 - 2 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#364](https://github.com/Byron/gitoxide/issues/364)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - Use all tags by default, instead of requiring annotated tags ([`00c42ca`](https://github.com/Byron/gitoxide/commit/00c42ca36e93a22f233fc1d3f9a1afc241fd4464))
    - Fix off-by-one error ([`d5b8fd5`](https://github.com/Byron/gitoxide/commit/d5b8fd5d93605912ccd8e3610ed3c10ec96cf5fe))
    - auto-calculation of a good hex-len, like what git does ([`47556f6`](https://github.com/Byron/gitoxide/commit/47556f6815148ed960a727fd122f7162345544c3))
    - Parse the hex-len from config on repo-initialization ([`aee55c0`](https://github.com/Byron/gitoxide/commit/aee55c02853ce91d1e9bc10349a630bf9c8f20d8))
    - Support for simple BString powered string values ([`2381c5d`](https://github.com/Byron/gitoxide/commit/2381c5d3b91e3a071c887d9e1e166625977d5830))
    - refactor configuration handling to allow pre-parsing of common values ([`e3d280f`](https://github.com/Byron/gitoxide/commit/e3d280fc3fe09787b27c0b6354797278f6f12c9f))
    - restrict signature changes to 'Ancestores::sorting()` ([`d71bd9d`](https://github.com/Byron/gitoxide/commit/d71bd9ded1e5e5a61a27be3d55f4b85ee4049bcf))
    - Adjust to changes in git-traverse ([`8240622`](https://github.com/Byron/gitoxide/commit/824062215865e6ec12afeb2d51b3c63f15291244))
    - set MSRV to 1.54 as we really need VecDeque::binary_search ([`514e468`](https://github.com/Byron/gitoxide/commit/514e468fdc102f36f9719fcf4c8f99eb22d9c0ae))
    - support for the --max-candidates flag ([`b9e6754`](https://github.com/Byron/gitoxide/commit/b9e67540801f2630be8aa1acbfddfec4202360ac))
    - Reduce amount of max candidates, add --debug flag ([`c8c13e3`](https://github.com/Byron/gitoxide/commit/c8c13e398671a21e96282547fc0e3bd445627e2f))
    - Use hashed-hasher for an eek of performance ([`324a839`](https://github.com/Byron/gitoxide/commit/324a839e6c72174f08779a97fa12cc313e2afac2))
    - `Commit::describe()` ([`654f4af`](https://github.com/Byron/gitoxide/commit/654f4afb794a370b7cd9d9502ff6d0c3378ec417))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - sketch `open_index()` on `Worktree`, but… ([`ff76261`](https://github.com/Byron/gitoxide/commit/ff76261f568f6b717a93b1f2dcf5d8e8b63acfca))
    - refactor ([`71dd056`](https://github.com/Byron/gitoxide/commit/71dd0566cbfa9cbda148145efc78f76557663ae7))
    - a sketch of basic Worktree support ([`732f6fb`](https://github.com/Byron/gitoxide/commit/732f6fb0aa9cdc843087352b12bed2cd142ed6ec))
    - obtain the base() path of a private worktree ([`f77d8c8`](https://github.com/Byron/gitoxide/commit/f77d8c8a60f1807f77aafd7b1d71334e9710e2e8))
    - fix docs ([`1e3acd0`](https://github.com/Byron/gitoxide/commit/1e3acd08b9df9fe0cc36bb6a4d4bac57c365443d))
    - use `git-discover` crate ([`f5f9a0d`](https://github.com/Byron/gitoxide/commit/f5f9a0d609316b2a64ee665f47faade7d8277315))
    - refactor ([`00a988e`](https://github.com/Byron/gitoxide/commit/00a988e3c2c964447f675164a6126bf6cb470c6b))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
    - Remove `worktree()` platform in favor of the current worktree ([`f2a2c55`](https://github.com/Byron/gitoxide/commit/f2a2c5581eb3dde5ef7352439b564d89e9f76461))
    - basic worktree iteration ([`992a6ce`](https://github.com/Byron/gitoxide/commit/992a6ce154b97520b0c4679d6c50f4e3cc6e3091))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - maybe fix failing test on windows ([`8f69af2`](https://github.com/Byron/gitoxide/commit/8f69af2eb48f01e2bbcf7b6483ae06f9b8dea61b))
    - assure worktree test repositories are regenerated ([`2eed703`](https://github.com/Byron/gitoxide/commit/2eed70392fd06f31f08acf2caa94437e967c7a1f))
    - Learn to read the common dir ([`e07c453`](https://github.com/Byron/gitoxide/commit/e07c453ea20e29994520dcd6346ac0a28f585813))
    - A first stab at more control over which worktrees and git-dirs to use ([`83ac263`](https://github.com/Byron/gitoxide/commit/83ac2638dd52e9da9a0dc8a62b4c9669c8eec372))
    - adapt to changes in git-ref ([`49b0e89`](https://github.com/Byron/gitoxide/commit/49b0e89440ffcc5fa5dc66be45112f9c1f7d9244))
    - devise a worktree API that can work even if a valid worktree isn't present ([`8d067d1`](https://github.com/Byron/gitoxide/commit/8d067d113acfaf9a3e28ba1a829b07303a80e992))
    - reorganize types to properly represent worktrees in their various 'states' ([`b46bff5`](https://github.com/Byron/gitoxide/commit/b46bff58e40bb9805af7ee7f96272f0dc19c0ac7))
    - parse baseline worktree listing ([`aabe8b2`](https://github.com/Byron/gitoxide/commit/aabe8b2edc0753f125dcdea71dd44908d1826a21))
    - A first test to run against a bare and non-bare repos with worktrees ([`70164d7`](https://github.com/Byron/gitoxide/commit/70164d7252f57bd4b645d8ca694e7458ce4d1a0f))
    - Instantiate worktree aware versions of file stores ([`0b670dd`](https://github.com/Byron/gitoxide/commit/0b670ddf97b316f0a6c332d999265a3bda7fcdab))
    - adjust to changes in git-ref ([`3299606`](https://github.com/Byron/gitoxide/commit/32996060c7405be787b8f5b91041e5d6dcd9ffc9))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - some TODOs related to precomposed unicode on MacOS ([`bc246aa`](https://github.com/Byron/gitoxide/commit/bc246aaa81cd7023e8533a006211a800621e8907))
 * **Uncategorized**
    - Release git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0 ([`f041c00`](https://github.com/Byron/gitoxide/commit/f041c00a7df2455ca52fac7b83af1e9f335f5688))
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - thanks clippy ([`7887d8b`](https://github.com/Byron/gitoxide/commit/7887d8b5bedc49890bd73beb058a9828aa734729))
    - thanks clippy ([`d624f4e`](https://github.com/Byron/gitoxide/commit/d624f4e7fafd821867a41548b49f2cd7f09def8c))
    - thanks clippy ([`0f5a943`](https://github.com/Byron/gitoxide/commit/0f5a9439d6b1716345f0e122c23c1a566fdd3088))
    - thanks clippy ([`9407532`](https://github.com/Byron/gitoxide/commit/9407532b98646d33bb0b947860a6a0022cfbae28))
    - thanks clippy ([`60cb858`](https://github.com/Byron/gitoxide/commit/60cb8589e901981802be11289352510a9d43cd87))
    - thanks clippy ([`6fb19cf`](https://github.com/Byron/gitoxide/commit/6fb19cfee79a49741dd439ade9c638aa89943f10))
    - thanks clippy ([`f2faa00`](https://github.com/Byron/gitoxide/commit/f2faa001ed2c8e96e25dbd56544320055f8dbe1b))
    - thanks clippy ([`a7ac64c`](https://github.com/Byron/gitoxide/commit/a7ac64cd801b985790b5717be1a5dc722b2ae3a9))
</details>

## 0.15.0 (2022-04-03)

<csr-id-5f7595305efc85d6ca3c541e9f9adac3915cbd84/>
<csr-id-c10f07c50f6dde4b39bf1e3ff26c239c5f202912/>
<csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/>

### New Features

 - <csr-id-1322dbf6827ea5cc1d71175afb669e01fb1242ef/> support for object replacement
   The Repository now respects replacement refs created by `git replace`
   and picks up environment variables for its configuration as well.
   
   Which environment variables are used is fully configurable.
 - <csr-id-a39bf71531ee0a6c8db082758d3212c805ce2bf0/> support for trimming of whitespace around name and email
   It's separated from parsing to assure we can round-trip, but it's
   made easy to obtain trimmed results using new methods.
   
   This high-level git-repository will also trim by default now.
 - <csr-id-00578040a699e1939b3d3813616d3cc4e1d8669e/> `Repository::head_commit()`
   A shortcut to get to the commit much faster.
 - <csr-id-def80df2e165b74f4b053e4030f563902b7d34a4/> `ein tool estimate-hours` now supports mailmaps
 - <csr-id-f0d8a49587c08713350252e1701a45bb308b6f9d/> `Repository::head_id()`
   A long-needed shortcut.
 - <csr-id-d2388d8d80f379eccc9ee84ebe07acd67d154630/> `gix repository mailmap entries`
 - <csr-id-e3bc1b410409a9e27894a5cac48b06d8c3295e36/> unstable mailmap module
 - <csr-id-1be00cf9e00ce9428ffddb2c79b2373926069b13/> `Commit::short_id()`
 - <csr-id-c7dff9e8b695d298a3fb21f19f51752a885a5ce3/> in-manifest and in-lib documentation of feature toggles
 - <csr-id-9f5663ed83d83c7335b346313837d4cada9cd846/> `easy::Commit::time()` to access the committers time conveniently.
 - <csr-id-7c88b62e439af7a60ddb68fb6737cb3b1cebf00d/> easy::Head::name() to learn about the name of the HEAD ref
   It's mainly for completeness to provide people with with a `FullNameRef`
   of HEAD.
 - <csr-id-3b0913a2e6695e4e9e94341ef48d2ba3b4a518e6/> `easy::Head::peel_to_commit_in_place()`
   It allows to quickly get a commit from the head, something most people
   want when getting started with any kind of tool.
 - <csr-id-1c22d76c26464db4a185e19bb6c1f9a17fa19bc9/> `Repsitory::load_index()`
   This method makes the index of the default workspace available.

### Bug Fixes

 - <csr-id-c329dd75420f82d506fd415cd377f7df6c6ccbad/> Properly classify worktrees as non-bare, helps with `ein t find`
   They use git-files which point to the actual repository data.

### Changed (BREAKING)

 - <csr-id-a8b6589a7c645f323f95da6cb94321fc967e9b06/> Easier access to local and remote branches

### New Features (BREAKING)

 - <csr-id-8945d95f7fa88562d37ff67ac6e38bead73dd2df/> `interrupt::Iter`, rename `interrupt::Iter` -> `interrupt::IterWithError`
 - <csr-id-813a3bea88cdbe1fd9b0a8070efeee2a44f7823e/> Let 'easy::Object::try_into_…()` return `try_into::Error`.
   That way, the typical usage of `try_into_commit()?` will not result
   in a strange error about `Object` not being convertible into some
   error. We think having a real error there is the least surprising.

### Bug Fixes (BREAKING)

 - <csr-id-c863ea5b34fa9ee3dac21c1f85587da16045f8d8/> do not install signal handlers by default
   The previous behaviour is meant to be convenient for the casual
   user even though it
   ends up being surprising when used in applications that install
   their own signal handlers and need more control over how the program
   shuts down.
   
   This is now fixed by **requiring an explicit `setup()`** call before
   the first tempfile is created, which makes it a breaking change.

### Other (BREAKING)

 - <csr-id-5f7595305efc85d6ca3c541e9f9adac3915cbd84/> `Id::prefix` -> `Id::shorten()`
   It's definitely more intuitive that way.

### Refactor (BREAKING)

 - <csr-id-c10f07c50f6dde4b39bf1e3ff26c239c5f202912/> disoolve 'easy' module by moving everything one level up
 - <csr-id-bbc6efeceb26050973e1425e68a52e51b9df4572/> clarify different repository types much better

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 86 commits contributed to the release over the course of 68 calendar days.
 - 69 days passed between releases.
 - 21 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#329](https://github.com/Byron/gitoxide/issues/329), [#336](https://github.com/Byron/gitoxide/issues/336), [#364](https://github.com/Byron/gitoxide/issues/364), [#366](https://github.com/Byron/gitoxide/issues/366)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - `easy::Commit::time()` to access the committers time conveniently. ([`9f5663e`](https://github.com/Byron/gitoxide/commit/9f5663ed83d83c7335b346313837d4cada9cd846))
    - easy::Head::name() to learn about the name of the HEAD ref ([`7c88b62`](https://github.com/Byron/gitoxide/commit/7c88b62e439af7a60ddb68fb6737cb3b1cebf00d))
    - fix build ([`d89a587`](https://github.com/Byron/gitoxide/commit/d89a587cd05c8d1697d250eb19ea29d32192de0e))
    - `Repsitory::load_index()` ([`1c22d76`](https://github.com/Byron/gitoxide/commit/1c22d76c26464db4a185e19bb6c1f9a17fa19bc9))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - actual default hex-len for short-ids is 7, but… ([`36e004b`](https://github.com/Byron/gitoxide/commit/36e004b05cb996c50f6446381e5574faacc329c2))
    - quick sketch of how the retrieval of ints from config could look like ([`af6326f`](https://github.com/Byron/gitoxide/commit/af6326f534ca0b5982fd752c40933ea4cf4af59f))
    - frame for simplified config access via Repository ([`eba2b9a`](https://github.com/Byron/gitoxide/commit/eba2b9a6bbfe6fe4ad1df657955061a66ea7c06c))
    - fix docs; consistent naming of 'repo' ([`1f79bc3`](https://github.com/Byron/gitoxide/commit/1f79bc32ee3d7a70985b7bef830ccdd1dc762f05))
    - rename `sync::Handle` into `ThreadSafeRepository` ([`1cc4faa`](https://github.com/Byron/gitoxide/commit/1cc4faaaffa9b5c461659a84b3abc6ebf577945c))
    - disoolve 'easy' module by moving everything one level up ([`c10f07c`](https://github.com/Byron/gitoxide/commit/c10f07c50f6dde4b39bf1e3ff26c239c5f202912))
    - prepare for moving `git-repository::easy:: one level up ([`ccecb9a`](https://github.com/Byron/gitoxide/commit/ccecb9ab5134bb0abb44de1c462b588e64cf5b9b))
    - make config available in easy::Repository ([`fbdb1a2`](https://github.com/Byron/gitoxide/commit/fbdb1a2ddc0ec3846418f69aa9b0304c061ff54f))
    - refactor ([`591b533`](https://github.com/Byron/gitoxide/commit/591b5338ecdc0da33151baa0781fd8dc1ee8d5a9))
    - refactor ([`a1a846a`](https://github.com/Byron/gitoxide/commit/a1a846a3d804fb62d87468717e591375410fdbca))
    - clarify different repository types much better ([`bbc6efe`](https://github.com/Byron/gitoxide/commit/bbc6efeceb26050973e1425e68a52e51b9df4572))
    - docs ([`a45f378`](https://github.com/Byron/gitoxide/commit/a45f3789696078848e2e96ddb8a55570c941dd53))
    - First stab at Oid::prefix() ([`35e77c1`](https://github.com/Byron/gitoxide/commit/35e77c16b05aa08d090d08a8442ff5dd58750e13))
    - `easy::Head::peel_to_commit_in_place()` ([`3b0913a`](https://github.com/Byron/gitoxide/commit/3b0913a2e6695e4e9e94341ef48d2ba3b4a518e6))
    - Let 'easy::Object::try_into_…()` return `try_into::Error`. ([`813a3be`](https://github.com/Byron/gitoxide/commit/813a3bea88cdbe1fd9b0a8070efeee2a44f7823e))
 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - `interrupt::Iter`, rename `interrupt::Iter` -> `interrupt::IterWithError` ([`8945d95`](https://github.com/Byron/gitoxide/commit/8945d95f7fa88562d37ff67ac6e38bead73dd2df))
    - refactor ([`9ea1e44`](https://github.com/Byron/gitoxide/commit/9ea1e4474a3ce803da7a56e1fc1748f65c11a876))
    - unset the pack cache if GITOXIDE_DISABLE_PACK_CACHE is set ([`6d8bc49`](https://github.com/Byron/gitoxide/commit/6d8bc4959765340be53f445d9709d3056eaeecd8))
    - frame for traversing tree entries ([`0e55fbb`](https://github.com/Byron/gitoxide/commit/0e55fbb2fb0cec6f402b7a3aed7ee55078d233a1))
    - basic version of index checkout via command-line ([`f23b8d2`](https://github.com/Byron/gitoxide/commit/f23b8d2f1c4b767d337ec51888afaa8b3719798c))
    - support for unicode-precomposition for gix apps ([`e90c123`](https://github.com/Byron/gitoxide/commit/e90c123675a98ab62fc6bb22019f889cee8b7301))
    - `Commit::short_id()` ([`1be00cf`](https://github.com/Byron/gitoxide/commit/1be00cf9e00ce9428ffddb2c79b2373926069b13))
    - verify that Id::prefix() makes use of the git configuration ([`76e9110`](https://github.com/Byron/gitoxide/commit/76e911083fbb789e1cd3f84c194759517625182c))
 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - in-manifest and in-lib documentation of feature toggles ([`c7dff9e`](https://github.com/Byron/gitoxide/commit/c7dff9e8b695d298a3fb21f19f51752a885a5ce3))
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **[#336](https://github.com/Byron/gitoxide/issues/336)**
    - do not install signal handlers by default ([`c863ea5`](https://github.com/Byron/gitoxide/commit/c863ea5b34fa9ee3dac21c1f85587da16045f8d8))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - support for object replacement ([`1322dbf`](https://github.com/Byron/gitoxide/commit/1322dbf6827ea5cc1d71175afb669e01fb1242ef))
    - initialize replacements in a configurable way ([`6a27985`](https://github.com/Byron/gitoxide/commit/6a27985a8e68896be0d7e10c27ddf5a20018e04b))
    - add some precaution to avoid strange interactions with packs ([`b052a9a`](https://github.com/Byron/gitoxide/commit/b052a9a3e9127fd9a4029594ea9de6e436db03c6))
    - commit traversals on shallow clones are non-fatal by default ([`1a75357`](https://github.com/Byron/gitoxide/commit/1a75357f3d7d3cb70f1700d471b1c9c5b953c292))
    - adapt to changes in git-ref ([`f606f88`](https://github.com/Byron/gitoxide/commit/f606f88075d84a52d145a1f87ec2eae0659af36b))
    - support for trimming of whitespace around name and email ([`a39bf71`](https://github.com/Byron/gitoxide/commit/a39bf71531ee0a6c8db082758d3212c805ce2bf0))
    - Add and improve Debug implementation on major types ([`d23c3d4`](https://github.com/Byron/gitoxide/commit/d23c3d4c6e7ac1d712f7c2b7ebf4cfe45923ee6e))
    - also inform about average and max commit sizes ([`5052a4e`](https://github.com/Byron/gitoxide/commit/5052a4e532fec63ccf49f6ce54df41707779e70b))
    - Adjust to breaking changes in `git-traverse` ([`d79b506`](https://github.com/Byron/gitoxide/commit/d79b5064eab2d1bef445e6c9e62a53466a8d5225))
    - fix install_dir(); refactor ([`11644bd`](https://github.com/Byron/gitoxide/commit/11644bd53a2c8fb60a22ef244bb7ef11024a83a2))
    - `Repository::head_commit()` ([`0057804`](https://github.com/Byron/gitoxide/commit/00578040a699e1939b3d3813616d3cc4e1d8669e))
    - `Repository::head_id()` ([`f0d8a49`](https://github.com/Byron/gitoxide/commit/f0d8a49587c08713350252e1701a45bb308b6f9d))
    - fix docs ([`29822c6`](https://github.com/Byron/gitoxide/commit/29822c65d398efcd95b7eb9c668a7841b3d54ed9))
    - `Id::prefix` -> `Id::shorten()` ([`5f75953`](https://github.com/Byron/gitoxide/commit/5f7595305efc85d6ca3c541e9f9adac3915cbd84))
    - refactor ([`b1b9871`](https://github.com/Byron/gitoxide/commit/b1b9871e8b0c2bcbdee0c3ea4c060b4a7c32bc15))
    - Easier access to local and remote branches ([`a8b6589`](https://github.com/Byron/gitoxide/commit/a8b6589a7c645f323f95da6cb94321fc967e9b06))
    - Fix lifetime declarations to allow ancestors().all() chaining ([`df24f16`](https://github.com/Byron/gitoxide/commit/df24f16ffdec355940a6c1b2d5e9d9d6f0ce24d1))
    - consolidate naming of directories, use same convention as git2 ([`a7dbed1`](https://github.com/Byron/gitoxide/commit/a7dbed193cc25d05e03c4f2148d0fa9562a4a586))
    - Full error handling for CommitRefIter ([`b94471a`](https://github.com/Byron/gitoxide/commit/b94471a0ced50204156cf5d4126c676f0258a5eb))
    - More speedy access to author/committer ([`6129607`](https://github.com/Byron/gitoxide/commit/61296077cebaaf2eb939fa6082121304bc6cf39b))
    - adjust to changes in git-actor ([`e5c0200`](https://github.com/Byron/gitoxide/commit/e5c02002467a6ad2ab2330cf6f38bcebabf4ba7c))
    - cleaner API for detaching objects, now for commits as well ([`59d75fc`](https://github.com/Byron/gitoxide/commit/59d75fce0d2292733afd455f1acbcb4711ba3f9b))
 * **[#366](https://github.com/Byron/gitoxide/issues/366)**
    - `ein tool estimate-hours` now supports mailmaps ([`def80df`](https://github.com/Byron/gitoxide/commit/def80df2e165b74f4b053e4030f563902b7d34a4))
    - `gix repository mailmap entries` ([`d2388d8`](https://github.com/Byron/gitoxide/commit/d2388d8d80f379eccc9ee84ebe07acd67d154630))
    - frame for printing mailmap entries using git-repository ([`2a01f47`](https://github.com/Byron/gitoxide/commit/2a01f4728ae858b47280b587501d343fdb86655d))
    - the first possibly working version of loading a mailmap with multiple sources ([`98d745e`](https://github.com/Byron/gitoxide/commit/98d745e8080975a91cff1ce75e187258c851d3f4))
    - frame for `Repository::load_mailmap_into()` ([`c8c87ec`](https://github.com/Byron/gitoxide/commit/c8c87ec12f7ff5061132f9e67828d59ac51a8043))
    - unstable mailmap module ([`e3bc1b4`](https://github.com/Byron/gitoxide/commit/e3bc1b410409a9e27894a5cac48b06d8c3295e36))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Merge branch 'for-onefetch' ([`8e5cb65`](https://github.com/Byron/gitoxide/commit/8e5cb65da75036a13ed469334e7ae6c527d9fff6))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - Merge branch 'svetli-n-refactor_git_config_tests' ([`babaa9f`](https://github.com/Byron/gitoxide/commit/babaa9f5725ab8cdf14e0c7e002c2e1de09de103))
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Properly classify worktrees as non-bare, helps with `ein t find` ([`c329dd7`](https://github.com/Byron/gitoxide/commit/c329dd75420f82d506fd415cd377f7df6c6ccbad))
    - fix MSRV ([`1bf5d11`](https://github.com/Byron/gitoxide/commit/1bf5d11ec0b9df2f1c6fb5239007ad8409b97f75))
    - thanks clippy ([`8e2e4e3`](https://github.com/Byron/gitoxide/commit/8e2e4e352b563b2accd2e9d91c6e8a33b5a9709c))
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
    - Set the MSRV version explicitly in git-repository ([`bbf6799`](https://github.com/Byron/gitoxide/commit/bbf6799db01e25c8e3f49e0fd6ff3ec802e773a0))
    - thanks clippy ([`4618f8a`](https://github.com/Byron/gitoxide/commit/4618f8aa7648c0553a8e1b023fceb6738654e38b))
    - thanks clippy ([`5db3993`](https://github.com/Byron/gitoxide/commit/5db39936fc003a79f18e545a8317305fe18af74d))
    - remove unused dependency ([`2fbc93c`](https://github.com/Byron/gitoxide/commit/2fbc93cc2ce855f24aea63c8513cf1e037c685a1))
    - thanks clippy ([`d5911b5`](https://github.com/Byron/gitoxide/commit/d5911b59e4bd039fe39702487640d18319c0ed7e))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
    - Merge branch 'AP2008-implement-worktree' ([`f32c669`](https://github.com/Byron/gitoxide/commit/f32c669bc519d59a1f1d90d61cc48a422c86aede))
    - Release git-actor v0.8.1 ([`08fe550`](https://github.com/Byron/gitoxide/commit/08fe5508472f2eb209db8a5fc4e4942a9d7db93d))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - fix lint ([`b339b41`](https://github.com/Byron/gitoxide/commit/b339b419bde0418fb4fcd998e232b1eba836f7a4))
    - remove debug-helper ([`c243215`](https://github.com/Byron/gitoxide/commit/c2432158ca4be3008847bce40cfe536e082d1f4a))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Don't use bleeding edge features ([`3de0ab1`](https://github.com/Byron/gitoxide/commit/3de0ab1163d267102e7605da1d7a114574508a00))
    - reference statistics for stats example ([`83b99ce`](https://github.com/Byron/gitoxide/commit/83b99cee89dd55550503290602a5ab62c62dec55))
    - Merge branch 'index-verification' ([`ad3c803`](https://github.com/Byron/gitoxide/commit/ad3c8032cee02052ef3940d1d7c950270a0a299a))
    - Release git-config v0.1.11 ([`a605b67`](https://github.com/Byron/gitoxide/commit/a605b67294773628590220600f5017c63911f620))
    - thanks clippy ([`2f25bf1`](https://github.com/Byron/gitoxide/commit/2f25bf1ebf44aef8c4886eaefb3e87836d535f61))
    - Merge branch 'index-information' ([`025f157`](https://github.com/Byron/gitoxide/commit/025f157de10a509a4b36a9aed41de80487e8c15c))
</details>

## 0.14.0 (2022-01-23)

<csr-id-7a91212631219e94b9454d2874b53f3ecc1db77e/>
<csr-id-b2cc0c63570d45de032d63e62d94c3344783440e/>
<csr-id-ebc7f47708a63c3df4415ba0e702660d976dfb3e/>
<csr-id-2290d006705ff47ad780b009fe58ee422b3285af/>

### New Features

 - <csr-id-667485e133ca29fcc6914a7142cf953564b5fce3/> Add `easy::Tree::traverse()` platform
 - <csr-id-8f650c089c88698483f778aa5c0070f606b94f09/> Add `easy::Commit` object
   It allows to more conveniently access commit information.
 - <csr-id-0ae2a8da010d848d98bef47ac923ae1d770091ff/> `easy::Oid::ancestors()` now supports `sorting()` and iteration by first commit only
   Especially the sorting is useful to avoid having to sort commits by
   hand after collecting them.
 - <csr-id-bc77534f9c385046f6c9adb994b1443307afda46/> Use GITOXIDE_OBJECT_CACHE_MEMORY to control how much object cache is used
   Note that this is mostly for debugging or quickly seeing if object
   caches help with certain operations.
   
   Ideally the implementation knows themselves and sets up caches
   accordingly, probably after trying it with these environment variables.

### Changed (BREAKING)

 - <csr-id-6e3a745dfada66a2fcac256dae0ac63959e74d08/> rename `easy::Object` methods returning `Ref` objects to have `ref` in their name
   That way, it's more clear that the `Ref` versions are low-level ones
   whereas the `into_` ones are higher-level ones that are part of the
   `easy` suite.
 - <csr-id-b6730979808ce28b98c65888a349f1e3d0ea1b9a/> Rename `OwnedObject` to `DetachedObject`
   The latter more clearly indicates what the difference is to
   `Object` (which is attached and carries a lifetime)
 - <csr-id-c4184f3c31ffc4597bd089e8140653906a6594d8/> Remove easy::borrow::Error entirely; support for multiple objects per handle
   This massive simplification finally allows any amounts of objects to be
   created while adding support for reusing their data buffers thanks
   to a simple free-list stored with the handle.
 - <csr-id-880b56426859306aa30038ff35e2ad14607e9e90/> rename `easy::Object` to `OwnedObject`; remove `Ref` suffix from `ObjectRef` and `TreeRef`
 - <csr-id-f9c0493460ab7c664aaa231ffcf7dfd56076c920/> use `git_odb::Find*` traits in prelude, instead of `git_pack::Find*`
   These are higher-level and generally more desirable.
   The Find traits in `git-pack` are more useful internally when packs
   have to be handled directly, for example when generating packs.
 - <csr-id-83d7b31e7dd6d09eea79fc3c68620d099459132f/> rename easy::State to easy::Handle
   As the first step to remove the 'Easy' abstraction.
 - <csr-id-5e7aa1689f5d7ea5b510611a3ca0868828226291/> fully rely on OdbHandle in repository State
 - <csr-id-57de915886b76f80b3641def0ccf4fd79e334fc8/> Rename `Repository::odb` to` Repository::objects`
   This way it's more inline with `Repository::refs`.
 - <csr-id-93db4a5e70456d2c33ea010e3c86e5f26eb1bcc0/> remove Repository::refresh_object_database()
   With the linked DB this is simply not possible anymore and we expect
   these updates to happen automatically in future for greater convenience.
   
   For now, in order to refresh a repository, one has to reopen it.
 - <csr-id-580e96c1b2d9782a2e8cf9d1123f6d53a5376a3d/> Rename `Handle` to `Cache`
   Because this is exactly what it is effectively.
   Also add some basic instantiation for the new object store.
 - remove borrowing Repo as possible failure cause
   The `easy::Handle` is now a full (but shared) clone of the original
   Rpeository with additional thread-local state, hence there is no more
   need for a way to access the original repository.
 - remove Easy… abstraction in favor of Handle
   This great reduction of complexity allows for being multi-threading
   capabie by default with the option to turn that off at compile time.
   
   All `to|into_easy…()` methods are removed in favor of `to_easy()`
   along with the removal of all `Easy` types in favor of the single
 - remove pack-cache from `Find::try_find(…)`
   With the new architecture this can be an implementation detail without
   forcing it to be Sync.
 - move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait
   This will break a lot, but has to happen to prepare these traits for the
   next generation of object databases.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 65 commits contributed to the release over the course of 51 calendar days.
 - 55 days passed between releases.
 - 18 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 7 unique issues were worked on: [#215](https://github.com/Byron/gitoxide/issues/215), [#266](https://github.com/Byron/gitoxide/issues/266), [#274](https://github.com/Byron/gitoxide/issues/274), [#279](https://github.com/Byron/gitoxide/issues/279), [#287](https://github.com/Byron/gitoxide/issues/287), [#293](https://github.com/Byron/gitoxide/issues/293), [#298](https://github.com/Byron/gitoxide/issues/298)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#215](https://github.com/Byron/gitoxide/issues/215)**
    - `easy::Oid::ancestors()` now supports `sorting()` and iteration by first commit only ([`0ae2a8d`](https://github.com/Byron/gitoxide/commit/0ae2a8da010d848d98bef47ac923ae1d770091ff))
    - refactor ([`9af2a94`](https://github.com/Byron/gitoxide/commit/9af2a9431005f6bd235881c34baf176b6fc9f686))
    - Use GITOXIDE_OBJECT_CACHE_MEMORY to control how much object cache is used ([`bc77534`](https://github.com/Byron/gitoxide/commit/bc77534f9c385046f6c9adb994b1443307afda46))
    - Don't read environment variables each time an pack cache is created ([`91d7ef2`](https://github.com/Byron/gitoxide/commit/91d7ef295e5bca4368b6161b497d7796c99c115f))
 * **[#266](https://github.com/Byron/gitoxide/issues/266)**
    - Default handle refresh mode is the least surprising, with option to configure ([`1b74c14`](https://github.com/Byron/gitoxide/commit/1b74c14c99a3076753f166dc1a6a4451bca490d2))
    - refactor ([`b88f253`](https://github.com/Byron/gitoxide/commit/b88f253e46e7ad0a50b670b96c1bfa09eaaecaef))
    - refactor ([`52a4dcd`](https://github.com/Byron/gitoxide/commit/52a4dcd3a6969fa8f423ab39c875f98f9d210e95))
    - A quick and dirty version index iteration ([`0384007`](https://github.com/Byron/gitoxide/commit/0384007cd9e813cf4bfb13642adef8a602d219ad))
    - Use new store in git-repository ([`2f9e342`](https://github.com/Byron/gitoxide/commit/2f9e342b63f9e5c925d8e85ebc0a0be693ca0901))
    - add docs for handle-related functions ([`cf1b1e6`](https://github.com/Byron/gitoxide/commit/cf1b1e6d82f691ab17975e4f1479d93720368803))
    - use `git_odb::Find*` traits in prelude, instead of `git_pack::Find*` ([`f9c0493`](https://github.com/Byron/gitoxide/commit/f9c0493460ab7c664aaa231ffcf7dfd56076c920))
    - fix git-repository docs ([`3496a97`](https://github.com/Byron/gitoxide/commit/3496a970c0918c309075a0ecad7b84b449a6e4cf))
    - remove borrowing Repo as possible failure cause ([`7a91212`](https://github.com/Byron/gitoxide/commit/7a91212631219e94b9454d2874b53f3ecc1db77e))
    - Adjust object-acess to test new contains method ([`8488b41`](https://github.com/Byron/gitoxide/commit/8488b41651751d9177f53a23233b7ddd655dd696))
    - remove Easy… abstraction in favor of Handle ([`b2cc0c6`](https://github.com/Byron/gitoxide/commit/b2cc0c63570d45de032d63e62d94c3344783440e))
    - rename easy::State to easy::Handle ([`83d7b31`](https://github.com/Byron/gitoxide/commit/83d7b31e7dd6d09eea79fc3c68620d099459132f))
    - Remove unnecessary error variants now that repo() is called less ([`afcd579`](https://github.com/Byron/gitoxide/commit/afcd579e53c09b8d1c39be16f516584f6ff93bfa))
    - assure loops can't happen anymore ([`f04ff80`](https://github.com/Byron/gitoxide/commit/f04ff8011198b7f6c45c2094530903316c6e91ea))
    - Use db handle for writing ([`053e7b6`](https://github.com/Byron/gitoxide/commit/053e7b61c093021b9931f1cca105a462ba4fc3cf))
    - Adapt to changes in git-repository ([`3ab9b03`](https://github.com/Byron/gitoxide/commit/3ab9b03eee7d449b7bb87cb7dcbf164fdbe4ca48))
    - fully rely on OdbHandle in repository State ([`5e7aa16`](https://github.com/Byron/gitoxide/commit/5e7aa1689f5d7ea5b510611a3ca0868828226291))
    - Rename `Repository::odb` to` Repository::objects` ([`57de915`](https://github.com/Byron/gitoxide/commit/57de915886b76f80b3641def0ccf4fd79e334fc8))
    - Add odb handle to state ([`4e38da3`](https://github.com/Byron/gitoxide/commit/4e38da35be4d753c30e07ed292ae8ce15513bcfe))
    - remove Repository::refresh_object_database() ([`93db4a5`](https://github.com/Byron/gitoxide/commit/93db4a5e70456d2c33ea010e3c86e5f26eb1bcc0))
    - remove pack-cache from `Find::try_find(…)` ([`ebc7f47`](https://github.com/Byron/gitoxide/commit/ebc7f47708a63c3df4415ba0e702660d976dfb3e))
    - Rename `Handle` to `Cache` ([`580e96c`](https://github.com/Byron/gitoxide/commit/580e96c1b2d9782a2e8cf9d1123f6d53a5376a3d))
    - First sketch of general store ([`fc1b640`](https://github.com/Byron/gitoxide/commit/fc1b6409380256b73cf271c105802f4494dbb8c5))
    - move git_pack::data::Object to git_object::Data, massively alter git_odb::Find trait ([`2290d00`](https://github.com/Byron/gitoxide/commit/2290d006705ff47ad780b009fe58ee422b3285af))
 * **[#274](https://github.com/Byron/gitoxide/issues/274)**
    - Rename `OwnedObject` to `DetachedObject` ([`b673097`](https://github.com/Byron/gitoxide/commit/b6730979808ce28b98c65888a349f1e3d0ea1b9a))
    - Fix docs ([`acb0ccc`](https://github.com/Byron/gitoxide/commit/acb0cccabf9f2a9cd966a2473da65db170e434e3))
    - Remove easy::borrow::Error entirely; support for multiple objects per handle ([`c4184f3`](https://github.com/Byron/gitoxide/commit/c4184f3c31ffc4597bd089e8140653906a6594d8))
    - rename `easy::Object` to `OwnedObject`; remove `Ref` suffix from `ObjectRef` and `TreeRef` ([`880b564`](https://github.com/Byron/gitoxide/commit/880b56426859306aa30038ff35e2ad14607e9e90))
 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - fix docs ([`b61a920`](https://github.com/Byron/gitoxide/commit/b61a9200d267865be76bdd2f36477c3940bc4dcc))
    - rename `easy::Object` methods returning `Ref` objects to have `ref` in their name ([`6e3a745`](https://github.com/Byron/gitoxide/commit/6e3a745dfada66a2fcac256dae0ac63959e74d08))
    - cargo fmt ([`8b9da35`](https://github.com/Byron/gitoxide/commit/8b9da35b3e0d3458efcac150f7062c9d7382a6c4))
    - Deal with changes to git-odb `Write` trait ([`4d67122`](https://github.com/Byron/gitoxide/commit/4d6712210555c7ac88940be2a271471ee1e7cb97))
    - adapt to changes to `git-odb` ([`5b0e2b9`](https://github.com/Byron/gitoxide/commit/5b0e2b927eac75548d5a9f3cf302aa5eda70a795))
    - First pieces of header parsing; allow to respect multi-index desired hash kind in git-odb ([`1a2a049`](https://github.com/Byron/gitoxide/commit/1a2a04930ab56ba778091e10b15cecf415f5058d))
    - refactor ([`4e89d8d`](https://github.com/Byron/gitoxide/commit/4e89d8d16dc0af56b07c9ef0de35035154162430))
    - Respect `core.multiPackIndex` option ([`1495efc`](https://github.com/Byron/gitoxide/commit/1495efcc914449f9680f9141805d60b1f3188001))
 * **[#287](https://github.com/Byron/gitoxide/issues/287)**
    - basic output for 'repo verify' json only ([`9f8d61f`](https://github.com/Byron/gitoxide/commit/9f8d61f164fb3fbdb76cc44fbd634ca5db35b3b8))
 * **[#293](https://github.com/Byron/gitoxide/issues/293)**
    - refactor ([`8bf585d`](https://github.com/Byron/gitoxide/commit/8bf585d67cd67b168d819ba05858cef7d9b90894))
    - JSON output for index entries ([`3fc1622`](https://github.com/Byron/gitoxide/commit/3fc1622488054c6ab655eb9d2f941b68cc3ccf18))
    - make clear what 'steal' actually steals from ([`1b0ab44`](https://github.com/Byron/gitoxide/commit/1b0ab449af18ebf876abeafdb35bf416039f665d))
    - Make obvious that we steal data from the free list ([`3523aa4`](https://github.com/Byron/gitoxide/commit/3523aa433d4d87d5f75ca7bb7c1b1e228c0aa07d))
    - handle won't try to reuse empty buffers to allow it to be claimed ([`0fb4c91`](https://github.com/Byron/gitoxide/commit/0fb4c91c32ee67642e52ce70e3b4060ca1dd3952))
 * **[#298](https://github.com/Byron/gitoxide/issues/298)**
    - frame for printing index information ([`9ea98fd`](https://github.com/Byron/gitoxide/commit/9ea98fda75fbef339647a0ca03776060356d1206))
 * **Uncategorized**
    - Release git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`b286b24`](https://github.com/Byron/gitoxide/commit/b286b24a51878be7d2e0fd77ff0c5c99b439a6a0))
    - Release git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`42ebb53`](https://github.com/Byron/gitoxide/commit/42ebb536cd6086f096b8422291776c9720fa0948))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Experiment with novel API idea around Tree breadthfirst traversal ([`2ee1890`](https://github.com/Byron/gitoxide/commit/2ee189068edcc06491e03c8551866ce5ac0cf0ba))
    - Add `easy::Tree::traverse()` platform ([`667485e`](https://github.com/Byron/gitoxide/commit/667485e133ca29fcc6914a7142cf953564b5fce3))
    - (change!: consistently use `object_hash` instead of `hash_kind` #279) ([`81bd453`](https://github.com/Byron/gitoxide/commit/81bd4531c8ab752eaadb201a18d7c26fdf83f893))
    - Release git-chunk v0.2.0, safety bump 4 crates ([`b792fab`](https://github.com/Byron/gitoxide/commit/b792fabf9f5f93ab906ac5a5bb3e4f01c179290a))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - Tests for Commit object ([`1130928`](https://github.com/Byron/gitoxide/commit/1130928b8b450387daff1b79faff4ffd012c1dba))
    - Add `easy::Commit` object ([`8f650c0`](https://github.com/Byron/gitoxide/commit/8f650c089c88698483f778aa5c0070f606b94f09))
    - Episode 5 ([`8ba7fc8`](https://github.com/Byron/gitoxide/commit/8ba7fc894689b2b163f06b8686dda4563c3c0838))
    - episode 4 ([`e7e54a2`](https://github.com/Byron/gitoxide/commit/e7e54a22fbc06c5e54216abc426d70d7bff0ac26))
    - episode 3 ([`e107d9a`](https://github.com/Byron/gitoxide/commit/e107d9ab9f150fec41fbcb008950df5050f9fe34))
    - make fmt ([`066f3ff`](https://github.com/Byron/gitoxide/commit/066f3ffb8740f242c1b03e680c3c5c1a0e4c36c3))
</details>

## 0.13.0 (2021-11-29)

<csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/>
<csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/>

With changes to `git-ref`, what follows is all the adjustments made to simplify the `git-repository` implementation.

### Changed (BREAKING)

 - <csr-id-5d498a33236391d8e456f267b1bf6af24de66f11/> file::Store::iter() is now a platform, with `.all()` and `.prefixed(…)` respectively
   This way, it's possible to keep shared ownership of the packed buffer
   while allowing the exact same iterator machinery to work as before.
 - <csr-id-15d429bb50602363292453606902bdce5042d9a5/> file::Store::(try_)find(…, packed) was removed
   The packed buffer is now handled internally while loading it on demand.
   When compiled with `git-features/parallel` the `file::Store` remains
   send and sync.
   
   The packed refs buffer is shared across clones and it's recommended
   to clone one `file::Store` instance per thread, each of which can
   use its own namespace.
 - <csr-id-95247322a8191edfa7fac9c5aa72b40239f3aa88/> move `git_ref::file::WriteRefLog` to `git_ref::store::WriteRefLog`

### Bug Fixes (BREAKING)

 - <csr-id-fc8e85cd71d4f16bc8daad0b790d875045faefff/> ref namespaces are now thread-local
   Previously these were shared in the shared Repo instance, which makes
   threaded applications impossible to remain deterministic across multiple
   connections.
   
   Now they are local to the thread, which allowed some methods to remove
   their Result<> as they cannot fail anymore, the reason for this being
   a breaking change.

### Other (BREAKING)

 - <csr-id-951c050ecbb70c9de216603e55c7cfbc89a067e3/> Reference::logs() -> Reference::log_iter()
   The latter now returns a standard Platform to iterate over all
   reflog entries from oldest to newest or vice versa.

### Refactor (BREAKING)

 - <csr-id-0e1875363fea09452789d7a90fc6860a7996d6d3/> `file::Store::base` is now `file::Store::base()` and read-only
   That way, file databases can't be repositioned anymore, it's recommended
   to recreate it if that's desired.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 11 calendar days.
 - 12 days passed between releases.
 - 6 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#263](https://github.com/Byron/gitoxide/issues/263)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#263](https://github.com/Byron/gitoxide/issues/263)**
    - Adjust to changes in git-ref ([`1e32855`](https://github.com/Byron/gitoxide/commit/1e3285572a640683660936297e5f072d827b3ded))
    - Reference::logs() -> Reference::log_iter() ([`951c050`](https://github.com/Byron/gitoxide/commit/951c050ecbb70c9de216603e55c7cfbc89a067e3))
    - Adapt to new iteration Platform in git-ref ([`b5a749e`](https://github.com/Byron/gitoxide/commit/b5a749e1e26e4490d94f1cbd6901596d90f3cf47))
    - file::Store::iter() is now a platform, with `.all()` and `.prefixed(…)` respectively ([`5d498a3`](https://github.com/Byron/gitoxide/commit/5d498a33236391d8e456f267b1bf6af24de66f11))
    - Adjustments to match new signature of peel_to_ids_in_place ([`f87a11f`](https://github.com/Byron/gitoxide/commit/f87a11fef632dd2442393c4655ada11f1d480332))
    - file::Store::(try_)find(…, packed) was removed ([`15d429b`](https://github.com/Byron/gitoxide/commit/15d429bb50602363292453606902bdce5042d9a5))
    - `file::Store::base` is now `file::Store::base()` and read-only ([`0e18753`](https://github.com/Byron/gitoxide/commit/0e1875363fea09452789d7a90fc6860a7996d6d3))
    - move `git_ref::file::WriteRefLog` to `git_ref::store::WriteRefLog` ([`9524732`](https://github.com/Byron/gitoxide/commit/95247322a8191edfa7fac9c5aa72b40239f3aa88))
    - ref namespaces are now thread-local ([`fc8e85c`](https://github.com/Byron/gitoxide/commit/fc8e85cd71d4f16bc8daad0b790d875045faefff))
    - Add cheap and sync loose ref DB directly to state ([`38c8146`](https://github.com/Byron/gitoxide/commit/38c81462b94d225861fd237bd0c2ce0c558664c4))
 * **Uncategorized**
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
    - thanks clippy ([`a74f27c`](https://github.com/Byron/gitoxide/commit/a74f27c042bdf0c1e30a1767b56032e32cbc81a9))
    - Merge branch 'git-loose-objects' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-git-loose-objects ([`ee737cd`](https://github.com/Byron/gitoxide/commit/ee737cd237ad70bf9f2c5e0d3e4557909e495bca))
</details>

## 0.12.0 (2021-11-16)

### New Features

 - <csr-id-b7aab9efd42975e8f2dcb5c97e51495996175702/> Allow `PartialNameRef` to be created from owned items

### Changed (BREAKING)

 - <csr-id-e8b091943f0c9a26317da0003f7fcdf5a56ef21a/> Rename gix->ein and gixp->gix

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 27 days passed between releases.
 - 2 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 5 unique issues were worked on: [#241](https://github.com/Byron/gitoxide/issues/241), [#247](https://github.com/Byron/gitoxide/issues/247), [#251](https://github.com/Byron/gitoxide/issues/251), [#254](https://github.com/Byron/gitoxide/issues/254), [#259](https://github.com/Byron/gitoxide/issues/259)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#241](https://github.com/Byron/gitoxide/issues/241)**
    - refactor ([`8cd5f6a`](https://github.com/Byron/gitoxide/commit/8cd5f6ad66781b83d69490754fe6e8b87974b125))
    - Improve usability of the pack-cache environment variable ([`47d8162`](https://github.com/Byron/gitoxide/commit/47d81629a0bfa2eccf75cbe081de55d80d0abd59))
 * **[#247](https://github.com/Byron/gitoxide/issues/247)**
    - Rename gix->ein and gixp->gix ([`e8b0919`](https://github.com/Byron/gitoxide/commit/e8b091943f0c9a26317da0003f7fcdf5a56ef21a))
 * **[#251](https://github.com/Byron/gitoxide/issues/251)**
    - refactor ([`244a646`](https://github.com/Byron/gitoxide/commit/244a646370dcc4e35478825922b86fe59646d86c))
    - Another example that probably is closer to the optimal case ([`a216d89`](https://github.com/Byron/gitoxide/commit/a216d89b8ef51a47aa9b19cc0296fbbe984b1066))
    - Allow `PartialNameRef` to be created from owned items ([`b7aab9e`](https://github.com/Byron/gitoxide/commit/b7aab9efd42975e8f2dcb5c97e51495996175702))
    - Add full-name workaround as example ([`06893cf`](https://github.com/Byron/gitoxide/commit/06893cf49f98b0da4878c8d808544b6ec309f24e))
    - add tests to verify common inputs would work for try_find_reference(…) ([`d986d09`](https://github.com/Byron/gitoxide/commit/d986d09cd9a4cb8e3a1444d781237f4a0ce550a1))
 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **[#259](https://github.com/Byron/gitoxide/issues/259)**
    - Describe and propose fix for ref namespace-sharing issue ([`55773b8`](https://github.com/Byron/gitoxide/commit/55773b87feb246927a7421a822c45d9e101e985e))
 * **Uncategorized**
    - Release git-repository v0.12.0, cargo-smart-release v0.6.0 ([`831a777`](https://github.com/Byron/gitoxide/commit/831a777487452a6f51a7bc0a9f9ca34b0fd778ed))
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Merge branch 'header-field-multi-improve' of https://github.com/xmo-odoo/gitoxide into xmo-odoo-header-field-multi-improve ([`d88e377`](https://github.com/Byron/gitoxide/commit/d88e377c21e566bf86c274d5e87eff06100698b9))
</details>

## v0.11.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#221](https://github.com/Byron/gitoxide/issues/221), [#222](https://github.com/Byron/gitoxide/issues/222)

## v0.10.0 (2021-10-15)

<csr-id-1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7/>
<csr-id-2f2d856efe733d3cf81110c0e0607d2e7c40d968/>
<csr-id-a19567eceab0dd7f5478b83c2ff9ce79754db308/>
<csr-id-61793ff42f5c2f9ddf302901adea2dac6149eac8/>
<csr-id-0cd585e20a5abd323a34ec32d92fbd48531b3b18/>
<csr-id-89f15051763a03627f332c46beedfc53b8b9b15b/>
<csr-id-f644d0ede7a2e8d344a81c7003c3877eed64a6b0/>
<csr-id-ac3b9efb7b90958274ce55800959d930f8641115/>
<csr-id-03fe8a7ebd34608d725d4585da5c1630123762ec/>
<csr-id-8fe461281842b58aa11437445637c6e587bedd63/>
<csr-id-b209da29f361512ba757febf56bc1aca039f2a41/>
<csr-id-741558dd8194590c5cc8566aa22f96e73df38edf/>
<csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/>
<csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/>
<csr-id-1f4e45a26a3d2727f00c3f248452dd41fc8a95be/>
<csr-id-1958e8aa65eb97f9755f065d713f0a48c5e41b1b/>
<csr-id-066f59b23a125b1ce9a015437a3f4468e5791da0/>
<csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/>
<csr-id-1a1959f487d69ffdd5394775b707139c44dbd11d/>
<csr-id-5e091fb2b4fd33879c176e6dadd3c9805d99af50/>
<csr-id-e3760679547e0dc1bf31761acdb6e63b04a50919/>
<csr-id-de004b318fdc6923711dd001bff5f4bcbba4270e/>
<csr-id-41afad3386461b658ee859225785b6de86d13cfb/>
<csr-id-f582439a3efe5c234f54c488792395e9de09a032/>
<csr-id-42080aefe3b286afb58235c1c22491579ab73919/>
<csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/>
<csr-id-e7c061b10c263001eb4abf03098d6694b770f828/>
<csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/>
<csr-id-5aadf75a0d93d1a990ad0305c38366c5c22bdcb2/>
<csr-id-d79a1b75304e397c16b5af7055906591a187ddfd/>
<csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/>
<csr-id-06996e032b1e451a674395ebaca94434fac46f05/>
<csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/>
<csr-id-4fe4786797d240a59d29dbf2c6310490a381c8b6/>
<csr-id-debe0094826f83839f907523715def929133fd58/>
<csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/>

### New Features

 - <csr-id-11b64fce4630371633b6415f227eecdc6b42b20b/> Make `git_url::Url` available under `git_repository::Url`
 - <csr-id-80b8331092f4856f52afa1d85fa375ae688bdd28/> add easy::ext::ObjectAccessExt::tag(…) to create tag objects
   It's a quick sketch on how tag object creation could work.
   
   Note the duplication the method name using traits, which seems like a good solution
   to the problem of differentiating tag objects and tag references while
   keeping the method name short.
   
   Most will only ever need one, right?
   
   Even in my example that's not the case, so maybe we have to rename it.
 - <csr-id-0ebfeb614264ca06ab763189e55e6c016c9997af/> Make `git_url::Url` available under `git_repository::Url`

### BREAKING Changes

 - Use 'to_*' when converting `easy::Object` to specific object kind
   This also makes the API more consistent while being more idiomatic.
 - Avoid duplicate module paths in 'tree' and 'commit'
 - rename ObjectIdExt::ancestors_iter() to *::ancestors()
 - rename `easy::Object::to_(commit|tag)_iter()`…
   …to  `easy::Object::try_to_(commit|tag)_iter()` for consistency.
 - rename `*::State` into `*::Platform`
 - various small API changes
 - move easy::head::peel::Error -> easy::head::peel::to_id::Error
 - rename path::is_git to path::is
 - rename easy::reference::log::State to easy::reference::Logs

### Refactor

 - <csr-id-8fe461281842b58aa11437445637c6e587bedd63/> split data::output::count::objects into files
 - <csr-id-b209da29f361512ba757febf56bc1aca039f2a41/> use new git_pack::cache::Object trait
 - <csr-id-741558dd8194590c5cc8566aa22f96e73df38edf/> remove object cache impl which now lives in git-pack

### Other

 - <csr-id-e16603b15b5488b81563c583cd8f5292ab9d24a2/> :remote_url() is now optional
   Otherwise it wouldn't work on repos that don't have a remote set yet.
   Instead of failing, we don't create links.
 - <csr-id-54a64a588ff72515451a3d0343306ac4abe1cb35/> try to create persistent Easy iterator, but can't make it Send…
   …which is fair as it contains borrowed RefCells, which really would have
   to be owned to work for this, which would in turn require the Ancestor's
   struct to be kind of self-referential
 - <csr-id-1f4e45a26a3d2727f00c3f248452dd41fc8a95be/> path::is
 - <csr-id-1958e8aa65eb97f9755f065d713f0a48c5e41b1b/> path::discover
 - <csr-id-066f59b23a125b1ce9a015437a3f4468e5791da0/> top-level of 'path' module
 - <csr-id-329d183ad4e256a4f9cdeb34589b5f3432495f79/> object_id
 - <csr-id-1a1959f487d69ffdd5394775b707139c44dbd11d/> repository
 - <csr-id-5e091fb2b4fd33879c176e6dadd3c9805d99af50/> ext::tree
 - <csr-id-e3760679547e0dc1bf31761acdb6e63b04a50919/> easy::object::peel
 - <csr-id-de004b318fdc6923711dd001bff5f4bcbba4270e/> easy::object::errors
 - <csr-id-41afad3386461b658ee859225785b6de86d13cfb/> a seemingly slow version of path lookup, but…
   …in debug mode it's faster than the fast path, despite doing more
   and being the same when it comes to searching path components.
 - <csr-id-f582439a3efe5c234f54c488792395e9de09a032/> easy::object, sans a few child-modules
 - <csr-id-42080aefe3b286afb58235c1c22491579ab73919/> update 'platform' information to reflect the current usage
 - <csr-id-d422b9a31a37a03551bec4382039aaf3a7e49902/> configure caches with env vars using `apply_environment()`
 - <csr-id-e7c061b10c263001eb4abf03098d6694b770f828/> refactor
 - <csr-id-66292fd1076c2c9db4694c5ded09799a0be11a03/> set package cache via RepositoryAccessExt
 - <csr-id-5aadf75a0d93d1a990ad0305c38366c5c22bdcb2/> Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size…
   …which can mean another considerable speed-up for many workloads, but
   usually needs some knowledge about the application, repos, and should
   thus be with the user.
 - <csr-id-d79a1b75304e397c16b5af7055906591a187ddfd/> allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE
 - <csr-id-7d2b6b66e09ff39727fccd68d190679b52d90126/> prepare for configurable pack cache
 - <csr-id-06996e032b1e451a674395ebaca94434fac46f05/> object-cache to allow for a speed boost…
   …by avoiding duplicate accesses to hit the object database.
   However, the cost for the cache are relatively high and involve some
   memory copying, so hit rates of about 50% is certainly what is needed
   to get any speed boost at all.
 - <csr-id-daec7167df524b329daad7dabb1b9920b6ef8936/> build commit history for later use in changelog generation
 - <csr-id-4fe4786797d240a59d29dbf2c6310490a381c8b6/> Allow object access during commit ancestor traversal…
   …by getting only a temporary handle to the pack-cache. The cost of this
   should be neglible compared to the cost of object decoding.
 - <csr-id-debe0094826f83839f907523715def929133fd58/> sketch history acquisition
 - <csr-id-56e39fac54bfa3871c42bbf76a9f7c49486b85be/> add 'Head::peeled()' method

### Changed (BREAKING)

 - <csr-id-c3385cd144298eb9f06d7751d180e26da7b4d338/> `easy::Object::try_to_commit()` now returns `Result<CommitRef>`…
   …without the nested `Option`, folding the type mismatch into a specific
   `conversion::Error` instad.
 - <csr-id-e59f901f47fb0180211494a1591aed62b856406a/> rename `ObjectAccessExt::tag(…)` to `*::tag_reference(…)`, add `easy::Object::try_to_tag()`
   This one also contains the first and probably only test for tag object
   creation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 89 commits contributed to the release over the course of 29 calendar days.
 - 34 days passed between releases.
 - 40 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#164](https://github.com/Byron/gitoxide/issues/164), [#198](https://github.com/Byron/gitoxide/issues/198), [#200](https://github.com/Byron/gitoxide/issues/200), [#67](https://github.com/Byron/gitoxide/issues/67)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 3 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#164](https://github.com/Byron/gitoxide/issues/164)**
    - path::is ([`1f4e45a`](https://github.com/Byron/gitoxide/commit/1f4e45a26a3d2727f00c3f248452dd41fc8a95be))
    - rename path::is_git to path::is ([`ac3b9ef`](https://github.com/Byron/gitoxide/commit/ac3b9efb7b90958274ce55800959d930f8641115))
    - path::discover ([`1958e8a`](https://github.com/Byron/gitoxide/commit/1958e8aa65eb97f9755f065d713f0a48c5e41b1b))
    - Avoid duplicate module paths in 'tree' and 'commit' ([`2f2d856`](https://github.com/Byron/gitoxide/commit/2f2d856efe733d3cf81110c0e0607d2e7c40d968))
    - top-level of 'path' module ([`066f59b`](https://github.com/Byron/gitoxide/commit/066f59b23a125b1ce9a015437a3f4468e5791da0))
    - object_id ([`329d183`](https://github.com/Byron/gitoxide/commit/329d183ad4e256a4f9cdeb34589b5f3432495f79))
    - rename ObjectIdExt::ancestors_iter() to *::ancestors() ([`a19567e`](https://github.com/Byron/gitoxide/commit/a19567eceab0dd7f5478b83c2ff9ce79754db308))
    - repository ([`1a1959f`](https://github.com/Byron/gitoxide/commit/1a1959f487d69ffdd5394775b707139c44dbd11d))
    - ext::tree ([`5e091fb`](https://github.com/Byron/gitoxide/commit/5e091fb2b4fd33879c176e6dadd3c9805d99af50))
    - easy::object::peel ([`e376067`](https://github.com/Byron/gitoxide/commit/e3760679547e0dc1bf31761acdb6e63b04a50919))
    - easy::object::errors ([`de004b3`](https://github.com/Byron/gitoxide/commit/de004b318fdc6923711dd001bff5f4bcbba4270e))
    - rename `easy::Object::to_(commit|tag)_iter()`… ([`61793ff`](https://github.com/Byron/gitoxide/commit/61793ff42f5c2f9ddf302901adea2dac6149eac8))
    - easy::object, sans a few child-modules ([`f582439`](https://github.com/Byron/gitoxide/commit/f582439a3efe5c234f54c488792395e9de09a032))
    - update 'platform' information to reflect the current usage ([`42080ae`](https://github.com/Byron/gitoxide/commit/42080aefe3b286afb58235c1c22491579ab73919))
    - rename easy::reference::log::State to easy::reference::Logs ([`03fe8a7`](https://github.com/Byron/gitoxide/commit/03fe8a7ebd34608d725d4585da5c1630123762ec))
    - rename `*::State` into `*::Platform` ([`0cd585e`](https://github.com/Byron/gitoxide/commit/0cd585e20a5abd323a34ec32d92fbd48531b3b18))
 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - :remote_url() is now optional ([`e16603b`](https://github.com/Byron/gitoxide/commit/e16603b15b5488b81563c583cd8f5292ab9d24a2))
    - `easy::Object::try_to_commit()` now returns `Result<CommitRef>`… ([`c3385cd`](https://github.com/Byron/gitoxide/commit/c3385cd144298eb9f06d7751d180e26da7b4d338))
    - rename `ObjectAccessExt::tag(…)` to `*::tag_reference(…)`, add `easy::Object::try_to_tag()` ([`e59f901`](https://github.com/Byron/gitoxide/commit/e59f901f47fb0180211494a1591aed62b856406a))
    - add easy::ext::ObjectAccessExt::tag(…) to create tag objects ([`80b8331`](https://github.com/Byron/gitoxide/commit/80b8331092f4856f52afa1d85fa375ae688bdd28))
    - prettify changelog also to practice user segments ([`5c57264`](https://github.com/Byron/gitoxide/commit/5c5726409d6f6343fc9a860f4b9ecde4730fe7d9))
    - Fix git-url re-export to respect feature flags ([`ec4e3ca`](https://github.com/Byron/gitoxide/commit/ec4e3ca4c7211655549a76cae252742633da1083))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - pass actual repository url down from commands ([`4e03515`](https://github.com/Byron/gitoxide/commit/4e03515622afd79b145db081ef9e3cb301ce6e97))
    - Make `git_url::Url` available under `git_repository::Url` ([`0ebfeb6`](https://github.com/Byron/gitoxide/commit/0ebfeb614264ca06ab763189e55e6c016c9997af))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - Allow 'refactor' and 'other' in conventional messages if they have breaking changes ([`4eebaac`](https://github.com/Byron/gitoxide/commit/4eebaac669e590beed112b622752997c64772ef1))
    - merge doesn't consider user generated sections, only the ones it would want to add ([`ebbebdd`](https://github.com/Byron/gitoxide/commit/ebbebdd70aeec9aa3ad453d61375429a7f555bbc))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Use hashmap based lookup for trees… ([`48a0c76`](https://github.com/Byron/gitoxide/commit/48a0c76ab163b6e35b19dd2a9efc2e101a721633))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Fix panic related to incorrect handling of character boundaries ([`9e92cff`](https://github.com/Byron/gitoxide/commit/9e92cff33f4f53d3b2d6b55a722d577c2dd6a4f2))
    - Use 'to_*' when converting `easy::Object` to specific object kind ([`1cb41f8`](https://github.com/Byron/gitoxide/commit/1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7))
    - Fix build ([`d0a956f`](https://github.com/Byron/gitoxide/commit/d0a956fdb5a822dbd116792bfbe70d1532a95ec9))
    - refactor!: Use git_object::commit::MessageRef::summary()… ([`13e7c3a`](https://github.com/Byron/gitoxide/commit/13e7c3ad5e079fe778d07d115c9e41c4c6eb038f))
    - Sketch data for parsed messages ([`32dd280`](https://github.com/Byron/gitoxide/commit/32dd280eaada635994e11b4f2722a4efc59faa8f))
    - a seemingly slow version of path lookup, but… ([`41afad3`](https://github.com/Byron/gitoxide/commit/41afad3386461b658ee859225785b6de86d13cfb))
    - configure caches with env vars using `apply_environment()` ([`d422b9a`](https://github.com/Byron/gitoxide/commit/d422b9a31a37a03551bec4382039aaf3a7e49902))
    - refactor ([`e7c061b`](https://github.com/Byron/gitoxide/commit/e7c061b10c263001eb4abf03098d6694b770f828))
    - set package cache via RepositoryAccessExt ([`66292fd`](https://github.com/Byron/gitoxide/commit/66292fd1076c2c9db4694c5ded09799a0be11a03))
    - Add GITOXIDE_PACK_CACHE_MEMORY_IN_BYTES=536870912 to control pack-cache size… ([`5aadf75`](https://github.com/Byron/gitoxide/commit/5aadf75a0d93d1a990ad0305c38366c5c22bdcb2))
    - allow disabling the pack cache with GITOXIDE_DISABLE_PACK_CACHE ([`d79a1b7`](https://github.com/Byron/gitoxide/commit/d79a1b75304e397c16b5af7055906591a187ddfd))
    - prepare for configurable pack cache ([`7d2b6b6`](https://github.com/Byron/gitoxide/commit/7d2b6b66e09ff39727fccd68d190679b52d90126))
    - object-cache to allow for a speed boost… ([`06996e0`](https://github.com/Byron/gitoxide/commit/06996e032b1e451a674395ebaca94434fac46f05))
    - build commit history for later use in changelog generation ([`daec716`](https://github.com/Byron/gitoxide/commit/daec7167df524b329daad7dabb1b9920b6ef8936))
    - Allow object access during commit ancestor traversal… ([`4fe4786`](https://github.com/Byron/gitoxide/commit/4fe4786797d240a59d29dbf2c6310490a381c8b6))
    - sketch history acquisition ([`debe009`](https://github.com/Byron/gitoxide/commit/debe0094826f83839f907523715def929133fd58))
    - various small API changes ([`89f1505`](https://github.com/Byron/gitoxide/commit/89f15051763a03627f332c46beedfc53b8b9b15b))
    - add 'Head::peeled()' method ([`56e39fa`](https://github.com/Byron/gitoxide/commit/56e39fac54bfa3871c42bbf76a9f7c49486b85be))
    - move easy::head::peel::Error -> easy::head::peel::to_id::Error ([`f644d0e`](https://github.com/Byron/gitoxide/commit/f644d0ede7a2e8d344a81c7003c3877eed64a6b0))
 * **[#200](https://github.com/Byron/gitoxide/issues/200)**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… ([`f293b63`](https://github.com/Byron/gitoxide/commit/f293b633d16c0f7393d0ede64e12f14e47d0296b))
 * **[#67](https://github.com/Byron/gitoxide/issues/67)**
    - split data::output::count::objects into files ([`8fe4612`](https://github.com/Byron/gitoxide/commit/8fe461281842b58aa11437445637c6e587bedd63))
    - use new git_pack::cache::Object trait ([`b209da2`](https://github.com/Byron/gitoxide/commit/b209da29f361512ba757febf56bc1aca039f2a41))
    - remove object cache impl which now lives in git-pack ([`741558d`](https://github.com/Byron/gitoxide/commit/741558dd8194590c5cc8566aa22f96e73df38edf))
    - Use Easy in the one spot where it is possible… ([`6a97bfa`](https://github.com/Byron/gitoxide/commit/6a97bfabcec6597efe9282e6d5c9f0ac3ada61dc))
    - try to create persistent Easy iterator, but can't make it Send… ([`54a64a5`](https://github.com/Byron/gitoxide/commit/54a64a588ff72515451a3d0343306ac4abe1cb35))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - thanks clippy ([`bcc9871`](https://github.com/Byron/gitoxide/commit/bcc98715b1bfd9613079071da59309aa8a5ab27b))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - make fmt, but now it picked up some parts that usually don't get altered… ([`01f7b72`](https://github.com/Byron/gitoxide/commit/01f7b729337bd2c99498321c479a9a13b1858e3e))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com/Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
    - Bump git-repository v0.10.0 ([`5a10dde`](https://github.com/Byron/gitoxide/commit/5a10dde1bcbc03157f3ba45104638a8b5b296cb9))
    - [repository #164] docs for easy::reference::log ([`7de7c7e`](https://github.com/Byron/gitoxide/commit/7de7c7eb51b7d709fd140dbf789e31e97161bfa7))
    - thanks clippy ([`ae7826e`](https://github.com/Byron/gitoxide/commit/ae7826e1cf79fce6ad12f407162f58b3bfb02b16))
    - [repository #164] docs for easy::reference::iter ([`d86c713`](https://github.com/Byron/gitoxide/commit/d86c71363a5a73dd8986566a9687e2b4756972cb))
    - [repository #164] refactor ([`437e63b`](https://github.com/Byron/gitoxide/commit/437e63b4e841ef478c12a91bf3e2dce63d5b1041))
    - [repository #164] docs for top-level of easy::reference ([`9e465e0`](https://github.com/Byron/gitoxide/commit/9e465e03dc636c360128c93864749c4a3f8a99e5))
    - [repository #164] docs for easy::oid ([`b66b6fe`](https://github.com/Byron/gitoxide/commit/b66b6fe759eeb55cb875fcb65aa58b62c6963ca8))
    - [repository #164] docs for easy::commit and easy::odb ([`abf37e5`](https://github.com/Byron/gitoxide/commit/abf37e54e5a4584f521988e27dd02f6d6badc4ef))
    - thanks clippy ([`b02edb5`](https://github.com/Byron/gitoxide/commit/b02edb5b1e9b7c8f8bd1b4a8e2d60667da629839))
    - [repository #164] Documentation for `easy::borrow` ([`3e612f4`](https://github.com/Byron/gitoxide/commit/3e612f441e1e837d7ba3d3ddd40b4a8c2ba05c61))
    - [repository #164] docs for easy::head::* ([`516fde7`](https://github.com/Byron/gitoxide/commit/516fde7ffb505603479b4de2a78200da480b66ed))
    - [repository #164] refactor ([`65b0e0f`](https://github.com/Byron/gitoxide/commit/65b0e0fbe7ab7cb405fd267802e7ad3de36d98f7))
    - [repository #164] docs for `easy::ext::ReferenceAccessExt` ([`ab4910f`](https://github.com/Byron/gitoxide/commit/ab4910f1b4bf98569a04596b43aba862caca029b))
    - [repository #164] docs for easy::ext::RepositoryAccessExt ([`9041d47`](https://github.com/Byron/gitoxide/commit/9041d474f178f45c86d628a7140c64810365b97d))
    - [repository #164] another test and fix for `commit()` ([`8d676d7`](https://github.com/Byron/gitoxide/commit/8d676d77cb69df203d3fcbf8c1a34f212035605f))
    - [repository #164] easy::ext::ObjectAccessExt docs ([`c4984af`](https://github.com/Byron/gitoxide/commit/c4984af4f6343a17290f6c85f8385e77354875bb))
    - [repository #164] ([`4111d22`](https://github.com/Byron/gitoxide/commit/4111d22ebe4cc9ddd726cce566e5872708067440))
</details>

## v0.9.1 (2021-09-10)

<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-650241251a420602f74037babfc24c9f64df78d8/>
<csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/>
<csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/>

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### Other

 - <csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/> loose reference iteration with non-dir prefixes…
   Previously it was expected for the prefix `Path` to always exist for
   the prefix to be valid. This, however, is not similar to packed
   prefixes, which allow non-dir prefixes as well.
   
   Now we will check if the prefix is actually a directory, and if not
   split it into its parent directory and the filename portion. The latter
   is then used for prefix matching file names within that directory.
 - <csr-id-650241251a420602f74037babfc24c9f64df78d8/> Add 'references().all().peeled().'…
   …to not only make typical usage of iterated references more convenient
   but also work around a double-borrow error one would see otherwise.
 - <csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/> filter refs correctly, but…
   …it needs a way to peel references right away without trying
   to double-borrow. This means the Iterator needs to implement this.
 - <csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/> improved changelog…
   …akin to 'Keep a changelog'.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 1 day passed between releases.
 - 4 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#198](https://github.com/Byron/gitoxide/issues/198)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - loose reference iteration with non-dir prefixes… ([`293bfc0`](https://github.com/Byron/gitoxide/commit/293bfc0278c5983c0beaec93253fb51f00d81156))
    - Add 'references().all().peeled().'… ([`6502412`](https://github.com/Byron/gitoxide/commit/650241251a420602f74037babfc24c9f64df78d8))
    - filter refs correctly, but… ([`2b4a615`](https://github.com/Byron/gitoxide/commit/2b4a61589a7cba3f7600710e21304e731ae3b36a))
 * **Uncategorized**
    - Release git-repository v0.9.1 ([`262c122`](https://github.com/Byron/gitoxide/commit/262c1229d6d2d55c70fe0e199ab15d10954d967b))
    - Release git-ref v0.7.3 ([`b0a9815`](https://github.com/Byron/gitoxide/commit/b0a98157ab3b240af027acb9965c981a543e55fa))
    - [repository] don't enforce feature flags that may fail on windows by default ([`afdec2e`](https://github.com/Byron/gitoxide/commit/afdec2e89eee0397b16602fdff16d3997ef370d0))
    - thanks clippy ([`68ea77d`](https://github.com/Byron/gitoxide/commit/68ea77dcdd5eb8033618e7af2e3eb0989007b89b))
    - Release git-ref v0.7.2 ([`e940e9a`](https://github.com/Byron/gitoxide/commit/e940e9a21938035eb8791bba19cc16814a0fb4e7))
    - Release git-protocol v0.10.4 ([`898ee08`](https://github.com/Byron/gitoxide/commit/898ee08befa1eb7dd22980063c7633f83d0a8958))
    - Release git-odb v0.21.3 ([`223f930`](https://github.com/Byron/gitoxide/commit/223f93075a28dd49f44505c039cfeae5a7296914))
    - improved changelog… ([`8b82f7d`](https://github.com/Byron/gitoxide/commit/8b82f7d44c7eb63b7922ddc31ada9cefdce776b0))
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
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-pack v0.11.0 ([`5ae6ff5`](https://github.com/Byron/gitoxide/commit/5ae6ff52cd2cd1ccd1e26bb987c154eb19603696))
    - Bump git-repository v0.9.0 ([`b797fc1`](https://github.com/Byron/gitoxide/commit/b797fc10f3f3d1fbc23916a4ff6e5e860e2dd4ed))
    - [repository #193] Add feature flags for async/blocking ([`57f482c`](https://github.com/Byron/gitoxide/commit/57f482c59ac47b7a5f1abf01b4a3e25364e061c2))
    - Bump git-object v0.14.0 ([`d4fc81f`](https://github.com/Byron/gitoxide/commit/d4fc81f6390443f8c8561d91ac27ea4a6318fb62))
    - [repository #164] Prepare `commit()` for a possible less-allocating future ([`0fd01f7`](https://github.com/Byron/gitoxide/commit/0fd01f7071c785c27c56d2c034aac8dcdf690677))
    - [repository #164] Support for refreshing the object database ([`46e10f8`](https://github.com/Byron/gitoxide/commit/46e10f863e1fea419483a7b086022c16cd0ca226))
    - [odb #164] Add refresh() functionality ([`ee16d04`](https://github.com/Byron/gitoxide/commit/ee16d041941a5777c8f6495a28f7633c327cbd6b))
</details>

## v0.8.2 (2021-09-07)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 23 commits contributed to the release.
 - 9 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.2 ([`3fc23be`](https://github.com/Byron/gitoxide/commit/3fc23beaf103c037253ace727c87ec457be5dedd))
    - [repository #190] test for oid.ancestors().all() ([`fdc3678`](https://github.com/Byron/gitoxide/commit/fdc3678c63fa128ac754b3fa9ae3d88a4a221d0d))
    - [repository #190] fix build, lets just make traversal available by default ([`6da3599`](https://github.com/Byron/gitoxide/commit/6da35994cf2a3c9ab741733af53761c9a2cebeed))
    - Bump git-pack v0.10.0 ([`e5e3c80`](https://github.com/Byron/gitoxide/commit/e5e3c8024e1c2e5e90cee83abbdae41d58eee156))
    - [repository #190] access to repository directories ([`f4d1ec4`](https://github.com/Byron/gitoxide/commit/f4d1ec4ac0be8aa46d97eb92fb8a8f3fb8da94fb))
    - [repository #185] rustfmt ([`dfbb015`](https://github.com/Byron/gitoxide/commit/dfbb015a89db47c79015135870013ecc384c4aea))
    - [repository #190] first shot at ancestor iteration… ([`85f1a48`](https://github.com/Byron/gitoxide/commit/85f1a48ea39f3b224e8d0ba3728dd75e03a6edc3))
    - [repository #190] refactor ([`e7188e0`](https://github.com/Byron/gitoxide/commit/e7188e047529cb0f4b20b3876f36b4592e9d2dc4))
    - [ref #190] fix tests ([`e426e15`](https://github.com/Byron/gitoxide/commit/e426e15188d8ec38ee0029f1d080dbab9afd8642))
    - [repository #185] remove quick-error infavor of thiserror ([`212c44c`](https://github.com/Byron/gitoxide/commit/212c44c84b903681f6d35d934ee5f7ad6e1da791))
    - [repository #190] fix tests; needs inbound transaction handling… ([`e5a5c09`](https://github.com/Byron/gitoxide/commit/e5a5c09bb108741fff416672566e381f50f02b38))
    - [repository #185] on the way to removing quick-error ([`6ecd431`](https://github.com/Byron/gitoxide/commit/6ecd431661e7ddc2f97e5a78a7932d2a7f1f27f0))
    - [repository #185] support for initializing bare repositories ([`9e8a39e`](https://github.com/Byron/gitoxide/commit/9e8a39e3cbd620bd48f379743df0d5783c33a86f))
    - [repository #185] use git-config to handle bare repos more properly ([`8a5aac5`](https://github.com/Byron/gitoxide/commit/8a5aac55cf62bdd7287a363fa29f12aa39d4c583))
    - [repository #190] leverage git-ref namespace support ([`1aa9c11`](https://github.com/Byron/gitoxide/commit/1aa9c113488175f03758f8a64338a33b3417dd87))
    - [repository #185] sketch of how to open a repository… ([`48207b5`](https://github.com/Byron/gitoxide/commit/48207b54b97ac1b6354f6b53c13ccc4d1d8ea98f))
    - [repository #185] refactor ([`63089ff`](https://github.com/Byron/gitoxide/commit/63089ff356ea0f62963ae213ea0dbb09f891ada6))
    - [repository #185] refactor ([`7604935`](https://github.com/Byron/gitoxide/commit/7604935b12eacb26a98bedc5f77636b5583629a5))
    - [repository #185] refactor repository initialization… ([`5ff7eaa`](https://github.com/Byron/gitoxide/commit/5ff7eaa86bddfa94aec97355a5d6adb117045693))
    - [repository #190] refactor ([`609c249`](https://github.com/Byron/gitoxide/commit/609c249916ca64f4beecdab86eb4562adbd1ca4f))
    - [repository #190] fix build ([`f5e118c`](https://github.com/Byron/gitoxide/commit/f5e118c8871e45ed3db9da9cd6bc63a5ea99621e))
    - [repository #190] note a known limitation about finding references in namespaces… ([`d335731`](https://github.com/Byron/gitoxide/commit/d3357318cf100fc3e0751e5b6de3922b1c209ddb))
    - [repository #190] transparent namespace support ([`d14f073`](https://github.com/Byron/gitoxide/commit/d14f073707c2f4641a271ba7965ec8281638e8df))
</details>

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.1 ([`b269a12`](https://github.com/Byron/gitoxide/commit/b269a1264f830bafcfe74f0f3ce01448c894146e))
    - [repository #164] make EasyArcExclusive available ([`2fa3dcb`](https://github.com/Byron/gitoxide/commit/2fa3dcb40a34a7ec19382e5f6a71348ecf7a7c36))
    - [repository #190] turns out we need bstr with unicode support ([`3d8796e`](https://github.com/Byron/gitoxide/commit/3d8796e670f9bb5d2ed22fb3b75130a599737341))
</details>

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 159 commits contributed to the release over the course of 10 calendar days.
 - 10 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #190] public bstr re-export ([`3b7ffde`](https://github.com/Byron/gitoxide/commit/3b7ffde385b1984393ee65a7505ad7221fecd0dc))
    - [repository #174] keep assets ([`e0fca77`](https://github.com/Byron/gitoxide/commit/e0fca771f5ee068b0a9a0975930317d0883701cc))
    - [repository #190] cleanup usage of bstr… ([`e4411ff`](https://github.com/Byron/gitoxide/commit/e4411ff43b24af79fefeaa4411e004dc504a4e2a))
    - [repository #174] remove arc_lock code entirely ([`dcbe742`](https://github.com/Byron/gitoxide/commit/dcbe742eb5244f0b5c6563cf59962183b708f54f))
    - [repository #190] prefixed reference iteration ([`a6e19c9`](https://github.com/Byron/gitoxide/commit/a6e19c9a49bdc6a7c5cabef0a8d93bfd48a74fcd))
    - [repository #190] implementation of reference iteration (all() for now)… ([`2c0939a`](https://github.com/Byron/gitoxide/commit/2c0939a146b5973de26bd03987e075a34a84bc88))
    - [repository #174] conditionally compile future parking_lot version… ([`5375fc8`](https://github.com/Byron/gitoxide/commit/5375fc872b9af2526683326f58e9c3d7f20ef166))
    - [repository #190] refactor ([`8c532a4`](https://github.com/Byron/gitoxide/commit/8c532a4c78452dd11115cf36a906a27741858774))
    - [repository #190] prepare reference iteration ([`427f146`](https://github.com/Byron/gitoxide/commit/427f14622fb98e0397de2cae4d36a29f5915d375))
    - Bump git-hash v0.6.0 ([`6efd90d`](https://github.com/Byron/gitoxide/commit/6efd90db54f7f7441b76159dba3be80c15657a3d))
    - [repository #190] obtain the kind fo hash used in a repo ([`a985491`](https://github.com/Byron/gitoxide/commit/a985491bcea5f76942b863de8a9a89dd235dd0c9))
    - [repository #190] refactor ([`7a111b1`](https://github.com/Byron/gitoxide/commit/7a111b126cfb318acb2d09d119315150a38b7cd3))
    - [repository #190] shortcut to create references ([`28afd8e`](https://github.com/Byron/gitoxide/commit/28afd8e7cf09a17410c4a6ad57cddda608371364))
    - [ref #190] add forward log iter and localize iter types… ([`c3e240d`](https://github.com/Byron/gitoxide/commit/c3e240da47021226311681f3bcd48983f354243f))
    - [repository #190] refactor ([`e751688`](https://github.com/Byron/gitoxide/commit/e751688a5378552b73cfddd07f38a0d0bb491b83))
    - thanks clippy ([`023dedc`](https://github.com/Byron/gitoxide/commit/023dedc41aa859cd49d208392a586deaf77bd1bd))
    - [ref #190] reverse reflog ergonomics ([`2de86f9`](https://github.com/Byron/gitoxide/commit/2de86f904f6ee63e292f9c701cc3524e8bfe87e4))
    - [repository #190] ref log for HEAD specifically ([`946bbf1`](https://github.com/Byron/gitoxide/commit/946bbf19ed3f793b0eb1c5c90a655140e12d7e21))
    - [repository #190] reflog tests ([`641edde`](https://github.com/Byron/gitoxide/commit/641edde5608ff22bf18cea845ba1925b84a7b9f2))
    - [ref #190] First working sketch of reverse log iter access ([`4a36ded`](https://github.com/Byron/gitoxide/commit/4a36dedc17ce3124802d1b72330abc524fd98c6f))
    - Bump git-repository v0.8.0 ([`cdb45ff`](https://github.com/Byron/gitoxide/commit/cdb45ffa0810e9fcc9fd25bff7b696c2d27eeef5))
    - [ref #190] move remaining file store functions to extension trait ([`60fc215`](https://github.com/Byron/gitoxide/commit/60fc215ccac529b4a14cb9d8260ab9ddec86758a))
    - [repository #174] adjust various changelogs ([`081faf5`](https://github.com/Byron/gitoxide/commit/081faf5c3a21b34b7068b44d8206fb5770c392f5))
    - Bump git-protocol v0.10.0 ([`82d5a0b`](https://github.com/Byron/gitoxide/commit/82d5a0bb38903a8389e43cd5416e02e5496e661a))
    - thanks clippy ([`376c045`](https://github.com/Byron/gitoxide/commit/376c045cf589e51b639cf6c3633c4a8fcae7b6aa))
    - [repository #190] refactor ([`15d4ac8`](https://github.com/Byron/gitoxide/commit/15d4ac8f4b08716f6b06938f01396fb8ba8e7086))
    - [repository #190] a major step forward with `head()` access ([`43ac4f5`](https://github.com/Byron/gitoxide/commit/43ac4f5acbe3ace5d43ed3ed1bc394d721f0e273))
    - [ref #190] cache peeled objects properly ([`2cb511e`](https://github.com/Byron/gitoxide/commit/2cb511efe5833f860f3c17b8e5f5b4cd643baddb))
    - Bump git-odb v0.21.0 ([`7b9854f`](https://github.com/Byron/gitoxide/commit/7b9854fb35e86958a5ca827ec9a55b1168f38395))
    - Bump git-ref v0.7.0 ([`ac4413c`](https://github.com/Byron/gitoxide/commit/ac4413ce4e45703d5fe722e7220d039217f0bdef))
    - [repository #190] experiment with 'HEAD' API… ([`c55ce4d`](https://github.com/Byron/gitoxide/commit/c55ce4d8453c1ab4a107f5c6fb01521b422ee5c4))
    - thanks clippy ([`14dff63`](https://github.com/Byron/gitoxide/commit/14dff63fbc0d318bbc8a2618e0d72aaa98948acf))
    - [ref #190] Use Raw Reference everywhere for great simplification… ([`7aeea9c`](https://github.com/Byron/gitoxide/commit/7aeea9c36d4da04a806e68968356f8cc0dc11475))
    - [repository #190] refactor ([`d6bef3a`](https://github.com/Byron/gitoxide/commit/d6bef3afe7168659a75e26fb3ae2aa722fecf853))
    - [pack #179] refactor ([`ab6554b`](https://github.com/Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [ref #190] introduce Raw reference type that simplifies everything… ([`8634341`](https://github.com/Byron/gitoxide/commit/86343416dec8026f32c57d164dec4bf9b75b6536))
    - [packetline #178] fix compile warnings ([`c8d2e72`](https://github.com/Byron/gitoxide/commit/c8d2e72d272243da7d853f78463552bfc58ed9d6))
    - [ref #190] refactor ([`07126d6`](https://github.com/Byron/gitoxide/commit/07126d65946e981b339b6535986597cb328a1c9e))
    - [ref #190] Allow for explicit expected previous values ([`1a4786f`](https://github.com/Byron/gitoxide/commit/1a4786fb3bdb3d3a86b026dbf04e6baef6d3c695))
    - [repository #190] show that unconditional creation of references doesn't is lacking… ([`06b9270`](https://github.com/Byron/gitoxide/commit/06b9270e67823e9e911a9fa9d6eeeedcd93e62cb))
    - Bump git-traverse v0.8.0 ([`54f3541`](https://github.com/Byron/gitoxide/commit/54f3541f1448a8afa044d3958fa1be5b074e4445))
    - [repository #190] another commit() test… ([`4ec631c`](https://github.com/Byron/gitoxide/commit/4ec631c92349bbffa69c786838d2127b0c51970e))
    - Bump git-diff v0.9.0 ([`2e2e798`](https://github.com/Byron/gitoxide/commit/2e2e7983178b3af7e5684995de68ed5d020927ec))
    - [repository #190] produce nice reflog messages ([`e7a8b62`](https://github.com/Byron/gitoxide/commit/e7a8b62eb24f840f639aa436b4e79a4a567d3d05))
    - [repository #190] commit::summary() ([`43f7568`](https://github.com/Byron/gitoxide/commit/43f7568bd11fc310bac8350991ff3d4183dcd17b))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com/Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [repository #190] thanks clippy ([`0763ac2`](https://github.com/Byron/gitoxide/commit/0763ac260450b53b42f3c139deae5736fef056ce))
    - [repository #190] first version of 'commit(…)' without reflog message handling ([`bfcf8f1`](https://github.com/Byron/gitoxide/commit/bfcf8f17c7a89027e5bbcb5f85e3d0ba4036e8a0))
    - [object #177] fix docs ([`2fd23ed`](https://github.com/Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly ([`63bedc7`](https://github.com/Byron/gitoxide/commit/63bedc7647bb584353289e19972adf351765a526))
    - [repository #190] put git-lock into ST1… ([`26a6637`](https://github.com/Byron/gitoxide/commit/26a6637222081997ad7c08f4dc8d8facfb9cf94e))
    - [object #177] migrate immutable::commit into crate::commit ([`45d3934`](https://github.com/Byron/gitoxide/commit/45d393438eac2c7ecd47670922437dd0de4cd69b))
    - [repository #190] refactor ([`1e029b4`](https://github.com/Byron/gitoxide/commit/1e029b4beb6266853d5035c52b3d85bf98469556))
    - [repository #190] A way to write objects and the empty tree specifically ([`7c559d6`](https://github.com/Byron/gitoxide/commit/7c559d6e1b68bc89220bca426257f383bce586ae))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com/Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com/Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [repository #190] Make local-offset available on demand only… ([`1927be7`](https://github.com/Byron/gitoxide/commit/1927be7764f6af04ecc715dd52c631a3c8e16577))
    - [object #177] move mutable objects to crate::* ([`c551c02`](https://github.com/Byron/gitoxide/commit/c551c0236c64f3237cb9be7f35159f753d4b871f))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com/Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - [ref #175] follow (try_)find(_what) naming convention ([`679895c`](https://github.com/Byron/gitoxide/commit/679895cf866d643e768e353af614a55aeed2ba5c))
    - Merge pull request #172 from mellowagain/main ([`61aebbf`](https://github.com/Byron/gitoxide/commit/61aebbfff02eb87e0e8c49438a093a21b1134baf))
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com/Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [ref #175] make 'mutable' module private ([`a80dbcf`](https://github.com/Byron/gitoxide/commit/a80dbcf083bfcf2e291013f7b13bba9e787c5cb4))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com/Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - [ref #175] refactor ([`292e567`](https://github.com/Byron/gitoxide/commit/292e567eaa04a121fb4d7262bb316d37dd8ad11f))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com/Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [smart-release #171] it's about time we get some tests ([`48a489b`](https://github.com/Byron/gitoxide/commit/48a489b4247ed6feff222924bdcdb53ce45c6ce6))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com/Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
    - [stability #171] mark git-hash and git-actor as ST1 as well ([`32caae1`](https://github.com/Byron/gitoxide/commit/32caae1c32aae38bde59756e52848bef1cef049b))
    - [stability #171] git-ref is now ST1 and available through git-repository ([`50154cd`](https://github.com/Byron/gitoxide/commit/50154cd02fdd90930a1d7c5a4406d53c8067cb4b))
    - Release git-pack v0.9.0 ([`7fbc961`](https://github.com/Byron/gitoxide/commit/7fbc9617da97d4ba4bb3784f41d4163c0839c03c))
    - [smart-release #171] Try to avoid unstable git-repository features… ([`c8f325b`](https://github.com/Byron/gitoxide/commit/c8f325bed5d644eded035109702098f9fed3fba3))
    - Merge branch 'main' into stability ([`11bae43`](https://github.com/Byron/gitoxide/commit/11bae437e473fef6ed09c178d54ad11eee001b1d))
    - cleanup imports ([`e669303`](https://github.com/Byron/gitoxide/commit/e6693032f1391416fd704c21617051ddfb862a3a))
    - [stability #171] Don't provide access to less stable crates in `Respository` ([`e4c5b58`](https://github.com/Byron/gitoxide/commit/e4c5b58ad935c907dfbd0d61049453dcb64a7e19))
    - Merge branch 'main' into 162-repo-design-sketch ([`e63b634`](https://github.com/Byron/gitoxide/commit/e63b63412c02db469fbdb17da82cd1e9fda1ef0f))
    - [repository #164] top-level easy docs ([`6b71c51`](https://github.com/Byron/gitoxide/commit/6b71c51f703aa3b6a7d5a110d04294dd7ea4e8b0))
    - [repository #165] see if `git-config` can already be placed… ([`d287a4a`](https://github.com/Byron/gitoxide/commit/d287a4aec70e5dd33976a25d9a849c44d62d77c9))
    - [stability #171] Don't leak unstable plumbing crates in git-repository… ([`71eb30f`](https://github.com/Byron/gitoxide/commit/71eb30f1caa41c1f9fe5d2785b71c9d77922c2af))
    - [repository #165] add limitations along with possible workarouds ([`7578f1e`](https://github.com/Byron/gitoxide/commit/7578f1e2e578010eee087a9176d53a5862ec8862))
    - [repository #165] assure packed-refs are always uptodate ([`a5605df`](https://github.com/Byron/gitoxide/commit/a5605df9b83a25f1726b181b78d751987d71a32b))
    - [repository #165] Allow cloning packed-refs and try to see how it differs… ([`7ec32b7`](https://github.com/Byron/gitoxide/commit/7ec32b7662995b5a60aba1bd932830e68ab1dbdc))
    - Release git-ref v0.6.0 ([`0bb4c13`](https://github.com/Byron/gitoxide/commit/0bb4c133da96f6a96d9f1767848ada792a27c2be))
    - [ref #165] refactor ([`66624c3`](https://github.com/Byron/gitoxide/commit/66624c3ef1faf7048ee86ed73cf5f622802c061e))
    - Revert "[repository #165] PROOF: GATs will work as expected!" ([`853f072`](https://github.com/Byron/gitoxide/commit/853f0723d3d202b1cc2e653109ae92aa14d4d437))
    - [repository #165] PROOF: GATs will work as expected! ([`7f56dbd`](https://github.com/Byron/gitoxide/commit/7f56dbd82db2abc18b8e6d228c8a5f54b3dbf32a))
    - [repository #165] refactor ([`1547d0b`](https://github.com/Byron/gitoxide/commit/1547d0b062e35bad2229dac532e6f30bf105db73))
    - [repository #165] refactor; fine grained allow(missing_docs)… ([`aa0511f`](https://github.com/Byron/gitoxide/commit/aa0511f80f11de8e83fc333e78db369ceb9b2794))
    - [repository #165] prepare for writing light docs for Easy ([`f8834c9`](https://github.com/Byron/gitoxide/commit/f8834c9c8d2ab2ce87857c6773c6204f60df240e))
    - thanks clippy ([`41d7a44`](https://github.com/Byron/gitoxide/commit/41d7a443aa63b6ee997fd38ceee05b9b1be3e577))
    - [repository #165] refactor ([`3a0160e`](https://github.com/Byron/gitoxide/commit/3a0160ed1c5bc33d330ad4e9189c4937d194e98d))
    - [repository #162] cleanup imports ([`983d11a`](https://github.com/Byron/gitoxide/commit/983d11a1f46c1ad21dbf2d57b63ecf979fab48b9))
    - [repository #165] fmt ([`a02d5aa`](https://github.com/Byron/gitoxide/commit/a02d5aa8ef0e4a1118a9d8523c3f34b836461952))
    - [smart-release #162] use TreeRef capabilities to lookup path ([`51d1943`](https://github.com/Byron/gitoxide/commit/51d19433e6704fabb6547a0ba1b5c32afce43d8b))
    - [repository #165] Don't panic on repo borrow error… ([`b2f644a`](https://github.com/Byron/gitoxide/commit/b2f644a73c2b1945ab71c5f5719c9b2b32c01b07))
    - [repository #162] what could be a correct implementation of a tree path lookup ([`1f638ee`](https://github.com/Byron/gitoxide/commit/1f638eee0aa5f6e1cc34c5bc59a18b5f22af4cbc))
    - thanks clippy ([`b496d99`](https://github.com/Byron/gitoxide/commit/b496d9952924afdb67e9ba8ea0b9b61c8c8fb1f2))
    - [repository #162] detachable ObjectRefs and a few conversions ([`ec123bb`](https://github.com/Byron/gitoxide/commit/ec123bb615035684e52f2d786dfb41d0449823d2))
    - [repository #165] Write about the GAT plan to make this better one day ([`d793ecd`](https://github.com/Byron/gitoxide/commit/d793ecd00f55b5bf7c6dcaee8772975e97bd5e30))
    - [repository #162] finally let smart-release use the correct abstraction for peeling ([`ba243a3`](https://github.com/Byron/gitoxide/commit/ba243a35ff6f059e5581c6f7ff80e1253ceca6f8))
    - [repository #165] quick test to see if Access2 can become Access… ([`45acc7a`](https://github.com/Byron/gitoxide/commit/45acc7a9d6a89977563872c2eac389a2b78b9e27))
    - [repository #162] Add id field to ObjectRef… ([`f5ba98e`](https://github.com/Byron/gitoxide/commit/f5ba98ebd0e1d7d0491871be58476cb6882b8436))
    - [repository #165] Generalizing over mutable Repos is possible too… ([`0f7efe3`](https://github.com/Byron/gitoxide/commit/0f7efe3f2e2608213ad5c75b52db876dd4214908))
    - [repository #162] Make clear that Objects are actually references… ([`d1e6843`](https://github.com/Byron/gitoxide/commit/d1e68435d0b7d9dcc9e0099be3c0c5723dc08e93))
    - [repository #165] show that Access2 works for all Easy* types… ([`b8ceefe`](https://github.com/Byron/gitoxide/commit/b8ceefed275953aa36d823d51b466cd100729905))
    - [repository #162] another attempt to find a decent peeling abstraction… ([`716d623`](https://github.com/Byron/gitoxide/commit/716d623fb189eb3002d2137827dbfeb143f6ed12))
    - [repository #165] First success with creating a shared borrow to the repo ([`f2a38b2`](https://github.com/Byron/gitoxide/commit/f2a38b20aee484e0354d3e2e3db9cc880ae95310))
    - [repository #162] attach the Object to 'Access' ([`9a12564`](https://github.com/Byron/gitoxide/commit/9a125640da19d5633e51df40dee5332eb9600462))
    - Revert "[repository #165] FAIL Look into `owned_ref` crate" ([`a1443e4`](https://github.com/Byron/gitoxide/commit/a1443e4982fa4d1a1615554a37294d56fd9026eb))
    - [repository #162] refactor ([`a32d361`](https://github.com/Byron/gitoxide/commit/a32d361fd5cb0eb1a4112d834b53c1625372a7bc))
    - [repository #165] FAIL Look into `owned_ref` crate ([`09aa714`](https://github.com/Byron/gitoxide/commit/09aa714f2db5ad220b0e76a65e01e394663f08b4))
    - [repository #162] trying new names ([`b3f453b`](https://github.com/Byron/gitoxide/commit/b3f453b33f8cda04526110a82f0e0a46a3bb2e34))
    - [repository #165] FAIL AsRef works for basic refs but… ([`02979b6`](https://github.com/Byron/gitoxide/commit/02979b61e6bc4e1de3b3badc784a950477b31cad))
    - [repository #162] put impl for finding object data into the extension trait ([`91b9446`](https://github.com/Byron/gitoxide/commit/91b9446fc7035047ebefaa7907e6a8224b56cf27))
    - [repository #165] FAIL try to generalize with Borrow… ([`295ba95`](https://github.com/Byron/gitoxide/commit/295ba95a341775b566c18e897a2d58a94e6d98f9))
    - [repository #162] experiment with finding objects… ([`312a692`](https://github.com/Byron/gitoxide/commit/312a69256a67a0f9d3f3f5c5f9eaf51b50971c5e))
    - [repository #165] FAIL See if EasyExclusive can work… ([`016debb`](https://github.com/Byron/gitoxide/commit/016debbfce7a29502742408da304c80405063230))
    - thanks clippy ([`f2fb026`](https://github.com/Byron/gitoxide/commit/f2fb0266ba64d002a9913699bcf5843647843beb))
    - [repository #165] introduce EasyShared ([`a119ad9`](https://github.com/Byron/gitoxide/commit/a119ad94096a3464b98f6a6bc26c92ba6efa9474))
    - [repository #162] Cannot ever store a RefCell Ref in an object… ([`5c17199`](https://github.com/Byron/gitoxide/commit/5c171995383fa9a3698b6aaf3fbd9537110c0299))
    - [repository #165] First thoughts about stale caches ([`7f8b63e`](https://github.com/Byron/gitoxide/commit/7f8b63e23ef3561117249668d14507cec1508ad3))
    - [repository #162] experiemnt with optionally keeping data in Object ([`b8a8e08`](https://github.com/Byron/gitoxide/commit/b8a8e08e1d972e5069b136c30407c079825b7e1d))
    - [repository #165] hide all easy::State fields behind result-enforcing methods ([`000c537`](https://github.com/Byron/gitoxide/commit/000c537ab766a50679764118af50731b3bab39e5))
    - [repository #165] pack cache access only with errors ([`2353e50`](https://github.com/Byron/gitoxide/commit/2353e5092599228f147ef58c0f0cd45c63c126e2))
    - [smart-release #162] Object can be used like a git_hash::ObjectId ([`c7bc730`](https://github.com/Byron/gitoxide/commit/c7bc730836f05fe9d967320a6858443a649a59ce))
    - [repository #165] assure packed-refs is only used non-panicking ([`a355d94`](https://github.com/Byron/gitoxide/commit/a355d943b986307216161bad38e5bb89f8608b49))
    - [smart-release #162] format everything ([`8ff83e5`](https://github.com/Byron/gitoxide/commit/8ff83e5c511ae29979348789bd6e7a2f72b16f1c))
    - [repository #165] refactor ([`16fce63`](https://github.com/Byron/gitoxide/commit/16fce637561af29727a8fa025f6ddece853fcc20))
    - [repository #165] a sample of a simpler way to create a tag ([`fb8f584`](https://github.com/Byron/gitoxide/commit/fb8f58412cdd32991a182a41cbc0d463127a4e0e))
    - [smart-release #162] don't throw away work… ([`b43b780`](https://github.com/Byron/gitoxide/commit/b43b780c0382683edc859e3fbd27739716a47141))
    - [smart-release #165] Use generic edit-reference functionality ([`be3e57f`](https://github.com/Byron/gitoxide/commit/be3e57f6221dc87505ba1aad1166e28c328c3b54))
    - [smart-release #162] a demo of attaching and detaching objects… ([`ff2927c`](https://github.com/Byron/gitoxide/commit/ff2927ce3fede654d491559fde1c7b07be6a6979))
    - [repository #165] sketch generic ref file editing ([`3a026ae`](https://github.com/Byron/gitoxide/commit/3a026aea2a98648a6b624bca9661555f5a147494))
    - [smart-release #162] an actual Data type… ([`7fd996f`](https://github.com/Byron/gitoxide/commit/7fd996f5f631f83665e81c0f89c34cc47f270d2b))
    - [repository #165] refactor ([`00ec15d`](https://github.com/Byron/gitoxide/commit/00ec15dcfdb839095e508139d238df384ea418eb))
    - [smart-release #162] unify 'ext' visibility ([`ca082a7`](https://github.com/Byron/gitoxide/commit/ca082a75ff29de2a471cec4331a80f84477cca56))
    - [repository #165] refactor ([`0f13104`](https://github.com/Byron/gitoxide/commit/0f13104375216ccf099ebc2fcf0d180ed0de5237))
    - thanks clippy ([`1f2d458`](https://github.com/Byron/gitoxide/commit/1f2d4584f8b650f7e751c8d2df9a5d27725f4f2f))
    - [repository #165] An experiment on transforming panics into errors… ([`1f52226`](https://github.com/Byron/gitoxide/commit/1f5222660970e24eb2d82fed3917f234dce7e0eb))
    - [smart-release #162] a sketch for accessing objects data… ([`ba27101`](https://github.com/Byron/gitoxide/commit/ba27101e08b2bab5d33b53fedcc0c6aa13b8f35e))
    - [repository #165] offer panicking type conversions for objects ([`f802f8c`](https://github.com/Byron/gitoxide/commit/f802f8c8c382f8063fa615fda022857a740a974a))
    - [repository #165] try a more common naming convention for fallbile things… ([`fc70393`](https://github.com/Byron/gitoxide/commit/fc703937a078937840ea1c254f11e64aaf31de90))
    - [smart-release #162] peeling objects to a certain target kind… ([`5785136`](https://github.com/Byron/gitoxide/commit/57851361f3fc729b964fd0ca5dca9f084fe20f5e))
    - [repository #165] refactor ([`6207735`](https://github.com/Byron/gitoxide/commit/6207735f7d955e8a1676c8ad549ce6c1137da760))
    - [smart-release #162] a single import path for ReferenceExt ([`7060797`](https://github.com/Byron/gitoxide/commit/7060797031e5bdbb8d635cc2da3269996bdfc4cc))
    - [smart-release #162] rename git-repository::object -> objs ([`ac70d81`](https://github.com/Byron/gitoxide/commit/ac70d81791cad04ffdeb04916d7a2a6b533eee6c))
    - [smart-release #162] replace reference peeling with git_easy ([`7cfd5f9`](https://github.com/Byron/gitoxide/commit/7cfd5f9e0a7f828152594f0393a919617c60a9d6))
    - [smart-release #162] smart-release uses Easy repository in 'plumbing' mode ([`4fb672a`](https://github.com/Byron/gitoxide/commit/4fb672a6e7116722577cbbeeee67887871f583bf))
    - [smart-release #162] refactor ([`ef623a6`](https://github.com/Byron/gitoxide/commit/ef623a6835ab86225ac65b933b0df62c00baa1af))
    - [smart-release #162] reduce visibility of Cache ([`397fbfe`](https://github.com/Byron/gitoxide/commit/397fbfe6bde7e03c23b66aa60f70d2e6649f5eef))
    - [smart-release #162] more granular cache control WORKS ([`25dce2a`](https://github.com/Byron/gitoxide/commit/25dce2a4b4522fb9f51fab506dddd8c6ebfb0f54))
    - Revert "[smart-release #162] FAIL: definitely need better granularity" ([`499993f`](https://github.com/Byron/gitoxide/commit/499993fe0b71ac08b3940119bc682533223a3ddb))
    - [smart-release #162] FAIL: definitely need better granularity ([`5f27871`](https://github.com/Byron/gitoxide/commit/5f27871b773c18a9f065a0c8e86101382d23c71f))
    - [smart-release #162] FAIL: promising at first, but not really working… ([`fa01f76`](https://github.com/Byron/gitoxide/commit/fa01f7684c0b7d3b90ec7bde651684a84014a576))
</details>

## v0.7.2 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.2 ([`c5791b1`](https://github.com/Byron/gitoxide/commit/c5791b1903e91987f2684eaa8d5d8d21255ae40f))
    - [smart-release #162] separate mutable state more cleanly… ([`f00de95`](https://github.com/Byron/gitoxide/commit/f00de9575358dec477667e2e7b5090fb75b46ad6))
    - [smart-release #162] FAIL: one level down, using the cache isn't really working… ([`65db010`](https://github.com/Byron/gitoxide/commit/65db0104146248b273081fc6616a6ed484aa948e))
    - [smart-release #162] a promising lead, this might just work ([`0c4f77b`](https://github.com/Byron/gitoxide/commit/0c4f77b27815d708be4fa6ed26414231f0d51a38))
    - bump git-protocol to v0.9.0 as there are breaking changes ([`b4e3340`](https://github.com/Byron/gitoxide/commit/b4e33408b8eb12c9418704f663322385fd1dfb25))
    - [smart-release #162] a barely working version of refs handling… ([`3e01025`](https://github.com/Byron/gitoxide/commit/3e0102565f0ecdac61e83ed9fb06cc7d788638c7))
    - [smart-release #162] a sign - can't store references, but… ([`7862652`](https://github.com/Byron/gitoxide/commit/7862652fad734a51ead99d6c3988c1bfe92ad2ad))
    - Revert "[smart-release #162] FAIL try to use Rc<RefCell<_>>…" ([`58529a1`](https://github.com/Byron/gitoxide/commit/58529a1e67b77ba1cfe0b794b6ce513162a65139))
    - [smart-release #162] FAIL try to use Rc<RefCell<_>>… ([`180be72`](https://github.com/Byron/gitoxide/commit/180be72d8fd37f326484ebdf99a1e1fc8843958d))
    - [smart-release #162] refactor ([`8f558af`](https://github.com/Byron/gitoxide/commit/8f558afc88276a66c42004e0ac66d89382d83426))
    - thanks clippy ([`b63cd40`](https://github.com/Byron/gitoxide/commit/b63cd40909d02af85f10b77bc40e1630caf355cf))
    - [smart-release #162] refactor ([`35ff637`](https://github.com/Byron/gitoxide/commit/35ff637ab8deaef23a29cfb9bd91f5ea07da7a0c))
    - [smart-release #162] First compiling version, non-threadsafe… ([`d2b2ce9`](https://github.com/Byron/gitoxide/commit/d2b2ce9c1fd78fa63ad24d40eac62f5cbd4f4682))
    - [smart-release #162] FAIL: RefCell as self param also doesn't work :D… ([`ec0c863`](https://github.com/Byron/gitoxide/commit/ec0c8632360e7c4c7ecf02d0915202d616730644))
    - [smart-release #162] back to a more humble, hard-coded approach… ([`bdceb7c`](https://github.com/Byron/gitoxide/commit/bdceb7cf6a3c83536c0a3b0cd5f392040d25bb00))
    - Revert "[smart-release #162] FAIL: cannot use extension traits…" ([`2878a14`](https://github.com/Byron/gitoxide/commit/2878a14613ed1083dd4ff7bc11b09820bade9058))
    - [smart-release #162] FAIL: cannot use extension traits… ([`e115631`](https://github.com/Byron/gitoxide/commit/e1156314f38e998f1b15a49a126382aa2c10022a))
    - [smart-release #162] FAIL: try to do things borrowck doesn't like… ([`853ae9c`](https://github.com/Byron/gitoxide/commit/853ae9cfb12f9ce981d1fa20b9d73d7e3d371f77))
    - [smart-release #162] a sketch of an API that seems to satisfy the constraints… ([`bec8473`](https://github.com/Byron/gitoxide/commit/bec847386a198b4ca5b70bd2a8bf337c007d0501))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.7.1 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.7.1 ([`4369697`](https://github.com/Byron/gitoxide/commit/4369697e6c5f80a899a5e38fa9fe8be44c6504f1))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com/Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com/Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - Release git-object v0.12.0 ([`7006150`](https://github.com/Byron/gitoxide/commit/7006150ac314d19814608723f69f6e70a72f9262))
    - Release git-actor-0.3.1 ([`727087d`](https://github.com/Byron/gitoxide/commit/727087dca243da4bc40bc87611a2f66234565be7))
    - (cargo-release) version 0.18.0 ([`b327590`](https://github.com/Byron/gitoxide/commit/b327590d02fec5536c380b2d39dd7be089ca7c40))
    - (cargo-release) version 0.6.0 ([`d704bca`](https://github.com/Byron/gitoxide/commit/d704bca7de0a6591f35345c842d6418b36ecd206))
    - (cargo-release) version 0.6.0 ([`4b71e15`](https://github.com/Byron/gitoxide/commit/4b71e15c3ba4a17ff2da5a5ef79986a2832fa3f2))
    - (cargo-release) version 0.5.0 ([`e21142b`](https://github.com/Byron/gitoxide/commit/e21142ba1a113b2afc4725d4d4225dff519c513a))
    - (cargo-release) version 0.17.0 ([`c52a491`](https://github.com/Byron/gitoxide/commit/c52a49176bd294bb36db74b4293cdb684a2ab7f6))
    - (cargo-release) version 0.5.0 ([`c2f94a5`](https://github.com/Byron/gitoxide/commit/c2f94a51bce287be301090450cb00cde57e92f76))
    - (cargo-release) version 0.4.0 ([`d69d0ac`](https://github.com/Byron/gitoxide/commit/d69d0ac21989243fdafa514fa41579fd51bc2558))
    - (cargo-release) version 0.6.0 ([`d58f37e`](https://github.com/Byron/gitoxide/commit/d58f37e3b5a000fbe069aa869bd84f66d5c3210b))
    - (cargo-release) version 0.5.0 ([`1687e59`](https://github.com/Byron/gitoxide/commit/1687e599be98d97925fbab594f31cf5558e9d2b1))
    - (cargo-release) version 0.4.0 ([`28e58f6`](https://github.com/Byron/gitoxide/commit/28e58f6b43a44e010da749a5618df02441f0d2e8))
    - (cargo-release) version 0.11.0 ([`a5be31c`](https://github.com/Byron/gitoxide/commit/a5be31c4cf7c0b538a1ed4a52ff5c3a992c6feff))
    - (cargo-release) version 0.3.0 ([`64efc05`](https://github.com/Byron/gitoxide/commit/64efc0534ddc372b6e668b23c1e9d276098679c9))
    - (cargo-release) version 0.4.0 ([`70ef344`](https://github.com/Byron/gitoxide/commit/70ef3442775b54ba9e4ee9ebfffb37af9804cc5b))
</details>

## v0.7.0 (2021-08-10)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 41 commits contributed to the release over the course of 63 calendar days.
 - 74 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 ([`1c5dfb8`](https://github.com/Byron/gitoxide/commit/1c5dfb86028f266435475ca8bdddc57f95002330))
    - (cargo-release) version 0.3.0 ([`0e9c73a`](https://github.com/Byron/gitoxide/commit/0e9c73abd17e0dd21952275077ae53ad7e7aa1af))
    - (cargo-release) version 0.5.0 ([`ae02dab`](https://github.com/Byron/gitoxide/commit/ae02dabae961089a92a21e6a60a7006de4b56dad))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - (cargo-release) version 0.5.0 ([`0e11e98`](https://github.com/Byron/gitoxide/commit/0e11e98f0562c7baa9c90e18db6240731d165217))
    - (cargo-release) version 0.2.0 ([`8ff5115`](https://github.com/Byron/gitoxide/commit/8ff511583e6d859e43ffda0ef75e2fecce3ed03c))
    - [repository #149] pre-emptively fix windows ([`b4d3934`](https://github.com/Byron/gitoxide/commit/b4d39345d723981bba1db8d313ef7ec4cd83cc82))
    - [repository #149] only canonicalize if absolutely required ([`d537fac`](https://github.com/Byron/gitoxide/commit/d537fac34e3fb18bd02281f7c74535b59510cff9))
    - [repository #149] canonicalize only when needed ([`57f42bd`](https://github.com/Byron/gitoxide/commit/57f42bdeda1895ca6aba84b58ad44762a17480c2))
    - [repository #149] prepare for canonicalizing only when needed ([`cac9d70`](https://github.com/Byron/gitoxide/commit/cac9d702f62cb2527b9c6357bfcbc9d31da69b01))
    - [repository #149] refactor ([`3c368ec`](https://github.com/Byron/gitoxide/commit/3c368ecb7a07aaff73f0db4432a6184479eb3929))
    - [repository] Fix TreeExt trait name - it's actually for TreeIters ([`f8e0747`](https://github.com/Byron/gitoxide/commit/f8e07475f8867fc98a9264b1270977b48283a94e))
    - Canonicalize path when discovering repositories ([`7bfaa14`](https://github.com/Byron/gitoxide/commit/7bfaa14aca1e96c1998e464971808f67c1c4077f))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - [ref] fix build ([`1dcc590`](https://github.com/Byron/gitoxide/commit/1dcc590133ff36e2b2c892b3f51df737a46ccc4c))
    - [ref] refactor ([`e26c72f`](https://github.com/Byron/gitoxide/commit/e26c72fb1bf9392932ffe42843f3dec52c7bbd7d))
    - [ref] and it compiles again, may todos left ([`16618b9`](https://github.com/Byron/gitoxide/commit/16618b916ff67316717d95575fc1344d956d2c49))
    - [ref] fix build ([`83002df`](https://github.com/Byron/gitoxide/commit/83002df0296a431de839ebb3522f57d42a17515f))
    - [ref] rename find_one to 'find' in git-ref… ([`ae7746a`](https://github.com/Byron/gitoxide/commit/ae7746a0815bb94659de67383ba372ac522d53b8))
    - [ref] refactor ([`758c090`](https://github.com/Byron/gitoxide/commit/758c0907df8dc6987f374e326304e0f9fad29812))
    - Revert "[ref] parameterize all uses of hash length…" ([`21f187e`](https://github.com/Byron/gitoxide/commit/21f187e6b7011bb59ed935fc1a2d0a5557890ffe))
    - [ref] parameterize all uses of hash length… ([`5c7285e`](https://github.com/Byron/gitoxide/commit/5c7285e7233390fd7589188084fcd05febcbbac2))
    - [ref] another deletion test succeeds ([`6037900`](https://github.com/Byron/gitoxide/commit/60379001d2729627c042f304217d6459f99f01bf))
    - [ref] file store can ignore all writes; sketch transaction API ([`52a81e9`](https://github.com/Byron/gitoxide/commit/52a81e98f38657023d3eb384fd6db69917dd57ca))
    - [actor] fix gix hours ([`b4e95fd`](https://github.com/Byron/gitoxide/commit/b4e95fdbb6664adcb2603d9cb6e6a69182de050f))
    - (cargo-release) version 0.4.0 ([`4512798`](https://github.com/Byron/gitoxide/commit/45127986daba0a409f5b405d463fa23f5c4a053b))
    - [lock] cleanup signal handling even more… ([`9fb13d2`](https://github.com/Byron/gitoxide/commit/9fb13d27ccce5b0742ee9289fca891dbeb8a65de))
    - (cargo-release) version 0.3.0 ([`92f3a83`](https://github.com/Byron/gitoxide/commit/92f3a830457766c88c68f8424828bfd9b5145f86))
    - (cargo-release) version 0.2.0 ([`7c2eb36`](https://github.com/Byron/gitoxide/commit/7c2eb36274d13646956ac850bee90abbbac91c5b))
    - fix docs ([`e68d460`](https://github.com/Byron/gitoxide/commit/e68d460716dc51c7f4757c11f3c8af6c3881e2cf))
    - fix build ([`dbfa49a`](https://github.com/Byron/gitoxide/commit/dbfa49acf58b2c0763c5e98e5276860b43dfb27b))
    - Remove mentions of interrupt handling feature toggles ([`833ac04`](https://github.com/Byron/gitoxide/commit/833ac0464b42bd3ecc76c6263b4b06e8ab4ff182))
    - Fix everything up so that… ([`5930563`](https://github.com/Byron/gitoxide/commit/5930563601d6c2148cf39e109f69f8b7c7dfcb36))
    - A first attempt to make intrerupt tools work, but… ([`8fb8d37`](https://github.com/Byron/gitoxide/commit/8fb8d374ecfeffa3ae1bd07bf9bc5014351730f5))
    - First step towards moving git-features::interrupt… ([`8a741d0`](https://github.com/Byron/gitoxide/commit/8a741d0c5423ed7c35d9382307c760a6b9460ccd))
    - [pack] add --statistics flag to pack-create ([`51a3077`](https://github.com/Byron/gitoxide/commit/51a307730b8514acffa75c78ecca3f02b1eb467b))
    - [async-client] frame for async connect ([`9ada080`](https://github.com/Byron/gitoxide/commit/9ada0805fc5896f8ef1a31dc821b789b7f0438a6))
    - Separate networking via feature toggles and pass that through in the main crate ([`2c749f1`](https://github.com/Byron/gitoxide/commit/2c749f10dd03ea0b027fb046e8c40c77869fb2e9))
    - (cargo-release) version 0.3.0 ([`6b33678`](https://github.com/Byron/gitoxide/commit/6b33678f83e6d261ca15c4a7634ff5b4e66d81dd))
    - Merge branch 'dependabot/cargo/crc-2.0.0' ([`683c44d`](https://github.com/Byron/gitoxide/commit/683c44db682d8dbef401286963e84cdca145abc8))
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
</details>

## v0.6.0 (2021-05-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release.
 - 49 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`d35c55d`](https://github.com/Byron/gitoxide/commit/d35c55d8ff4b52e25befb8bff839d805b9f3caf4))
    - [git-repository] better docs ([`f60a7c5`](https://github.com/Byron/gitoxide/commit/f60a7c567a2ae856840b276479582b87bb0530f5))
    - [git-repository] gitoxide-core uses more of git-repository ([`bb5b074`](https://github.com/Byron/gitoxide/commit/bb5b0747dfd3a3985a904b7748f296a591fcb26e))
    - [git-repository] replaces git-features and git-protocol in gitoxide-core ([`081d20f`](https://github.com/Byron/gitoxide/commit/081d20f927f222daa69f2a1a492957fd3146bfc1))
    - [git-repository] used by gix-hours ([`24e0258`](https://github.com/Byron/gitoxide/commit/24e0258b9691b82df5c35a35111d19df56087cdc))
    - [git-repository] refactor ([`b5ebcfa`](https://github.com/Byron/gitoxide/commit/b5ebcfa278a0be85ea10893fd40a8b3e2e28efd5))
    - [git-repository] now used by gixp-organize ([`aa91fad`](https://github.com/Byron/gitoxide/commit/aa91fad3cf237f6d6f9d588ed390baa6e55f6540))
    - [git-repository] make it easy to get maximum performance in apps using this crate ([`dc150a5`](https://github.com/Byron/gitoxide/commit/dc150a5913ac5db6211c5881873254bc8377aad2))
    - [git-repository] prevent other implementations of extension traits; refactor ([`e14df75`](https://github.com/Byron/gitoxide/commit/e14df75fa999508a1e3102add4829ba55ec3aa50))
    - [git-repository] finish 'diffing' program upgrade ([`7eea39a`](https://github.com/Byron/gitoxide/commit/7eea39a8d945f28b376698af9b1a0f67ffaa7e6f))
    - [git-repository] more details on how this crate is intended ([`cd85050`](https://github.com/Byron/gitoxide/commit/cd85050a506ef99192909db6d8373a99282df53d))
    - [git-repository] refactor ([`b9f4d25`](https://github.com/Byron/gitoxide/commit/b9f4d25ae80c3dc6e03b734202eae44d444cb442))
    - [git-repository] try out an API for ancestor iteration ([`de0b5bb`](https://github.com/Byron/gitoxide/commit/de0b5bbe71ce8cfb49665b4f7e429d719dcb08dd))
    - [git-repository] the first extension trait for more convenience ([`63a1fee`](https://github.com/Byron/gitoxide/commit/63a1fee9195c9d3c23001e09cccece2b2af43324))
    - [git-repository] now with a prelude for traits ([`7f7a5ea`](https://github.com/Byron/gitoxide/commit/7f7a5eaf080217628b3645af3ff5f1872d5ce11c))
    - [git-repository] more re-exports for convenience ([`6a5c00e`](https://github.com/Byron/gitoxide/commit/6a5c00e2e1fb7ca911d1f8ce3534a74316478149))
    - (cargo-release) version 0.4.0 ([`866f86f`](https://github.com/Byron/gitoxide/commit/866f86f59e66652968dcafc1a57912f9849cb21d))
    - [git-repository] towards git-repository as one stop shop ([`aea6cc5`](https://github.com/Byron/gitoxide/commit/aea6cc536f438050cc0e02223de7702cd7912e75))
    - [git-repository] repo-init sketch ([`5855c95`](https://github.com/Byron/gitoxide/commit/5855c952e2703412a5f7c1ffbfe57b85f339bab1))
    - [git-repository] refactor ([`63c22af`](https://github.com/Byron/gitoxide/commit/63c22afe153b08453c3c12c3bb81626d2381f472))
    - [git-repository] refactor ([`996944a`](https://github.com/Byron/gitoxide/commit/996944a75160538588d34385b6a6717b05ee9c47))
    - [git-repository] refactor ([`a2d58c1`](https://github.com/Byron/gitoxide/commit/a2d58c100ca696bceaaa0788347bba41f29ab0b8))
    - [git-repository] a sketch of how the repository could look like ([`3854cef`](https://github.com/Byron/gitoxide/commit/3854cef47205e449bfc638255eefe303a99897d8))
    - [git-repository] traversal uses git-repository ([`db564c5`](https://github.com/Byron/gitoxide/commit/db564c5016272ff6d2038fd2b554cb6dacb0a6c5))
    - [git-repository] an actual repository abstraction ([`3f20b26`](https://github.com/Byron/gitoxide/commit/3f20b267b97f0855d958a37b36984da288263cc2))
    - [git-repository] refactor ([`c8323e4`](https://github.com/Byron/gitoxide/commit/c8323e484f08d5ea59400636cb26334d6976e4c0))
    - [git-repository] traversal program uses new facilities, and it's cumbersome ([`29ea2de`](https://github.com/Byron/gitoxide/commit/29ea2de9ad48036f78d3776d8526d959f68bf287))
    - [git-repository] bare repository handling ([`3a8e6ff`](https://github.com/Byron/gitoxide/commit/3a8e6ff041efc57482252458acf379b43ef6b523))
    - [git-repository] tests pass, bare repo tests missing ([`a5ed9ea`](https://github.com/Byron/gitoxide/commit/a5ed9ea3004f81c2132b86fe26ad34abf620c765))
    - [git-repository] most of the git repository discovery ([`72a49c8`](https://github.com/Byron/gitoxide/commit/72a49c816253520230a04290619f469df608be19))
    - [git-repository] frame for repository testing; sketch of discovery API ([`467e340`](https://github.com/Byron/gitoxide/commit/467e340b6c36cad299d35546a60cdb308e29b289))
</details>

## v0.5.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 188 calendar days.
 - 208 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`02df134`](https://github.com/Byron/gitoxide/commit/02df1345a22889a573adfc1be80bda271b2dc9a5))
    - refactor ([`170215d`](https://github.com/Byron/gitoxide/commit/170215dc941af9b6a8f19c1fef91f3b5802e1cc7))
    - Ensured linter checks pass ([`51f2183`](https://github.com/Byron/gitoxide/commit/51f2183357573f9ea30dffbf61af73d5e845f5aa))
    - Ensured output of directory-less git init unchanged ([`539a573`](https://github.com/Byron/gitoxide/commit/539a5737459de10404b6ba6f06a20224b6d534af))
    - Added [directory] argument to init. ([`62f8dc6`](https://github.com/Byron/gitoxide/commit/62f8dc62ec3e76efd7311ced32094035856dbcbb))
    - Spelling fix in error message ([`944d0f4`](https://github.com/Byron/gitoxide/commit/944d0f4ae830c8f2e7eabe3bd58cd023f5674ce1))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 28 calendar days.
 - 30 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`2b1bca8`](https://github.com/Byron/gitoxide/commit/2b1bca83c453544972e370dc0adff57cb7590b42))
    - refactor ([`ba1d883`](https://github.com/Byron/gitoxide/commit/ba1d88364424eb60a0874a5726b62740dc348592))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 31 calendar days.
 - 31 days passed between releases.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump minor version to 0.3 ([`4351e28`](https://github.com/Byron/gitoxide/commit/4351e2871c9dcf342b8471fffa74cae338a53269))
    - update to quick-error 2.0 ([`4b1b784`](https://github.com/Byron/gitoxide/commit/4b1b7849b47a54092b49821c39e864c86adda979))
    - Switch to latest quick-error ([`9760856`](https://github.com/Byron/gitoxide/commit/976085614ee13a19fc1347209259a3dcf36ef95b))
    - refactor ([`2888f1b`](https://github.com/Byron/gitoxide/commit/2888f1b10a2baf40155544e667ddd461f3ddc938))
    - explicitly include assets in git-repository crate ([`9da6071`](https://github.com/Byron/gitoxide/commit/9da6071c97d668e5af4eedb554ca8f91d184ee7e))
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
    - Make crates publishable ([`5688a34`](https://github.com/Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - Fix tests ([`59ed51d`](https://github.com/Byron/gitoxide/commit/59ed51d0c84bf067ef0a921730260f2c444e5409))
    - Use 'main' branches instead of the previous default when initializing a repository ([`da77cc8`](https://github.com/Byron/gitoxide/commit/da77cc807f34d23da76e4d94e4220ed638713f59))
    - Allow for more screen space when formatting ([`6794300`](https://github.com/Byron/gitoxide/commit/67943002e7f4215b5383bd0538786ce2857f011e))
    - goodbye git-core, hello git-repository ([`7cec2b6`](https://github.com/Byron/gitoxide/commit/7cec2b648f86fc665b4fc5bfe269e9ca16679a55))
</details>

