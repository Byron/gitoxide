# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.30.0 (2022-12-19)

<csr-id-fceee748c114b2d0760074e911e533cd020f6996/>

### Changed

 - <csr-id-a4ac9cf3e667a3059e33aac8188150529578622d/> represent `GIT_(COMMITTER|AUTHOR)_(NAME|EMAIL|DATE)` with git configuration.
   That way it becomes more obvious where values are coming from.

### New Features

 - <csr-id-1683a848459cae2b9182b365e3e22b0e8ba73534/> expose `git-features` crate at root under `features`.
   That way application developers can use more of the utilities
   that power most of the `gitoxide` plumbing crates.
 - <csr-id-90ef6fc36b440cc4baf3fde4a30060f1b4a0c8cf/> `Remote` knows about its `tagOpt` configuration.
   That way it's clear if it should or shouldn't fetch included/reachable
   tags automatically.
   
   The default setting for this is to include tags, similar to `git`.
   
   The `fetch_tags()` accessor allows to query this information, and the
   `with_fetch_tags()` builder method allows to set the value comfortably
   right after creating the `Remote` instance.
   
   The `tagOpt` key will also be written as part of the remote's git
   configuration.
   
   Clone operations can set the `Tags` setting when configuring the
   remote in a callback.
   
   This also comes with a fix to assure that ref-updates aren't skipped
   just because there was no pack to receive. That way, locally missing
   refs or tags will automatically be put back.
 - <csr-id-28e48083052216ddf1fd1f187cc22d506d3d9f86/> network related Error type support `is_spurious()` method.
   That way the caller can determine more easily if it makes sense
   to try again.
 - <csr-id-457c2e081b1aa5dfaab3f663b9aba66c16369939/> Make `prodash::tree` avaialble as `progress::tree`.
 - <csr-id-d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2/> read worktree specific configuration to override the one from the shared repository.
   This is intensively used when space checkouts are created, along with
   Cone mode. Thus it's the basis for properly interpreting sparse checkout
   options which are set on a per-worktree basis.
 - <csr-id-fc64693d5af0fda402c560d10d15652c24d14219/> add `permissions::Environment::http_transport`.
   That way it's possible to deny using environment variables that affect
   the HTTP transport, like setting the proxy.
 - <csr-id-0ce29a965cf16273cf74bd22e40f464e322e2f62/> `open::Options::modify()` as general pattern to allow builder methods usage in `&mut self`.
   That way it's easier to configure both the `full` and the `partial` trust instances
   of discovery options.
 - <csr-id-8482f90d0a2b61259cd51ca4f40ce322e388cb34/> Add `Repository::commit_as(committer, author, …)` convenience method.
   That way it's, very much beyond convenience, possible to set the time
   of a commit.
   
   Many thanks to @epage for the suggestion.
 - <csr-id-c8835c6edae784c9ffcb69a674c0a6545dbb2af3/> upgrade to `prodash 21.1` and add `Ids` to all progress instances.
   That way callers can identify progress they are interested in, say, for
   selective visualizations.

### Bug Fixes

 - <csr-id-d659bda2e1b0fcab529df7af6467f063ae5d0dd7/> provide a clearer error message when trying to open a git repository that isn't one.
 - <csr-id-ff0332e815c228cc5cdfe58c3598ad261bb2879e/> http transports can now reuse a connection.
   This makes connections more efficient generally and `cargo` relies
   on that behaviour in their tests as well.
 - <csr-id-9079b9d2e5f7cc133c6f2b2c2e64245b150c7d74/> allow to open a `Repository` from if 'config' file is missing.
   In this case, treat it similar to having an empty repository configuration
   file and assume defaults everywhere.
 - <csr-id-40f7379b7a89f7fe6f916801384e9e65e5b85c57/> improve error verbosity when fetching and cloning
 - <csr-id-b77fc86ab580dd81b08022996f07cc7925104e77/> `tree::diff::Platform::for_each_to_obtain_tree()` now properly surfaces user provided errors.
   Previously it would squelch them unintentionally.
   
   First discovered via https://github.com/Byron/crates-index-diff-rs/issues/35.
 - <csr-id-5386eed6a13a32a850c59706b15d8988c67733ce/> when fetching from file://, don't upset windows by trying `d:/foo`, use `d:\\foo` instead.
 - <csr-id-363ac7a74ec841505b5fc7cc1b8fae11c0a63ea9/> `config::CommitAutoRollback` now implements `DerefMut`.

### Changed (BREAKING)

 - <csr-id-3c84cebc5997356ff5f531c6cc9567bdd9b83eb5/> default features are set to `max-performance-safe` to assure compatibility.
   Previously the `max-performance` setting might have caused issues during compilation
   or issues at runtime if libraries like `git2` are used in the same binary, and the
   new default feature settings maximizes compatbility so this won't happen.
   
   For best performance, however, one will have to activate the `max-performance`
   feature on top of that.
 - <csr-id-5fe6aa3f3f2f81d84f0d96e874e68a8bf4de1db1/> environment variable permissions are per topic.
   Now `Permissions` for environment variables are so that they
   are by topic instead of by prefix, by default. That way
   it's easier to allow or deny particular sets of related
   environment variables.
   
   The catch-all permissions by prefix are still present for all
   other variables that aren't contained in one particular topic.
 - <csr-id-49f39d6bb487c0254176a5082f2c7851b83952a1/> `open::ReplacementObjects` is removed in favor of two custom git-configuration flags.
   Now it's possible to map the environment variables `GIT_REPLACE_REF_BASE` and `GIT_NO_REPLACE_OBJECTS`
   to custom git configuration keys which can also be set, namely `gitoxide.odb.replaceObjectsRefBase`
   and `gitoxide.odb.noReplaceObjects`.
   
   Along with the possibility of disabling the usage of `GIT_` prefixed environment variables one
   reaches the previous level of control without making object replacement a special case.

### New Features (BREAKING)

 - <csr-id-f8a2bfb93dadbc64393135e0a447f3d76628509c/> `interrupts::init_handler()` can now be undone.
   This can be done by calling `deregister()` or `auto_deregister()` on the return value
   of `interrupts::init_handler(…)`.
   
   That way it's possible to temporarily do interrupt handling only while some methods
   that rquire it are running.
 - <csr-id-becbd8d896a1663f1607be4e86e632773e926f1f/> represent object cache configuration like `GITOXIDE_PACK_CACHE_MEMORY` in git-configuration.
   That way there is a unified system for how to set values, which may be overridable by configuration
   variables or not.
   
   With this changes, the explicit application of environment variables for setting the cache
   isn't required anymore as everything happens using git-configuration, and automatically,
   while providing full control like before.
 - <csr-id-f16e36168cc93768ba5d53c9848ff2e8432d06b1/> remove `SnapshotMut::apply_cli_overrides()` in favor of `open::Options::cli_overrides()`.
 - <csr-id-84d594caf3f04f1ce337e455343278a6f4674957/> more type-safety for remote names by making clear they can be named after URLs.

### Other (BREAKING)

 - <csr-id-fceee748c114b2d0760074e911e533cd020f6996/> `Remote::with_refspec()` to `Remote::with_refspecs()` to allow adding more than one refspec as part of the builder.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 74 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 25 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'read-header' ([`3d01252`](https://github.com/Byron/gitoxide/commit/3d0125271ec7bd606734bd74757a7e31a18c7ce5))
    - expose `git-features` crate at root under `features`. ([`1683a84`](https://github.com/Byron/gitoxide/commit/1683a848459cae2b9182b365e3e22b0e8ba73534))
    - adjust to changes in `git-odb` ([`50ea7fb`](https://github.com/Byron/gitoxide/commit/50ea7fba30c752f86609fabf579a8a038b505c17))
    - Merge branch 'patch-1' ([`fbce7bb`](https://github.com/Byron/gitoxide/commit/fbce7bb55c8c2474c0dfc5413649ecf744d00d92))
    - Use specific Iter constructors in stats example ([`0a72c18`](https://github.com/Byron/gitoxide/commit/0a72c1876b8530f44d464b1597abd6428263d36e))
    - Merge branch 'adjustments-for-cargo' ([`083909b`](https://github.com/Byron/gitoxide/commit/083909bc7eb902eeee2002034fdb6ed88280dc5c))
    - thanks clippy ([`f1160fb`](https://github.com/Byron/gitoxide/commit/f1160fb42acf59b37cbeda546a7079af3c9bc050))
    - adapt to changes in `git-features::fs`. ([`35f7d59`](https://github.com/Byron/gitoxide/commit/35f7d5960210738d88d35aef9c1ed3480681c481))
    - `Remote` knows about its `tagOpt` configuration. ([`90ef6fc`](https://github.com/Byron/gitoxide/commit/90ef6fc36b440cc4baf3fde4a30060f1b4a0c8cf))
    - adjust to changes in `git-testtools` ([`4eb842c`](https://github.com/Byron/gitoxide/commit/4eb842c7150b980e1c2637217e1f9657a671cea7))
    - Merge branch 'adjustments-for-cargo' ([`94750e1`](https://github.com/Byron/gitoxide/commit/94750e15831969059551af35d31c21009462084d))
    - improve docs related to authentication when fetching ([`5979503`](https://github.com/Byron/gitoxide/commit/5979503bd884f53ae02200c76e55b3709f85c1d6))
    - provide a clearer error message when trying to open a git repository that isn't one. ([`d659bda`](https://github.com/Byron/gitoxide/commit/d659bda2e1b0fcab529df7af6467f063ae5d0dd7))
    - http transports can now reuse a connection. ([`ff0332e`](https://github.com/Byron/gitoxide/commit/ff0332e815c228cc5cdfe58c3598ad261bb2879e))
    - allow to open a `Repository` from if 'config' file is missing. ([`9079b9d`](https://github.com/Byron/gitoxide/commit/9079b9d2e5f7cc133c6f2b2c2e64245b150c7d74))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - upgrade clru, remove it from git-repository dependencies (unused) ([`7e7547d`](https://github.com/Byron/gitoxide/commit/7e7547d995afc16192a1ee08add5a87560197fc9))
    - Merge branch 'main' into adjustments-for-cargo ([`bb60d3d`](https://github.com/Byron/gitoxide/commit/bb60d3d5cb9dbd7abe61accded6d21e320c624db))
    - adapt to changes in git-repository ([`89230f4`](https://github.com/Byron/gitoxide/commit/89230f4e151056abaa2bce39d9d18f6dd1512d59))
    - improve error verbosity when fetching and cloning ([`40f7379`](https://github.com/Byron/gitoxide/commit/40f7379b7a89f7fe6f916801384e9e65e5b85c57))
    - network related Error type support `is_spurious()` method. ([`28e4808`](https://github.com/Byron/gitoxide/commit/28e48083052216ddf1fd1f187cc22d506d3d9f86))
    - Merge branch 'paulyoung/scheme-ext' ([`3e27550`](https://github.com/Byron/gitoxide/commit/3e27550577ea942427a57c902570f0416f540753))
    - realign test expectations ([`93e6d71`](https://github.com/Byron/gitoxide/commit/93e6d7199408e492574c43fcfb81faccea2b6fd4))
    - `tree::diff::Platform::for_each_to_obtain_tree()` now properly surfaces user provided errors. ([`b77fc86`](https://github.com/Byron/gitoxide/commit/b77fc86ab580dd81b08022996f07cc7925104e77))
    - when fetching from file://, don't upset windows by trying `d:/foo`, use `d:\\foo` instead. ([`5386eed`](https://github.com/Byron/gitoxide/commit/5386eed6a13a32a850c59706b15d8988c67733ce))
    - `Remote::with_refspec()` to `Remote::with_refspecs()` to allow adding more than one refspec as part of the builder. ([`fceee74`](https://github.com/Byron/gitoxide/commit/fceee748c114b2d0760074e911e533cd020f6996))
    - default features are set to `max-performance-safe` to assure compatibility. ([`3c84ceb`](https://github.com/Byron/gitoxide/commit/3c84cebc5997356ff5f531c6cc9567bdd9b83eb5))
    - `interrupts::init_handler()` can now be undone. ([`f8a2bfb`](https://github.com/Byron/gitoxide/commit/f8a2bfb93dadbc64393135e0a447f3d76628509c))
    - Make `prodash::tree` avaialble as `progress::tree`. ([`457c2e0`](https://github.com/Byron/gitoxide/commit/457c2e081b1aa5dfaab3f663b9aba66c16369939))
    - read worktree specific configuration to override the one from the shared repository. ([`d1b7ec6`](https://github.com/Byron/gitoxide/commit/d1b7ec605f8016c52c088477b6b0c5adf7ea0ab2))
    - refactor ([`2d83222`](https://github.com/Byron/gitoxide/commit/2d83222dbf607f78acad4874580d1f007d838c13))
    - move tests::repository::config::worktree to `tests::repository::open::worktree` ([`62afb7b`](https://github.com/Byron/gitoxide/commit/62afb7ba87311c5b04c8cd8002308d1b44959131))
    - improve documentation about the configuration we always load ([`75488a7`](https://github.com/Byron/gitoxide/commit/75488a7d91abb90337d42f04e86e3d1373b8c19e))
    - Assure that worktree configuration is marked as such with `Source::Worktree`. ([`a191948`](https://github.com/Byron/gitoxide/commit/a191948b758ab4e06a19eef748f16a5f458fe477))
    - test to check if worktree overrides shared configs ([`b69f219`](https://github.com/Byron/gitoxide/commit/b69f21997bac7751e879608fe5b0ba08814aab4d))
    - remove `canonicalize` calls from test… ([`bea689a`](https://github.com/Byron/gitoxide/commit/bea689a97a8e42a92af7f77f7d8706cd96c6dc10))
    - fix type - prevent creating a fixture archive ([`33992ab`](https://github.com/Byron/gitoxide/commit/33992ab6510c65dc97e5eb9565141b977f5b021f))
    - exclude fixture archive from being uploaded… ([`5c2b44c`](https://github.com/Byron/gitoxide/commit/5c2b44c53feae9f23c715dbad962baaf64135963))
    - refactor… ([`ae812bd`](https://github.com/Byron/gitoxide/commit/ae812bde55d55ce06f95ca73513d9749e876ea0e))
    - refactor… ([`394aab9`](https://github.com/Byron/gitoxide/commit/394aab90bdcaa1683b0318e70c455d09b1a7d4cc))
    - remove worktree permission configuration option… ([`7ebf229`](https://github.com/Byron/gitoxide/commit/7ebf229bec6075a149702273c86137f54ef721ed))
    - load worktree config if necessary ([`760e736`](https://github.com/Byron/gitoxide/commit/760e736931c13d155dbbe46459fe11b602084549))
    - add test for worktree configs ([`23d8474`](https://github.com/Byron/gitoxide/commit/23d847480eff1a0d26fd801dfa8ad6ed205c71d4))
    - Release git-hash v0.10.1, git-hashtable v0.1.0 ([`7717170`](https://github.com/Byron/gitoxide/commit/771717095d9a67b0625021eb0928828ab686e772))
    - Merge branch 'remove-lines-parsing' ([`9d8e32d`](https://github.com/Byron/gitoxide/commit/9d8e32d3c276fec34e3fce0feb29de0d24a8d1d2))
    - environment variable permissions are per topic. ([`5fe6aa3`](https://github.com/Byron/gitoxide/commit/5fe6aa3f3f2f81d84f0d96e874e68a8bf4de1db1))
    - make fmt ([`747008d`](https://github.com/Byron/gitoxide/commit/747008d9d370844574dda94e5bec1648c4deb57e))
    - switch from `atty` to `is-terminal` ([`7304bc1`](https://github.com/Byron/gitoxide/commit/7304bc1c0efaad64a39520962072343ef02f6c25))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - represent object cache configuration like `GITOXIDE_PACK_CACHE_MEMORY` in git-configuration. ([`becbd8d`](https://github.com/Byron/gitoxide/commit/becbd8d896a1663f1607be4e86e632773e926f1f))
    - represent `GIT_(COMMITTER|AUTHOR)_(NAME|EMAIL|DATE)` with git configuration. ([`a4ac9cf`](https://github.com/Byron/gitoxide/commit/a4ac9cf3e667a3059e33aac8188150529578622d))
    - `open::ReplacementObjects` is removed in favor of two custom git-configuration flags. ([`49f39d6`](https://github.com/Byron/gitoxide/commit/49f39d6bb487c0254176a5082f2c7851b83952a1))
    - apply related environment variables as config overrides ([`9441c26`](https://github.com/Byron/gitoxide/commit/9441c261bcae61d1d1e674b5e783f38b0471be29))
    - add `permissions::Environment::http_transport`. ([`fc64693`](https://github.com/Byron/gitoxide/commit/fc64693d5af0fda402c560d10d15652c24d14219))
    - refactor ([`603f341`](https://github.com/Byron/gitoxide/commit/603f341e71c021bcc0f154c2ce6c39f4e6546c12))
    - `open::Options::modify()` as general pattern to allow builder methods usage in `&mut self`. ([`0ce29a9`](https://github.com/Byron/gitoxide/commit/0ce29a965cf16273cf74bd22e40f464e322e2f62))
    - remove `SnapshotMut::apply_cli_overrides()` in favor of `open::Options::cli_overrides()`. ([`f16e361`](https://github.com/Byron/gitoxide/commit/f16e36168cc93768ba5d53c9848ff2e8432d06b1))
    - Merge branch 'optimize_hashtables' ([`95ad56c`](https://github.com/Byron/gitoxide/commit/95ad56c11489bc46d6eb2b2f48cf0bf01e954c58))
    - use newly added git-hashtable ([`50cb436`](https://github.com/Byron/gitoxide/commit/50cb4362010e1a5799fe782df36ac5fcdb48dd8a))
    - Merge branch 'path-normalize' ([`805329a`](https://github.com/Byron/gitoxide/commit/805329a0a5f6543bbc1d5885977b47bf7baa7f08))
    - switch to custom Hasher implementation ([`269d59e`](https://github.com/Byron/gitoxide/commit/269d59e0bee1f072096667b143800a0d85b18403))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - more faithfully parse http.followRedirect ([`b84ae6a`](https://github.com/Byron/gitoxide/commit/b84ae6a94082876bfc23cda167aabea88fda67be))
    - adjust for changes in `git-path` ([`cf25e35`](https://github.com/Byron/gitoxide/commit/cf25e3594b99909defb431f34fb3a4d8a25bd37c))
    - thanks clippy ([`10f4f21`](https://github.com/Byron/gitoxide/commit/10f4f2149830734cc551ea96a3d087f07d43fe29))
    - Allow remote overrides for http options ([`340dcad`](https://github.com/Byron/gitoxide/commit/340dcad91832668bc1b570f35714178aa2c53ece))
    - more type-safety for remote names by making clear they can be named after URLs. ([`84d594c`](https://github.com/Byron/gitoxide/commit/84d594caf3f04f1ce337e455343278a6f4674957))
    - Add `Repository::commit_as(committer, author, …)` convenience method. ([`8482f90`](https://github.com/Byron/gitoxide/commit/8482f90d0a2b61259cd51ca4f40ce322e388cb34))
    - upgrade to `prodash 21.1` and add `Ids` to all progress instances. ([`c8835c6`](https://github.com/Byron/gitoxide/commit/c8835c6edae784c9ffcb69a674c0a6545dbb2af3))
    - Merge branch 'http-config' ([`a4ff140`](https://github.com/Byron/gitoxide/commit/a4ff140a0d3607cf282c49228c1248bd36d464fd))
    - `config::CommitAutoRollback` now implements `DerefMut`. ([`363ac7a`](https://github.com/Byron/gitoxide/commit/363ac7a74ec841505b5fc7cc1b8fae11c0a63ea9))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - make fmt ([`0abab7d`](https://github.com/Byron/gitoxide/commit/0abab7da2ec1b8560e6c1eb009f534c9fc7814fe))
</details>

## 0.29.0 (2022-11-21)

<csr-id-f302fc1bcd06fadccd126f4f5f9c0165afabedda/>

### New Features

<csr-id-ff9e1571b558475e727dc6ba11dab24ef15fb6f4/>

 - <csr-id-3ddbd2de369b521fa3f21935f10fe9c248840893/> Make `reqwest` TLS backend configuration easy.
   We provide the choice of `native-tls` or `rust-tls`. If none is
   provided, the user can configure on their on similar to how it's done
   in `git-repository`.
   
   Please note that a choice now has to be made or HTTPS will not be
   available, so use one of…
   
   * blocking-http-transport-reqwest-rust-tls
* blocking-http-transport-reqwest-native-tls

### Bug Fixes

 - <csr-id-c6a690219915b2b401d2d11f61db35b2931e5b3a/> `git_repository::Commit::describe()` chooses tag names (more) correctly.
   Previously, if there were multiple choices for tags on the same commit,
   `git describe` would disagree with `gitoxide` due to different
   prioritization of names.
   
   This has now been fixed.
 - <csr-id-84ed89c3bf6692f18c4bb97173527de1bcba7ac6/> also sort entries lexicographically

### Other

 - <csr-id-f302fc1bcd06fadccd126f4f5f9c0165afabedda/> Set GIT_EDITOR in make_rebase_i_repo.sh
   If the user has core.editor set in their global git config, then that value
   takes precidence over the EDITOR environment variable. The GIT_EDITOR
   environment variable, however, has higher precidence than core.editor. For
   this test, using GIT_EDITOR ensures that the desired sed command line is
   used.

### New Features (BREAKING)

 - <csr-id-bc2a399f2fbb69d23b0b05e8dfb95f3c64ff93b9/> rename `blocking-http-transport` feature to `blocking-http-transport-curl`; add `blocking-http-transport-reqwest`.
   With the new and relatively immature second tier http backend we pave
   the way to support builds without the use of open-ssl and probably many
   other C libraries.
   
   Note that it's early and not `http` configuration option is implemented
   yet.
 - <csr-id-3d8fa8fef9800b1576beab8a5bc39b821157a5ed/> upgrade edition to 2021 in most crates.
   MSRV for this is 1.56, and we are now at 1.60 so should be compatible.
   This isn't more than a patch release as it should break nobody
   who is adhering to the MSRV, but let's be careful and mark it
   breaking.
   
   Note that `git-features` and `git-pack` are still on edition 2018
   as they make use of a workaround to support (safe) mutable access
   to non-overlapping entries in a slice which doesn't work anymore
   in edition 2021.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 29 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#606](https://github.com/Byron/gitoxide/issues/606)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#606](https://github.com/Byron/gitoxide/issues/606)**
    - `git_repository::Commit::describe()` chooses tag names (more) correctly. ([`c6a6902`](https://github.com/Byron/gitoxide/commit/c6a690219915b2b401d2d11f61db35b2931e5b3a))
 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Make `reqwest` TLS backend configuration easy. ([`3ddbd2d`](https://github.com/Byron/gitoxide/commit/3ddbd2de369b521fa3f21935f10fe9c248840893))
    - Merge branch 'max-pure' ([`03ff188`](https://github.com/Byron/gitoxide/commit/03ff1882f2982fba38fbbf245eea13ef9df50f33))
    - rename `blocking-http-transport` feature to `blocking-http-transport-curl`; add `blocking-http-transport-reqwest`. ([`bc2a399`](https://github.com/Byron/gitoxide/commit/bc2a399f2fbb69d23b0b05e8dfb95f3c64ff93b9))
    - Merge branch 'jpgrayson/main' ([`72abac6`](https://github.com/Byron/gitoxide/commit/72abac68055947d3ff3fb4443f29da14a389e45d))
    - Merge branch 'breadthfirst-improvements' ([`b755b5b`](https://github.com/Byron/gitoxide/commit/b755b5bd4cbf8839ba43a143183ae785584f1d59))
    - improve docs for breadthfirst traversal - talking about sorting seems odd ([`6dc3ec1`](https://github.com/Byron/gitoxide/commit/6dc3ec1936b8c74e162e95a5aa1ff0a0d13e6fc8))
    - Set GIT_EDITOR in make_rebase_i_repo.sh ([`f302fc1`](https://github.com/Byron/gitoxide/commit/f302fc1bcd06fadccd126f4f5f9c0165afabedda))
    - add `Repository::empty_tree()` to obtain the empty tree object. ([`ff9e157`](https://github.com/Byron/gitoxide/commit/ff9e1571b558475e727dc6ba11dab24ef15fb6f4))
    - Merge branch 'cwd-consistency' ([`ea7c6a3`](https://github.com/Byron/gitoxide/commit/ea7c6a3b069c9e13905b51b87538c57ba9182dca))
    - Adapt to changes in `git-discover` and `git-path` and `git-odb` ([`98c2501`](https://github.com/Byron/gitoxide/commit/98c250175a39598b9d37613c43dda2299da8eff3))
    - Merge branch 'pierrechevalier83/main' ([`a5b1d73`](https://github.com/Byron/gitoxide/commit/a5b1d738d23d0a343bee1b72bcb72250b5fdae11))
    - restore original representation of `Tag` at the cost of some duplication ([`dd0a23d`](https://github.com/Byron/gitoxide/commit/dd0a23d710be0eb6c7ea7f883aeb1400bcbc0709))
    - refactor ([`c02a6bd`](https://github.com/Byron/gitoxide/commit/c02a6bdcc3669a48cd4b5b640280701fd089575d))
    - stabilize tests (unwrap() -> ?) and improve fixture ([`d4f58a9`](https://github.com/Byron/gitoxide/commit/d4f58a941e3936fb2f11ec66b75156e1b9120fa2))
    - [refactor] Deduplicate Tag and TagRef ([`6003fa2`](https://github.com/Byron/gitoxide/commit/6003fa22085b5031565c51b2b5a0a9feb1579fb0))
    - add additional tests ([`5b97d1b`](https://github.com/Byron/gitoxide/commit/5b97d1b8c787927fba246647427915fa2ca9dd4e))
    - Sort like described in the comment ([`dfe125e`](https://github.com/Byron/gitoxide/commit/dfe125edb9ba15ec4b44155ac0028c44ba0bdb1f))
    - rename tuple fields to what they actually are (without changing logic) ([`3177b2b`](https://github.com/Byron/gitoxide/commit/3177b2bf3f7ee9185d3afab05e50cd25e9561127))
    - refactor ([`cf523cd`](https://github.com/Byron/gitoxide/commit/cf523cdaee36ea084826660ba0605dd5107cfe1f))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - also sort entries lexicographically ([`84ed89c`](https://github.com/Byron/gitoxide/commit/84ed89c3bf6692f18c4bb97173527de1bcba7ac6))
    - curl can authenticate the proxy now and store or reject credentials. ([`63b9050`](https://github.com/Byron/gitoxide/commit/63b9050240b80c5493dab3e8d0b1c675f83d78d6))
    - Pass along the action to kick off the proxy-authentication as well ([`ae74985`](https://github.com/Byron/gitoxide/commit/ae74985b84134795cad0fa88e3fbe9ca776ffa9a))
    - configure the http proxy configuration method if needed ([`6b2d18e`](https://github.com/Byron/gitoxide/commit/6b2d18eb8da09ee6d209c9dbccd02dc0df62a967))
    - Support for reading `http.proxyAuthMethod` ([`92f88c9`](https://github.com/Byron/gitoxide/commit/92f88c94ff288b5675ca3296c27ffb66e1716c22))
</details>

## 0.28.0 (2022-11-17)

<csr-id-6beb6f263fd40884b440092f39034dd43d3a95de/>

### New Features

 - <csr-id-58e14884b1d025651f874d899cb2d627c4a2afbf/> `Id` implements `std::fmt::Display`
 - <csr-id-25f7aabe38267b6b6c0547806028b2becb806416/> `Remote::repo()` to obtain the underlying repository.
   For convenience.
 - <csr-id-709a73229b7cde56ddffa099158661632c606468/> Support for user-costomizable user agent strings.
   Doable by setting the `gitoxide.userAgent` variable.
 - <csr-id-e60d07997989993216c2bd93efeb6f1b48da0a87/> add `env::agent()` for obtaining the default client agent string.

### Other

 - <csr-id-6beb6f263fd40884b440092f39034dd43d3a95de/> try to apply maybe-async in a place where it's probably not possible.
   The goal is to re-use the existing tests, but right now they only
   compile in async mode as the `maybe-async` crates needs
   a feature to be set. Doing so is hard(er) if it's not already used
   in the main crate, which we do not and will do our best to avoid.

### New Features (BREAKING)

 - <csr-id-db9040f0bb3a16879c8da0252a77df80bd417387/> add `remote::Connection::with_transport_config()`, change the way `*::transport_mut()` is used.
   Previously `transport_mut()` was supposed to be used for calling
   `configure()`, but that doesn't work anymore as `configure()` can
   only effectivey be called once the initialization of the Connection
   is complete, as it may depend on the Remote name AND the credential
   provider for proxy auth credential acquisition.
   
   Thus we allow callers to set the transport options they need in advance
   for it to be used when needed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 53 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Merge branch 'http-config' ([`665b53e`](https://github.com/Byron/gitoxide/commit/665b53e1c2e1de65fafa28b669f58977868bbc81))
    - fix docs ([`b5c316e`](https://github.com/Byron/gitoxide/commit/b5c316e285369a84e57ec6f7425b92fec2978a49))
    - adapt to changes in `git-protocol` ([`bd70847`](https://github.com/Byron/gitoxide/commit/bd70847651577feb9b0bdf4e91afaffbcd212ff5))
    - adapt to changes in `git-protocol` ([`c421187`](https://github.com/Byron/gitoxide/commit/c42118771b2fba2ad135b00c2bf1e338e81ac2e0))
    - `Id` implements `std::fmt::Display` ([`58e1488`](https://github.com/Byron/gitoxide/commit/58e14884b1d025651f874d899cb2d627c4a2afbf))
    - Assure clones write their refs into packed-refs right away. ([`25dcae7`](https://github.com/Byron/gitoxide/commit/25dcae7883691b014f9045cf9b8fc939281a127a))
    - fix warnings ([`8eec815`](https://github.com/Byron/gitoxide/commit/8eec8159452f590850c6963170e12f1e80efc45e))
    - use convenience traits everywhere when applying leniency ([`9ff64bb`](https://github.com/Byron/gitoxide/commit/9ff64bbb67dd55f2dfa8cf8a316444c9c826f2e0))
    - refactor ([`db7ad53`](https://github.com/Byron/gitoxide/commit/db7ad53a2ac54dc68c0153a8a6aef0dfc87f2fa4))
    - Make application of lenient configuration values way easier and nicer to read ([`a59c791`](https://github.com/Byron/gitoxide/commit/a59c791da30bf4ef7d5a9c1daf270132fea21636))
    - refactor ([`b5ca8a6`](https://github.com/Byron/gitoxide/commit/b5ca8a6c4841da14c14a5b9b06dc6f796cacbd74))
    - Also read the connectTimeout in simple HTTP options ([`e055617`](https://github.com/Byron/gitoxide/commit/e05561782c3ea85dfe4e7136efe2ff73336e9336))
    - keep track of `no_proxy` environment variable support ([`f0625de`](https://github.com/Byron/gitoxide/commit/f0625de13073de4767881ed0398d0cd2791b0ad2))
    - Add proxy-prefix and explicitly allow empty proxy values ([`70303c1`](https://github.com/Byron/gitoxide/commit/70303c139825143cf17004086a374c69c9d55949))
    - Empty proxies can disable the proxy; cleanup test fixture, let it have its own ([`21f3283`](https://github.com/Byron/gitoxide/commit/21f328352b4a7b97a58233eba4dff824ac8ed29f))
    - Merge branch 'main' into http-config ([`f4ff821`](https://github.com/Byron/gitoxide/commit/f4ff821fd4233dd1dc1a449af4d4600becf3b4ac))
    - Merge branch 'async-fetch' ([`0c9c48b`](https://github.com/Byron/gitoxide/commit/0c9c48b3b91a1396eb1796f288a2cb10380d1f14))
    - remove blocking-only tests in favor of tests that test blocking and async implementations ([`7c4dd21`](https://github.com/Byron/gitoxide/commit/7c4dd218c25f20c5bfd1b9c7ac66e6cee83e08a9))
    - remove optional `blocking` dependency as it's not going to be used for now. ([`9b5f0eb`](https://github.com/Byron/gitoxide/commit/9b5f0eb06aa32ce3dd77781e85525a065525a7eb))
    - Add last remaining test to validate entire packs can be fetched in async mode without issues. ([`5cc3087`](https://github.com/Byron/gitoxide/commit/5cc3087cea747369434eeadf95dccdf07ffadca2))
    - Another test seems to be working ([`01e99b4`](https://github.com/Byron/gitoxide/commit/01e99b493c34163a80f52556f7b0993a14aa74db))
    - the first working test ([`ce84fb3`](https://github.com/Byron/gitoxide/commit/ce84fb3c049760464bf0df4f5ed246b2ef7cc9a8))
    - remove `futures-executor` in favor of `futures-lite::future::block_on` ([`2cd28ee`](https://github.com/Byron/gitoxide/commit/2cd28ee5f789fca1f7b443dd24035b52d91989f3))
    - Prepare a first test for receiving an empty pack, but… ([`7471ab3`](https://github.com/Byron/gitoxide/commit/7471ab3cc4e6dd1fb8c9645cfd84dda5cd3618a5))
    - thanks clippy ([`854ca68`](https://github.com/Byron/gitoxide/commit/854ca6853ed8dfcc0241c8bdbe9576e59cd70c68))
    - Share all code when performing a ref-map test ([`886c017`](https://github.com/Byron/gitoxide/commit/886c0178c6f5b09be07af80d67284f177d65869c))
    - actually get maybe-async to work! ([`f3a6424`](https://github.com/Byron/gitoxide/commit/f3a64240bfd675f241c3d40273a928ed6841f1a6))
    - the first simple test to validate we can connect. ([`2bf860a`](https://github.com/Byron/gitoxide/commit/2bf860acb29694e2a00e9d4f0815de4ed1c35209))
    - try to apply maybe-async in a place where it's probably not possible. ([`6beb6f2`](https://github.com/Byron/gitoxide/commit/6beb6f263fd40884b440092f39034dd43d3a95de))
    - `Remote::repo()` to obtain the underlying repository. ([`25f7aab`](https://github.com/Byron/gitoxide/commit/25f7aabe38267b6b6c0547806028b2becb806416))
    - fix build warnings ([`32b1ba9`](https://github.com/Byron/gitoxide/commit/32b1ba92a9f91229c1996ec0a86b2f923d804135))
    - leniency for all UTF-8 conversion failures ([`1b53efb`](https://github.com/Byron/gitoxide/commit/1b53efb7ee80b9bf14843e5426c096e0921f7a53))
    - support for handling of illformed UTF-8 ([`4a29331`](https://github.com/Byron/gitoxide/commit/4a293311d098ae3d951a882814ebc72cf2d1c0ad))
    - lenient support for all values that could previously fail ([`d302c67`](https://github.com/Byron/gitoxide/commit/d302c67071713b9b855b2ba4718b3408ec618221))
    - refactor ([`e93768b`](https://github.com/Byron/gitoxide/commit/e93768bfa8357fa01cfdfee86c8c911c9cc64bf6))
    - Currently http transport is only available for blocking io ([`1236cf2`](https://github.com/Byron/gitoxide/commit/1236cf2fdd00cdd8b0c331cae22aa7e649a2a73c))
    - thanks clippy ([`1553308`](https://github.com/Byron/gitoxide/commit/1553308bc112f8e5974123b41fcb04b586c9ea7f))
    - add `remote::Connection::with_transport_config()`, change the way `*::transport_mut()` is used. ([`db9040f`](https://github.com/Byron/gitoxide/commit/db9040f0bb3a16879c8da0252a77df80bd417387))
    - extra-headers respects empty entries to clear the list ([`9707f7f`](https://github.com/Byron/gitoxide/commit/9707f7f23ce683f8f04e2d18e15fecc9e8f69cf8))
    - adjust for changes in `git-transport` ([`ef64395`](https://github.com/Byron/gitoxide/commit/ef64395d23f4a2816ae41ca123dd4cd880c78af1))
    - First simple-http optiosn test passing ([`585047b`](https://github.com/Byron/gitoxide/commit/585047b3f353ca8781bc938803c5056686bb1305))
    - refactor ([`0ced3a4`](https://github.com/Byron/gitoxide/commit/0ced3a4c8e6e01870d1b603738aa1af4b8947dc8))
    - refactor ([`e3a24e6`](https://github.com/Byron/gitoxide/commit/e3a24e6f3b9e9a2e22c48fc3ebf8c6cc9ca36603))
    - implement a couple of http values, needs some refactoring ([`a44c9ea`](https://github.com/Byron/gitoxide/commit/a44c9ea0a0fc0285607454951303792c83dff4b9))
    - fix docs ([`d4089e7`](https://github.com/Byron/gitoxide/commit/d4089e786d67c10cdf94dddbf0dc2f1b2b0410dc))
    - add missing assertions for simple options ([`9ff70e9`](https://github.com/Byron/gitoxide/commit/9ff70e9c7b1838738dbcd3e1a17e9088670aebb6))
    - first step for basic test of simple http configuration ([`21bd85d`](https://github.com/Byron/gitoxide/commit/21bd85da23d3de1ac4dbc798ef6b3a8cf00a15a7))
    - fix build ([`2ef0e09`](https://github.com/Byron/gitoxide/commit/2ef0e09f3889f5493794550482e07709455c7f21))
    - Support for user-costomizable user agent strings. ([`709a732`](https://github.com/Byron/gitoxide/commit/709a73229b7cde56ddffa099158661632c606468))
    - add `env::agent()` for obtaining the default client agent string. ([`e60d079`](https://github.com/Byron/gitoxide/commit/e60d07997989993216c2bd93efeb6f1b48da0a87))
    - Merge branch 'main' into http-config ([`7c5b37d`](https://github.com/Byron/gitoxide/commit/7c5b37d28e98f59a6847368a0d0166d2dbb4acc1))
</details>

## 0.27.0 (2022-11-08)

### Changed (BREAKING)

 - <csr-id-c50868c7ed7309515b4f0a188213d332d57dd146/> Move `object::tree::diff::change::DiffPlatform` to `object::blob::diff::Platform`.
 - <csr-id-4ee32713093c2e41a12d148c6030add1df6aa966/> new `DiffPlatform::counts()`, open `DiffPlatform` for use of `git-diff::blob::*`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.22.0, git-index v0.7.1, git-pack v0.26.0, git-odb v0.36.0, git-transport v0.21.2, git-repository v0.27.0, safety bump 6 crates ([`f0cab31`](https://github.com/Byron/gitoxide/commit/f0cab317bb0c2799fa80d16f3ae1b89d6aee4284))
    - prepare changelogs prior to release ([`f5f3a9e`](https://github.com/Byron/gitoxide/commit/f5f3a9edd038a89c8c6c4da02054e5439bcc0071))
    - Merge branch 'fixes-for-crates-index-diff' ([`255be4d`](https://github.com/Byron/gitoxide/commit/255be4ddcd6cbca0a89f286eeecdd19ff70e000f))
    - remove unused import; fix docs ([`efe0a51`](https://github.com/Byron/gitoxide/commit/efe0a51931fc7e42c82563575e3068dd6e401409))
    - plan for more tests for line diffs ([`58934a4`](https://github.com/Byron/gitoxide/commit/58934a468040df45b9ca4062df2c7f245b0c791e))
    - Fix borrowcheck issues by being less specific ([`aff6820`](https://github.com/Byron/gitoxide/commit/aff6820e3a4dbb1f189cb33adb577b7a2b90d109))
    - a hunk based mechanism for line diffs is nearly there, just has lifetime issues ([`b1fc68f`](https://github.com/Byron/gitoxide/commit/b1fc68f27331f0648d117490ac404eeb47b5f15a))
    - Move `object::tree::diff::change::DiffPlatform` to `object::blob::diff::Platform`. ([`c50868c`](https://github.com/Byron/gitoxide/commit/c50868c7ed7309515b4f0a188213d332d57dd146))
    - new `DiffPlatform::counts()`, open `DiffPlatform` for use of `git-diff::blob::*`. ([`4ee3271`](https://github.com/Byron/gitoxide/commit/4ee32713093c2e41a12d148c6030add1df6aa966))
    - keep track of http related configuration keys. ([`1afaebd`](https://github.com/Byron/gitoxide/commit/1afaebdcae977af8a9a0f0788ec754746d6d05bb))
</details>

## 0.26.0 (2022-11-06)

<csr-id-c6f92c15529ddff7539667b74bafa2348f3040e3/>

### New Features

 - <csr-id-b1edb9e3537df86669714f03666f4a88e0ac3709/> diff algorithm is controlled by git configuration `diff.algorithm`
 - <csr-id-072f5bc9c91c4c09bd6a73f9d7ac672805cae533/> Query of `core.logAllRefUpdates` is now fallible.
   This is the same behaviour as shown by `git`, which requires valid
   values or aborts.
 - <csr-id-2eaf69e5f8f8da10e5af85cb9f0c39577ad1707f/> automatically handle `.keep` files after writing a pack bundle to disk.
   The logic implemented here tries to do the right thing, that is when
   we have reason to believe that the objects in the pack are linked up
   with a ref, we delete the keep file automatically.
   
   However, if there was no local ref edit as the ref specs didn't contain
   local destinations, or if the pack was empty, then keep the .keep file
   and leave it to the caller to handle.
 - <csr-id-8b9fbd4e9ed7be37976c7203cd9a89c6116a6d3d/> Use `core.askpass` when building the credential helper.
   Previously it would only consider the environment variable, which can
   still override the provided git-configuration at core.askpass .
 - <csr-id-a9d14492322785a14f4ecb5b0d3dbdc87e56f8c5/> `remote::fetch::Prepare::handshake_outcome()` to obtain server information right after listing refs.
 - <csr-id-0b5c53ec43bdb58b2b7cf46e453ddf858770a95a/> `open::Options::config_overrides()` for early configuration; support for `init.defaultBranch`.

### Bug Fixes

 - <csr-id-f869b224170b0c49a0e4d89e88bfbf5caedaa725/> don't allow non-bare repositories to be initialized into non-empty directories.
 - <csr-id-91baefad02a0d52c745106359da3693d06aace46/> `init_bare()` now creates the destination directory if it doesn't exist.
 - <csr-id-5c11b84f4e74e3eefdd0f5804976ebfc505e0f2f/> build correct path for `$HOME/.config/…` files.
   The special per-user `ignore` and `attributes` files can also be
   defaulted if some environment variables are set and may be accessed.
   
   Previously the default for `$HOME` was incorrect, as it was missing the
   intermediate `.config/` directory. This is now present to build paths
   exactly like git.
 - <csr-id-275e80f3d602b63ef91efe31a92b4aafb2eeca44/> ref-map filtering now uses correct prefixes.
   Previously specs could get filtered out server-side as a matching prefix
   was entirely missing.

### Changed (BREAKING)

 - <csr-id-449ff066d2b5dd423c639618193dd9e54d03c1f8/> `Repository::branch_remote_name()` returns `reference::remote::Name`.
   That way it's made clear the remote can also be a URL, while rejecting
   illformed UTF8. The latter isn't valid for remote names anyway as these
   only support a very limited character set.
   
   Note that this error currently is degenerated, making it appear if the
   remote name doesn't exists if illformed UTF-8 is found in what appears
   to be a symbolic ref.
 - <csr-id-71f15fc46fbaea455cf84a2b4cfe3e680047d790/> be specific about the kind of `diff::Error`, moving it to `diff::for_each::Error`.

### New Features (BREAKING)

 - <csr-id-7413a284eb7754e63ba45d0f526347b9f79b557d/> `Tree::lookup_entry*()` returns attached `Entry` type.
   That way chaining gets even easier.

### Bug Fixes (BREAKING)

 - <csr-id-2bece79285e244a7029f9393dafc990e39420e2d/> `create::into(…)` takes `create::Kind` to determine if it's bare or not.
   First of all, `bare` is not an option anymore, but a parameter as
   it can't be defaulted.
   Other public signatures change as well to accomodate for it.

### Other (BREAKING)

 - <csr-id-c6f92c15529ddff7539667b74bafa2348f3040e3/> `DiffPlatform::text()` to `*::lines()`.
   This is more specific as one could also do character changes in a single
   line, and it adapts the signature to the new `imra-diff` powered
   mechanism, for a 2x speed boost.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 114 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 12 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - make last test work to allow us to clone properly ([`3890f1a`](https://github.com/Byron/gitoxide/commit/3890f1a804a3f8b3f952a38fffc6e6bd6034164d))
    - initail implementation of writing branch tracking information ([`e2a5714`](https://github.com/Byron/gitoxide/commit/e2a57146b4399c1447cf219e362deb5c3016a5bc))
    - adapt to changes in `git-protocol` ([`d61eb2c`](https://github.com/Byron/gitoxide/commit/d61eb2c9f6f2f49ffb903179f793b126471347a5))
    - Don't deviate by creating strange reflogs (with null-source & null-destination) ([`f1b5570`](https://github.com/Byron/gitoxide/commit/f1b5570629d8963aff961d252a9578277484adee))
    - Support unborn remotes and pick up their default branch name. ([`619fd61`](https://github.com/Byron/gitoxide/commit/619fd6105b41d31ea5125151f98fae1299b179f5))
    - adapt to changes in `git-protocol` ([`179ccd7`](https://github.com/Byron/gitoxide/commit/179ccd7d3e7777b7c72bd5e7ab7d045c9c4c1b98))
    - prepare test for handling the 'unborn' lsrefs extension ([`547e450`](https://github.com/Byron/gitoxide/commit/547e450e6afc4528fc4ab1e422dbe33fdd14b885))
    - fix: support for proper identification of '.' remote paths in `reference::remote::Name` ([`b219033`](https://github.com/Byron/gitoxide/commit/b219033960026615054f19eb0f643e4701fcc3d0))
    - `Repository::branch_remote_name()` returns `reference::remote::Name`. ([`449ff06`](https://github.com/Byron/gitoxide/commit/449ff066d2b5dd423c639618193dd9e54d03c1f8))
    - failing test for us setting up remote information after cloning ([`07efbce`](https://github.com/Byron/gitoxide/commit/07efbce5f2fff21d43c709ec91d445d60a17918c))
    - update docs ([`c788b51`](https://github.com/Byron/gitoxide/commit/c788b51e73f41c95058c56add378e9b65f342322))
    - refactor ([`7bd5263`](https://github.com/Byron/gitoxide/commit/7bd5263e3302c7989d599792c45ef2768366582b))
    - definitely write all non-symbolic refs into packed-refs ([`b62d5b4`](https://github.com/Byron/gitoxide/commit/b62d5b4e6cf1ae6051203ba243e24c03edafcd6b))
    - auto-pack references when creating them during clone ([`a9a621e`](https://github.com/Byron/gitoxide/commit/a9a621e117e5bbe38368bd6f0cc21db1365b7e18))
    - refactor ([`728f688`](https://github.com/Byron/gitoxide/commit/728f688f95778f4d3d182f5e9db90ebe93b4f65b))
    - use `tempfile` via `git-testools` ([`2c9edff`](https://github.com/Byron/gitoxide/commit/2c9edffad34245e6a0d2972cb66c849435caec92))
    - failing test to show we don't pack refs yet ([`58cc01a`](https://github.com/Byron/gitoxide/commit/58cc01a9075f8f8038f0bc1c6876a85f25ff4712))
    - sort out last test-case to assure setting symbolic refs is safe ([`d06900d`](https://github.com/Byron/gitoxide/commit/d06900dd2e5782ab1a72dee8f20df5c660ba0d6d))
    - adjust to changes in `git-ref` ([`669249a`](https://github.com/Byron/gitoxide/commit/669249a88bf1e0074a0e9479688473b79e60db9c))
    - assure that inital refs are placed into the correct spot ([`3e4c0cb`](https://github.com/Byron/gitoxide/commit/3e4c0cb1b88b7c3a26e24a1b6f68fa9ac5076bcb))
    - and show that we don't manage to write the reflog for some reason ([`a70a6aa`](https://github.com/Byron/gitoxide/commit/a70a6aa698adedc86a63c28ccebb89c8c67cdfe0))
    - the first successful test showing that we can write symbolic refs ([`2330305`](https://github.com/Byron/gitoxide/commit/2330305153b467b37956b2d9f1ca211506b96489))
    - figure out more details on how to handle symbolic refs just enough ([`86a18bd`](https://github.com/Byron/gitoxide/commit/86a18bd3923d6f6e9a84c9b5f54e2456113c7b18))
    - finally figure out how symbolic ref updates should work ([`376829a`](https://github.com/Byron/gitoxide/commit/376829a4c86ea2c84a35c20b62c78868feb18993))
    - first step towards supporting writing of symbolic refs locally ([`61cd430`](https://github.com/Byron/gitoxide/commit/61cd4303a2a5818da969266b1490000db31d51e6))
    - failing test to check for presence of remote HEAD ([`2f649e9`](https://github.com/Byron/gitoxide/commit/2f649e95c988947dca21c4cf0ccacc5d7d9a5406))
    - thanks clippy ([`767fb7b`](https://github.com/Byron/gitoxide/commit/767fb7b20c922fd8e5477f28adf9de0419d3ac96))
    - Use correct diff algorithm when diffing text ([`6fe93c2`](https://github.com/Byron/gitoxide/commit/6fe93c2473c6ecb673922344f55d13637092be22))
    - refactor ([`71c6a20`](https://github.com/Byron/gitoxide/commit/71c6a203eeaf8ec58ab8385d1df73ca2daaea013))
    - be specific about the kind of `diff::Error`, moving it to `diff::for_each::Error`. ([`71f15fc`](https://github.com/Byron/gitoxide/commit/71f15fc46fbaea455cf84a2b4cfe3e680047d790))
    - initial support for obtaining the `diff.algorithm` lazily ([`f362ab2`](https://github.com/Byron/gitoxide/commit/f362ab221765c6fab20e9c88bbfda20d6da64216))
    - refactor ([`af0c28d`](https://github.com/Byron/gitoxide/commit/af0c28d95b51acebd31cf707ee69ed727552d571))
    - proper reflog entries for all other updated refs during cloning ([`ff4412e`](https://github.com/Byron/gitoxide/commit/ff4412e8580e2e47150dd2ba41a347964128486d))
    - show that HEAD's referent also has the correct reflog ([`c25cb00`](https://github.com/Byron/gitoxide/commit/c25cb007ea5d8757a08e6579e17ae49f296f16b8))
    - the first test to prove that HEAD reflogs are correct ([`1e7fd4e`](https://github.com/Byron/gitoxide/commit/1e7fd4e35a3f28bee6ca091f5a9d47fd081670cb))
    - canonicalize URL right away for it to manifest in the changelog ([`3cfe13d`](https://github.com/Byron/gitoxide/commit/3cfe13d1edb01604772825ee8de6937b97165243))
    - Query of `core.logAllRefUpdates` is now fallible. ([`072f5bc`](https://github.com/Byron/gitoxide/commit/072f5bc9c91c4c09bd6a73f9d7ac672805cae533))
    - Fully reload in-memory configuration after configuration changes… ([`bc5b4e7`](https://github.com/Byron/gitoxide/commit/bc5b4e77bc66317b2f5fc49a60f4f9dcd3d46037))
    - allow to re-read the logallrefupdates config after overrides ([`ff06de4`](https://github.com/Byron/gitoxide/commit/ff06de420661d30ff0a97b4be53dea92e746052e))
    - re-apply overrides more correctly after clone, however… ([`372e9d4`](https://github.com/Byron/gitoxide/commit/372e9d41bb6a521690c66f8cb989172034500c70))
    - validate that the remote HEAD branch overrides local init.defaultBranch settings ([`1c3dd3a`](https://github.com/Byron/gitoxide/commit/1c3dd3ae3d2ec599418e96362146064724808db9))
    - assure stored file urls are absolute ([`5d7a055`](https://github.com/Byron/gitoxide/commit/5d7a05510922148bb7c9fe2fd172fd577684b2a4))
    - make it possible to clone empty remote repositories ([`e97eeda`](https://github.com/Byron/gitoxide/commit/e97eeda45c9cc0736273c735a9959ac1ff29fc9d))
    - refactor ([`d29bb62`](https://github.com/Byron/gitoxide/commit/d29bb6215b1a824a1811be8da84816954234f4e4))
    - test for cloning empty repositories ([`0aa97fe`](https://github.com/Byron/gitoxide/commit/0aa97fea17c9cd08b21e65ff6447527357d10c0c))
    - checkout returns index checkout result ([`2ef8d53`](https://github.com/Byron/gitoxide/commit/2ef8d53e57dfe0590899c4d3bf9bf777fccd8491))
    - avoid showing thread progress during clone-pack-resolution ([`056f4dd`](https://github.com/Byron/gitoxide/commit/056f4ddb21f92a098499cadc8711438e9ecae031))
    - `create::into(…)` takes `create::Kind` to determine if it's bare or not. ([`2bece79`](https://github.com/Byron/gitoxide/commit/2bece79285e244a7029f9393dafc990e39420e2d))
    - less noisy way of writing trait bounds ([`b593806`](https://github.com/Byron/gitoxide/commit/b593806ca3571d680801130ad528f266d3eab83e))
    - upgrade to `prodash` v21 ([`a0655dc`](https://github.com/Byron/gitoxide/commit/a0655dc7bc5dff388bc69a648e7f16b44fd1abd9))
    - don't allow non-bare repositories to be initialized into non-empty directories. ([`f869b22`](https://github.com/Byron/gitoxide/commit/f869b224170b0c49a0e4d89e88bfbf5caedaa725))
    - assure the reflog settings aren't permanently overidden during init/fetch ([`bc5e3e4`](https://github.com/Byron/gitoxide/commit/bc5e3e4c00daf37491d48ad2e575f58065b00966))
    - Make it possible to ignore specs that don't match when iterating mappings. ([`bc991ff`](https://github.com/Byron/gitoxide/commit/bc991ff5b7a1c6c1b107da3b61b955e583923658))
    - `init_bare()` now creates the destination directory if it doesn't exist. ([`91baefa`](https://github.com/Byron/gitoxide/commit/91baefad02a0d52c745106359da3693d06aace46))
    - first rough sketch of `gix clone` ([`23a5e8b`](https://github.com/Byron/gitoxide/commit/23a5e8b658c5642c3f3060e013fd0eab06cbf027))
    - the first working checkout as per simple simple test ([`9ce28ac`](https://github.com/Byron/gitoxide/commit/9ce28ac3342a65afb96c006d7d2fa70fae80c2dc))
    - finally perform actual checkout, but test fails without clear reason ([`3821b4f`](https://github.com/Byron/gitoxide/commit/3821b4fb2d22e7b447ca3c11ae1ba9c6897916cd))
    - prepare attribute-group setup as far as possible. ([`f5e2eeb`](https://github.com/Byron/gitoxide/commit/f5e2eebe9560f664f044b82ffa0cd19fd0df311f))
    - build correct path for `$HOME/.config/…` files. ([`5c11b84`](https://github.com/Byron/gitoxide/commit/5c11b84f4e74e3eefdd0f5804976ebfc505e0f2f))
    - sketch access to the attributes file, realize that there is an issue to be fixed first ([`0081e2f`](https://github.com/Byron/gitoxide/commit/0081e2f62185ee874b4e6927afbf33fe5ca37c46))
    - refactor ([`28de9df`](https://github.com/Byron/gitoxide/commit/28de9dfd859376fd72bf3a0446bfa3457acf88f2))
    - read core.checkstat to configure the checkout as well ([`05a666c`](https://github.com/Byron/gitoxide/commit/05a666c50e270c30d47fe5bfc3195a4fd1f1aea8))
    - refactor ([`2f1c9dc`](https://github.com/Byron/gitoxide/commit/2f1c9dc3ea707e726d1820ec720b70e0d652b797))
    - don't be lenient towards paths that can't be interpolated in case of excludes file ([`3df7788`](https://github.com/Byron/gitoxide/commit/3df7788632871072c4eaa944e93a83040c00f74f))
    - collect all filesystem attributes affecting checkout ([`91b360f`](https://github.com/Byron/gitoxide/commit/91b360f08ee53497441b53491e8629f102c9a80c))
    - prepare to move checkout_options into `config` ([`e731757`](https://github.com/Byron/gitoxide/commit/e731757f85f87f5468ccc6870f5f9af3b7771753))
    - Use `core.askpass` when building the credential helper. ([`8b9fbd4`](https://github.com/Byron/gitoxide/commit/8b9fbd4e9ed7be37976c7203cd9a89c6116a6d3d))
    - obtain worker count from configuration; prep for more options ([`d947d8b`](https://github.com/Byron/gitoxide/commit/d947d8be1f00f2b16d6648389a9ead85f4885d3e))
    - write index from root tree and get ready for checkout ([`485a252`](https://github.com/Byron/gitoxide/commit/485a252b7398b2c77450ff05c91a5d4f8d3c538a))
    - prepare checkout, but needs to be able to create an index from a tree ([`e462bd5`](https://github.com/Byron/gitoxide/commit/e462bd51c0af77cd06b56f189755cc4fa5154139))
    - also update the HEAD reference after a fetch ([`e561021`](https://github.com/Byron/gitoxide/commit/e561021e3332edb12cabbc2b556adf32522e6808))
    - ref-map filtering now uses correct prefixes. ([`275e80f`](https://github.com/Byron/gitoxide/commit/275e80f3d602b63ef91efe31a92b4aafb2eeca44))
    - prepare for handling the server object-format correctly. ([`54c91eb`](https://github.com/Byron/gitoxide/commit/54c91eb66467c0925004ab87a815cbe504542c93))
    - a sketch to check and update the object format upon cloning. ([`ebfd7d6`](https://github.com/Byron/gitoxide/commit/ebfd7d6941c864e880a73bf2dd6298365825d3e1))
    - refactor ([`36c5ca9`](https://github.com/Byron/gitoxide/commit/36c5ca9e921571f47476a5c26986dcc297b589d0))
    - `remote::fetch::Prepare::handshake_outcome()` to obtain server information right after listing refs. ([`a9d1449`](https://github.com/Byron/gitoxide/commit/a9d14492322785a14f4ecb5b0d3dbdc87e56f8c5))
    - refactor ([`992522a`](https://github.com/Byron/gitoxide/commit/992522ad698781eae69b7442c39fa8190159d95a))
    - `open::Options::config_overrides()` for early configuration; support for `init.defaultBranch`. ([`0b5c53e`](https://github.com/Byron/gitoxide/commit/0b5c53ec43bdb58b2b7cf46e453ddf858770a95a))
 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'main' into write-sparse-index (upgrade to Rust 1.65) ([`5406630`](https://github.com/Byron/gitoxide/commit/5406630466145990b5adbdadb59151036993060d))
    - fix tests on windows ([`f2a8a45`](https://github.com/Byron/gitoxide/commit/f2a8a45be80c4d12b1e0e8d8401bce7ff0be5959))
    - thanks clippy ([`04cfa63`](https://github.com/Byron/gitoxide/commit/04cfa635a65ae34ad6d22391f2febd2ca7eabca9))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - make fmt ([`ea2136b`](https://github.com/Byron/gitoxide/commit/ea2136b065979cecb3a1fdbf7b20ed7514128d9a))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - thanks clippy ([`3495c56`](https://github.com/Byron/gitoxide/commit/3495c561841a76686e7a2b363feeab9f6e4bc301))
    - fix build and docs ([`971fe0c`](https://github.com/Byron/gitoxide/commit/971fe0cb437cb1dcd470574d7b7338f0dabd09ac))
    - diff algorithm is controlled by git configuration `diff.algorithm` ([`b1edb9e`](https://github.com/Byron/gitoxide/commit/b1edb9e3537df86669714f03666f4a88e0ac3709))
    - Merge branch 'main' into gix-clone ([`fa27570`](https://github.com/Byron/gitoxide/commit/fa27570f491388cce6137af44330d76870d07202))
    - Merge branch 'imra-diff' ([`f53f942`](https://github.com/Byron/gitoxide/commit/f53f9426f206686b30abd73a201a92b1405e782d))
    - fix docs ([`7d5fb3c`](https://github.com/Byron/gitoxide/commit/7d5fb3c6a0aa6fd696f95912b28b4b007ad1b825))
    - thanks clippy ([`24254a4`](https://github.com/Byron/gitoxide/commit/24254a46961c002a3e9792211541a08c41415ac2))
    - `DiffPlatform::text()` to `*::lines()`. ([`c6f92c1`](https://github.com/Byron/gitoxide/commit/c6f92c15529ddff7539667b74bafa2348f3040e3))
    - thanks clippy ([`d2f56df`](https://github.com/Byron/gitoxide/commit/d2f56df5405f6c27ebf7d51f33381f2c548433fb))
    - Merge branch 'main' into gix-clone ([`3b48317`](https://github.com/Byron/gitoxide/commit/3b48317d6a9f41765d4f2a9f0a49c31afcdb68b6))
    - thanks clippy ([`93e7691`](https://github.com/Byron/gitoxide/commit/93e7691be421e40cc72e3e2e0506584a2fbd4857))
    - automatically handle `.keep` files after writing a pack bundle to disk. ([`2eaf69e`](https://github.com/Byron/gitoxide/commit/2eaf69e5f8f8da10e5af85cb9f0c39577ad1707f))
    - thanks clippy ([`6f8356c`](https://github.com/Byron/gitoxide/commit/6f8356ca12676a9d7045ed28fb4a558f81281caa))
    - thanks clippy ([`700cc2d`](https://github.com/Byron/gitoxide/commit/700cc2decb4388d165ac799c88c3a18b062ff58a))
    - thanks clippy ([`73b6ec0`](https://github.com/Byron/gitoxide/commit/73b6ec0882b3ae9934f49c4c2bb645b54fa26607))
    - thanks clippy ([`f22bdc0`](https://github.com/Byron/gitoxide/commit/f22bdc0360f61460f21eeb212f475ed8724018a8))
    - Merge branch 'main' into gix-clone ([`de4fe06`](https://github.com/Byron/gitoxide/commit/de4fe06202906ea5c62e667826b42cf7b57b1ff0))
    - thanks clippy ([`6ac6580`](https://github.com/Byron/gitoxide/commit/6ac65806202d8cf23c43da706482647fba0a1ce9))
    - thanks clippy ([`c6e7663`](https://github.com/Byron/gitoxide/commit/c6e7663c1b53e0794a19d6e431e9db2ce5fa4cbc))
    - Mark the upcoming usage of init.defaultBranch. ([`6225f35`](https://github.com/Byron/gitoxide/commit/6225f35398bc494ad74da342c4ebbe0487b106f8))
    - realize that we don't know hot to set HEAD correctly just yet ([`11d636c`](https://github.com/Byron/gitoxide/commit/11d636cd3c5478b37a10619644e0cff8923949c4))
    - Merge branch 'fix-gix-index-from-tree' ([`da5f63c`](https://github.com/Byron/gitoxide/commit/da5f63cbc7506990f46d310f8064678decb86928))
    - adjust to changes in `git-index` ([`fa6bcde`](https://github.com/Byron/gitoxide/commit/fa6bcde735792fa10b66dfc7f81588bb68dcf46f))
    - fix docs ([`34b3e03`](https://github.com/Byron/gitoxide/commit/34b3e03a197fd27c2d7b8e5d88c3b5dc627cbca4))
    - sketch of method to checkout the main worktree ([`a88d5a3`](https://github.com/Byron/gitoxide/commit/a88d5a35d683e1da0bacaef54350d5e8047cb8f7))
    - finish sketch of `fetch_and_checkout()` ([`e39a9d5`](https://github.com/Byron/gitoxide/commit/e39a9d59654633736d9933064da9d9e2833892eb))
    - sketch checkout API and refactor ([`9145a32`](https://github.com/Byron/gitoxide/commit/9145a32c1f59d3cb1644b7028f1f7504761d7419))
    - `Tree::lookup_entry*()` returns attached `Entry` type. ([`7413a28`](https://github.com/Byron/gitoxide/commit/7413a284eb7754e63ba45d0f526347b9f79b557d))
</details>

## 0.25.0 (2022-10-10)

<csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/>

### New Features

 - <csr-id-22d3b37ea6239170a478b859361a7d1d7ba01a9a/> `Url::try_from(path: &std::path::Path)` for more convenient instantiation.
 - <csr-id-31a7089f2583832727e2175ada6fb5c30c3beebe/> make some private methods public to give callers more flexibility.
   This allows to implement the fetch-negotiation part oneself and break
   free from constraints of the delegate.
 - <csr-id-4367994a8a7476eb44e1309e833e345fdb78f246/> add `config::SnapshotMut::commit()` to make clear it's transactional.
 - <csr-id-d2bea003230078ffb4e6cd80d1b01c3995435a34/> add `config::SnapshotMut::forget()` to forget all changes before applying them.
   The documentation was update to make clear when the changes are applied.
 - <csr-id-4b1e3b3d91c51da3dbea9191e60f959a1266cf47/> add `Repository::find_default_remote()` which works on detached heads as well.
 - <csr-id-25f06400c49ddd1688fd76f9285542b121b223b4/> `Remote::rem_map()` now specifies ref-prefixes to the remote.
   This can greatly reduce the amount of refs sent.

### Other

 - <csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/> try to make the transport configurable after being boxed, but…
   …that would force it to be 'static, which is something we excplicitly
   cannot have. We need references to be contained within, if I remember
   correctly.

### Changed (BREAKING)

 - <csr-id-e88de0f948325773db1925b07aa878e1dbb76bad/> All methods editing references don't take the author as parameter anymore.
   Instead, these are taken from the git configuration and can be
   configured on the fly with temporarily altered configuration.
 - <csr-id-3a0fb1b45c757add49677450836c0aaf6179a2b5/> remote `lock_mode` from all methods dealing with reference edits.
   It is now read from `core.filesRefLockTimeout` accordingly.

### New Features (BREAKING)

 - <csr-id-3b29fc18672c0176684c797a0f16f85d09369bf8/> make jwalk fully optional
 - <csr-id-78ad3df64f2c016ba17b158bd9ab1d2341aab399/> add `fetch::Transport::configure` to generically configure any transport.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 129 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 8 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - fix test expectations to handle V1/V2 differences. ([`e616174`](https://github.com/Byron/gitoxide/commit/e616174e40b5dd11217d540ba7b17e3164f7ca30))
    - no need to fetch the whole pack in dry-run mode, even in V1. ([`1777fa1`](https://github.com/Byron/gitoxide/commit/1777fa13e9e821c7a720227b6d7f1ad79f751840))
    - fix hang in V1 mode ([`ce9b591`](https://github.com/Byron/gitoxide/commit/ce9b59115bc66f052324e169c574f2515565a496))
    - Display for `remote::fetch::update::refs::Mode` ([`169a979`](https://github.com/Byron/gitoxide/commit/169a979228e4f82b7c465d88dce0a304d864aeab))
    - better error messages if no ref-specs are provided where needed ([`981488b`](https://github.com/Byron/gitoxide/commit/981488b413ede3442f2213632ee52f4c207629bb))
    - Improve error description with local context ([`8e9e0b2`](https://github.com/Byron/gitoxide/commit/8e9e0b20840e0a2479525e79d3e63071d20859c8))
    - always perform fast-forward checks even if force is specified. ([`a34370c`](https://github.com/Byron/gitoxide/commit/a34370c1aea028cd9d6b839f136a16a11b32b804))
    - refactor ([`c0d3ced`](https://github.com/Byron/gitoxide/commit/c0d3ced850a1a369e1c562e172266d78b0bc6698))
    - refactor ([`8334148`](https://github.com/Byron/gitoxide/commit/8334148c63bfdb10c7388e4ce6480070e39cfa9f))
    - add test to validate actual fast-forwards, without dry-run ([`e3b937e`](https://github.com/Byron/gitoxide/commit/e3b937ef35839402498988af322f943e4745178f))
    - first non-fastforward tests in dry-run mode ([`2d0d782`](https://github.com/Byron/gitoxide/commit/2d0d782a59a90021e388b2c55354e9b3472d9cd3))
    - dry-run mode for fetch ([`ef9fa98`](https://github.com/Byron/gitoxide/commit/ef9fa9835f9abaaa5034d62359e2899c0dd51408))
    - test all reflog messages that are expected, sans fast-forward ([`2a76908`](https://github.com/Byron/gitoxide/commit/2a76908f3ebe846e96b783075bb5395d0bb9aaa0))
    - support reflog message prefix ([`6ebbbc1`](https://github.com/Byron/gitoxide/commit/6ebbbc153dcf4eedb2afbd561c4d2ce342f6289b))
    - refactor ([`9a072bd`](https://github.com/Byron/gitoxide/commit/9a072bd2e495b19c479e831f62172482da511d8c))
    - facilities to test reflog messages ([`91859d5`](https://github.com/Byron/gitoxide/commit/91859d5140a45874d8d934773a47eee6b6a5a126))
    - assure objects exist before setting them. ([`d79a7a6`](https://github.com/Byron/gitoxide/commit/d79a7a67ef77ab9d6dc871e16d535a509eb78e49))
    - no-clobber special case for tags ([`ba41b6c`](https://github.com/Byron/gitoxide/commit/ba41b6c10c073926802e38a8fa035df8444affef))
    - refactor ([`18581d0`](https://github.com/Byron/gitoxide/commit/18581d0dca277b8933885e30561d51640b88dfa4))
    - reject updating checked-out branches ([`5a6c102`](https://github.com/Byron/gitoxide/commit/5a6c1025012d63e019e8b554078a33d23190bf18))
    - add failing test to validate worktree-checkout check ([`870b680`](https://github.com/Byron/gitoxide/commit/870b6806f0199b13916f05f46a22fdd3e2a76513))
    - fix docs ([`2a67531`](https://github.com/Byron/gitoxide/commit/2a675312b7a4d28cc84e09d54e1f929b0e56f75f))
    - further harden tests to only allow environment variables for those who need it. ([`9704c2f`](https://github.com/Byron/gitoxide/commit/9704c2f075af05bf258854b378ed91b8a5d71e93))
    - unset other variables that we know may affect some functions that need stability. ([`25d6106`](https://github.com/Byron/gitoxide/commit/25d61067640016b21cdf1eb90998be512b805e8b))
    - more robust tests that depend on time. ([`6981b71`](https://github.com/Byron/gitoxide/commit/6981b71a23444827dff4dfe6cc5f1c04beceddc1))
    - isolate test properly ([`005469c`](https://github.com/Byron/gitoxide/commit/005469cdaef4defb35ae65c23962c9f7da98c12f))
    - more tests for SnapshotMut and now it's working properly ([`dde9e63`](https://github.com/Byron/gitoxide/commit/dde9e6345f53e36d6a9528d4132b16a6659999dd))
    - more robust assignment of re-evaluated cached values ([`b514966`](https://github.com/Byron/gitoxide/commit/b5149661d9160540359b19c204388c87c778727f))
    - Add test for commit-and-rollback method, and fix it ([`cc75647`](https://github.com/Byron/gitoxide/commit/cc7564745b45df1848dfb1d7c8f9e1178f0fb64d))
    - adapt to changes in `git-ref` ([`d40beb3`](https://github.com/Byron/gitoxide/commit/d40beb3b5744139b56ed68de4caa62a242df2d3a))
    - refactor ([`f47a31d`](https://github.com/Byron/gitoxide/commit/f47a31d7a533c7debc9a44020fa597ff2d48068c))
    - read core.excludesFile lazily (and don't cache it yet) ([`830c450`](https://github.com/Byron/gitoxide/commit/830c45039e9377914bc715002c4e280187498c5c))
    - auto-update previously cached values after changing the configuration ([`da147bf`](https://github.com/Byron/gitoxide/commit/da147bffd7538453221b08f9f68a1332cfa3ebe3))
    - All methods editing references don't take the author as parameter anymore. ([`e88de0f`](https://github.com/Byron/gitoxide/commit/e88de0f948325773db1925b07aa878e1dbb76bad))
    - remote `lock_mode` from all methods dealing with reference edits. ([`3a0fb1b`](https://github.com/Byron/gitoxide/commit/3a0fb1b45c757add49677450836c0aaf6179a2b5))
    - prepare for worktree-aware checked-out branch handling… ([`1bb910e`](https://github.com/Byron/gitoxide/commit/1bb910ee2dbe0c5f19aefd9669cebc305870953e))
    - actually apply ref updates ([`8fe4bf4`](https://github.com/Byron/gitoxide/commit/8fe4bf4bcc463154c54082df5f38f0cd801915fb))
    - Add failing test to show we need to respect dry-run mode (or the lack thereof) ([`e25460b`](https://github.com/Byron/gitoxide/commit/e25460b3519609b2836e5bf57ad59e6cf06872e4))
    - tests for all the cases excluding fast-forwards ([`7ced240`](https://github.com/Byron/gitoxide/commit/7ced2402eb28301adc5330f336ece5eaf3bd9222))
    - the first successful test ([`e4edc18`](https://github.com/Byron/gitoxide/commit/e4edc1897cc8ffa0dfd0c34cfaf6eb2f9d5b86c6))
    - Make `fetch::refs::update()` private again, move tests accordingly. ([`9f9b610`](https://github.com/Byron/gitoxide/commit/9f9b61070d0b6e10e795e1401401d55c554c59b9))
    - Provide refspecs to refs::update() to obtain force information ([`a9f2c45`](https://github.com/Byron/gitoxide/commit/a9f2c458c1858e8d40ed0efdc762f78b9efb7783))
    - a big step towards ref updates, now it needs specs ([`c101d50`](https://github.com/Byron/gitoxide/commit/c101d50c315d922885309e5939a10853c553eb68))
    - more update tests ([`2828674`](https://github.com/Byron/gitoxide/commit/2828674509f847528bb225c1a35e51efd7457c50))
    - the first somewhat synthetic test to check for no changes. ([`c355823`](https://github.com/Byron/gitoxide/commit/c355823d405dbf8eb1287021895d5fb35a39e5f5))
    - fix build ([`8e1555d`](https://github.com/Byron/gitoxide/commit/8e1555d0ef0ea450979567c9aa9716c993e6320a))
    - make `remote::fetch::refs::update()` public to facilitate testing ([`4a5d3b4`](https://github.com/Byron/gitoxide/commit/4a5d3b4bb4f39a1e227da4ea77e96820cbed2e0d))
    - lay the ground-works for testing the update of refs ([`1f2d609`](https://github.com/Byron/gitoxide/commit/1f2d6095d946f6327e67a7388fd87ab9c74be31d))
    - refactor ([`96f2fd8`](https://github.com/Byron/gitoxide/commit/96f2fd8d848dd170855721f85ec6386f9391f0a1))
    - greatly improved performance for write-test. ([`2ec8175`](https://github.com/Byron/gitoxide/commit/2ec8175a55a9cd02408cab45d84da2823c44dec4))
    - improve naieve algorithm to be a bit better in our test-case ([`0387794`](https://github.com/Byron/gitoxide/commit/038779420edddb651c3463e4679778ceabf902b8))
    - try to make naive negotiation better, but… ([`d5c1f92`](https://github.com/Byron/gitoxide/commit/d5c1f9280e8a20f8ce8c087bde04f4098cafe993))
    - speed up fetch tests by giving them their own repo-fixture ([`ce1a373`](https://github.com/Byron/gitoxide/commit/ce1a373c80e076c148112532990b781044b7aeb8))
    - A first sketch of validating a fetch. ([`2962dc2`](https://github.com/Byron/gitoxide/commit/2962dc28c8e93ac81bde70dacfc3081aa697676f))
    - Don't degenerate information in case there is no update needed. ([`5f73b25`](https://github.com/Byron/gitoxide/commit/5f73b257c551ef899c9e34dd5772654d51444d8b))
    - A first test for validating nothing-new is a no-op ([`aad17ba`](https://github.com/Byron/gitoxide/commit/aad17ba50f8c77465004a00da2146a87fc770646))
    - port part of the negotation logic over, but a lot is still missing ([`4997e56`](https://github.com/Byron/gitoxide/commit/4997e5616c39f3d3be74f289c25080d9898b28f5))
    - make some private methods public to give callers more flexibility. ([`31a7089`](https://github.com/Byron/gitoxide/commit/31a7089f2583832727e2175ada6fb5c30c3beebe))
    - refactor ([`5e93ef5`](https://github.com/Byron/gitoxide/commit/5e93ef53e43c7ce1e5f964d792ff97b426802b4a))
    - complete pack generation options based on configuration ([`97a5e97`](https://github.com/Byron/gitoxide/commit/97a5e972f179c000cec888dcbe4cff13e02d77e5))
    - also extract index threads ([`8d17dc6`](https://github.com/Byron/gitoxide/commit/8d17dc68cea8a6b6b417f12d45fcf4331cf562fd))
    - obtain configuration for index version (with respect for lenient config) ([`5a3155a`](https://github.com/Byron/gitoxide/commit/5a3155a019ed5c9157cc699d4bbdf1b0b3623242))
    - add `config::SnapshotMut::commit()` to make clear it's transactional. ([`4367994`](https://github.com/Byron/gitoxide/commit/4367994a8a7476eb44e1309e833e345fdb78f246))
    - add `config::SnapshotMut::forget()` to forget all changes before applying them. ([`d2bea00`](https://github.com/Byron/gitoxide/commit/d2bea003230078ffb4e6cd80d1b01c3995435a34))
    - sketch the receive() method to finally receive a pack. ([`67801a3`](https://github.com/Byron/gitoxide/commit/67801a344a4fc6d7c171d93277635bdf84e6c15a))
    - allow stopping fetches after preparing it ([`249c54e`](https://github.com/Byron/gitoxide/commit/249c54ef237c8147dce4cd999ccd4ddba4775150))
    - add `Repository::find_default_remote()` which works on detached heads as well. ([`4b1e3b3`](https://github.com/Byron/gitoxide/commit/4b1e3b3d91c51da3dbea9191e60f959a1266cf47))
    - refactor ([`f8fb04a`](https://github.com/Byron/gitoxide/commit/f8fb04ad76d282ea3b31cba512f7421f31569e8b))
    - fix build ([`7993f6a`](https://github.com/Byron/gitoxide/commit/7993f6a4b95a18809e98f34366dc1746b944f8d5))
    - remove connect_http() method to encourage changing settings using `transport_mut().configure()`. ([`9b86a1f`](https://github.com/Byron/gitoxide/commit/9b86a1f38b3ea2bd0e639d392849c3660fc08cff))
    - add `fetch::Transport::configure` to generically configure any transport. ([`78ad3df`](https://github.com/Byron/gitoxide/commit/78ad3df64f2c016ba17b158bd9ab1d2341aab399))
    - sketch of 'Prepare' struct to configure fetch after ref-map was obtained. ([`f0f4db6`](https://github.com/Byron/gitoxide/commit/f0f4db6fcb61a5a93786c74c7997657b2fc4f233))
    - don't fail if we can't indicate the end of interaction to the server ([`47d5cd6`](https://github.com/Byron/gitoxide/commit/47d5cd67b31d4b18c224859b7d9e145c993a4f2d))
    - Make it easier to connect to http if well-known to allow additional configuration. ([`211e65d`](https://github.com/Byron/gitoxide/commit/211e65d185470ade84a3cc73e1898599b7f15f7c))
    - Revert "FAIL: try to make the transport configurable after being boxed, but…" ([`fbb96e4`](https://github.com/Byron/gitoxide/commit/fbb96e4d55e322243bf5500605f72e93b103e308))
    - try to make the transport configurable after being boxed, but… ([`5bef0a0`](https://github.com/Byron/gitoxide/commit/5bef0a00e8d01110c054a517f6d9696f981a7efc))
    - first sktech of fetch module ([`0dc7206`](https://github.com/Byron/gitoxide/commit/0dc7206ee0dff760d632362de3b41d9e4dc22598))
    - pass extra handshake parameters via options in `ref-map` ([`5475cc2`](https://github.com/Byron/gitoxide/commit/5475cc2e60dd1cde3ecb24ccf873bc06421f09c9))
    - Allow to turn remote-filtering off. ([`38373bc`](https://github.com/Byron/gitoxide/commit/38373bc61c938d58a9d6ed1feae86ccf36fde67d))
    - `Remote::rem_map()` now specifies ref-prefixes to the remote. ([`25f0640`](https://github.com/Byron/gitoxide/commit/25f06400c49ddd1688fd76f9285542b121b223b4))
 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-smart-release' ([`aa80b60`](https://github.com/Byron/gitoxide/commit/aa80b606e5570f327660cca42ea81581a6e9d5e3))
    - make fmt ([`7b9c065`](https://github.com/Byron/gitoxide/commit/7b9c06547b75929e3e5bf4240f43c7e9bc7d54e0))
    - move `fetch::Error` one level up ([`f6b1368`](https://github.com/Byron/gitoxide/commit/f6b136890e3bcc59ac692ddc4e295f043adcfece))
    - Merge branch 'clone' ([`507dc7e`](https://github.com/Byron/gitoxide/commit/507dc7e706cb3c9d89d048b3aff5df239a9b6788))
    - fix docs ([`4474352`](https://github.com/Byron/gitoxide/commit/44743524fd35c7987fee2f46d134e4b0e328e133))
    - validate default remote name as well and show minimal clone without worktree ([`dff66e8`](https://github.com/Byron/gitoxide/commit/dff66e898bb3cc033c85464ccd937b1a0e891750))
    - validate the outcome of the fetch as well ([`3865aea`](https://github.com/Byron/gitoxide/commit/3865aea50b05b4a28caf0d364fea4876c393f976))
    - adjust to changes in `git-refspec` ([`c62f37a`](https://github.com/Byron/gitoxide/commit/c62f37a4e3599049d10e3d7534a553e8efa2dcd7))
    - thanks clippy ([`bc2710f`](https://github.com/Byron/gitoxide/commit/bc2710f1833d3f3423868bd4c7d8df991200b39d))
    - the first successful test for cloning a bare repository ([`5ecc965`](https://github.com/Byron/gitoxide/commit/5ecc965b8804500c537acdac2f1d8751324f4736))
    - Complete implementation of `fetch_only`, even though configuration isn't handled correctly yet ([`b0f5836`](https://github.com/Byron/gitoxide/commit/b0f5836c53fc95c408dac4b6e370532695373063))
    - sketch and test for `clone::Prepare::fetch_only()` ([`f993cd4`](https://github.com/Byron/gitoxide/commit/f993cd4ca03a22d27c85ff229bab66adcb4e493a))
    - thanks clippy ([`f65a1f4`](https://github.com/Byron/gitoxide/commit/f65a1f4b08bdacd92baf118722a45a02a71117f1))
    - make it possible to renmae the remote ([`3f8b0e5`](https://github.com/Byron/gitoxide/commit/3f8b0e5c480efcdc73ce781bc3e9e9b156ec6551))
    - properly validate remotes when instantiating them and when naming them ([`1fb97fb`](https://github.com/Byron/gitoxide/commit/1fb97fbfe893d6a8030e3ef0bae34f40a3e9b7e6))
    - adjust to changes in `git-features` ([`3155197`](https://github.com/Byron/gitoxide/commit/31551976ca9327df34d961614311a10b71e50a93))
    - make jwalk fully optional ([`3b29fc1`](https://github.com/Byron/gitoxide/commit/3b29fc18672c0176684c797a0f16f85d09369bf8))
    - thanks clippy ([`8c8fba2`](https://github.com/Byron/gitoxide/commit/8c8fba2f5d6b464bb9fc275bbf2db89635e75d43))
    - start adding support for naming the remote, but… ([`6cbea96`](https://github.com/Byron/gitoxide/commit/6cbea960b798181f46e736d509eed5a44b91e0c8))
    - sketch a way to configure remotes ([`615a3a9`](https://github.com/Byron/gitoxide/commit/615a3a90ffb67c76e00bf3a024aaafa67ef148cb))
    - refactor ([`d7f495d`](https://github.com/Byron/gitoxide/commit/d7f495d6d5c436decc6ea6236f2129a4758facc0))
    - Api sketch to show how clones and bare clones can be done. ([`ef1e783`](https://github.com/Byron/gitoxide/commit/ef1e7834875fd2e1ab793a17a393ad3e5470059c))
    - `Url::try_from(path: &std::path::Path)` for more convenient instantiation. ([`22d3b37`](https://github.com/Byron/gitoxide/commit/22d3b37ea6239170a478b859361a7d1d7ba01a9a))
    - adjust to changes in `git-url` ([`a1068a3`](https://github.com/Byron/gitoxide/commit/a1068a3f24ce55b9eb92b97c84f650f901c7a5d3))
    - add test to assert on lossless saving of existing named remotes ([`884b6e9`](https://github.com/Byron/gitoxide/commit/884b6e93962bf782bfca14873277a8c563b58d61))
    - adapt to changes in `git-config` ([`058473d`](https://github.com/Byron/gitoxide/commit/058473d5eae4789ca7b3fcb203ae6349a3e6d25a))
    - assure unrelated and unwritten keys are not touched ([`ca5bb5e`](https://github.com/Byron/gitoxide/commit/ca5bb5ec44f098f74a47835ee0dbc694cbb76e0c))
    - thanks clippy ([`d82a08d`](https://github.com/Byron/gitoxide/commit/d82a08db71f6a486363a398eacdee6cdfbbeb04b))
    - remove prior existing values for remotes before saving ([`4126d99`](https://github.com/Byron/gitoxide/commit/4126d996b5c110e2a73f4b94f7166d7e66e059b3))
    - save all data we currently use on a Remote ([`79a4952`](https://github.com/Byron/gitoxide/commit/79a495249ded5dbab0084283606d8775911dbfca))
    - test another prerequisite ([`1375368`](https://github.com/Byron/gitoxide/commit/1375368006942bdf246a85466a144b05df5a55ac))
    - A basic save_to() implementation for url and push-url ([`e1f7c5f`](https://github.com/Byron/gitoxide/commit/e1f7c5f05095b6b3d2d26c34fda04b74a91c9fb0))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - thanks clippy ([`1cba50f`](https://github.com/Byron/gitoxide/commit/1cba50f908ea016c9117e2de2181f55326f14e7e))
    - layout save API for remotes ([`50fbcca`](https://github.com/Byron/gitoxide/commit/50fbcca85fdf02261fba954363bcf257cdf445fe))
    - Merge branch 'crates-index-diff-fixes' ([`b09bad7`](https://github.com/Byron/gitoxide/commit/b09bad7b5702838c1119d052b322d90c9b5968bb))
    - Add test to validate empty fetches are OK ([`cb9973b`](https://github.com/Byron/gitoxide/commit/cb9973b95e71657f8b66f5938f1312f7e118d197))
    - Make `fetch::prepare::Error` publicly accessible ([`9aa2ab2`](https://github.com/Byron/gitoxide/commit/9aa2ab2866640a5e4d21e681eda3c3ba24dc4cdd))
    - make fmt ([`53acf25`](https://github.com/Byron/gitoxide/commit/53acf2565743eff7cead7a42011107b2fc8d7e0e))
    - Merge branch 'fetch-pack' ([`f47c891`](https://github.com/Byron/gitoxide/commit/f47c89129732bcb06fe76a4696fe38ab1151fb0c))
    - thanks clippy ([`83f2156`](https://github.com/Byron/gitoxide/commit/83f21565bbbc43442410be3b63030ea2537e6d2c))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - thanks clippy ([`b521748`](https://github.com/Byron/gitoxide/commit/b521748974c6cb021121cdcc3bbbca5b80987336))
    - thanks clippy ([`8499c3e`](https://github.com/Byron/gitoxide/commit/8499c3e8afba0767fe9f0ca0e3016fe9f84951e5))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - improve docs around `into_remote()` to explain why it returns an `Option` ([`71146ad`](https://github.com/Byron/gitoxide/commit/71146ad274410e16489ae6181fb289814028ded5))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.24.0 (2022-09-20)

<csr-id-f5959edc1477573278afcfe23e9e52ddaacb37db/>
<csr-id-79c22557ce0aea1ee8f3a58192c2c76087ccd3d8/>

### New Features

 - <csr-id-0871a96b9cc84d7a496d39393e081999c0a3fe17/> `Object::peel_to_tree()` as convenience method.
   It's very common to try to work with trees, so let's make that easier.
 - <csr-id-1027be960852618915014f9ba6e6632bd4999b0e/> `interrupt::Iter` now allows accessing the inner iterator without consumption.
   This is useful if these provide additional out-of-band information.
 - <csr-id-8c2e5c60f9f5f8d0859ecd84c17af20e88812512/> Once a change is obtained, it's easy to obtain changes line by line.
 - <csr-id-7e96d1841989b37133cddf334724a2d6df557e69/> obtain a refmap after listing refs via `remote::Connection::list_refs_to_map()`.
   With it it's possible to establish a relationship between what's about
   to be fetched to local tracking branches as established by refspecs for
   fetching.
 - <csr-id-d51e7c901fe5ed60d5dd56006c5faedb71cad537/> Add `permissions::Config::git_binary` field
   When true, default false, inject the git installation configuration file
   if present at the cost of one `git config` invocation.
   
   Note that we rely on the underlying `git-config` crate to not load
   duplicate files.
   
   We also currently lie about the scope which is actually unclear - have
   seen 'unknown' or normal scopes like `system`.
 - <csr-id-1c13f1125664fbcc276a1ca440d168d07d0bf493/> add `prompt` to top level forwarding #450)

### Bug Fixes

 - <csr-id-ae3866065c9c3c6d01709f8dde1cea1ae1345779/> rev-spec parsing can now handle the empty tree as full hex hash.
   Even though the empty-tree object can be found when searched via
   `Repository::find_object()`, previously it was not locatable when
   used during rev-spec parsing.
 - <csr-id-74ede2031d1beedf11f1cdf006fff71e597a2cb5/> `Reference::remote()` can produce remotes for URLs

### Refactor

 - <csr-id-f5959edc1477573278afcfe23e9e52ddaacb37db/> use specific error type for `rev_parse_single()`

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### New Features (BREAKING)

 - <csr-id-2992b1ba4e7bbeab26f41175cd31fd664abf2a11/> Add reference remote name type to make usage of `remote_name()` result clear

### Other (BREAKING)

 - <csr-id-79c22557ce0aea1ee8f3a58192c2c76087ccd3d8/> `Tree::lookup_path()` -> `Tree::lookup_entry()`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 102 commits contributed to the release over the course of 19 calendar days.
 - 19 days passed between releases.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 6 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - adapt to changes in `git-refspec` ([`91d1a3a`](https://github.com/Byron/gitoxide/commit/91d1a3abf02841c9b43fd8a3102375315a6db160))
    - show fixes as well ([`2237495`](https://github.com/Byron/gitoxide/commit/2237495d82624b39bf75c6430549424a5e36b8bb))
    - Add method to allow replacing a Remote's refspecs entirely. ([`d8f1608`](https://github.com/Byron/gitoxide/commit/d8f160826cd6446e0422aef6020e1413895f340e))
    - produce only a ref-map as it contains all data somebody would want. ([`c9ff885`](https://github.com/Byron/gitoxide/commit/c9ff885e73c97b8712cff89bce420f3060f8bd3c))
    - adjust to changes in `git-config` ([`0f9833a`](https://github.com/Byron/gitoxide/commit/0f9833af529e35c4930f6231397368256231dcdb))
    - also provide the spec-index with the returned refmap. ([`910cedc`](https://github.com/Byron/gitoxide/commit/910cedc09212e0d08c4a03db959974bc0810fa9d))
    - Provide all information generated by the handshake into the refmap result ([`3958d71`](https://github.com/Byron/gitoxide/commit/3958d71177927305399369088d184891c182d9e2))
    - adjust to changes in `git-refspec` ([`ffa24a1`](https://github.com/Byron/gitoxide/commit/ffa24a1365480523197b5247bded6a7a4772bdfc))
    - obtain a refmap after listing refs via `remote::Connection::list_refs_to_map()`. ([`7e96d18`](https://github.com/Byron/gitoxide/commit/7e96d1841989b37133cddf334724a2d6df557e69))
    - A step closer to actually obtaining a validated ref-mapping. ([`777ba7f`](https://github.com/Byron/gitoxide/commit/777ba7ffe4b20e2cdf4069eeb504df432db58d69))
    - A more concrete sketch of how the API for obtaining mappings should look like ([`6d1c372`](https://github.com/Byron/gitoxide/commit/6d1c37219cde7ce5c969d1581da4bb4593cfb1fd))
    - sketch API for obtaining a reflist filtered by refspecs ([`5d5e211`](https://github.com/Byron/gitoxide/commit/5d5e2113a8353bf849ed4bfe9c8a341b8e00cd0b))
    - Extract URL from transport to support custom remotes better and avoid error case ([`fac87d0`](https://github.com/Byron/gitoxide/commit/fac87d0dc4d4524713dbd7ef1cfc6acae54f9748))
    - use Rust credential implementation ([`fb39608`](https://github.com/Byron/gitoxide/commit/fb396089495518c45104cc8d62ee4d6221c6b76a))
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
    - refactor ([`9e4e4c4`](https://github.com/Byron/gitoxide/commit/9e4e4c4fbe0189951eeac0ed8cbdcfcd409a9f6c))
    - adjust to changes in `git-sec` ([`d6ef2ce`](https://github.com/Byron/gitoxide/commit/d6ef2ce9d0a7883d7d3b5ddb0010c8ab3d26bb78))
    - adjust to changes in `git-ref` ([`296f23f`](https://github.com/Byron/gitoxide/commit/296f23fea43f543e10448fc0c948f629f241561e))
    - adjust to changes in git-object ([`9aadac3`](https://github.com/Byron/gitoxide/commit/9aadac3341bb778e8f8b0800847b574338e16a05))
    - Select `gix` commands will now load the git installation configuration ([`23d2dec`](https://github.com/Byron/gitoxide/commit/23d2dec375305c39d472c4f8ff764274dd033f6b))
    - Add `permissions::Config::git_binary` field ([`d51e7c9`](https://github.com/Byron/gitoxide/commit/d51e7c901fe5ed60d5dd56006c5faedb71cad537))
    - A way to parse the first git-config file path ([`ea52e4e`](https://github.com/Byron/gitoxide/commit/ea52e4ec917ee4caa55329ba1a1814d723e6c654))
    - first step towards efficiently obtaining git config information ([`cc78473`](https://github.com/Byron/gitoxide/commit/cc784730f29eb239e18dad7c5013486e9b6355d1))
    - fix exports to allow error type to be visible publicly ([`b978619`](https://github.com/Byron/gitoxide/commit/b978619eaada0ffdc88c24edbf3fc68d4cc32312))
    - Also provide prompt configuration to allow Cascade::invoke() to be called with all arguments ([`5b4cb83`](https://github.com/Byron/gitoxide/commit/5b4cb8391c1bd4b2aab8dea249f161bafcab93c2))
    - Add reference remote name type to make usage of `remote_name()` result clear ([`2992b1b`](https://github.com/Byron/gitoxide/commit/2992b1ba4e7bbeab26f41175cd31fd664abf2a11))
    - refactor ([`93ac4c3`](https://github.com/Byron/gitoxide/commit/93ac4c38e5837250e158613820a6ac1bb7119ba0))
    - remark about improvements to the type system around certain remote names. ([`a872bf7`](https://github.com/Byron/gitoxide/commit/a872bf7fbbda48df34ad92cdbc48f93d33ebd86e))
    - `Reference::remote()` can produce remotes for URLs ([`74ede20`](https://github.com/Byron/gitoxide/commit/74ede2031d1beedf11f1cdf006fff71e597a2cb5))
    - refactor ([`4fa8a84`](https://github.com/Byron/gitoxide/commit/4fa8a844826916007c583d684d250e6bfbc72e53))
    - test our understanding of case-folding when matching urls ([`77c27eb`](https://github.com/Byron/gitoxide/commit/77c27ebf93dcaf81e38b2dc6271133c6bb9f3121))
    - subdomain globbing works ([`5b19729`](https://github.com/Byron/gitoxide/commit/5b197292e6ed08633e4e20e411e973d269e30c4b))
    - tests for domain globbing ([`60d8997`](https://github.com/Byron/gitoxide/commit/60d89970710faae804f712bb7832677278f90133))
    - more default-port tests ([`eb34156`](https://github.com/Byron/gitoxide/commit/eb34156ea6b4ac8523d38d6181c9cc949853bb6e))
    - username matching works ([`b99e9d8`](https://github.com/Byron/gitoxide/commit/b99e9d84721c0d8b58b5b56c8dde60952237b042))
    - All but one test work after username handling refactor ([`64b6b30`](https://github.com/Byron/gitoxide/commit/64b6b30fb42d7ee3d65bc6238a4ad867a85c802d))
    - better normalization and clear-list support ([`5aefe66`](https://github.com/Byron/gitoxide/commit/5aefe6684438750fe9aadb41675c816f9671d8a9))
    - remove glob-matching as it's not correct ([`6f4e052`](https://github.com/Byron/gitoxide/commit/6f4e052560b397f2bc81a4a25f573bef517e5870))
    - refactor ([`593e4a8`](https://github.com/Byron/gitoxide/commit/593e4a8a6befbaa14401d659ac3901c92376f94b))
    - more tests for better host and protocol matching of http and https urls ([`2221778`](https://github.com/Byron/gitoxide/commit/22217780acf3c241125bb000f6a205ca0914a025))
    - Support for host-based matching ([`2d81b9f`](https://github.com/Byron/gitoxide/commit/2d81b9f629a899a73fb68967ceeae18719fbd0e1))
    - basic url matching with a simple glob ([`1b19611`](https://github.com/Byron/gitoxide/commit/1b196116fa01f1045a242d0ebe3d9627cb1fa8ec))
    - Make tests more robust; fix windows tests ([`1983fbc`](https://github.com/Byron/gitoxide/commit/1983fbc39be3da5598cf3af6fb97f6ea0bc3ec6b))
    - tests as far as possible without implementing url matching ([`8c686e0`](https://github.com/Byron/gitoxide/commit/8c686e072af544adea48f81ba2dc65dff528e5ab))
    - mark a difference in the way credential helper urls are validated ([`dc57b67`](https://github.com/Byron/gitoxide/commit/dc57b676e2fcb869f8e73cc543a73c73b83bfe22))
    - the first successful url matching test ([`beddce1`](https://github.com/Byron/gitoxide/commit/beddce1b1814b60da5881a3676b019d35c47a0a8))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
    - `Object::peel_to_tree()` as convenience method. ([`0871a96`](https://github.com/Byron/gitoxide/commit/0871a96b9cc84d7a496d39393e081999c0a3fe17))
    - fix docs ([`593f57b`](https://github.com/Byron/gitoxide/commit/593f57b486d03b4d689cda6c0800e9f349cc4ad5))
    - Add note on why we consume the tree for looking up an entry. ([`b285097`](https://github.com/Byron/gitoxide/commit/b28509724c8877d6488bf5dc72308e8b18928124))
    - `Tree::lookup_path()` -> `Tree::lookup_entry()`. ([`79c2255`](https://github.com/Byron/gitoxide/commit/79c22557ce0aea1ee8f3a58192c2c76087ccd3d8))
    - simplify looking up entries by path ([`15a18e4`](https://github.com/Byron/gitoxide/commit/15a18e47f4a767a2cc31a30296844c207c7a8732))
    - a little more complexity for diff tests ([`5878ad1`](https://github.com/Byron/gitoxide/commit/5878ad17bc0c0d9d99b36f3ff9416cf9a47a4086))
    - `interrupt::Iter` now allows accessing the inner iterator without consumption. ([`1027be9`](https://github.com/Byron/gitoxide/commit/1027be960852618915014f9ba6e6632bd4999b0e))
    - Once a change is obtained, it's easy to obtain changes line by line. ([`8c2e5c6`](https://github.com/Byron/gitoxide/commit/8c2e5c60f9f5f8d0859ecd84c17af20e88812512))
    - Slightly improved docs for traversal docs. ([`963055b`](https://github.com/Byron/gitoxide/commit/963055b45643fb48460671959b10dc12658bb5d4))
    - performance note ([`0670468`](https://github.com/Byron/gitoxide/commit/06704683cdde64c0ed9b38df5e4e8ce29dbce524))
    - Support for Path tracking ([`64bbb3d`](https://github.com/Byron/gitoxide/commit/64bbb3da42f740206514baf2fa504371fd6f06c4))
    - rev-spec parsing can now handle the empty tree as full hex hash. ([`ae38660`](https://github.com/Byron/gitoxide/commit/ae3866065c9c3c6d01709f8dde1cea1ae1345779))
    - refactor ([`9d01fb4`](https://github.com/Byron/gitoxide/commit/9d01fb41c8b367b8bd73061fc5f0f7dc4d33f7d1))
    - refactor ([`90b9c90`](https://github.com/Byron/gitoxide/commit/90b9c906bc3779d62b0317ea318c07693fda0d3c))
    - fix docs ([`9b7aaa0`](https://github.com/Byron/gitoxide/commit/9b7aaa00ed7750e0f6a5898212d78ffa98456749))
    - improved usability of the `Action` enum ([`d04807b`](https://github.com/Byron/gitoxide/commit/d04807bc9a70ddb6139446356df5c1bdb902a497))
    - Support for file-name tracking ([`88c4a57`](https://github.com/Byron/gitoxide/commit/88c4a57b8e84db74d9bc2a1d626bc5c51a069fad))
    - Provisions for tracking the location of a change. ([`7fd9b0e`](https://github.com/Byron/gitoxide/commit/7fd9b0eed4a18c2a9e9ae44bc8a8f72769995889))
    - first test for simple file modification detection ([`a9056fd`](https://github.com/Byron/gitoxide/commit/a9056fdd3f22347737b4d3c80a793d1d26f4218b))
    - allow user callbacks to have any error ([`5be96b3`](https://github.com/Byron/gitoxide/commit/5be96b3c5c276bc0176820da4a5f554c1a1623f3))
    - break through API surface and sketch delegate calling user-provided function ([`e51f3cd`](https://github.com/Byron/gitoxide/commit/e51f3cda68606dafd908ec268099fc455493ebaf))
    - diff platform for basic diff configuration ([`c857b9b`](https://github.com/Byron/gitoxide/commit/c857b9b6a113bd60fa3c9aeaf3edb81164ae772a))
    - refactor ([`a938fe4`](https://github.com/Byron/gitoxide/commit/a938fe491e98577230b2aefd536b600e74050225))
    - refactor ([`6ac7dbe`](https://github.com/Byron/gitoxide/commit/6ac7dbe689b5599e72a05b4e23c1943cd2bba145))
 * **Uncategorized**
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - thanks clippy ([`77ff8ae`](https://github.com/Byron/gitoxide/commit/77ff8ae5fa9bbdb7c5e1c577845334f966294426))
    - make fmt ([`429cccc`](https://github.com/Byron/gitoxide/commit/429cccc5831c25a7205a12dc7a0443ac48616e2c))
    - avoid risking comparing two different current-time timestamps ([`389cb2a`](https://github.com/Byron/gitoxide/commit/389cb2abb585f4640ddd128541b05ad338599b5e))
    - Merge branch 'filter-refs' ([`3773b92`](https://github.com/Byron/gitoxide/commit/3773b92b8372c9a40a74d281149ca65b057a7da9))
    - thanks clippy ([`52fa247`](https://github.com/Byron/gitoxide/commit/52fa247d3ba211ccacfc867d2f0a17022e2b9b62))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - upgrade all dependencies, except for `windows` ([`2968181`](https://github.com/Byron/gitoxide/commit/29681819ffe53d3926d631dc482f71d6200cb549))
    - Merge branch 'index-from-tree' ([`172f73c`](https://github.com/Byron/gitoxide/commit/172f73cf26878d153d51790fa01853fa4ba6beb7))
    - use specific error type for `rev_parse_single()` ([`f5959ed`](https://github.com/Byron/gitoxide/commit/f5959edc1477573278afcfe23e9e52ddaacb37db))
    - use rev_parse_single() instead of rev_parse().single() ([`09948a5`](https://github.com/Byron/gitoxide/commit/09948a56449c372444e1ee13138128482a97b0be))
    - new error for `rev_parse_single` ([`9491528`](https://github.com/Byron/gitoxide/commit/9491528456310c458b35217dc6442d1c07a41c05))
    - Merge branch 'filter-refs' ([`e10554d`](https://github.com/Byron/gitoxide/commit/e10554d2a3b9c027353a432b0c84f7d3797b7cae))
    - rev_parse_single ([`d9097ab`](https://github.com/Byron/gitoxide/commit/d9097ab758611d94ad33e41e508c299796dbc77e))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - thanks clippy ([`ab81525`](https://github.com/Byron/gitoxide/commit/ab81525853db031e5f80d397c4074e2c85497e2e))
    - add `prompt` to top level forwarding #450) ([`1c13f11`](https://github.com/Byron/gitoxide/commit/1c13f1125664fbcc276a1ca440d168d07d0bf493))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - thanks clippy ([`34bc1b3`](https://github.com/Byron/gitoxide/commit/34bc1b33068122aa123397a1f4e5a1d9df244fa5))
    - Merge branch 'main' into filter-refs-by-spec ([`1f6e5ab`](https://github.com/Byron/gitoxide/commit/1f6e5ab15f5fd8d23719b13e6aea59cd231ac0fe))
    - Merge branch 'git_date_parse' ([`75591fb`](https://github.com/Byron/gitoxide/commit/75591fb108ce440ba2f920bebf99158b407e3046))
    - thanks clippy ([`82ee79e`](https://github.com/Byron/gitoxide/commit/82ee79e2bb7d87ed615a0cdda75d175a52978f7b))
    - thanks clippy ([`97e23dd`](https://github.com/Byron/gitoxide/commit/97e23ddb2c0d206efab393b984ba36aed11da72f))
    - Merge branch 'fix-522' ([`5869e9f`](https://github.com/Byron/gitoxide/commit/5869e9ff2508d5a93c07635277af8764fcb57713))
    - Release git-hash v0.9.9 ([`da0716f`](https://github.com/Byron/gitoxide/commit/da0716f8c27b4f29cfff0e5ce7fcb3d7240f4aeb))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`51dc828`](https://github.com/Byron/gitoxide/commit/51dc8282fb77b519ff7d2c94c6bd73af306cfe8b))
</details>

## 0.23.1 (2022-09-01)

### Changed (BREAKING)

 - <csr-id-36d8c57824a2b921012439105e49261fac66c955/> Remove 'unstable' feature.
   It's not worth maintaining it especially since everything is in
   pre-release mode right now.
   
   It might be something to re-introduce after go-live.

### Bug Fixes

 - <csr-id-d18e76cfb512ef7fe5bfee6e87726372c6a4a8b6/> `max-performance-safe` mode does not include zlib-ng adjustments anymore.
   git2 cannot handle this and fails to fetch packs after a couple of
   seconds.
   
   It's unclear what is causing this except that git2 doesn't like libz
   with zlibng support enabled, which happens if git2 in the
   same tree is with us.
 - Transitively through a kindly contributed fix in the `git-discover` crate, `Respository` can now be opened on `exFat` volumes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 21 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - adjust to changes in `git-credentials` ([`45ba6cb`](https://github.com/Byron/gitoxide/commit/45ba6cbe8b41944aef7e1a72d9fa2823bff46586))
    - test is using baseline exclusively now; get section's username as well ([`64a9d09`](https://github.com/Byron/gitoxide/commit/64a9d090fc7a4b8d4c3db060b59462c8867b1849))
    - get to the point where url matching plays a role. ([`94911f2`](https://github.com/Byron/gitoxide/commit/94911f26cd1cdf1229544b238e0bd11e34026317))
    - Remove 'unstable' feature. ([`36d8c57`](https://github.com/Byron/gitoxide/commit/36d8c57824a2b921012439105e49261fac66c955))
    - first baseline setup to trigger implementation to reproduce credential helper setup ([`22694df`](https://github.com/Byron/gitoxide/commit/22694df6bcca3bda8fc6dc10b18bbabd08523f18))
    - sketch the API for obtaining credential information ([`d56b9d5`](https://github.com/Byron/gitoxide/commit/d56b9d5bbd2d0e204cb5d315ad19eb9df472343f))
    - frame for baseline for credential helpers ([`55eb965`](https://github.com/Byron/gitoxide/commit/55eb9655ae66feea44aa742c7699d9b7f3e96582))
    - refactor ([`4c7799c`](https://github.com/Byron/gitoxide/commit/4c7799cb6fbac97e9d15e4e26243f2325002e2f9))
    - A sketch to allow obtaining a function for credentials by url on the remote ([`99fb1cf`](https://github.com/Byron/gitoxide/commit/99fb1cfa5093de2f4d0f1df7047e28082135e28c))
    - frame for overridable credential helpers ([`2916b08`](https://github.com/Byron/gitoxide/commit/2916b08c4b6503eb9de2362a37bada4d774dad17))
    - refactor ([`959fc09`](https://github.com/Byron/gitoxide/commit/959fc0921f3d01efe7932c02d426d20f30c8b9d3))
    - always compile prompting support in ([`bd0ea68`](https://github.com/Byron/gitoxide/commit/bd0ea68225a73fb83c9fc1b8594fc6ad288a77a9))
    - set version of git-prompt to 0.1 and turn prompting on ([`7657693`](https://github.com/Byron/gitoxide/commit/7657693b8e23dfb69d6da4376bcd1b8e4e264f7e))
 * **Uncategorized**
    - Release git-diff v0.18.1, git-discover v0.4.2, git-traverse v0.16.4, git-repository v0.23.1 ([`2571831`](https://github.com/Byron/gitoxide/commit/2571831e5939bf4ea6f19537b0c1ccd71dc99088))
    - prepare changelog  prior to release ([`fc6b958`](https://github.com/Byron/gitoxide/commit/fc6b9583d0534f70e0c8afdcad46e09a5001d62b))
    - Merge branch 'fix-git2-interactions' ([`b85fd4e`](https://github.com/Byron/gitoxide/commit/b85fd4e3ff814e56a14150870b7aac50e5e66d19))
    - `max-performance-safe` mode does not include zlib-ng adjustments anymore. ([`d18e76c`](https://github.com/Byron/gitoxide/commit/d18e76cfb512ef7fe5bfee6e87726372c6a4a8b6))
    - `parse` is pure function. ([`9ad1a5f`](https://github.com/Byron/gitoxide/commit/9ad1a5fa2ce54e978396ff3eaa7061a8edd10d4a))
    - `parse()` returns Result. ([`206f392`](https://github.com/Byron/gitoxide/commit/206f3923f5da2e9e26677e917550e6e5baa2913a))
    - Release git-path v0.4.2, git-config-value v0.7.0 ([`c48fb31`](https://github.com/Byron/gitoxide/commit/c48fb3107d29f9a06868b0c6de40567063a656d1))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.23.0 (2022-08-28)

### New Features

 - <csr-id-70aa850591de268488ae9bf2d3839a5c9c543c35/> The empty tree can always be returned by `Repository::(try_)find_object()`
   This matches the behaviour of git and libgit2.
   We conciously chose to only do this on the highest level, allowing lower
   levels to determine if the object exists or not.
 - <csr-id-8d0786646e17a82d20ca6b2799b54f6349d389f4/> Make `find::object::*::Error` publicly available.
 - <csr-id-2d0b63997b276a53b3cf8f09fac51f8e3f044bcd/> Add `Reference::delete()` for simple reference deletion
 - <csr-id-9170562059c3eaa529850025ef35ac5ffffc0fdf/> `Reference::set_target_id()` to easily set the target id of a reference
 - <csr-id-950da602925e6376b08640ed3ebfdf407394db34/> `Reference::head_ref()` to quickly access the reference the head points to.

### Bug Fixes

 - <csr-id-2834311b4f262c57e76627addaa4932196fd26b3/> `Commit::tree_id()` now returns a connected id

### New Features (BREAKING)

 - <csr-id-e090f843f5cffc8e8e47a8cac2e6fb98e4c47771/> `git-diff` is now included by default as part of core functionality

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 20 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - adjust to changes in `git-credentials` ([`dc32898`](https://github.com/Byron/gitoxide/commit/dc32898e9dda9cb8eee2bcad31cbe1c13d31f214))
    - adjust to changes in `git-credentials` ([`cabe40a`](https://github.com/Byron/gitoxide/commit/cabe40a15460cffd14618d4bb936e1f2805d687a))
    - refactor ([`7487b5a`](https://github.com/Byron/gitoxide/commit/7487b5a4142679ef423c5bd996e40e473c5dfc27))
    - refactor ([`335bb64`](https://github.com/Byron/gitoxide/commit/335bb64b19a79d8a1d3fe571145d5ed1c33c6417))
 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
    - The empty tree can always be returned by `Repository::(try_)find_object()` ([`70aa850`](https://github.com/Byron/gitoxide/commit/70aa850591de268488ae9bf2d3839a5c9c543c35))
    - Make `find::object::*::Error` publicly available. ([`8d07866`](https://github.com/Byron/gitoxide/commit/8d0786646e17a82d20ca6b2799b54f6349d389f4))
    - `git-diff` is now included by default as part of core functionality ([`e090f84`](https://github.com/Byron/gitoxide/commit/e090f843f5cffc8e8e47a8cac2e6fb98e4c47771))
    - `Commit::tree_id()` now returns a connected id ([`2834311`](https://github.com/Byron/gitoxide/commit/2834311b4f262c57e76627addaa4932196fd26b3))
 * **Uncategorized**
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - adjust to changes in `git-diff` ([`54954ee`](https://github.com/Byron/gitoxide/commit/54954ee5022a900f9f97baec63e9a073eca514e9))
    - fix docs ([`740c658`](https://github.com/Byron/gitoxide/commit/740c658e40eb8533bfb60d29c857f8693e355dba))
    - refactor ([`5892192`](https://github.com/Byron/gitoxide/commit/5892192cb246185981a11dc7aac96a07a47ed25a))
    - Add `Reference::delete()` for simple reference deletion ([`2d0b639`](https://github.com/Byron/gitoxide/commit/2d0b63997b276a53b3cf8f09fac51f8e3f044bcd))
    - `Reference::set_target_id()` to easily set the target id of a reference ([`9170562`](https://github.com/Byron/gitoxide/commit/9170562059c3eaa529850025ef35ac5ffffc0fdf))
    - `Reference::head_ref()` to quickly access the reference the head points to. ([`950da60`](https://github.com/Byron/gitoxide/commit/950da602925e6376b08640ed3ebfdf407394db34))
    - Release git-attributes v0.3.3, git-ref v0.15.3, git-index v0.4.3, git-worktree v0.4.3, git-testtools v0.8.0 ([`baad4ce`](https://github.com/Byron/gitoxide/commit/baad4ce51fe0e8c0c1de1b08148d8303878ca37b))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
    - Merge branch 'fix-ci-installation' ([`9245083`](https://github.com/Byron/gitoxide/commit/92450839621a4d99cb22d08cbf9f9a89ff6b9e3f))
</details>

## 0.22.1 (2022-08-24)

A maintenance release without user facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.1.0, git-actor v0.11.4, git-revision v0.4.3, git-repository v0.22.1, cargo-smart-release v0.11.0, git-commitgraph v0.8.2, gitoxide-core v0.17.0, gitoxide v0.15.0 ([`1fb931a`](https://github.com/Byron/gitoxide/commit/1fb931a7ea59f1cf895a6c1392fd8615b723c743))
    - update changelogs prior to release ([`23cb58f`](https://github.com/Byron/gitoxide/commit/23cb58f02043e0e5027136fd6e8e724c03a2efbe))
    - adjust to new version of git-date ([`b3fe26b`](https://github.com/Byron/gitoxide/commit/b3fe26bf03db7e1babb5ffbc89d71bf9614e3df3))
</details>

## 0.22.0 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes

### New Features

 - <csr-id-c28bcec19b5526acf888f928e6ddc4671873368e/> support avoiding usage of `fast-sha1` in git-features separately.
   That way one has an angle on compile failures in client libraries,
   see https://github.com/o2sh/onefetch/pull/752 for motivation.
 - <csr-id-4f87a0672f7288486a9b6403c0bb07a6279d2cfe/> `Repository::write_blob[_stream]()` to more easily write blobs.
   That way, one won't have to use the underlying `objects` database but
   can remain in the land of `Repository` enabled types for longer.
 - <csr-id-d35cd2a12c6db3d655ce10cad5c027bde99e19b4/> `SnapshotMut::apply_cli_overrides()` to make it easy to support things like `-c`
 - <csr-id-2a839f3209f3bd35e0c0f7edff664cc953059f65/> `Repository::config_snapshot_mut()` to mutate configuration values in memory.
   It's a first step towards writing changes back to disk, which can work
   already, but probably wouldn't as we currently don't localize changes
   to only one section type, i.e. Api, but instead may change values
   from other sections.
 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Bug Fixes

 - <csr-id-ff71730b4e3635533d9969d9dd44c0f3c75c6648/> Don't fail worktree tests if the system's git version is not supporting required features

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 54 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#488](https://github.com/Byron/gitoxide/issues/488)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - Respect `permissions.allow` ([`f8a5188`](https://github.com/Byron/gitoxide/commit/f8a51888d6f55efeb96bba13fdfc23a53781b4ba))
    - deal with changes in `git-url` ([`6f89659`](https://github.com/Byron/gitoxide/commit/6f89659bed6d44de2149c36ce188d816e11f5a64))
    - The first succeeding test for allowing schemes (without 'user' setting) ([`3ec28f0`](https://github.com/Byron/gitoxide/commit/3ec28f04622f7a2a99a47cde1376ff3088595e4d))
    - support for failing configuration, for now without 'lenient' config support ([`2bce6a3`](https://github.com/Byron/gitoxide/commit/2bce6a3f04be3ea9bcb731d05f7651357810c186))
    - sketch for lazy initialization of information to deal with allowing schemes ([`3f708f3`](https://github.com/Byron/gitoxide/commit/3f708f346f021662fa15867b5dbc2d42ccfc6fc8))
    - tests for `protocol.allow` ([`cd59965`](https://github.com/Byron/gitoxide/commit/cd59965f46398adc8f2ac29d2369b0f5f4e0c0a1))
    - Support for -c CLI config overrides in `gix config`. ([`19c1746`](https://github.com/Byron/gitoxide/commit/19c1746cefca9bc76537637ec99634eb4cf66a92))
    - remove TODO, doesn't apply anymore ([`dcd6619`](https://github.com/Byron/gitoxide/commit/dcd66197315a9826102b7439b1ab33e72593fccf))
    - fix docs ([`7d30eb3`](https://github.com/Byron/gitoxide/commit/7d30eb36e6aa7f679c97c5056cd5453f8e89ab10))
    - `SnapshotMut::apply_cli_overrides()` to make it easy to support things like `-c` ([`d35cd2a`](https://github.com/Byron/gitoxide/commit/d35cd2a12c6db3d655ce10cad5c027bde99e19b4))
    - Adjust to changes in `git-config` ([`8f4ad3c`](https://github.com/Byron/gitoxide/commit/8f4ad3cbd4c9254b6b234d9944d6298b5ca2bb60))
    - set remote protocol version using configuration instead of using a special mechanism. ([`1a74da5`](https://github.com/Byron/gitoxide/commit/1a74da5bb6969306f77663dfb8d63b04428d031f))
    - `Repository::config_snapshot_mut()` to mutate configuration values in memory. ([`2a839f3`](https://github.com/Byron/gitoxide/commit/2a839f3209f3bd35e0c0f7edff664cc953059f65))
    - fix build ([`5e924cb`](https://github.com/Byron/gitoxide/commit/5e924cb5d8e2a11cb4b44ec451c840136314da54))
    - Support for overriding the protocol version to use when connecting. ([`e5c53a8`](https://github.com/Byron/gitoxide/commit/e5c53a8d44914fd3e57b3d2cc2755210ea18e28b))
    - change connection API to be consuming, otherwise async mode doesn't work due to lack of async drop ([`129176f`](https://github.com/Byron/gitoxide/commit/129176f013052b6ef6eb37b4274fa68c1e0b11a3))
    - add docs ([`332a978`](https://github.com/Byron/gitoxide/commit/332a9784e61c102b46faa710ad9f6e5a208caa04))
    - refactor ([`eba5b13`](https://github.com/Byron/gitoxide/commit/eba5b13aa08229ff97f0a2be66ec80aadd4b9d1f))
    - A first working test to show all refs of a remote. ([`86c80e6`](https://github.com/Byron/gitoxide/commit/86c80e6db53fdc548221ab2dab2f84d66fef691f))
    - first vague implementation of ls-refs ([`563e56f`](https://github.com/Byron/gitoxide/commit/563e56f8f970f0bb0cf8a6404479422a398e712e))
    - Make `Remote::connect()` both sync and async. ([`f30db4c`](https://github.com/Byron/gitoxide/commit/f30db4c683fbd0250dce8073b3b2f3bd13e67d83))
    - prepare for refs implementation, it won't need a delegate anymore. ([`2a881ca`](https://github.com/Byron/gitoxide/commit/2a881ca1357897c049592d94c58ee1f005b47787))
 * **[#488](https://github.com/Byron/gitoxide/issues/488)**
    - Don't fail worktree tests if the system's git version is not supporting required features ([`ff71730`](https://github.com/Byron/gitoxide/commit/ff71730b4e3635533d9969d9dd44c0f3c75c6648))
 * **Uncategorized**
    - Release git-worktree v0.4.2, git-repository v0.22.0 ([`2f0f530`](https://github.com/Byron/gitoxide/commit/2f0f530fb1d644bc0694e04f3c9309149f526e42))
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - support avoiding usage of `fast-sha1` in git-features separately. ([`c28bcec`](https://github.com/Byron/gitoxide/commit/c28bcec19b5526acf888f928e6ddc4671873368e))
    - thanks clippy ([`dc74fbd`](https://github.com/Byron/gitoxide/commit/dc74fbd9a58e1d424713fc5f2442cedcc09c1200))
    - thanks clippy ([`c5bd452`](https://github.com/Byron/gitoxide/commit/c5bd45251ae0f47975e9fe77f0b9a9051e319d5c))
    - Release git-path v0.4.1 ([`5e82346`](https://github.com/Byron/gitoxide/commit/5e823462b3deb904f5d6154a7bf114cef1988224))
    - Merge branch 'example-write-blob' ([`afedd7f`](https://github.com/Byron/gitoxide/commit/afedd7f86ec8ea18a8165f71698ecc886f5cf643))
    - `Repository::write_blob[_stream]()` to more easily write blobs. ([`4f87a06`](https://github.com/Byron/gitoxide/commit/4f87a0672f7288486a9b6403c0bb07a6279d2cfe))
    - refactor ([`2faa919`](https://github.com/Byron/gitoxide/commit/2faa9192f32c5a8e57f003ba7fdef78403ba8509))
    - refactor ([`05ab87b`](https://github.com/Byron/gitoxide/commit/05ab87b6fc420656f314c24c616375f13e3b24ed))
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - Merge branch 'main' into remote-ls-refs ([`95f2f4f`](https://github.com/Byron/gitoxide/commit/95f2f4f17f7eae174a64c7d9f6a1513d73b21bbb))
    - thanks clippy ([`856f803`](https://github.com/Byron/gitoxide/commit/856f8031c607c120d34a08c51b2750e3f6d4d127))
    - write blob ([`a3bec66`](https://github.com/Byron/gitoxide/commit/a3bec66e87530ecbc8109f32a6252f3df2eebec9))
    - thanks clippy ([`f90d772`](https://github.com/Byron/gitoxide/commit/f90d772cf625afea605e42c92db15ed870d7e120))
    - Merge branch 'example-new-repo' ([`946dd3a`](https://github.com/Byron/gitoxide/commit/946dd3a80522ef437e09528a93aa1433f01b0ee8))
    - rename example init-repo-and-commit.rs ([`3630684`](https://github.com/Byron/gitoxide/commit/3630684ea4dba898108a7f394f37fd01ee812561))
    - add comments ([`04bf807`](https://github.com/Byron/gitoxide/commit/04bf807041fd00c18e3806ac3966f066e59b34db))
    - Use anyhow for error handling in example; prefer user-defined directory. ([`3a5169d`](https://github.com/Byron/gitoxide/commit/3a5169d6b4962b92a77ffafe9ec3b5eb4d676fbe))
    - Make clear what `tmp.into_path()` is for ([`e540a76`](https://github.com/Byron/gitoxide/commit/e540a763a3e08bba669652116cb696ab51ebc21f))
    - A more explicit name for the example ([`15aa860`](https://github.com/Byron/gitoxide/commit/15aa860fb96f78497d774f2537a46edbfc3d75af))
    - refactor ([`bfc3432`](https://github.com/Byron/gitoxide/commit/bfc34320e87f6ccee22674060da43457a4ced136))
    - git-repository example for creating new repo ([`367b809`](https://github.com/Byron/gitoxide/commit/367b809155c5c11153ae10e78691ef10c7fc6e90))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - fix typo in git-repository doc ([`05c6b42`](https://github.com/Byron/gitoxide/commit/05c6b421563c8fd464664bc007ca11041f8d7dd1))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - thanks clippy ([`581f8ae`](https://github.com/Byron/gitoxide/commit/581f8ae2313fa886d788feed74c10b4624e8de63))
    - Merge branch 'main' into remote-ls-refs ([`c82bbfa`](https://github.com/Byron/gitoxide/commit/c82bbfaddc45bf9b5b55f056613046d977d9ef09))
</details>

## 0.21.1 (2022-08-19)

A maintenance release that speeds up `commit.describe()` performance if `max_candidates()` is 0.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.4, git-actor v0.11.2, git-revision v0.4.1, git-repository v0.21.1 ([`2f9dc84`](https://github.com/Byron/gitoxide/commit/2f9dc847e0d54f4181ce35ddadd9286ba80ca01f))
    - prepare for release of git-repository ([`8aa5389`](https://github.com/Byron/gitoxide/commit/8aa5389d5a1bdd3a07f1caa1c2f55c8af4f9844a))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
    - greatly improve `gix commit describe` performance by adding an object cache ([`d07daaa`](https://github.com/Byron/gitoxide/commit/d07daaae8ed33161097f3007057c9993546ceb75))
</details>

## 0.21.0 (2022-08-17)

<csr-id-b38a212459e2646ab97ad7b5c24e54d962aae960/>

### Changed

 - <csr-id-0235111a4fcc40c7b57d973bfce27a66eddea901/> Invert behaviour to `open::Options::strict_config()`, with lenient being the default.
   This means API users will get libgit2 behaviour but commands like `gix` can
   change options to emulate `git` behaviour.

### New Features

 - <csr-id-a01525d159a33d6ad60a5324f2e9abbbe17fcfad/> `Kind` can now represent submodules.
   This should complete the list of git repository types and flavors.
 - <csr-id-5dac021bbbc5621167e7f49d62b11f68f76e42b6/> `open()` and `discover()` support opening submodules.
   This includes submodule checkouts as well as their original module git
   directories.
 - <csr-id-067c3342f3564dd7f152a720e93e3aa590ae6524/> `open::Options::lenient_config(…)` to default otherwise invalid configuration values where possible
   Originally required by https://github.com/starship/starship/issues/4266 .
 - <csr-id-0bf8371706d319681c3f794af5cd13f2f50a27d0/> support core.worktree option
 - <csr-id-b47bbb787ef2e31dd2612a56f9e7759ef8a188b8/> display for `object::tree::EntryRef`
 - <csr-id-727768a49c41165b03ddcdbc71ca88b66c330f32/> `Head::prior_checked_out_branches()`
 - <csr-id-ffe72918baf5c4c9f0f3709f75f068a663778588/> `Repository::index()` and `Worktree::index()`.
   These methods provide a possibly updated shared index.
 - <csr-id-47619f7c06a49dcf60a30e1a43a5352914183092/> add `Repository::object_cache_size_if_unset()`
 - <csr-id-d2611cee61841bc7bd978bef5af7dc66154248a2/> `Commit::message_raw_sloppy()` to provide yet another way to obtain a commit message.
 - <csr-id-906c95845fa4aa2d4390c522bb566a188b8c0e78/> add `rev_spec::parse::ObjectKindHint` to support `core.disambiguate`.
   The latter is seemingly undocumented in the typical place, git-config.
 - <csr-id-ef187f0180d89544d9015cbc2bc03d8cb51f4615/> `Remote::with_refspec()` to add new unique refspecs
 - <csr-id-d51ba42c643d8ee03a3c6b648f8524ff04827170/> `Remote::push_url()` to set it after the fact
 - <csr-id-9b07b91ad065836e7473df6635025659af2865ee/> `Repository::remote_at(…)` to create an unnamed remote
 - <csr-id-a67fc26b80e5d1183ddc5c6598396214f3e19945/> more conversions for `TryFrom`: `String` and `&str`
 - <csr-id-7a512ecdf236afc0b3d562d60fa81ab62c00cd9d/> `Head::into_remote()` to try really hard to find the correct remote
 - <csr-id-f392f26bec6069ac43ecd461b4f50e0def8b8972/> `Repository::remote_default_name()` to obtain the repo-wide remote for a a direction.
 - <csr-id-f47464f64f7c21500a24f563b25a8fc070c41778/> `Repository::branch_names()` to obtain branch names for which configuration exists.

### Bug Fixes

 - <csr-id-be6114e7c4ac48467db6acb2180b443dc9f59f32/> assure permissions per trust level are properly inherited into `open::Options`.
 - <csr-id-270242c707bd086b7746ee45b55791587f1484b1/> provide additional explanation about when to use `open::Options::with()`

### Refactor

 - <csr-id-b38a212459e2646ab97ad7b5c24e54d962aae960/> embrace `revision` module and move `rev_walk` there.
   Let's embrace the idea of structured modules and platforms in the right
   spot in the module hierarchy instead of forcing known names on it that
   over-simplify.

### Changed (BREAKING)

 - <csr-id-0deda0df55c11647f51374ed5a8bf11c932e2bae/> remove `permissions::Config::strict()` as they were unused internally.
   Furthermore, they were allowing everything as before so better not to
   have it.
 - <csr-id-1c12d49eefa6d79ef50b2960f41b29de680ac8eb/> rename `Repository::load_mailmap*` to `Repository::open_mailmap*`.
   For consistency with other similar methods.
 - <csr-id-ea35183b53f2ff71bdf2270ac4f7470a85d7756f/> remove `Repository::load_index()` in favor of `repo.worktree().open_index()`.
 - <csr-id-4fd096840ba27da6ce86678a85ede33e3be974ff/> `git_revision` is now available in `revision::plumbing`.
   That way it won't clash with the higher-level constructs on top of it
   which use the same names.
 - <csr-id-2424957cff75daacf6f6f14f74b9639f6875c4fb/> Turn `id::Ancestors` into general-purpose `RevWalk`.
 - <csr-id-1df379ab0046887a330c0a670ad0414e79cfae7b/> remove `Permissions::git_dir` field entirely.
   It was meant to help dealing with bailing out if the git dir isn't
   fully trusted, but the way this was done was over-engineered especially
   since the read-only permission level wasn't implemented at all.
   
   That function is now performed by a new flag, the `bail_on_untrusted`
   which is off by default.
 - <csr-id-5ab81ece15ec802ad4328ce31304233bd25b2929/> rename `Repository::remote_ref()` to `::branch_remote_ref()`

### New Features (BREAKING)

 - <csr-id-e2aff28e818951785d933f4b55b2f1b882729cb6/> `Repository::rev_parse()` returns a `RevSpec`.
   This lays the foundation for actually handling rev-specs faithfully.
   Previous users should use `rev_parse().single()` to obtain a single
   object id which was the only supported usecase previously.

### Bug Fixes (BREAKING)

 - <csr-id-c68b125a46f666700cdbda6f8cd39a044f4feb1b/> Don't panic for `@{1}` in new repos; rename `Head::into_referent()` to `::try_into_referent()`
   The signature change will prevent such issues in the future as one
   cannot simply ignore new repositories.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 189 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 30 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#427](https://github.com/Byron/gitoxide/issues/427), [#450](https://github.com/Byron/gitoxide/issues/450), [#482](https://github.com/Byron/gitoxide/issues/482)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 11 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - display for `object::tree::EntryRef` ([`b47bbb7`](https://github.com/Byron/gitoxide/commit/b47bbb787ef2e31dd2612a56f9e7759ef8a188b8))
    - make fmt ([`4b320e7`](https://github.com/Byron/gitoxide/commit/4b320e773368ac5e8c38dd8a779ef3d6d2d024ec))
    - `Head::prior_checked_out_branches()` ([`727768a`](https://github.com/Byron/gitoxide/commit/727768a49c41165b03ddcdbc71ca88b66c330f32))
    - improve docs ([`1e47bc1`](https://github.com/Byron/gitoxide/commit/1e47bc1741fe4d57213ed1dcec176faff712f508))
    - refactor ([`e67deab`](https://github.com/Byron/gitoxide/commit/e67deab56af20811f60757e27f1e21c81415f9cc))
    - implement @^{} syntax ([`fbd5aab`](https://github.com/Byron/gitoxide/commit/fbd5aab63cce905ca0fb482bfbb990624d7df376))
    - access to reflog entries ([`5cd06cf`](https://github.com/Byron/gitoxide/commit/5cd06cf668bcefe81c6feba8e9475d97d6debf43))
    - declare reflog access by date to be planned. ([`95bcf3b`](https://github.com/Byron/gitoxide/commit/95bcf3b9cc1963ab785d8b9f488779228205b01e))
    - officially make sibling branches 'planned' ([`145631b`](https://github.com/Byron/gitoxide/commit/145631bde653828cc35cad16ec2ec308cf204b23))
    - implement nth prior checkout ([`ff37fae`](https://github.com/Byron/gitoxide/commit/ff37fae10caf5c6546aa935d6f974f11926e1254))
    - test for nth prior checkout ([`4fd2314`](https://github.com/Byron/gitoxide/commit/4fd2314e30f7fc75b583b2cea87a932a9e91fff3))
    - refactor, add complex test for traversal ([`4f83470`](https://github.com/Byron/gitoxide/commit/4f83470e533d1214e3e0132c3adaf9dc30cb44ef))
    - support for ancestor traversal ([`ac2105f`](https://github.com/Byron/gitoxide/commit/ac2105f2e3dfdf2b1775e6479594daf0503b27cc))
    - refactor ([`6ffbf4c`](https://github.com/Byron/gitoxide/commit/6ffbf4c8f37fe379db2d4c9daa4a1432cf62e918))
    - support for parent traversal ([`aa80030`](https://github.com/Byron/gitoxide/commit/aa80030f3a57ae0edcffb4388156971bb8421f0a))
    - Index lookup works and provides hints en-par with git in terms of information at least. ([`a049bd3`](https://github.com/Byron/gitoxide/commit/a049bd3de924113a011057aa4e76a8df3b28d437))
    - failing tests for index rev-parsing ([`502d8c9`](https://github.com/Byron/gitoxide/commit/502d8c90464af35789e7725da831c81d41278bd1))
    - Make `git-index` non-optional and part of the standard setup. ([`c2e84a4`](https://github.com/Byron/gitoxide/commit/c2e84a40bd61583a46faaf9ecdbaf2a651a71c4e))
    - `Repository::index()` and `Worktree::index()`. ([`ffe7291`](https://github.com/Byron/gitoxide/commit/ffe72918baf5c4c9f0f3709f75f068a663778588))
    - rename `Repository::load_mailmap*` to `Repository::open_mailmap*`. ([`1c12d49`](https://github.com/Byron/gitoxide/commit/1c12d49eefa6d79ef50b2960f41b29de680ac8eb))
    - remove `Repository::load_index()` in favor of `repo.worktree().open_index()`. ([`ea35183`](https://github.com/Byron/gitoxide/commit/ea35183b53f2ff71bdf2270ac4f7470a85d7756f))
    - tests for reference name retrieval. ([`7a8c8f3`](https://github.com/Byron/gitoxide/commit/7a8c8f3f23be3738401699af10719ba1bbe94eff))
    - add `Repository::object_cache_size_if_unset()` ([`47619f7`](https://github.com/Byron/gitoxide/commit/47619f7c06a49dcf60a30e1a43a5352914183092))
    - Use `Display` for revision printing instead of `Debug` ([`d194f15`](https://github.com/Byron/gitoxide/commit/d194f155212d71d7d258b5a3e63ce12e8327f158))
    - adapt to changes in `git-revision` ([`65b337d`](https://github.com/Byron/gitoxide/commit/65b337d8ae4b64374ba1a010c3ca9e945e53d44e))
    - Assure only commits serve as starting point ([`5ad0f96`](https://github.com/Byron/gitoxide/commit/5ad0f960a0349f538b954dcf5b28d48d8b396ac7))
    - test regex negation, which brought up a traversal ordering bug ([`7b1733e`](https://github.com/Byron/gitoxide/commit/7b1733ecb8d28065b29faaa78198c575ff711ab4))
    - multi-tip regex work ([`4ca4919`](https://github.com/Byron/gitoxide/commit/4ca49194b6e60c0a7a3a7c184496384a3aec3360))
    - A way to obtain a rev-walk platform directly from the top-level repo. ([`835dcf4`](https://github.com/Byron/gitoxide/commit/835dcf46323905387e1e09b15d74d97be2478d30))
    - Only run regex based searches if a substring search won't do ([`295bf9f`](https://github.com/Byron/gitoxide/commit/295bf9f623eb9dcda581d5ce384f4fcfd72015a2))
    - fix docs ([`25fd8fe`](https://github.com/Byron/gitoxide/commit/25fd8fe62b5c313ec18233b8480014ef3d3b7434))
    - a first failing test for regex-search. ([`13e0938`](https://github.com/Byron/gitoxide/commit/13e09380253695505688097a093a87346d4362d5))
    - make clear in error text if regex aren't actually used. ([`cfb8c40`](https://github.com/Byron/gitoxide/commit/cfb8c40a8a38731a5b4687d3fa2e90186d7decf6))
    - Move `RevSpec` to `revision::Spec`. ([`1b8df18`](https://github.com/Byron/gitoxide/commit/1b8df1821d7f18c3a2bcb28a5586389ce37ab24e))
    - refactor ([`e05aa3b`](https://github.com/Byron/gitoxide/commit/e05aa3b05118c9946524b8ff1324970489b64976))
    - align tests to upcoming structure of RevSpec ([`41d6dd2`](https://github.com/Byron/gitoxide/commit/41d6dd201bf4e6ec78c4374258eebe05ed0362b8))
    - `git_revision` is now available in `revision::plumbing`. ([`4fd0968`](https://github.com/Byron/gitoxide/commit/4fd096840ba27da6ce86678a85ede33e3be974ff))
    - embrace `revision` module and move `rev_walk` there. ([`b38a212`](https://github.com/Byron/gitoxide/commit/b38a212459e2646ab97ad7b5c24e54d962aae960))
    - Turn `id::Ancestors` into general-purpose `RevWalk`. ([`2424957`](https://github.com/Byron/gitoxide/commit/2424957cff75daacf6f6f14f74b9639f6875c4fb))
    - git sorts commit traversals and so do we ([`538ecd4`](https://github.com/Byron/gitoxide/commit/538ecd4c9896896c4b3068464f20b840ca16bf44))
    - Use prefix consisently when reporting object ids ([`bf9e27b`](https://github.com/Byron/gitoxide/commit/bf9e27bd5df5de756bd1885b0ec07263d517f0ca))
    - `Commit::message_raw_sloppy()` to provide yet another way to obtain a commit message. ([`d2611ce`](https://github.com/Byron/gitoxide/commit/d2611cee61841bc7bd978bef5af7dc66154248a2))
    - support for regex-based matching for single-tips ([`6369153`](https://github.com/Byron/gitoxide/commit/63691530c2c8193809500b1f7debe8993e021c6a))
    - assure regex search failure is registered as such ([`b58d7cb`](https://github.com/Byron/gitoxide/commit/b58d7cbf424530d9ebc875d3b54a0f53e64d3086))
    - non-regex implementation of single-tip search for commit messages ([`7ad4e54`](https://github.com/Byron/gitoxide/commit/7ad4e54126614f253c17ff44aa3df7bff24a56a2))
    - Provide an error if regex is not compiled in but is used ([`838a6ba`](https://github.com/Byron/gitoxide/commit/838a6ba94c34449996cc2507bb935e9870181214))
    - frame for optional regex support in git-repository ([`7f43d95`](https://github.com/Byron/gitoxide/commit/7f43d955ecc6639b77e255eaaae5d2eae2583c7c))
    - Add complex repositry example similar to the one by Jon Loeliger ([`1e0b431`](https://github.com/Byron/gitoxide/commit/1e0b4310d7a15b74b781b16993d27f5288fa4912))
    - assure all forms of ranges/merge-bases disambiguate equally ([`4fdc120`](https://github.com/Byron/gitoxide/commit/4fdc1202f040bd0321ef3fcb34c66d56ff7c6b03))
    - fix tests that was flaky due to time-dependent comparison. ([`9fa9850`](https://github.com/Byron/gitoxide/commit/9fa98501b7f63c68f5685d6da5f79680035baa92))
    - follow refs as well when resolving names to ids. ([`34c8140`](https://github.com/Byron/gitoxide/commit/34c8140a564d8b12f904f595ca98b6b1b72ddbbc))
    - Correctly disambiguate objects in ranges without falling back to repo-disambiguation configuration ([`17a1edf`](https://github.com/Byron/gitoxide/commit/17a1edf6e77b7ece1f79e59d6230375ad9c9653c))
    - Disambiguation of ranges by committish works ([`8b0ceb5`](https://github.com/Byron/gitoxide/commit/8b0ceb5f71938cd18968ca1b90f019f6b3ad6c14))
    - first failing test to check for range disambiguation ([`a5eb4fb`](https://github.com/Byron/gitoxide/commit/a5eb4fbdb6c3bf6acffbc0cb21fe3cd738f732f0))
    - better error messages in case all ambiguous objects fail a transformation ([`97922f8`](https://github.com/Byron/gitoxide/commit/97922f8c8337145f6a8432d2c35e3bb054bfdad7))
    - Make use of `git-revision::Spec` in `RevSpec` data structure. ([`004915e`](https://github.com/Byron/gitoxide/commit/004915ea118e3ea6b710aa405eedc6a7a5a1a1f3))
    - sketch the new version of the RevSpec data structure ([`98d32c6`](https://github.com/Byron/gitoxide/commit/98d32c67b462bcb9498ce581e0e92a13a12d5d3b))
    - sketch data structure for actually using baseline range results ([`f6da78f`](https://github.com/Byron/gitoxide/commit/f6da78f9905f66c4fecdc7c1b14b47132687c260))
    - basic parsing of range baseline ([`b6013b6`](https://github.com/Byron/gitoxide/commit/b6013b631b310cd221a07bf29f57111a2e186203))
    - adjust to change in git-revision ([`51762bb`](https://github.com/Byron/gitoxide/commit/51762bb627a9e001a5a3eebc4ec4a7cfc30dbb9e))
    - adjust to changes in git-revision ([`df7da1f`](https://github.com/Byron/gitoxide/commit/df7da1f575111cd8de0155271b6d220b92174eb7))
    - fix docs ([`5425de9`](https://github.com/Byron/gitoxide/commit/5425de974373a504daeeea7adeadc9beeb3023d0))
    - Adjust RevSpec::range() to match changes in `git-revision` ([`05ea453`](https://github.com/Byron/gitoxide/commit/05ea45337e85583db5e57f14e995be49ba888ee1))
    - adjust to changes in `git-revision` ([`a70f262`](https://github.com/Byron/gitoxide/commit/a70f26274bb2d67428f4917882a94a8fc8b648c8))
    - All disambiguation tests work as good as git or better. ([`c397761`](https://github.com/Byron/gitoxide/commit/c397761387f94e06ed3bdcb0236294465cfe8b6d))
    - re-enable more tests ([`04e1558`](https://github.com/Byron/gitoxide/commit/04e1558012bf2a33ba154967fbe4770440dc5d22))
    - more tests, still in progress ([`8d92eb6`](https://github.com/Byron/gitoxide/commit/8d92eb6cb0d2a7740e02ee95b3a3c56581c582b2))
    - improve error messages related to peeling ([`b61a343`](https://github.com/Byron/gitoxide/commit/b61a3439afe739ffe68e0eb3cc05d90e03cae3a5))
    - `gix rev parse` now uses `Repository::rev_parse()` ([`e191681`](https://github.com/Byron/gitoxide/commit/e191681a9e700d8a49e2ab6ffb19cfc5f43312a5))
    - also maintain git-style sort order of objects ([`c14754a`](https://github.com/Byron/gitoxide/commit/c14754a0cce159a6e4306c43a44790a8474a80a9))
    - git-style disambiguation errors ([`5717194`](https://github.com/Byron/gitoxide/commit/57171946081c03da98f3d33f5b963c3bc4b2d6d9))
    - refactor ([`6f7823f`](https://github.com/Byron/gitoxide/commit/6f7823f5e6b8d95ccfa2770031ab36724e11beb0))
    - refactor; prepare for detailed ambiguous object information ([`0a08583`](https://github.com/Byron/gitoxide/commit/0a085831c2d74d30aea7119be16bd2f4aa969e7e))
    - refactor ([`017727a`](https://github.com/Byron/gitoxide/commit/017727ab2e830b2593a6381e72654ab5cbeacb38))
    - refactor ([`87e7d97`](https://github.com/Byron/gitoxide/commit/87e7d974faac58bf90c6e291514482c5c54219c9))
    - refactor ([`1d2ef52`](https://github.com/Byron/gitoxide/commit/1d2ef52e9760b9c637f84cc9c22f8e3f0be5bd08))
    - a way to not degenerate information when chaining errors ([`8d723ad`](https://github.com/Byron/gitoxide/commit/8d723ad81a81c42c30af708a3afdb4afe64c53c8))
    - compare all configurable disambiguation types against baseline ([`c98195b`](https://github.com/Byron/gitoxide/commit/c98195bd1dbee48004a3e69f89b77da27c035a2b))
    - refactor ([`929308d`](https://github.com/Byron/gitoxide/commit/929308d3e83c97b9f23a3ba08d96118c4aad5312))
    - more impelmentation of object disambiguation ([`9127d15`](https://github.com/Byron/gitoxide/commit/9127d15dabe0682a0dfe265da6ee5e5f75f9eff3))
    - first repository-local disambiguation works ([`31db570`](https://github.com/Byron/gitoxide/commit/31db5703f5cd677ab5bdca91af1abf803de8ed4f))
    - a general setup to peel objects while managing candidates. ([`c418527`](https://github.com/Byron/gitoxide/commit/c4185275ac32b6416df5f00522b982b029076190))
    - set foundation for core.disambiguate implementation ([`96cb5ee`](https://github.com/Byron/gitoxide/commit/96cb5eed35165addffc22900a436771063ffb316))
    - parse `core.disambiguate` from configuration and cache it. ([`335c459`](https://github.com/Byron/gitoxide/commit/335c4590c8f11f43b3aa9c9602ec8fad0d22c0ec))
    - add `rev_spec::parse::ObjectKindHint` to support `core.disambiguate`. ([`906c958`](https://github.com/Byron/gitoxide/commit/906c95845fa4aa2d4390c522bb566a188b8c0e78))
    - `Repository::rev_parse()` returns a `RevSpec`. ([`e2aff28`](https://github.com/Byron/gitoxide/commit/e2aff28e818951785d933f4b55b2f1b882729cb6))
    - improve error message for Fail mode with ref matching as well as object(s) ([`1ef7281`](https://github.com/Byron/gitoxide/commit/1ef728112e71cef737699a7745acb9ae97e58fce))
    - add support for keeping multiple candidates in case of ambiguous objects. ([`8b4e5e0`](https://github.com/Byron/gitoxide/commit/8b4e5e0e59a3c363c2b0a40dd5ca526409bcaafc))
    - refactor ([`2b2cb6d`](https://github.com/Byron/gitoxide/commit/2b2cb6d68aaecfbe81385c5fff5d8cc88527fdc0))
 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - sketch of simple delegate to collect listed refs ([`1c5f561`](https://github.com/Byron/gitoxide/commit/1c5f5617940efe818a5e2aca5afe2cbd7f4ad940))
    - fix docs ([`c2dfe33`](https://github.com/Byron/gitoxide/commit/c2dfe338a91a1899ecf4e1eecbab708a2f6bac38))
    - Rough sketch of the `Connection` API to list references ([`73cb41c`](https://github.com/Byron/gitoxide/commit/73cb41cf0cc0785c87319b25c72b8b5552f81666))
    - A first sketch on how connections could be working ([`e55b43e`](https://github.com/Byron/gitoxide/commit/e55b43ef72bb3f23655c7e0884b8efcf2496f944))
    - refactor ([`ad101ef`](https://github.com/Byron/gitoxide/commit/ad101ef973afe559e71de78152a6a25b310d28dd))
    - adapt to changes in `git-transport` and `git-url ([`4ae2390`](https://github.com/Byron/gitoxide/commit/4ae2390578e086705c640fa74d273e1f82c9ab62))
    - refactor ([`b3362ae`](https://github.com/Byron/gitoxide/commit/b3362ae76b1c0ba0291412c2f96941a522860cf2))
    - a sketch for the remote connection API, for async and blocking ([`f933ae3`](https://github.com/Byron/gitoxide/commit/f933ae3dea69bd7d432aaf47de62f2ecbb31605c))
    - `Remote::with_refspec()` to add new unique refspecs ([`ef187f0`](https://github.com/Byron/gitoxide/commit/ef187f0180d89544d9015cbc2bc03d8cb51f4615))
    - refactor ([`bf47405`](https://github.com/Byron/gitoxide/commit/bf47405234ba9915d77b64d4a5c1a372be102001))
    - refactor ([`7581177`](https://github.com/Byron/gitoxide/commit/75811778d067ec68442bc0700514935977ac4447))
    - Make explicit url rewrites more forgiving similar to how git does it ([`e7b451d`](https://github.com/Byron/gitoxide/commit/e7b451d15751923c002c0e67ed9b8defd27127e0))
    - a test for handling bad rewrite urls and its implications ([`c2afd87`](https://github.com/Byron/gitoxide/commit/c2afd874aa64e56223af0671964acf995706484d))
    - fix docs ([`dbc6f5d`](https://github.com/Byron/gitoxide/commit/dbc6f5da51417842a722b8b3576b6ea21f4dd885))
    - adapt to changes in `git-url` ([`f0f5ee6`](https://github.com/Byron/gitoxide/commit/f0f5ee602fb46741114affed076716ac12b95138))
    - Add escape-hatch to eliminate rewrite rule failures on instantiation ([`897c8c1`](https://github.com/Byron/gitoxide/commit/897c8c19ca8566834fcb9c9bf5c372451c473760))
    - `Remote::push_url()` to set it after the fact ([`d51ba42`](https://github.com/Byron/gitoxide/commit/d51ba42c643d8ee03a3c6b648f8524ff04827170))
    - `Repository::remote_at(…)` to create an unnamed remote ([`9b07b91`](https://github.com/Byron/gitoxide/commit/9b07b91ad065836e7473df6635025659af2865ee))
    - more conversions for `TryFrom`: `String` and `&str` ([`a67fc26`](https://github.com/Byron/gitoxide/commit/a67fc26b80e5d1183ddc5c6598396214f3e19945))
    - `Head::into_remote()` to try really hard to find the correct remote ([`7a512ec`](https://github.com/Byron/gitoxide/commit/7a512ecdf236afc0b3d562d60fa81ab62c00cd9d))
    - refactor ([`ba1c162`](https://github.com/Byron/gitoxide/commit/ba1c1622d848769784f5f2eaf7945f29cc8a864e))
    - remote-name by reference, which can be useful to find remotes with multiple fallbacks ([`92c0aa3`](https://github.com/Byron/gitoxide/commit/92c0aa343e5cba86dc4b2d4006927542610bc802))
    - refactor ([`f41e588`](https://github.com/Byron/gitoxide/commit/f41e588595ff179abc39817dd1fa9f39fb14e6c0))
    - refactor ([`2905e1b`](https://github.com/Byron/gitoxide/commit/2905e1b0c5d75214fc8dc279f149e1b3bc8caaf3))
    - refactor ([`6c15bf4`](https://github.com/Byron/gitoxide/commit/6c15bf450066525df439df1f719a0bd4720730cc))
    - support for instant url rewriting (trusted values), with option to use the originals. ([`76f76f5`](https://github.com/Byron/gitoxide/commit/76f76f533c5cc8e44fc20a05ee31c0c1a0122cc3))
    - failing test for url rewrites ([`58aee33`](https://github.com/Byron/gitoxide/commit/58aee3395c0a70d1659df99d2fe4953676dbe346))
    - Fix windwos errors, hopefully ([`0fbbe34`](https://github.com/Byron/gitoxide/commit/0fbbe346571bdade15346fdf6978c3a360845d06))
    - Allow to use `git-path` at all times ([`b4f6bbd`](https://github.com/Byron/gitoxide/commit/b4f6bbd10f4aa6a8d7cd1e57a462514cbc0ebb94))
    - fix tests on windows ([`dc0186e`](https://github.com/Byron/gitoxide/commit/dc0186ef72812b6362b17e7a21ecf5014cd202c5))
    - deduplicate refs when reading them ([`60780cc`](https://github.com/Byron/gitoxide/commit/60780cc3a341e3de744f949c428f05e31dc8ffab))
    - better error reporting for ref-spec parsing ([`fcea9d1`](https://github.com/Byron/gitoxide/commit/fcea9d1c48d84d30893d3e15272abd85a26bb4e2))
    - valid push-url and push-specs as well ([`214dd16`](https://github.com/Byron/gitoxide/commit/214dd1694c7f29b250e515ab4128a303d6ffac97))
    - tests to validate typical remotes can be instantiated ([`72545dd`](https://github.com/Byron/gitoxide/commit/72545ddce8cb9e1399336526a3ffc8311fb1195a))
    - first sketch of finding remotes ([`0e57aa2`](https://github.com/Byron/gitoxide/commit/0e57aa24a96dfb94da02c78bbc03a0d3010c80c1))
    - make git-url public for good ([`2f7960f`](https://github.com/Byron/gitoxide/commit/2f7960f55ead318cedded2b8041df31233f8a11b))
    - sketch Remote type for implementing find_remote() ([`9495947`](https://github.com/Byron/gitoxide/commit/94959472e1a40e79d7894ff732512ef03066d22b))
    - `Repository::remote_default_name()` to obtain the repo-wide remote for a a direction. ([`f392f26`](https://github.com/Byron/gitoxide/commit/f392f26bec6069ac43ecd461b4f50e0def8b8972))
    - `Repository::branch_names()` to obtain branch names for which configuration exists. ([`f47464f`](https://github.com/Byron/gitoxide/commit/f47464f64f7c21500a24f563b25a8fc070c41778))
    - Assure remote-names are unique and we don't double-count sections. ([`7ef35b2`](https://github.com/Byron/gitoxide/commit/7ef35b2d67b74be8420b821d5a477bad56d2026b))
    - rename `Repository::remote_ref()` to `::branch_remote_ref()` ([`5ab81ec`](https://github.com/Byron/gitoxide/commit/5ab81ece15ec802ad4328ce31304233bd25b2929))
    - Add test to list remote names ([`2b21ac5`](https://github.com/Byron/gitoxide/commit/2b21ac5948623beadf8e89c3d0030886f3fdaeee))
    - first sketch of method to access remote names ([`bca9fe9`](https://github.com/Byron/gitoxide/commit/bca9fe91c015633ed83e9e8ba248a16a0fdbddd6))
    - refactor ([`0f97c44`](https://github.com/Byron/gitoxide/commit/0f97c44cf5f52fbd4431cddcbff188c791fe667e))
 * **[#482](https://github.com/Byron/gitoxide/issues/482)**
    - Bring back conversion from discovery kind to `git-repository::Kind` ([`ebb5bee`](https://github.com/Byron/gitoxide/commit/ebb5bee7b71272013f43f18a5a7ce48eccb587a0))
    - `Kind` can now represent submodules. ([`a01525d`](https://github.com/Byron/gitoxide/commit/a01525d159a33d6ad60a5324f2e9abbbe17fcfad))
    - `open()` and `discover()` support opening submodules. ([`5dac021`](https://github.com/Byron/gitoxide/commit/5dac021bbbc5621167e7f49d62b11f68f76e42b6))
    - Add archive for submodule test ([`7ab3279`](https://github.com/Byron/gitoxide/commit/7ab32793b7eb70a5d5c47da456dc37d788a0e58b))
    - test showing that submodules dirs can't be opened right now ([`5a9c537`](https://github.com/Byron/gitoxide/commit/5a9c537ab00c9acfde201a3296a64c96c8c18424))
 * **Uncategorized**
    - Release git-worktree v0.4.1, git-repository v0.21.0 ([`ee383f3`](https://github.com/Byron/gitoxide/commit/ee383f347371007f1c4d3a2a98c5511d7e0793a8))
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Merge branch 'submodule-open' ([`8f5f3ab`](https://github.com/Byron/gitoxide/commit/8f5f3ab588cf0165d50a82365119ad5804745017))
    - Merge branch 'core-abbrev-handling' ([`dbaff13`](https://github.com/Byron/gitoxide/commit/dbaff13eaf7ba5f9c0ee2c90ac9f17e6078cad9e))
    - Invert behaviour to `open::Options::strict_config()`, with lenient being the default. ([`0235111`](https://github.com/Byron/gitoxide/commit/0235111a4fcc40c7b57d973bfce27a66eddea901))
    - `open::Options::lenient_config(…)` to default otherwise invalid configuration values where possible ([`067c334`](https://github.com/Byron/gitoxide/commit/067c3342f3564dd7f152a720e93e3aa590ae6524))
    - thanks clippy ([`8f730ae`](https://github.com/Byron/gitoxide/commit/8f730ae47b0d9765b51b8b04500ca9e70a1ca743))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge branch 'main' into remote-ls-refs ([`c4bf958`](https://github.com/Byron/gitoxide/commit/c4bf9585d815bc342e5fb383336cc654280dd34f))
    - Merge branch 'index-write-refactor' ([`805f432`](https://github.com/Byron/gitoxide/commit/805f432bf8e9d2dd9ede56caf959de386d5d80c7))
    - first PoC for writing long paths, even though it doens't produce the entire file yet ([`581cbd7`](https://github.com/Byron/gitoxide/commit/581cbd7afeac0f7654611c83deacae802ef5da6f))
    - adjust `git_date::parsea(str)` to use a str ([`0f8680a`](https://github.com/Byron/gitoxide/commit/0f8680a60913556b7fbd7543fda6a409ac05b121))
    - Merge branch 'format_git_date_time' ([`99e12be`](https://github.com/Byron/gitoxide/commit/99e12bee16ab3f344c71818bfd1c95cf50e1721b))
    - refactor ([`bd64387`](https://github.com/Byron/gitoxide/commit/bd64387d8ad3377571755dff14577cc3c53ee9cc))
    - Merge branch 'main' into write-index-v2 ([`a938986`](https://github.com/Byron/gitoxide/commit/a938986877302c197d1aed087594c5605416fe5f))
    - Merge branch 'main' into remote-ls-refs ([`de61c4d`](https://github.com/Byron/gitoxide/commit/de61c4db7855d6925d66961f62ae3d12cc4acf78))
    - thanks clippy ([`4bd747c`](https://github.com/Byron/gitoxide/commit/4bd747cb3e126fe5b1d540270cfbd731cffd42ef))
    - thanks clippy ([`80e4ab7`](https://github.com/Byron/gitoxide/commit/80e4ab782ebb23bb553b6e47209753a2bd8d05a1))
    - Merge branch 'main' into remote-ls-refs ([`e8fc89d`](https://github.com/Byron/gitoxide/commit/e8fc89d36ab17a66e799bdec3ed71388b9730266))
    - Don't panic for `@{1}` in new repos; rename `Head::into_referent()` to `::try_into_referent()` ([`c68b125`](https://github.com/Byron/gitoxide/commit/c68b125a46f666700cdbda6f8cd39a044f4feb1b))
    - Merge branch 'feat-core-worktree' ([`df42d22`](https://github.com/Byron/gitoxide/commit/df42d22b39f0dba113a10d66dcd1a151d1eb1a76))
    - remove `permissions::Config::strict()` as they were unused internally. ([`0deda0d`](https://github.com/Byron/gitoxide/commit/0deda0df55c11647f51374ed5a8bf11c932e2bae))
    - test absolute worktree dirs as well when overridden in core.worktree ([`4e17864`](https://github.com/Byron/gitoxide/commit/4e178640d64cc88e42b1410f46592d0d6dd7e1b9))
    - validate core.worktree handling in bare repositories ([`cba6983`](https://github.com/Byron/gitoxide/commit/cba69837773c9fad48c01ff175f5bae29b8a7923))
    - Add more tests around invalid `core.worktree` values ([`1591a50`](https://github.com/Byron/gitoxide/commit/1591a50d1a2cba027b9ee88a64fa90b282c849fb))
    - move tests into `worktree` module and prepare for more of them ([`a05b15b`](https://github.com/Byron/gitoxide/commit/a05b15b1b73d5589881df95f4cf50361434e0660))
    - Use time format strings. ([`f84e8f5`](https://github.com/Byron/gitoxide/commit/f84e8f5f16ec2197d1967fb1cc06e9609ea52c16))
    - thanks clippy ([`f84360c`](https://github.com/Byron/gitoxide/commit/f84360ca56be1ec9d95ad03566932622d9b0d2a6))
    - support core.worktree option ([`0bf8371`](https://github.com/Byron/gitoxide/commit/0bf8371706d319681c3f794af5cd13f2f50a27d0))
    - thanks clippy ([`4347a96`](https://github.com/Byron/gitoxide/commit/4347a96df7742dd1b2b1e0d56543ba16612b7924))
    - thanks clippy ([`c57cb6f`](https://github.com/Byron/gitoxide/commit/c57cb6f14c8add07398107e25545a7bc699afe1a))
    - refactor ([`556dd8c`](https://github.com/Byron/gitoxide/commit/556dd8cb78ea9321031984e2c6b4f9bc415f1be5))
    - Format `git-date::Time` with `time::format_description`. ([`d4243bc`](https://github.com/Byron/gitoxide/commit/d4243bc4feb994bde99156ba77fff63bc9c875e9))
    - support for `@:` == `@^{tree}` in rev-parsing ([`6c06406`](https://github.com/Byron/gitoxide/commit/6c064067d2d2cde1c987928c0a44526841f1749f))
    - Merge branch 'rev-parse-delegate' ([`2f506c7`](https://github.com/Byron/gitoxide/commit/2f506c7c2988477b0f97d272a9ac9ed47b236457))
    - thanks clippy ([`90dccc3`](https://github.com/Byron/gitoxide/commit/90dccc3d31340f91f7847c7b1ed8c32c96696021))
    - thanks clippy ([`df83e23`](https://github.com/Byron/gitoxide/commit/df83e23cc0b0ea486bf139c6ccb3a25c2604a323))
    - thanks clippy ([`6163caa`](https://github.com/Byron/gitoxide/commit/6163caa19ad23f7bc24e7a7c35026b03c61642be))
    - thanks clippy ([`d8511bb`](https://github.com/Byron/gitoxide/commit/d8511bb3025fa447b7a325ac41de41f25edead3b))
    - thanks clippy ([`a479bd3`](https://github.com/Byron/gitoxide/commit/a479bd3079c54693f82277dfde6068e7b401fb12))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - remove `Permissions::git_dir` field entirely. ([`1df379a`](https://github.com/Byron/gitoxide/commit/1df379ab0046887a330c0a670ad0414e79cfae7b))
    - assure permissions per trust level are properly inherited into `open::Options`. ([`be6114e`](https://github.com/Byron/gitoxide/commit/be6114e7c4ac48467db6acb2180b443dc9f59f32))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - provide additional explanation about when to use `open::Options::with()` ([`270242c`](https://github.com/Byron/gitoxide/commit/270242c707bd086b7746ee45b55791587f1484b1))
    - make fmt ([`47724c0`](https://github.com/Byron/gitoxide/commit/47724c0edb382c036a3fc99884becfd2b0740d4b))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.20.0 (2022-07-22)

### New Features

 - <csr-id-1b765ec6ae70d1f4cc5a885b3c68d6f3335ba827/> respect `safe.directory`.
   In practice, this code will rarely be hit as it would require very
   strict settings that forbid any operation within a non-owned git
   directory.
 - <csr-id-840d9a3018d11146bb8e80fc92693c65eb534d91/> permissions for configuration.
   It provides fine-grained control over what sources to load.
 - <csr-id-657080829867d9dcb0c9b9cb6c1c8126c4df3783/> `git-config` is now accessible in `git-repository::config`.
 - <csr-id-d99453ebeb970ed493be236def299d1e82b01f83/> `gix config` lists all entries of all configuration files git considers.
   Filters allow to narrow down the output.
 - <csr-id-ebedd03e119aa5d46da07e577bfccad621eaecb5/> repository now initializes global configuration files and resolves includes
 - <csr-id-de8572ff2ced9422832e1ba433955c33f0994675/> resolve includes in local repository configuration
 - <csr-id-d5a48b82230b047434610550aacd2dc741b4b5f0/> `config::Snapshot::trusted_path()` to obtain trustworthy paths.
   We also apply trust-based config query during initialization to assure
   we don't use paths which aren't owned by the current user.
 - <csr-id-5f9bfa89ceb61f484be80575b0379bbf9d7a36b3/> `Repository::config_snapshot()` to access configuration values.
 - <csr-id-7f67b23b9462b805591b1fe5a8406f8d7404f372/> Use `git-config` to write config file on initialization, including `logallrefupdates` and `precomposeunicode`.
 - <csr-id-e263e13d312e41aa1481d104fa79ede509fbe1c5/> respect `core.logallrefupdates` configuration setting.

### Changed (BREAKING)

 - <csr-id-68f4bc2570d455c762da7e3d675b9b507cec69bb/> Make `SignatureRef<'_>` mandatory for editing reference changelogs.
   If defaults are desired, these can be set by the caller.
 - <csr-id-f932cea68ece997f711add3368db53aeb8cdf064/> `Repository::committer()` now returns an `Option`, see `::committer_or_default()` for a method that doesn't.
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

### New Features (BREAKING)

 - <csr-id-d003c0f139d61e3bd998a0283a9c7af25a60db02/> Support for `lossy` load mode.
   There is a lot of breaking changes as `file::from_paths::Options` now
   became `file::init::Options`, and the same goes for the error type.
 - <csr-id-311d4b447daf8d4364670382a20901468748d34d/> change mostily internal uses of [u8] to BString/BStr

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 109 commits contributed to the release over the course of 38 calendar days.
 - 39 days passed between releases.
 - 16 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#331](https://github.com/Byron/gitoxide/issues/331), [#427](https://github.com/Byron/gitoxide/issues/427)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Make lossy-configuration configurable ([`b0e4da6`](https://github.com/Byron/gitoxide/commit/b0e4da621114d188a73b9f40757f59564da3c079))
    - tests for author/committer/user ([`6d2e53c`](https://github.com/Byron/gitoxide/commit/6d2e53c32145770e8314f0879d6d769090667f90))
    - refactor ([`4dc6594`](https://github.com/Byron/gitoxide/commit/4dc6594686478d9d6cd09e2ba02048624c3577e7))
    - default user signature now with 'now' time, like advertised. ([`ad40202`](https://github.com/Byron/gitoxide/commit/ad4020224114127612eaf5d1e732baf81818812d))
    - Make `SignatureRef<'_>` mandatory for editing reference changelogs. ([`68f4bc2`](https://github.com/Byron/gitoxide/commit/68f4bc2570d455c762da7e3d675b9b507cec69bb))
    - `Repository::committer()` now returns an `Option`, see `::committer_or_default()` for a method that doesn't. ([`f932cea`](https://github.com/Byron/gitoxide/commit/f932cea68ece997f711add3368db53aeb8cdf064))
    - first sketch of using configuration and environment variables for author/committer ([`330d0a1`](https://github.com/Byron/gitoxide/commit/330d0a19d54aabac868b76ef6281fffdbdcde53c))
    - remove local-time-support feature toggle. ([`89a41bf`](https://github.com/Byron/gitoxide/commit/89a41bf2b37db29b9983b4e5492cfd67ed490b23))
    - a first sketch on how identity management could look like. ([`780f14f`](https://github.com/Byron/gitoxide/commit/780f14f5c270802e51cf039639c2fbdb5ac5a85e))
    - refactor ([`4f61312`](https://github.com/Byron/gitoxide/commit/4f613120f9f761b86fc7eb16227d08fc5b9828d8))
    - respect `safe.directory`. ([`1b765ec`](https://github.com/Byron/gitoxide/commit/1b765ec6ae70d1f4cc5a885b3c68d6f3335ba827))
    - permissions for configuration. ([`840d9a3`](https://github.com/Byron/gitoxide/commit/840d9a3018d11146bb8e80fc92693c65eb534d91))
    - `git-config` is now accessible in `git-repository::config`. ([`6570808`](https://github.com/Byron/gitoxide/commit/657080829867d9dcb0c9b9cb6c1c8126c4df3783))
    - `gix config` lists all entries of all configuration files git considers. ([`d99453e`](https://github.com/Byron/gitoxide/commit/d99453ebeb970ed493be236def299d1e82b01f83))
    - adapt to changes in `git-config` ([`b52b540`](https://github.com/Byron/gitoxide/commit/b52b5407638adef2216aeb4215a7c0437d6ee2d5))
    - adapt to changes in `git-config` ([`3c57344`](https://github.com/Byron/gitoxide/commit/3c57344325ad20ae891824cd8791d2d17f4148e5))
    - adjust to changes in `git-config` for greater efficiency ([`e9afede`](https://github.com/Byron/gitoxide/commit/e9afedeebafb70d81a8fa2e6dc320b387e6ee926))
    - adapt to changes in git-config ([`14ba883`](https://github.com/Byron/gitoxide/commit/14ba8834b8738817d2bfb0ca66d1fb86fc8f3075))
    - refactor ([`95ed219`](https://github.com/Byron/gitoxide/commit/95ed219c5f414b6fa96d80eacf297f24d823a4fe))
    - repository now initializes global configuration files and resolves includes ([`ebedd03`](https://github.com/Byron/gitoxide/commit/ebedd03e119aa5d46da07e577bfccad621eaecb5))
    - adapt to changes in git-config ([`627a0e1`](https://github.com/Byron/gitoxide/commit/627a0e1e12e15a060a70d880ffdfb05f1f7db36c))
    - only a select few early config attributes must be repo-local ([`be0971c`](https://github.com/Byron/gitoxide/commit/be0971c5191f7866063ebcc0407331e683cf7d68))
    - resolve includes in local repository configuration ([`de8572f`](https://github.com/Byron/gitoxide/commit/de8572ff2ced9422832e1ba433955c33f0994675))
    - Adjust to changes in `git-config` ([`30cbe29`](https://github.com/Byron/gitoxide/commit/30cbe299860d84b5aeffced54839529dc068a8c7))
    - solve cycle between config and ref-store ([`1679d56`](https://github.com/Byron/gitoxide/commit/1679d5684cec852b39a0d51d5001fbcecafc6748))
    - adapt to changes in `git-config` ([`7f41f1e`](https://github.com/Byron/gitoxide/commit/7f41f1e267c9cbf87061821dd2f0edb6b0984226))
    - prepare for resolving a complete config… ([`9be1dd6`](https://github.com/Byron/gitoxide/commit/9be1dd6f7cdb9aea7c85df896e370b3c40f5e4ec))
    - Allow to configure a different filter for configuration section. ([`e512ab0`](https://github.com/Byron/gitoxide/commit/e512ab09477629957e469719f05e7de65955f3db))
    - adjust to changes in `git-config` ([`ca89d0d`](https://github.com/Byron/gitoxide/commit/ca89d0d4785ec4d66a0a4316fbc74be63dcc0f48))
    - refactor ([`5723730`](https://github.com/Byron/gitoxide/commit/57237303d9ae8a746c64d05ecedf3d43a0d041f6))
    - load configuration with trust information, needs cleanup ([`d8e41e2`](https://github.com/Byron/gitoxide/commit/d8e41e20de741c3d4701d862033cf50582a0d015))
    - Add remaining config access, and an escape hatch. ([`81715ff`](https://github.com/Byron/gitoxide/commit/81715ffca33e40cb6e37fff25baa68fca70c4844))
    - `config::Snapshot::trusted_path()` to obtain trustworthy paths. ([`d5a48b8`](https://github.com/Byron/gitoxide/commit/d5a48b82230b047434610550aacd2dc741b4b5f0))
    - `Debug` for `config::Snapshot`. ([`2c21956`](https://github.com/Byron/gitoxide/commit/2c2195640818319795a93e73bed79174fa358f55))
    - `Repository::config_snapshot()` to access configuration values. ([`5f9bfa8`](https://github.com/Byron/gitoxide/commit/5f9bfa89ceb61f484be80575b0379bbf9d7a36b3))
    - adapt to changes in `git-config` ([`c9423db`](https://github.com/Byron/gitoxide/commit/c9423db5381064296d22f48b532f29d3e8162ce9))
    - Support for `lossy` load mode. ([`d003c0f`](https://github.com/Byron/gitoxide/commit/d003c0f139d61e3bd998a0283a9c7af25a60db02))
    - Associate `file::Metadata` with each `File`. ([`6f4eea9`](https://github.com/Byron/gitoxide/commit/6f4eea936d64fb9827277c160f989168e7b1dba2))
    - adjust to changes in `git-config` ([`81e63cc`](https://github.com/Byron/gitoxide/commit/81e63cc3590301ca32c1172b358ffb45a13b6a8f))
    - Use `git-config` to write config file on initialization, including `logallrefupdates` and `precomposeunicode`. ([`7f67b23`](https://github.com/Byron/gitoxide/commit/7f67b23b9462b805591b1fe5a8406f8d7404f372))
    - respect `core.logallrefupdates` configuration setting. ([`e263e13`](https://github.com/Byron/gitoxide/commit/e263e13d312e41aa1481d104fa79ede509fbe1c5))
    - adapt to breaking changes in `git-config` ([`a02d575`](https://github.com/Byron/gitoxide/commit/a02d5759c14eb1d42fe24e61afc32a4cd463d1b7))
    - adapt to changes in `git-config` ([`858dc8b`](https://github.com/Byron/gitoxide/commit/858dc8b1b721ce5a45a76d9a97935cb0daf61e1a))
    - adjustments due to breaking changes in `git-config` ([`924f148`](https://github.com/Byron/gitoxide/commit/924f14879bd14ca1ff13fdd6ccafe43d6de01b68))
    - adjustments for breaking changes in `git-config` ([`d3841ee`](https://github.com/Byron/gitoxide/commit/d3841ee752e426bf58130cde1e4e40215ccb8f33))
    - adjust to changes in `git-config` ([`c52cb95`](https://github.com/Byron/gitoxide/commit/c52cb958f85b533e791ec6b38166a9d819f12dd4))
    - adjustments due to breaking changes in `git-config` ([`07bf647`](https://github.com/Byron/gitoxide/commit/07bf647c788afbe5a595ed3091744459e3623f13))
    - adapt to changes in `git-config` ([`363a826`](https://github.com/Byron/gitoxide/commit/363a826144ad59518b5c1a3dbbc82d04e4fc062d))
    - adjust to changes in `git-config` ([`920d56e`](https://github.com/Byron/gitoxide/commit/920d56e4f5141eeb536956cdc5fac042ddee3525))
    - adjustments required due to changed in `git-config` ([`41bfd3b`](https://github.com/Byron/gitoxide/commit/41bfd3b4122e37370d268608b60cb00a671a8879))
    - adjust to breaking changes in `git-config` ([`5b66202`](https://github.com/Byron/gitoxide/commit/5b66202d96bf664ed84755afc3ec49c301ecd62c))
    - adjustments due to breaking changes in `git_path` ([`4420ae9`](https://github.com/Byron/gitoxide/commit/4420ae932d5b20a9662a6d36353a27111b5cd672))
 * **[#427](https://github.com/Byron/gitoxide/issues/427)**
    - adjustments to handle changes in git-odb ([`23b7e4a`](https://github.com/Byron/gitoxide/commit/23b7e4ad0aab31cf029447b8f2866e71e9cb45a7))
    - refactor ([`8b79775`](https://github.com/Byron/gitoxide/commit/8b79775a9e5ea949c554f16222dbe1308bd2ef37))
    - refactor ([`aa2b933`](https://github.com/Byron/gitoxide/commit/aa2b9337c0c5db7ae7cdc5cba102df8ec61df3ed))
    - All remaining tests for ref and prefix disambiguation ([`dcd0dca`](https://github.com/Byron/gitoxide/commit/dcd0dca581ba9fc5e30cf52c267412fda03a1e5f))
    - Implement ref + object prefix disambiguation similar to git ([`5995e44`](https://github.com/Byron/gitoxide/commit/5995e443cb96307b4614ee4d484d4610e1d4c87e))
    - Sketch how options for `from_bytes()` can look like ([`0345b08`](https://github.com/Byron/gitoxide/commit/0345b08d985c46b30334e5808b32307d0482f191))
    - Finish ambiguous commit testing ([`a018fc6`](https://github.com/Byron/gitoxide/commit/a018fc6e8be458b63108af533c3fe70188a1f80f))
    - Support for per-repo baseline; more ambiguity tests ([`661283a`](https://github.com/Byron/gitoxide/commit/661283accd9e1b8fbd6234a2370652de17e914e8))
    - Add commit history with colliding prefixes ([`a1a6b13`](https://github.com/Byron/gitoxide/commit/a1a6b13411e42502174681fe7b2e23bbd31522ac))
    - improve describe hinting to allow hinting with describe-anchors as well ([`d993992`](https://github.com/Byron/gitoxide/commit/d99399287966ba2adf143222c3bd9ccdb4d135f9))
    - support disambiguation of describe prefixes ([`637dcb0`](https://github.com/Byron/gitoxide/commit/637dcb09771c8df83436dc48d6a72804b400c5e1))
    - Many more complex disambiguation tests ([`5fdf693`](https://github.com/Byron/gitoxide/commit/5fdf693b342700e8c7bf4690d465280749388b81))
    - Add test for blob access through tree ([`0955ff2`](https://github.com/Byron/gitoxide/commit/0955ff27c4883fb087884c3c8f4a8025487c07fb))
    - Add disambiguation test that we can't handle ([`5278cbc`](https://github.com/Byron/gitoxide/commit/5278cbc9b91ce01761a96a6962564a92daa77b7f))
    - Add test which shows that we don't allow disambiguation of by type yet ([`9d2e1eb`](https://github.com/Byron/gitoxide/commit/9d2e1eb3defc3ddd7ade7fe2bdd26d8a21afe55f))
    - test for broken zlib sream when rev-parsing an object ([`7c8e3f2`](https://github.com/Byron/gitoxide/commit/7c8e3f237860c8ab975f42cbd02172e055137138))
    - Turn on performance mode for sha-1 computation ([`44371a1`](https://github.com/Byron/gitoxide/commit/44371a10f464f32db346aa6b8309e983cfa20933))
    - Allow `RevSpec` to be serialized and deserialized with `serde`. ([`0660588`](https://github.com/Byron/gitoxide/commit/0660588b64f8d68ffa2f585ad49b384e86e3caec))
    - First implementation of object peeling ([`b1ef03a`](https://github.com/Byron/gitoxide/commit/b1ef03abc8342adb4a0b67d7c86136720ee600e2))
    - validate actual hash in baseline as well ([`779c6a4`](https://github.com/Byron/gitoxide/commit/779c6a43b8a3c888b3eb012ece014880b5fccc41))
    - the first successful test validating the actual revspec ([`2ae7534`](https://github.com/Byron/gitoxide/commit/2ae75346d62b926e414e48c574f026107e2bbe24))
    - Actually compare against the git baseline ([`d957995`](https://github.com/Byron/gitoxide/commit/d9579959438470b53af3e0de534c671f741faac1))
    - record a git baseline and prepare for reading it ([`5a3f6f5`](https://github.com/Byron/gitoxide/commit/5a3f6f5ab1ec9f89967deeac7c521ecf72bec84f))
    - The first successful disambiguation test ([`6bc6337`](https://github.com/Byron/gitoxide/commit/6bc6337037708243346afeee07ad24a02565894b))
    - A sketch for the RevSpec access API ([`91ac1e2`](https://github.com/Byron/gitoxide/commit/91ac1e289d1d44577de031aac0a469b3aee621a7))
    - refactor ([`7258326`](https://github.com/Byron/gitoxide/commit/7258326b6ddf8f40c8e3c59cd1b416d213e186d6))
 * **Uncategorized**
    - Release git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`d4df661`](https://github.com/Byron/gitoxide/commit/d4df661dbf60dad75d07002ef9979cabe8a86935))
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge branch 'gix-repo-config' ([`afecb63`](https://github.com/Byron/gitoxide/commit/afecb6337dcf0fc51d5c74747c3c60fa06ae6346))
    - Merge branch 'config-cascade' ([`f144eaf`](https://github.com/Byron/gitoxide/commit/f144eaf5863ae5cac63103f0db51c35fcf03a948))
    - thanks clippy ([`fddc720`](https://github.com/Byron/gitoxide/commit/fddc7206476423a6964d61acd060305572ecd02b))
    - thanks clippy ([`0346aaa`](https://github.com/Byron/gitoxide/commit/0346aaaeccfe18a443410652cada7b14eb34d8b9))
    - thanks clippy ([`b630543`](https://github.com/Byron/gitoxide/commit/b630543669af5289508ce066bd026e2b9a9d5044))
    - Merge branch 'config-sec-access' ([`b420eba`](https://github.com/Byron/gitoxide/commit/b420eba520ecc31fb2d07c939fa64f1a7be5737e))
    - thanks clippy ([`d9eb34c`](https://github.com/Byron/gitoxide/commit/d9eb34cad7a69b56f10eec5b88b86ebd6a9a74af))
    - Merge branch 'config-reduce-events' ([`fd046f4`](https://github.com/Byron/gitoxide/commit/fd046f463a9200d0d8f1a5c3e5f452792f015bd5))
    - Merge branch 'config-metadata' ([`453e9bc`](https://github.com/Byron/gitoxide/commit/453e9bca8f4af12e49222c7e3a46d6222580c7b2))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - avoid extra copies of paths using `PathCursor` tool during repo init ([`5771721`](https://github.com/Byron/gitoxide/commit/5771721ff5f86dd808d9961126c9c4a61867507c))
    - Merge branch 'config-comfort' ([`84b98d9`](https://github.com/Byron/gitoxide/commit/84b98d94177ceaf931aaa521e44eca0fa484d2d3))
    - Merge branch 'config-output' ([`20e188f`](https://github.com/Byron/gitoxide/commit/20e188ff3b06ac7e866956ea5216b00dcffd1307))
    - make fmt ([`0700b09`](https://github.com/Byron/gitoxide/commit/0700b09d6828849fa2470df89af1f75a67bfb27d))
    - change mostily internal uses of [u8] to BString/BStr ([`311d4b4`](https://github.com/Byron/gitoxide/commit/311d4b447daf8d4364670382a20901468748d34d))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
    - Merge branch 'main' into cont_include_if ([`0e9df36`](https://github.com/Byron/gitoxide/commit/0e9df364c4cddf006b1de18b8d167319b7cc1186))
    - Use git_path::realpath in all places that allow it right now ([`229dc91`](https://github.com/Byron/gitoxide/commit/229dc917fc7d9241b85e5818260a6fbdd3a5daaa))
    - fix build warnings ([`84109f5`](https://github.com/Byron/gitoxide/commit/84109f54877d045f8ccc7a380c012802708c2f1e))
    - Make a note to be sure we use the home-dir correctly in git-repository; avoid `dirs` crate ([`0e8cf19`](https://github.com/Byron/gitoxide/commit/0e8cf19d7f742f9400afa4863d302ba18a452adc))
    - adjust to changes in git-config ([`7a1678d`](https://github.com/Byron/gitoxide/commit/7a1678d8da0c361e0a0cc4380a04ebfb3ce5035d))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
    - thanks clippy ([`e898bfa`](https://github.com/Byron/gitoxide/commit/e898bfaf00f7c0eeb5cf9cb0d2cde8c1911ba497))
    - thanks clippy ([`59c4b10`](https://github.com/Byron/gitoxide/commit/59c4b10a16da5119efe4bf7f6fa4997cf2ec8136))
</details>

## 0.19.0 (2022-06-13)

### New Features (BREAKING)

 - <csr-id-266d4379e9132fd7dd21e6c8fccb36e125069d6e/> Make `realpath()` easier to use by introducing `realpath_opt()`.
   That way there is consistency about how many symlinks to follow.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 18 calendar days.
 - 20 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-worktree v0.3.0, git-repository v0.19.0 ([`0d8e856`](https://github.com/Byron/gitoxide/commit/0d8e8566dc5c6955487d67e235f86fbc75a3a88a))
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - make fmt ([`c665aef`](https://github.com/Byron/gitoxide/commit/c665aef4270c5ee54da89ee015cc0affd6337608))
    - Merge branch 'main' into svetli-n-cont_include_if ([`315c87e`](https://github.com/Byron/gitoxide/commit/315c87e18c6cac0fafa7b4e59fdd3c076a58a45a))
    - fix docs ([`daef221`](https://github.com/Byron/gitoxide/commit/daef2215cc6c4fddded5229951e8ac71c395468d))
    - refactor ([`b27a8c2`](https://github.com/Byron/gitoxide/commit/b27a8c243cdc14730478c2a94cafdc8ccf5c60d3))
    - refactor ([`06e96a4`](https://github.com/Byron/gitoxide/commit/06e96a435d820a1ef1e567bf93e7b9ca5fa74829))
    - Make `realpath()` easier to use by introducing `realpath_opt()`. ([`266d437`](https://github.com/Byron/gitoxide/commit/266d4379e9132fd7dd21e6c8fccb36e125069d6e))
    - Refact. ([`a342e53`](https://github.com/Byron/gitoxide/commit/a342e53dac58cea1787a94eaa1a9d24fb1389df2))
    - Add discovery opt env-overrides & env discovery helpers ([`e521d39`](https://github.com/Byron/gitoxide/commit/e521d39e1b0f4849280bae1527bf28977eec5093))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
</details>

## 0.18.1 (2022-05-23)

### New Features

 - <csr-id-c78baecbb37fd92a0a86231810c9e35e9a4c21cd/> `Debug` for `Reference`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.3, git-discover v0.1.2, git-repository v0.18.1, cargo-smart-release v0.10.1 ([`b7399cc`](https://github.com/Byron/gitoxide/commit/b7399cc44ee419355a649a7b0ba7b352cd48b400))
    - `Debug` for `Reference`. ([`c78baec`](https://github.com/Byron/gitoxide/commit/c78baecbb37fd92a0a86231810c9e35e9a4c21cd))
</details>

## 0.18.0 (2022-05-21)

<csr-id-e63e722791a7795cd99048bed834459595c60abc/>

### Other

 - <csr-id-e63e722791a7795cd99048bed834459595c60abc/> add ceiling_dirs option to upwards discovery

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-path v0.1.2, git-sec v0.1.1, git-config v0.4.0, git-discover v0.1.1, git-pack v0.19.1, git-repository v0.18.0, cargo-smart-release v0.10.0, safety bump 2 crates ([`ceb6dff`](https://github.com/Byron/gitoxide/commit/ceb6dff13362a2b4318a551893217c1d11643b9f))
    - Merge branch 'svetli-n-git_includeif' ([`cf24fbe`](https://github.com/Byron/gitoxide/commit/cf24fbe4b62d67b06138243d470dcc1805ebd55b))
    - refactor ([`07e0f5e`](https://github.com/Byron/gitoxide/commit/07e0f5e91b3c41614b9182cf9716120fe41ddf40))
    - Merge branch 'davidkna-discover-ceiling' ([`66944ba`](https://github.com/Byron/gitoxide/commit/66944ba986114ece2d3b31440c721d0e84b4f267))
    - Merge branch 'main' into git_includeif ([`229d938`](https://github.com/Byron/gitoxide/commit/229d9383bef8844111d2bf3c406a2ea570109c8b))
    - add ceiling_dirs option to upwards discovery ([`e63e722`](https://github.com/Byron/gitoxide/commit/e63e722791a7795cd99048bed834459595c60abc))
    - Fix markdown rendering issue ([`e4aaa44`](https://github.com/Byron/gitoxide/commit/e4aaa44f6d38008111640da72375f9c0578f1507))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

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
 - <csr-id-e4f4c4b2c75a63a40a174e3a006ea64ef8d78809/> `path::discover()` now returns the shortest path.
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

 - 141 commits contributed to the release over the course of 42 calendar days.
 - 43 days passed between releases.
 - 10 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 10 unique issues were worked on: [#298](https://github.com/Byron/gitoxide/issues/298), [#301](https://github.com/Byron/gitoxide/issues/301), [#331](https://github.com/Byron/gitoxide/issues/331), [#364](https://github.com/Byron/gitoxide/issues/364), [#382](https://github.com/Byron/gitoxide/issues/382), [#383](https://github.com/Byron/gitoxide/issues/383), [#384](https://github.com/Byron/gitoxide/issues/384), [#386](https://github.com/Byron/gitoxide/issues/386), [#389](https://github.com/Byron/gitoxide/issues/389), [#393](https://github.com/Byron/gitoxide/issues/393)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

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
    - adjusts to changes in git-ref ([`b362fb5`](https://github.com/Byron/gitoxide/commit/b362fb594546400e6c42688103df438954df7eeb))
    - Avoid using `Cow` at all in favor of a simple `&PartialNameref` ([`1bc9a87`](https://github.com/Byron/gitoxide/commit/1bc9a875d2b09b906f40db9e2c031c99e4fd9928))
    - See what it means to use `Cow<PartialNameRef>` ([`2ae129a`](https://github.com/Byron/gitoxide/commit/2ae129ad6183f36179031ea905d8974705e70da8))
    - adapt to changes in git-ref ([`21109ca`](https://github.com/Byron/gitoxide/commit/21109ca9ab21df0ab45f3be552e83114817e98d0))
    - adapt to changes in git-ref ([`49b0e89`](https://github.com/Byron/gitoxide/commit/49b0e89440ffcc5fa5dc66be45112f9c1f7d9244))
    - Instantiate worktree aware versions of file stores ([`0b670dd`](https://github.com/Byron/gitoxide/commit/0b670ddf97b316f0a6c332d999265a3bda7fcdab))
    - adjust to changes in git-ref ([`3299606`](https://github.com/Byron/gitoxide/commit/32996060c7405be787b8f5b91041e5d6dcd9ffc9))
    - Adjust permissions mildly to make a little more sense ([`9c05629`](https://github.com/Byron/gitoxide/commit/9c05629d1ca1ad779f1e07f3a3be102f049874df))
    - rely on `absolutize_components()` ([`e844006`](https://github.com/Byron/gitoxide/commit/e84400660dad6281fe3869ad649470f2adf31979))
    - brutally fix path handling of common dirs ([`e120232`](https://github.com/Byron/gitoxide/commit/e120232252875cd3fdacb9b7df90c3db58e7e24e))
    - Assure 'commondir' is actually resolved and not kept relative ([`3de63ff`](https://github.com/Byron/gitoxide/commit/3de63ff789e518cf07a8e1f02c385ae4ce857615))
    - first working worktree tests for git-repository ([`508cdd2`](https://github.com/Byron/gitoxide/commit/508cdd213ec48aec879ee4ddc3b76b6c851dd3c7))
    - refactor ([`6d9b2d9`](https://github.com/Byron/gitoxide/commit/6d9b2d9e7a32c825848a1df1fdec2e53b8705662))
    - A new version of opening a repository while accepting environment overrides ([`9d73424`](https://github.com/Byron/gitoxide/commit/9d7342429d2c4d7e5ef98a51a47d5caaa11297e0))
    - basic parsing for git-dir files ([`e11c677`](https://github.com/Byron/gitoxide/commit/e11c67770c301942188f204dbb2cd61880087959))
    - prepare for refactoring the overrides code ([`238e1b0`](https://github.com/Byron/gitoxide/commit/238e1b013d3f4d67b6384c6123c5ab6ea9f236fa))
    - fix build ([`d7dac11`](https://github.com/Byron/gitoxide/commit/d7dac11be455ee99299a8d7dfd412853f0d709f3))
    - Adjust to `git-discovery` to support linked worktrees in `Kind` type ([`2a99b7d`](https://github.com/Byron/gitoxide/commit/2a99b7d32374b4863dee0a0cdf55711686c94001))
    - refactor `repositoryKind` adjusted to handle linked worktrees ([`84677cb`](https://github.com/Byron/gitoxide/commit/84677cb09634e1d18ce20850bb7c6c9d63a13818))
    - Handle potential issue with overrides using documentation ([`feb4eb2`](https://github.com/Byron/gitoxide/commit/feb4eb26b33a5d3824fe98259193bed7961f6fef))
    - refactor how environment overrides work ([`99d98ec`](https://github.com/Byron/gitoxide/commit/99d98ece1688880f3b0b35bc4f7ab7ddd9289f1f))
    - fix docs ([`3366696`](https://github.com/Byron/gitoxide/commit/3366696af9ec58ebb43ed7d4dde9d2c79ca71d3d))
    - adjust to changes in git-discover ([`3271979`](https://github.com/Byron/gitoxide/commit/3271979f86bd5fb009a946cb06cd4ce8ea03119c))
    - refactor; add worktree id and determine main status of worktree ([`54be8e3`](https://github.com/Byron/gitoxide/commit/54be8e3da14b92f0c2dad32a969a651ad9ba9eec))
    - properly handle common-dir on repo open ([`de0cc1b`](https://github.com/Byron/gitoxide/commit/de0cc1bd1a1ccb26fa4fc5f7d8aedb422226b4a1))
    - first step towards getting repos from worktree proxies ([`60d0433`](https://github.com/Byron/gitoxide/commit/60d0433d8f6dda1e3556a73f85edff1c04d46dff))
    - refactor ([`7b5fe1d`](https://github.com/Byron/gitoxide/commit/7b5fe1de5332ca8a85741c7c0872130b5ebd31f2))
    - Keep instantiation options in Repository for worktrees ([`d25c938`](https://github.com/Byron/gitoxide/commit/d25c938e01e2fc8e9dd44724ea5017997d38e945))
    - refactor ([`71dd056`](https://github.com/Byron/gitoxide/commit/71dd0566cbfa9cbda148145efc78f76557663ae7))
    - obtain the base() path of a private worktree ([`f77d8c8`](https://github.com/Byron/gitoxide/commit/f77d8c8a60f1807f77aafd7b1d71334e9710e2e8))
    - fix docs ([`1e3acd0`](https://github.com/Byron/gitoxide/commit/1e3acd08b9df9fe0cc36bb6a4d4bac57c365443d))
    - use `git-discover` crate ([`f5f9a0d`](https://github.com/Byron/gitoxide/commit/f5f9a0d609316b2a64ee665f47faade7d8277315))
    - refactor ([`00a988e`](https://github.com/Byron/gitoxide/commit/00a988e3c2c964447f675164a6126bf6cb470c6b))
    - Remove `worktree()` platform in favor of the current worktree ([`f2a2c55`](https://github.com/Byron/gitoxide/commit/f2a2c5581eb3dde5ef7352439b564d89e9f76461))
    - basic worktree iteration ([`992a6ce`](https://github.com/Byron/gitoxide/commit/992a6ce154b97520b0c4679d6c50f4e3cc6e3091))
    - maybe fix failing test on windows ([`8f69af2`](https://github.com/Byron/gitoxide/commit/8f69af2eb48f01e2bbcf7b6483ae06f9b8dea61b))
    - assure worktree test repositories are regenerated ([`2eed703`](https://github.com/Byron/gitoxide/commit/2eed70392fd06f31f08acf2caa94437e967c7a1f))
    - Learn to read the common dir ([`e07c453`](https://github.com/Byron/gitoxide/commit/e07c453ea20e29994520dcd6346ac0a28f585813))
    - A first stab at more control over which worktrees and git-dirs to use ([`83ac263`](https://github.com/Byron/gitoxide/commit/83ac2638dd52e9da9a0dc8a62b4c9669c8eec372))
    - devise a worktree API that can work even if a valid worktree isn't present ([`8d067d1`](https://github.com/Byron/gitoxide/commit/8d067d113acfaf9a3e28ba1a829b07303a80e992))
    - reorganize types to properly represent worktrees in their various 'states' ([`b46bff5`](https://github.com/Byron/gitoxide/commit/b46bff58e40bb9805af7ee7f96272f0dc19c0ac7))
    - parse baseline worktree listing ([`aabe8b2`](https://github.com/Byron/gitoxide/commit/aabe8b2edc0753f125dcdea71dd44908d1826a21))
    - A first test to run against a bare and non-bare repos with worktrees ([`70164d7`](https://github.com/Byron/gitoxide/commit/70164d7252f57bd4b645d8ca694e7458ce4d1a0f))
    - Don't have expectations on the path, rather deal with it gracefully ([`3a41d5c`](https://github.com/Byron/gitoxide/commit/3a41d5cd7a6eb9f21c3461d499af4399b8f6e5be))
    - REMOVE ME: debug info for failing CI test ([`b0b3df4`](https://github.com/Byron/gitoxide/commit/b0b3df4e7fa93dba7f03003160f38036cbb6d80f))
    - see if this fixes the CI test issue on windows ([`7697f51`](https://github.com/Byron/gitoxide/commit/7697f517ec7c39a15076b1190056882812fe6a12))
    - :discover()` now returns the shortest path. ([`e4f4c4b`](https://github.com/Byron/gitoxide/commit/e4f4c4b2c75a63a40a174e3a006ea64ef8d78809))
    - Basic prefix support as well the first working version of `exclude query` ([`9cb8385`](https://github.com/Byron/gitoxide/commit/9cb83859f9bb76f38ab5bbd0ae6d6f20a691e9e1))
    - fix build ([`cb1c80f`](https://github.com/Byron/gitoxide/commit/cb1c80f8343691600797b61c61cba9cef82a59fc))
    - refactor ([`a89a667`](https://github.com/Byron/gitoxide/commit/a89a66792855fea7d695ec72899da954b8c16f3d))
    - Permission controlled access to xdg config ([`42a6c8c`](https://github.com/Byron/gitoxide/commit/42a6c8c9d19f9aab0b33537156e2774c61621864))
    - preliminary access to a fully configured exclusion cache ([`259d015`](https://github.com/Byron/gitoxide/commit/259d015c4c0195fb77d372545d790ea4c4d01b8a))
    - refactor ([`a86ed7b`](https://github.com/Byron/gitoxide/commit/a86ed7bc0e10ebed2918f19d2fc3304fbed87df3))
    - remove `values::*Error` in favor of `value::parse::Error`. ([`38dfdcf`](https://github.com/Byron/gitoxide/commit/38dfdcf80f9b7368ccaa10f4b78b2129849848d0))
    - refactor ([`807b7f8`](https://github.com/Byron/gitoxide/commit/807b7f826b4e614478aadd36d6361e9970e5d746))
    - A first version of opening index files with proper configuration ([`f11cc44`](https://github.com/Byron/gitoxide/commit/f11cc441f10e4a7c2c09e7aa9f9435c837c5e77a))
    - Remove IntegerSuffix error which wasn't ever used ([`732c0fa`](https://github.com/Byron/gitoxide/commit/732c0fa6e1832efcc0de4adc894e820b3bd27b8f))
    - Adjust to improvements to the `git-config` API ([`ffc5dec`](https://github.com/Byron/gitoxide/commit/ffc5dec6b9ed2b2d19d927848006053f73741a27))
    - fix build warnings ([`4496b5a`](https://github.com/Byron/gitoxide/commit/4496b5a26abaf91fd4844e0494aaa1b4cce73628))
    - fix build ([`cb56f12`](https://github.com/Byron/gitoxide/commit/cb56f12ad83cf2932a068ef4fa0ca5ce4aa73e84))
    - Adapt to changes in git-config ([`61ea4c4`](https://github.com/Byron/gitoxide/commit/61ea4c4a254bafd3d1f0c18cf1c10cbd66c15a4d))
    - sketch `open_index()` on `Worktree`, but… ([`ff76261`](https://github.com/Byron/gitoxide/commit/ff76261f568f6b717a93b1f2dcf5d8e8b63acfca))
    - a sketch of basic Worktree support ([`732f6fb`](https://github.com/Byron/gitoxide/commit/732f6fb0aa9cdc843087352b12bed2cd142ed6ec))
    - Use `git-path` crate instead of `git_features::path` ([`47e607d`](https://github.com/Byron/gitoxide/commit/47e607dc256a43a3411406c645eb7ff04239dd3a))
    - docs for git-glob ([`8f4969f`](https://github.com/Byron/gitoxide/commit/8f4969fe7c2e3f3bb38275d5e4ccb08d0bde02bb))
    - make fmt ([`50ff7aa`](https://github.com/Byron/gitoxide/commit/50ff7aa7fa86e5e2a94fb15aab86470532ac3f51))
 * **[#331](https://github.com/Byron/gitoxide/issues/331)**
    - Use `Integer::to_decimal()` in git-repository ([`8fb95bf`](https://github.com/Byron/gitoxide/commit/8fb95bf62a33ccef3b037162f49e9a72abb0e3d9))
 * **[#364](https://github.com/Byron/gitoxide/issues/364)**
    - some TODOs related to precomposed unicode on MacOS ([`bc246aa`](https://github.com/Byron/gitoxide/commit/bc246aaa81cd7023e8533a006211a800621e8907))
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
    - Release git-worktree v0.2.0, git-repository v0.17.0 ([`3f71246`](https://github.com/Byron/gitoxide/commit/3f7124616ab9752007b8cf03e1c6a3a796ffee0b))
    - Release git-worktree v0.2.0, git-repository v0.17.0 ([`5845934`](https://github.com/Byron/gitoxide/commit/584593448b560afdd60dbdbdff901d267082765e))
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - make fmt ([`e043807`](https://github.com/Byron/gitoxide/commit/e043807abf364ca46d00760e2f281528efe20c75))
    - Merge branch 'refs-and-worktrees' ([`8131227`](https://github.com/Byron/gitoxide/commit/8131227ddff6f36919b6a0f7b33792ebde0f8ae9))
    - Merge branch 'main' into refs-and-worktrees ([`9cf0c7b`](https://github.com/Byron/gitoxide/commit/9cf0c7bd0cc5419137db5796f3a5b91bdf3dcc94))
    - Merge branch 'davidkna-remote-branch-name' ([`068a2de`](https://github.com/Byron/gitoxide/commit/068a2de764fabff949ff49a50594563cc625e343))
    - refactor ([`5ab5842`](https://github.com/Byron/gitoxide/commit/5ab58428358938bced45cc348ec76b527bca9be3))
    - Use a Cow for remote name handling ([`633a30d`](https://github.com/Byron/gitoxide/commit/633a30dc919ef4a16e4382f6f81825ff2deb7f6b))
    - adjust to changes in git-ref ([`0671586`](https://github.com/Byron/gitoxide/commit/06715861d3a1d236c310d71737ec1d1a5ca6c770))
    - allow reading information about remote branch ([`53c06c7`](https://github.com/Byron/gitoxide/commit/53c06c7e6a3003b34edaab10db1f158e2fb57403))
    - thanks clippy ([`a7ac64c`](https://github.com/Byron/gitoxide/commit/a7ac64cd801b985790b5717be1a5dc722b2ae3a9))
    - Merge branch 'main' into git_includeif ([`b1bfc8f`](https://github.com/Byron/gitoxide/commit/b1bfc8fe8efb6d8941f54dddd0fcad99aa13ed6c))
    - Merge branch 'basic-worktree-support' ([`e058bda`](https://github.com/Byron/gitoxide/commit/e058bdabf8449b6a6fdff851e3929137d9b71568))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - thanks clippy ([`d624f4e`](https://github.com/Byron/gitoxide/commit/d624f4e7fafd821867a41548b49f2cd7f09def8c))
    - thanks clippy ([`6fb19cf`](https://github.com/Byron/gitoxide/commit/6fb19cfee79a49741dd439ade9c638aa89943f10))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'worktree-stack' ([`98da8ba`](https://github.com/Byron/gitoxide/commit/98da8ba52cef8ec27f705fcbc84773e5bacc4e10))
    - thanks clippy ([`f802a03`](https://github.com/Byron/gitoxide/commit/f802a03dc0b04d12fa360fb570d460ad4e1eb53a))
    - Merge branch 'inferiorhumanorgans-repo-status-additional-tests' ([`66dee9a`](https://github.com/Byron/gitoxide/commit/66dee9a89780063630c657e4d9a08bce6cd940d4))
    - Add test coverage for RepositoryState::CherryPickSequence… ([`fc281d8`](https://github.com/Byron/gitoxide/commit/fc281d820d130b74c80d8fc139188a4c4b7b7331))
    - erge branch 'fix-describe' ([`56d7ad7`](https://github.com/Byron/gitoxide/commit/56d7ad7a2e7994545581ad5955c25feb9cefdf4e))
    - fix docs ([`5ee2307`](https://github.com/Byron/gitoxide/commit/5ee23070ecfbf73e5897344421a1f1ec2917a3bd))
    - Merge branch 'main' into repo-status ([`9679d6b`](https://github.com/Byron/gitoxide/commit/9679d6b0e68c28438e22cb65c554d0b31dfaf159))
    - remove unused variant ([`da8059c`](https://github.com/Byron/gitoxide/commit/da8059ce26343c8cd275f43c879d98c92f77fa51))
    - Merge branch 'git-sec' ([`cd723b5`](https://github.com/Byron/gitoxide/commit/cd723b5ae11148e7e9fd07daf28bc04455d5c46f))
    - Clean up the error message and comments. ([`463a705`](https://github.com/Byron/gitoxide/commit/463a705dc23cddf0ba0ec2dc578a618c793b1d9d))
    - Print out some human readable text if GNU sed cannot be found. ([`cf19a18`](https://github.com/Byron/gitoxide/commit/cf19a1854091dc5c709dc367ca5f9568dd7e6da8))
    - Merge branch 'main' into repo-status ([`0eb2372`](https://github.com/Byron/gitoxide/commit/0eb23721dca78f6e6bf864c5c3a3e44df8b419f0))
    - repo_path -> git_dir ([`53c22ee`](https://github.com/Byron/gitoxide/commit/53c22ee00834ce5912ec28d20026032b063fd2ec))
    - Merge branch 'test-archive-support' ([`350df01`](https://github.com/Byron/gitoxide/commit/350df01042d6ca8b93f8737fa101e69b50535a0f))
    - Let's try not parsing the git version. ([`475e7d1`](https://github.com/Byron/gitoxide/commit/475e7d1ebdd27e3efd4bd7de6e0a1ee9447feb4b))
    - Take a couple more steps to appease the CI gods. ([`ac3c8c7`](https://github.com/Byron/gitoxide/commit/ac3c8c7397bf5294cbce97e0718bac23588b2ca5))
    - Fix the GNU sed detection so it works where /usr/bin/sed is GNU. ([`5c162e0`](https://github.com/Byron/gitoxide/commit/5c162e05299256e99abe84213b078652b5c637a0))
    - Make clippy happier. ([`a5406b5`](https://github.com/Byron/gitoxide/commit/a5406b5c06a9ecb147f5850db001de2782dd283d))
    - Pass version appropriate rebase flags to git. ([`bb18a13`](https://github.com/Byron/gitoxide/commit/bb18a13cd05ddce3e850760814e4bdc6e35e0f0e))
    - Merge branch 'main' into repo-status ([`4086335`](https://github.com/Byron/gitoxide/commit/40863353a739ec971b49410fbc2ba048b2762732))
    - Add a few tests for good measure. ([`499c811`](https://github.com/Byron/gitoxide/commit/499c81106d520e3c8ae1aa02e905c8048a054f79))
    - in_progress_operation now returns an Option ([`172b464`](https://github.com/Byron/gitoxide/commit/172b4640984d23d7adafacd96cf9d88569d29769))
    - Tweak the naming and comments a bit ([`56038ed`](https://github.com/Byron/gitoxide/commit/56038ed075d6774043651f14abb61550539b5c26))
    - Release git-glob v0.2.0, safety bump 3 crates ([`ab6bed7`](https://github.com/Byron/gitoxide/commit/ab6bed7e2aa19eeb9990441741008c430f373708))
    - Merge branch 'worktree-stack' ([`e90d3fd`](https://github.com/Byron/gitoxide/commit/e90d3fd0a9764511e6280596f21d3a0494ed7021))
    - First pass at Repository::in_progress_state() ([`c2f66e4`](https://github.com/Byron/gitoxide/commit/c2f66e4ea26fb28bde80dc44ea3ea7278c2fd967))
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

 - 20 commits contributed to the release.
 - 2 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#298](https://github.com/Byron/gitoxide/issues/298)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 5 times to make code idiomatic. 

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
 * **Uncategorized**
    - Release git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0 ([`f041c00`](https://github.com/Byron/gitoxide/commit/f041c00a7df2455ca52fac7b83af1e9f335f5688))
    - Release git-config v0.2.1, git-diff v0.15.0, git-traverse v0.14.0, git-pack v0.18.0, git-odb v0.28.0, git-ref v0.12.1, git-revision v0.1.0, git-repository v0.16.0, gitoxide-core v0.14.0, gitoxide v0.12.0, safety bump 6 crates ([`b612021`](https://github.com/Byron/gitoxide/commit/b612021683ba709b693bd48aef3e2e3c2f5b9ead))
    - thanks clippy ([`7887d8b`](https://github.com/Byron/gitoxide/commit/7887d8b5bedc49890bd73beb058a9828aa734729))
    - thanks clippy ([`0f5a943`](https://github.com/Byron/gitoxide/commit/0f5a9439d6b1716345f0e122c23c1a566fdd3088))
    - thanks clippy ([`9407532`](https://github.com/Byron/gitoxide/commit/9407532b98646d33bb0b947860a6a0022cfbae28))
    - thanks clippy ([`60cb858`](https://github.com/Byron/gitoxide/commit/60cb8589e901981802be11289352510a9d43cd87))
    - thanks clippy ([`f2faa00`](https://github.com/Byron/gitoxide/commit/f2faa001ed2c8e96e25dbd56544320055f8dbe1b))
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

 - 82 commits contributed to the release over the course of 69 calendar days.
 - 69 days passed between releases.
 - 21 commits were understood as [conventional](https://www.conventionalcommits.org).
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
    - refactor ([`8bf585d`](https://github.com/Byron/gitoxide/commit/8bf585d67cd67b168d819ba05858cef7d9b90894))
    - JSON output for index entries ([`3fc1622`](https://github.com/Byron/gitoxide/commit/3fc1622488054c6ab655eb9d2f941b68cc3ccf18))
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
    - Add and improve Debug implementation on major types ([`d23c3d4`](https://github.com/Byron/gitoxide/commit/d23c3d4c6e7ac1d712f7c2b7ebf4cfe45923ee6e))
    - support for trimming of whitespace around name and email ([`a39bf71`](https://github.com/Byron/gitoxide/commit/a39bf71531ee0a6c8db082758d3212c805ce2bf0))
    - also inform about average and max commit sizes ([`5052a4e`](https://github.com/Byron/gitoxide/commit/5052a4e532fec63ccf49f6ce54df41707779e70b))
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
    - Adjust to breaking changes in `git-traverse` ([`d79b506`](https://github.com/Byron/gitoxide/commit/d79b5064eab2d1bef445e6c9e62a53466a8d5225))
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
    - make fmt ([`7cf3545`](https://github.com/Byron/gitoxide/commit/7cf354509b545f7e7c99e159b5989ddfbe86273d))
    - Properly classify worktrees as non-bare, helps with `ein t find` ([`c329dd7`](https://github.com/Byron/gitoxide/commit/c329dd75420f82d506fd415cd377f7df6c6ccbad))
    - fix MSRV ([`1bf5d11`](https://github.com/Byron/gitoxide/commit/1bf5d11ec0b9df2f1c6fb5239007ad8409b97f75))
    - thanks clippy ([`8e2e4e3`](https://github.com/Byron/gitoxide/commit/8e2e4e352b563b2accd2e9d91c6e8a33b5a9709c))
    - Merge branch 'short-id' ([`5849d5b`](https://github.com/Byron/gitoxide/commit/5849d5b326b83f98a16cf1d956c720c7f0fd4445))
    - thanks clippy ([`4618f8a`](https://github.com/Byron/gitoxide/commit/4618f8aa7648c0553a8e1b023fceb6738654e38b))
    - thanks clippy ([`5db3993`](https://github.com/Byron/gitoxide/commit/5db39936fc003a79f18e545a8317305fe18af74d))
    - thanks clippy ([`d5911b5`](https://github.com/Byron/gitoxide/commit/d5911b59e4bd039fe39702487640d18319c0ed7e))
    - Set the MSRV version explicitly in git-repository ([`bbf6799`](https://github.com/Byron/gitoxide/commit/bbf6799db01e25c8e3f49e0fd6ff3ec802e773a0))
    - remove unused dependency ([`2fbc93c`](https://github.com/Byron/gitoxide/commit/2fbc93cc2ce855f24aea63c8513cf1e037c685a1))
    - Release git-tempfile v2.0.0, safety bump 6 crates ([`90b1c42`](https://github.com/Byron/gitoxide/commit/90b1c42d5487904a9f329362d185b035d0ddb975))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
    - Release git-actor v0.8.1 ([`08fe550`](https://github.com/Byron/gitoxide/commit/08fe5508472f2eb209db8a5fc4e4942a9d7db93d))
    - fix lint ([`b339b41`](https://github.com/Byron/gitoxide/commit/b339b419bde0418fb4fcd998e232b1eba836f7a4))
    - Release git-hash v0.9.2, git-object v0.17.1, git-pack v0.16.1 ([`0db19b8`](https://github.com/Byron/gitoxide/commit/0db19b8deaf11a4d4cbc03fa3ae40eea104bc302))
    - Merge branch 'index-verification' ([`ad3c803`](https://github.com/Byron/gitoxide/commit/ad3c8032cee02052ef3940d1d7c950270a0a299a))
    - thanks clippy ([`2f25bf1`](https://github.com/Byron/gitoxide/commit/2f25bf1ebf44aef8c4886eaefb3e87836d535f61))
    - Release git-config v0.1.11 ([`a605b67`](https://github.com/Byron/gitoxide/commit/a605b67294773628590220600f5017c63911f620))
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

 - 67 commits contributed to the release over the course of 51 calendar days.
 - 55 days passed between releases.
 - 18 commits were understood as [conventional](https://www.conventionalcommits.org).
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
    - Adjust object-acess to test new contains method ([`8488b41`](https://github.com/Byron/gitoxide/commit/8488b41651751d9177f53a23233b7ddd655dd696))
    - assure loops can't happen anymore ([`f04ff80`](https://github.com/Byron/gitoxide/commit/f04ff8011198b7f6c45c2094530903316c6e91ea))
    - Rename `Handle` to `Cache` ([`580e96c`](https://github.com/Byron/gitoxide/commit/580e96c1b2d9782a2e8cf9d1123f6d53a5376a3d))
    - First sketch of general store ([`fc1b640`](https://github.com/Byron/gitoxide/commit/fc1b6409380256b73cf271c105802f4494dbb8c5))
    - add docs for handle-related functions ([`cf1b1e6`](https://github.com/Byron/gitoxide/commit/cf1b1e6d82f691ab17975e4f1479d93720368803))
    - use `git_odb::Find*` traits in prelude, instead of `git_pack::Find*` ([`f9c0493`](https://github.com/Byron/gitoxide/commit/f9c0493460ab7c664aaa231ffcf7dfd56076c920))
    - fix git-repository docs ([`3496a97`](https://github.com/Byron/gitoxide/commit/3496a970c0918c309075a0ecad7b84b449a6e4cf))
    - remove borrowing Repo as possible failure cause ([`7a91212`](https://github.com/Byron/gitoxide/commit/7a91212631219e94b9454d2874b53f3ecc1db77e))
    - remove Easy… abstraction in favor of Handle ([`b2cc0c6`](https://github.com/Byron/gitoxide/commit/b2cc0c63570d45de032d63e62d94c3344783440e))
    - rename easy::State to easy::Handle ([`83d7b31`](https://github.com/Byron/gitoxide/commit/83d7b31e7dd6d09eea79fc3c68620d099459132f))
    - Remove unnecessary error variants now that repo() is called less ([`afcd579`](https://github.com/Byron/gitoxide/commit/afcd579e53c09b8d1c39be16f516584f6ff93bfa))
    - Use db handle for writing ([`053e7b6`](https://github.com/Byron/gitoxide/commit/053e7b61c093021b9931f1cca105a462ba4fc3cf))
    - Adapt to changes in git-repository ([`3ab9b03`](https://github.com/Byron/gitoxide/commit/3ab9b03eee7d449b7bb87cb7dcbf164fdbe4ca48))
    - fully rely on OdbHandle in repository State ([`5e7aa16`](https://github.com/Byron/gitoxide/commit/5e7aa1689f5d7ea5b510611a3ca0868828226291))
    - Rename `Repository::odb` to` Repository::objects` ([`57de915`](https://github.com/Byron/gitoxide/commit/57de915886b76f80b3641def0ccf4fd79e334fc8))
    - Add odb handle to state ([`4e38da3`](https://github.com/Byron/gitoxide/commit/4e38da35be4d753c30e07ed292ae8ce15513bcfe))
    - remove Repository::refresh_object_database() ([`93db4a5`](https://github.com/Byron/gitoxide/commit/93db4a5e70456d2c33ea010e3c86e5f26eb1bcc0))
    - remove pack-cache from `Find::try_find(…)` ([`ebc7f47`](https://github.com/Byron/gitoxide/commit/ebc7f47708a63c3df4415ba0e702660d976dfb3e))
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
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - remove debug-helper ([`c243215`](https://github.com/Byron/gitoxide/commit/c2432158ca4be3008847bce40cfe536e082d1f4a))
    - Don't use bleeding edge features ([`3de0ab1`](https://github.com/Byron/gitoxide/commit/3de0ab1163d267102e7605da1d7a114574508a00))
    - reference statistics for stats example ([`83b99ce`](https://github.com/Byron/gitoxide/commit/83b99cee89dd55550503290602a5ab62c62dec55))
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

 - 16 commits contributed to the release over the course of 7 calendar days.
 - 12 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#259](https://github.com/Byron/gitoxide/issues/259), [#263](https://github.com/Byron/gitoxide/issues/263)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#259](https://github.com/Byron/gitoxide/issues/259)**
    - Describe and propose fix for ref namespace-sharing issue ([`55773b8`](https://github.com/Byron/gitoxide/commit/55773b87feb246927a7421a822c45d9e101e985e))
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
    - Merge branch 'pack-consistency' ([`5982406`](https://github.com/Byron/gitoxide/commit/5982406b4e1b26fd383d9ec21a3cf652ec8ab25f))
</details>

## 0.12.0 (2021-11-16)

### New Features

 - <csr-id-b7aab9efd42975e8f2dcb5c97e51495996175702/> Allow `PartialNameRef` to be created from owned items

### Changed (BREAKING)

 - <csr-id-e8b091943f0c9a26317da0003f7fcdf5a56ef21a/> Rename gix->ein and gixp->gix

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 20 calendar days.
 - 27 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#241](https://github.com/Byron/gitoxide/issues/241), [#247](https://github.com/Byron/gitoxide/issues/247), [#251](https://github.com/Byron/gitoxide/issues/251), [#254](https://github.com/Byron/gitoxide/issues/254)

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
 * **Uncategorized**
    - Release git-repository v0.12.0, cargo-smart-release v0.6.0 ([`831a777`](https://github.com/Byron/gitoxide/commit/831a777487452a6f51a7bc0a9f9ca34b0fd778ed))
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
</details>

## v0.11.0 (2021-10-19)

A maintenance release to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
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
<csr-id-293bfc0278c5983c0beaec93253fb51f00d81156/>
<csr-id-650241251a420602f74037babfc24c9f64df78d8/>
<csr-id-2b4a61589a7cba3f7600710e21304e731ae3b36a/>
<csr-id-8b82f7d44c7eb63b7922ddc31ada9cefdce776b0/>

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

 - 95 commits contributed to the release over the course of 33 calendar days.
 - 34 days passed between releases.
 - 44 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#164](https://github.com/Byron/gitoxide/issues/164), [#198](https://github.com/Byron/gitoxide/issues/198), [#200](https://github.com/Byron/gitoxide/issues/200), [#67](https://github.com/Byron/gitoxide/issues/67)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

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
    - Use 'to_*' when converting `easy::Object` to specific object kind ([`1cb41f8`](https://github.com/Byron/gitoxide/commit/1cb41f81cffe19c75aadf49a5cc7ec390ec6cae7))
    - Fix panic related to incorrect handling of character boundaries ([`9e92cff`](https://github.com/Byron/gitoxide/commit/9e92cff33f4f53d3b2d6b55a722d577c2dd6a4f2))
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
    - loose reference iteration with non-dir prefixes… ([`293bfc0`](https://github.com/Byron/gitoxide/commit/293bfc0278c5983c0beaec93253fb51f00d81156))
    - Add 'references().all().peeled().'… ([`6502412`](https://github.com/Byron/gitoxide/commit/650241251a420602f74037babfc24c9f64df78d8))
    - filter refs correctly, but… ([`2b4a615`](https://github.com/Byron/gitoxide/commit/2b4a61589a7cba3f7600710e21304e731ae3b36a))
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
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com/Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - thanks clippy ([`ae7826e`](https://github.com/Byron/gitoxide/commit/ae7826e1cf79fce6ad12f407162f58b3bfb02b16))
    - thanks clippy ([`b02edb5`](https://github.com/Byron/gitoxide/commit/b02edb5b1e9b7c8f8bd1b4a8e2d60667da629839))
    - thanks clippy ([`68ea77d`](https://github.com/Byron/gitoxide/commit/68ea77dcdd5eb8033618e7af2e3eb0989007b89b))
    - improved changelog… ([`8b82f7d`](https://github.com/Byron/gitoxide/commit/8b82f7d44c7eb63b7922ddc31ada9cefdce776b0))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
    - Bump git-repository v0.10.0 ([`5a10dde`](https://github.com/Byron/gitoxide/commit/5a10dde1bcbc03157f3ba45104638a8b5b296cb9))
    - [repository #164] docs for easy::reference::log ([`7de7c7e`](https://github.com/Byron/gitoxide/commit/7de7c7eb51b7d709fd140dbf789e31e97161bfa7))
    - [repository #164] docs for easy::reference::iter ([`d86c713`](https://github.com/Byron/gitoxide/commit/d86c71363a5a73dd8986566a9687e2b4756972cb))
    - [repository #164] refactor ([`437e63b`](https://github.com/Byron/gitoxide/commit/437e63b4e841ef478c12a91bf3e2dce63d5b1041))
    - [repository #164] docs for top-level of easy::reference ([`9e465e0`](https://github.com/Byron/gitoxide/commit/9e465e03dc636c360128c93864749c4a3f8a99e5))
    - [repository #164] docs for easy::oid ([`b66b6fe`](https://github.com/Byron/gitoxide/commit/b66b6fe759eeb55cb875fcb65aa58b62c6963ca8))
    - [repository #164] docs for easy::commit and easy::odb ([`abf37e5`](https://github.com/Byron/gitoxide/commit/abf37e54e5a4584f521988e27dd02f6d6badc4ef))
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

 - 6 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.9.1 ([`262c122`](https://github.com/Byron/gitoxide/commit/262c1229d6d2d55c70fe0e199ab15d10954d967b))
    - Release git-ref v0.7.3 ([`b0a9815`](https://github.com/Byron/gitoxide/commit/b0a98157ab3b240af027acb9965c981a543e55fa))
    - [repository] don't enforce feature flags that may fail on windows by default ([`afdec2e`](https://github.com/Byron/gitoxide/commit/afdec2e89eee0397b16602fdff16d3997ef370d0))
    - Release git-ref v0.7.2 ([`e940e9a`](https://github.com/Byron/gitoxide/commit/e940e9a21938035eb8791bba19cc16814a0fb4e7))
    - Release git-protocol v0.10.4 ([`898ee08`](https://github.com/Byron/gitoxide/commit/898ee08befa1eb7dd22980063c7633f83d0a8958))
    - Release git-odb v0.21.3 ([`223f930`](https://github.com/Byron/gitoxide/commit/223f93075a28dd49f44505c039cfeae5a7296914))
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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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

 - 67 commits contributed to the release over the course of 8 calendar days.
 - 9 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.2 ([`3fc23be`](https://github.com/Byron/gitoxide/commit/3fc23beaf103c037253ace727c87ec457be5dedd))
    - Merge branch 'repository-integration' ([`49f5453`](https://github.com/Byron/gitoxide/commit/49f5453629646ac24d752f53c532e5f67eb09374))
    - [repository #190] test for oid.ancestors().all() ([`fdc3678`](https://github.com/Byron/gitoxide/commit/fdc3678c63fa128ac754b3fa9ae3d88a4a221d0d))
    - [repository #190] fix build, lets just make traversal available by default ([`6da3599`](https://github.com/Byron/gitoxide/commit/6da35994cf2a3c9ab741733af53761c9a2cebeed))
    - Bump git-pack v0.10.0 ([`e5e3c80`](https://github.com/Byron/gitoxide/commit/e5e3c8024e1c2e5e90cee83abbdae41d58eee156))
    - [repository #190] access to repository directories ([`f4d1ec4`](https://github.com/Byron/gitoxide/commit/f4d1ec4ac0be8aa46d97eb92fb8a8f3fb8da94fb))
    - [repository #190] first shot at ancestor iteration… ([`85f1a48`](https://github.com/Byron/gitoxide/commit/85f1a48ea39f3b224e8d0ba3728dd75e03a6edc3))
    - [repository #190] refactor ([`e7188e0`](https://github.com/Byron/gitoxide/commit/e7188e047529cb0f4b20b3876f36b4592e9d2dc4))
    - [ref #190] fix tests ([`e426e15`](https://github.com/Byron/gitoxide/commit/e426e15188d8ec38ee0029f1d080dbab9afd8642))
    - [repository #190] fix tests; needs inbound transaction handling… ([`e5a5c09`](https://github.com/Byron/gitoxide/commit/e5a5c09bb108741fff416672566e381f50f02b38))
    - [repository #190] leverage git-ref namespace support ([`1aa9c11`](https://github.com/Byron/gitoxide/commit/1aa9c113488175f03758f8a64338a33b3417dd87))
    - [repository #190] refactor ([`609c249`](https://github.com/Byron/gitoxide/commit/609c249916ca64f4beecdab86eb4562adbd1ca4f))
    - [repository #190] fix build ([`f5e118c`](https://github.com/Byron/gitoxide/commit/f5e118c8871e45ed3db9da9cd6bc63a5ea99621e))
    - [repository #190] note a known limitation about finding references in namespaces… ([`d335731`](https://github.com/Byron/gitoxide/commit/d3357318cf100fc3e0751e5b6de3922b1c209ddb))
    - [repository #190] transparent namespace support ([`d14f073`](https://github.com/Byron/gitoxide/commit/d14f073707c2f4641a271ba7965ec8281638e8df))
    - [repository #190] turns out we need bstr with unicode support ([`3d8796e`](https://github.com/Byron/gitoxide/commit/3d8796e670f9bb5d2ed22fb3b75130a599737341))
    - [repository #190] public bstr re-export ([`3b7ffde`](https://github.com/Byron/gitoxide/commit/3b7ffde385b1984393ee65a7505ad7221fecd0dc))
    - [repository #190] cleanup usage of bstr… ([`e4411ff`](https://github.com/Byron/gitoxide/commit/e4411ff43b24af79fefeaa4411e004dc504a4e2a))
    - [repository #190] prefixed reference iteration ([`a6e19c9`](https://github.com/Byron/gitoxide/commit/a6e19c9a49bdc6a7c5cabef0a8d93bfd48a74fcd))
    - [repository #190] implementation of reference iteration (all() for now)… ([`2c0939a`](https://github.com/Byron/gitoxide/commit/2c0939a146b5973de26bd03987e075a34a84bc88))
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
    - [ref #190] move remaining file store functions to extension trait ([`60fc215`](https://github.com/Byron/gitoxide/commit/60fc215ccac529b4a14cb9d8260ab9ddec86758a))
    - thanks clippy ([`376c045`](https://github.com/Byron/gitoxide/commit/376c045cf589e51b639cf6c3633c4a8fcae7b6aa))
    - [repository #190] refactor ([`15d4ac8`](https://github.com/Byron/gitoxide/commit/15d4ac8f4b08716f6b06938f01396fb8ba8e7086))
    - [repository #190] a major step forward with `head()` access ([`43ac4f5`](https://github.com/Byron/gitoxide/commit/43ac4f5acbe3ace5d43ed3ed1bc394d721f0e273))
    - [ref #190] cache peeled objects properly ([`2cb511e`](https://github.com/Byron/gitoxide/commit/2cb511efe5833f860f3c17b8e5f5b4cd643baddb))
    - Bump git-ref v0.7.0 ([`ac4413c`](https://github.com/Byron/gitoxide/commit/ac4413ce4e45703d5fe722e7220d039217f0bdef))
    - [repository #190] experiment with 'HEAD' API… ([`c55ce4d`](https://github.com/Byron/gitoxide/commit/c55ce4d8453c1ab4a107f5c6fb01521b422ee5c4))
    - thanks clippy ([`14dff63`](https://github.com/Byron/gitoxide/commit/14dff63fbc0d318bbc8a2618e0d72aaa98948acf))
    - [ref #190] Use Raw Reference everywhere for great simplification… ([`7aeea9c`](https://github.com/Byron/gitoxide/commit/7aeea9c36d4da04a806e68968356f8cc0dc11475))
    - [repository #190] refactor ([`d6bef3a`](https://github.com/Byron/gitoxide/commit/d6bef3afe7168659a75e26fb3ae2aa722fecf853))
    - [ref #190] introduce Raw reference type that simplifies everything… ([`8634341`](https://github.com/Byron/gitoxide/commit/86343416dec8026f32c57d164dec4bf9b75b6536))
    - [ref #190] refactor ([`07126d6`](https://github.com/Byron/gitoxide/commit/07126d65946e981b339b6535986597cb328a1c9e))
    - [ref #190] Allow for explicit expected previous values ([`1a4786f`](https://github.com/Byron/gitoxide/commit/1a4786fb3bdb3d3a86b026dbf04e6baef6d3c695))
    - [repository #190] show that unconditional creation of references doesn't is lacking… ([`06b9270`](https://github.com/Byron/gitoxide/commit/06b9270e67823e9e911a9fa9d6eeeedcd93e62cb))
    - [repository #190] another commit() test… ([`4ec631c`](https://github.com/Byron/gitoxide/commit/4ec631c92349bbffa69c786838d2127b0c51970e))
    - [repository #190] produce nice reflog messages ([`e7a8b62`](https://github.com/Byron/gitoxide/commit/e7a8b62eb24f840f639aa436b4e79a4a567d3d05))
    - [repository #190] commit::summary() ([`43f7568`](https://github.com/Byron/gitoxide/commit/43f7568bd11fc310bac8350991ff3d4183dcd17b))
    - [repository #190] thanks clippy ([`0763ac2`](https://github.com/Byron/gitoxide/commit/0763ac260450b53b42f3c139deae5736fef056ce))
    - [repository #190] first version of 'commit(…)' without reflog message handling ([`bfcf8f1`](https://github.com/Byron/gitoxide/commit/bfcf8f17c7a89027e5bbcb5f85e3d0ba4036e8a0))
    - [refs #190] refactor; handle value-checks in dereffed symlinks correctly ([`63bedc7`](https://github.com/Byron/gitoxide/commit/63bedc7647bb584353289e19972adf351765a526))
    - [repository #190] put git-lock into ST1… ([`26a6637`](https://github.com/Byron/gitoxide/commit/26a6637222081997ad7c08f4dc8d8facfb9cf94e))
    - [repository #190] refactor ([`1e029b4`](https://github.com/Byron/gitoxide/commit/1e029b4beb6266853d5035c52b3d85bf98469556))
    - [repository #190] A way to write objects and the empty tree specifically ([`7c559d6`](https://github.com/Byron/gitoxide/commit/7c559d6e1b68bc89220bca426257f383bce586ae))
    - [various #190] rename 'local-offset' to 'local-time-support' ([`3a7d379`](https://github.com/Byron/gitoxide/commit/3a7d3793a235ac872437f3bfedb9dd8fde9b31b1))
    - [repository #190] Make local-offset available on demand only… ([`1927be7`](https://github.com/Byron/gitoxide/commit/1927be7764f6af04ecc715dd52c631a3c8e16577))
    - [repository #185] rustfmt ([`dfbb015`](https://github.com/Byron/gitoxide/commit/dfbb015a89db47c79015135870013ecc384c4aea))
    - [repository #185] remove quick-error infavor of thiserror ([`212c44c`](https://github.com/Byron/gitoxide/commit/212c44c84b903681f6d35d934ee5f7ad6e1da791))
    - [repository #185] on the way to removing quick-error ([`6ecd431`](https://github.com/Byron/gitoxide/commit/6ecd431661e7ddc2f97e5a78a7932d2a7f1f27f0))
    - [repository #185] support for initializing bare repositories ([`9e8a39e`](https://github.com/Byron/gitoxide/commit/9e8a39e3cbd620bd48f379743df0d5783c33a86f))
    - [repository #185] use git-config to handle bare repos more properly ([`8a5aac5`](https://github.com/Byron/gitoxide/commit/8a5aac55cf62bdd7287a363fa29f12aa39d4c583))
    - [repository #185] sketch of how to open a repository… ([`48207b5`](https://github.com/Byron/gitoxide/commit/48207b54b97ac1b6354f6b53c13ccc4d1d8ea98f))
    - [repository #185] refactor ([`63089ff`](https://github.com/Byron/gitoxide/commit/63089ff356ea0f62963ae213ea0dbb09f891ada6))
    - [repository #185] refactor ([`7604935`](https://github.com/Byron/gitoxide/commit/7604935b12eacb26a98bedc5f77636b5583629a5))
    - [repository #185] refactor repository initialization… ([`5ff7eaa`](https://github.com/Byron/gitoxide/commit/5ff7eaa86bddfa94aec97355a5d6adb117045693))
</details>

## v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-repository v0.8.1 ([`b269a12`](https://github.com/Byron/gitoxide/commit/b269a1264f830bafcfe74f0f3ce01448c894146e))
    - [repository #164] make EasyArcExclusive available ([`2fa3dcb`](https://github.com/Byron/gitoxide/commit/2fa3dcb40a34a7ec19382e5f6a71348ecf7a7c36))
</details>

## v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 116 commits contributed to the release over the course of 10 calendar days.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - [repository #174] keep assets ([`e0fca77`](https://github.com/Byron/gitoxide/commit/e0fca771f5ee068b0a9a0975930317d0883701cc))
    - [repository #174] remove arc_lock code entirely ([`dcbe742`](https://github.com/Byron/gitoxide/commit/dcbe742eb5244f0b5c6563cf59962183b708f54f))
    - [repository #174] conditionally compile future parking_lot version… ([`5375fc8`](https://github.com/Byron/gitoxide/commit/5375fc872b9af2526683326f58e9c3d7f20ef166))
    - Bump git-repository v0.8.0 ([`cdb45ff`](https://github.com/Byron/gitoxide/commit/cdb45ffa0810e9fcc9fd25bff7b696c2d27eeef5))
    - [repository #174] adjust various changelogs ([`081faf5`](https://github.com/Byron/gitoxide/commit/081faf5c3a21b34b7068b44d8206fb5770c392f5))
    - Bump git-protocol v0.10.0 ([`82d5a0b`](https://github.com/Byron/gitoxide/commit/82d5a0bb38903a8389e43cd5416e02e5496e661a))
    - Bump git-odb v0.21.0 ([`7b9854f`](https://github.com/Byron/gitoxide/commit/7b9854fb35e86958a5ca827ec9a55b1168f38395))
    - [pack #179] refactor ([`ab6554b`](https://github.com/Byron/gitoxide/commit/ab6554b0cd5838f1ea4e82f6b5019798288076fa))
    - [packetline #178] fix compile warnings ([`c8d2e72`](https://github.com/Byron/gitoxide/commit/c8d2e72d272243da7d853f78463552bfc58ed9d6))
    - Bump git-traverse v0.8.0 ([`54f3541`](https://github.com/Byron/gitoxide/commit/54f3541f1448a8afa044d3958fa1be5b074e4445))
    - Bump git-diff v0.9.0 ([`2e2e798`](https://github.com/Byron/gitoxide/commit/2e2e7983178b3af7e5684995de68ed5d020927ec))
    - [object #177] cleanup CommitRefIter imports and git_object::Error ([`058f68a`](https://github.com/Byron/gitoxide/commit/058f68a9e1cd79fd5a2a1235da42358bc92ed255))
    - [object #177] fix docs ([`2fd23ed`](https://github.com/Byron/gitoxide/commit/2fd23ed9ad556b8e46cf650e23f0c6726e304708))
    - [object #177] migrate immutable::commit into crate::commit ([`45d3934`](https://github.com/Byron/gitoxide/commit/45d393438eac2c7ecd47670922437dd0de4cd69b))
    - [object #177] tag::RefIter -> TagRefIter ([`28587c6`](https://github.com/Byron/gitoxide/commit/28587c691eb74e5cb097afb2b63f9d9e2561c45d))
    - [object #177] move mutable objects to crate::* ([`c551c02`](https://github.com/Byron/gitoxide/commit/c551c0236c64f3237cb9be7f35159f753d4b871f))
    - [object #177] migrate immutable::tree to crate::tree ([`fa5cd06`](https://github.com/Byron/gitoxide/commit/fa5cd0648d5c855060ab2b75ee933851987c2dcf))
    - [object #177] move immutable::* to crate::*Ref, start `iter` adjustments ([`461dc53`](https://github.com/Byron/gitoxide/commit/461dc53ba3bc07d55fdb4aad7570ba9176a8b360))
    - [object #177] rename immutable::* to immutable::*Ref ([`6deb012`](https://github.com/Byron/gitoxide/commit/6deb01291fb382b7fb9206682e319afa81bacc05))
    - Release git-object v0.13.0 ([`708fc5a`](https://github.com/Byron/gitoxide/commit/708fc5abd8af4dd7459f388c7092bf35915c6662))
    - Merge branch 'git-ref-refactor' ([`5dbf753`](https://github.com/Byron/gitoxide/commit/5dbf753ce2035ffd07e4bce7ceb3bcd4e309c16e))
    - [ref #175] follow (try_)find(_what) naming convention ([`679895c`](https://github.com/Byron/gitoxide/commit/679895cf866d643e768e353af614a55aeed2ba5c))
    - [ref #175] make 'mutable' module private ([`a80dbcf`](https://github.com/Byron/gitoxide/commit/a80dbcf083bfcf2e291013f7b13bba9e787c5cb4))
    - Release git-actor v0.5.0 ([`a684b0f`](https://github.com/Byron/gitoxide/commit/a684b0ff96ebfc5e4b3ce78452dc21ce856a6869))
    - [ref #175] refactor ([`292e567`](https://github.com/Byron/gitoxide/commit/292e567eaa04a121fb4d7262bb316d37dd8ad11f))
    - Release git-actor v0.4.0 ([`16358c9`](https://github.com/Byron/gitoxide/commit/16358c9bf03604857d51bfa4dbfd2fc8c5210da7))
    - [actor #173] rename immutable::Signature to SignatureRef! ([`96461ac`](https://github.com/Byron/gitoxide/commit/96461ace776d6b351b313d4f2697f2d95b9e196e))
    - Release git-lock v1.0.0 ([`f38f72c`](https://github.com/Byron/gitoxide/commit/f38f72c73f69775358d8b047de2e354364fcafc2))
    - Release git-tempfile v1.0.0 ([`1238535`](https://github.com/Byron/gitoxide/commit/123853539dc30ddea2d822ab177ee09b191bdf1b))
    - [smart-release #171] it's about time we get some tests ([`48a489b`](https://github.com/Byron/gitoxide/commit/48a489b4247ed6feff222924bdcdb53ce45c6ce6))
    - [stability #171] Prime git-tempfile and git-lock for release ([`01278fe`](https://github.com/Byron/gitoxide/commit/01278fe4e28bf97ce6a2b8947198683646e361ee))
    - [stability #171] mark git-hash and git-actor as ST1 as well ([`32caae1`](https://github.com/Byron/gitoxide/commit/32caae1c32aae38bde59756e52848bef1cef049b))
    - [stability #171] git-ref is now ST1 and available through git-repository ([`50154cd`](https://github.com/Byron/gitoxide/commit/50154cd02fdd90930a1d7c5a4406d53c8067cb4b))
    - [smart-release #171] Try to avoid unstable git-repository features… ([`c8f325b`](https://github.com/Byron/gitoxide/commit/c8f325bed5d644eded035109702098f9fed3fba3))
    - Merge branch 'main' into stability ([`11bae43`](https://github.com/Byron/gitoxide/commit/11bae437e473fef6ed09c178d54ad11eee001b1d))
    - [stability #171] Don't provide access to less stable crates in `Respository` ([`e4c5b58`](https://github.com/Byron/gitoxide/commit/e4c5b58ad935c907dfbd0d61049453dcb64a7e19))
    - cleanup imports ([`e669303`](https://github.com/Byron/gitoxide/commit/e6693032f1391416fd704c21617051ddfb862a3a))
    - [stability #171] Don't leak unstable plumbing crates in git-repository… ([`71eb30f`](https://github.com/Byron/gitoxide/commit/71eb30f1caa41c1f9fe5d2785b71c9d77922c2af))
    - Release git-pack v0.9.0 ([`7fbc961`](https://github.com/Byron/gitoxide/commit/7fbc9617da97d4ba4bb3784f41d4163c0839c03c))
    - [repository #164] top-level easy docs ([`6b71c51`](https://github.com/Byron/gitoxide/commit/6b71c51f703aa3b6a7d5a110d04294dd7ea4e8b0))
    - [repository #165] see if `git-config` can already be placed… ([`d287a4a`](https://github.com/Byron/gitoxide/commit/d287a4aec70e5dd33976a25d9a849c44d62d77c9))
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
    - [repository #165] refactor ([`3a0160e`](https://github.com/Byron/gitoxide/commit/3a0160ed1c5bc33d330ad4e9189c4937d194e98d))
    - [repository #165] fmt ([`a02d5aa`](https://github.com/Byron/gitoxide/commit/a02d5aa8ef0e4a1118a9d8523c3f34b836461952))
    - [repository #165] Don't panic on repo borrow error… ([`b2f644a`](https://github.com/Byron/gitoxide/commit/b2f644a73c2b1945ab71c5f5719c9b2b32c01b07))
    - thanks clippy ([`b496d99`](https://github.com/Byron/gitoxide/commit/b496d9952924afdb67e9ba8ea0b9b61c8c8fb1f2))
    - [repository #165] Write about the GAT plan to make this better one day ([`d793ecd`](https://github.com/Byron/gitoxide/commit/d793ecd00f55b5bf7c6dcaee8772975e97bd5e30))
    - [repository #165] quick test to see if Access2 can become Access… ([`45acc7a`](https://github.com/Byron/gitoxide/commit/45acc7a9d6a89977563872c2eac389a2b78b9e27))
    - [repository #165] Generalizing over mutable Repos is possible too… ([`0f7efe3`](https://github.com/Byron/gitoxide/commit/0f7efe3f2e2608213ad5c75b52db876dd4214908))
    - [repository #165] show that Access2 works for all Easy* types… ([`b8ceefe`](https://github.com/Byron/gitoxide/commit/b8ceefed275953aa36d823d51b466cd100729905))
    - [repository #165] First success with creating a shared borrow to the repo ([`f2a38b2`](https://github.com/Byron/gitoxide/commit/f2a38b20aee484e0354d3e2e3db9cc880ae95310))
    - Revert "[repository #165] FAIL Look into `owned_ref` crate" ([`a1443e4`](https://github.com/Byron/gitoxide/commit/a1443e4982fa4d1a1615554a37294d56fd9026eb))
    - [repository #165] FAIL Look into `owned_ref` crate ([`09aa714`](https://github.com/Byron/gitoxide/commit/09aa714f2db5ad220b0e76a65e01e394663f08b4))
    - [repository #165] FAIL AsRef works for basic refs but… ([`02979b6`](https://github.com/Byron/gitoxide/commit/02979b61e6bc4e1de3b3badc784a950477b31cad))
    - [repository #165] FAIL try to generalize with Borrow… ([`295ba95`](https://github.com/Byron/gitoxide/commit/295ba95a341775b566c18e897a2d58a94e6d98f9))
    - [repository #165] FAIL See if EasyExclusive can work… ([`016debb`](https://github.com/Byron/gitoxide/commit/016debbfce7a29502742408da304c80405063230))
    - [repository #165] introduce EasyShared ([`a119ad9`](https://github.com/Byron/gitoxide/commit/a119ad94096a3464b98f6a6bc26c92ba6efa9474))
    - [repository #165] First thoughts about stale caches ([`7f8b63e`](https://github.com/Byron/gitoxide/commit/7f8b63e23ef3561117249668d14507cec1508ad3))
    - [repository #165] hide all easy::State fields behind result-enforcing methods ([`000c537`](https://github.com/Byron/gitoxide/commit/000c537ab766a50679764118af50731b3bab39e5))
    - [repository #165] pack cache access only with errors ([`2353e50`](https://github.com/Byron/gitoxide/commit/2353e5092599228f147ef58c0f0cd45c63c126e2))
    - [repository #165] assure packed-refs is only used non-panicking ([`a355d94`](https://github.com/Byron/gitoxide/commit/a355d943b986307216161bad38e5bb89f8608b49))
    - [repository #165] refactor ([`16fce63`](https://github.com/Byron/gitoxide/commit/16fce637561af29727a8fa025f6ddece853fcc20))
    - [repository #165] a sample of a simpler way to create a tag ([`fb8f584`](https://github.com/Byron/gitoxide/commit/fb8f58412cdd32991a182a41cbc0d463127a4e0e))
    - [smart-release #165] Use generic edit-reference functionality ([`be3e57f`](https://github.com/Byron/gitoxide/commit/be3e57f6221dc87505ba1aad1166e28c328c3b54))
    - [repository #165] sketch generic ref file editing ([`3a026ae`](https://github.com/Byron/gitoxide/commit/3a026aea2a98648a6b624bca9661555f5a147494))
    - [repository #165] refactor ([`00ec15d`](https://github.com/Byron/gitoxide/commit/00ec15dcfdb839095e508139d238df384ea418eb))
    - [repository #165] refactor ([`0f13104`](https://github.com/Byron/gitoxide/commit/0f13104375216ccf099ebc2fcf0d180ed0de5237))
    - [repository #165] An experiment on transforming panics into errors… ([`1f52226`](https://github.com/Byron/gitoxide/commit/1f5222660970e24eb2d82fed3917f234dce7e0eb))
    - [repository #165] offer panicking type conversions for objects ([`f802f8c`](https://github.com/Byron/gitoxide/commit/f802f8c8c382f8063fa615fda022857a740a974a))
    - [repository #165] try a more common naming convention for fallbile things… ([`fc70393`](https://github.com/Byron/gitoxide/commit/fc703937a078937840ea1c254f11e64aaf31de90))
    - [repository #165] refactor ([`6207735`](https://github.com/Byron/gitoxide/commit/6207735f7d955e8a1676c8ad549ce6c1137da760))
    - thanks clippy ([`41d7a44`](https://github.com/Byron/gitoxide/commit/41d7a443aa63b6ee997fd38ceee05b9b1be3e577))
    - [repository #162] cleanup imports ([`983d11a`](https://github.com/Byron/gitoxide/commit/983d11a1f46c1ad21dbf2d57b63ecf979fab48b9))
    - [smart-release #162] use TreeRef capabilities to lookup path ([`51d1943`](https://github.com/Byron/gitoxide/commit/51d19433e6704fabb6547a0ba1b5c32afce43d8b))
    - [repository #162] what could be a correct implementation of a tree path lookup ([`1f638ee`](https://github.com/Byron/gitoxide/commit/1f638eee0aa5f6e1cc34c5bc59a18b5f22af4cbc))
    - [repository #162] detachable ObjectRefs and a few conversions ([`ec123bb`](https://github.com/Byron/gitoxide/commit/ec123bb615035684e52f2d786dfb41d0449823d2))
    - [repository #162] finally let smart-release use the correct abstraction for peeling ([`ba243a3`](https://github.com/Byron/gitoxide/commit/ba243a35ff6f059e5581c6f7ff80e1253ceca6f8))
    - [repository #162] Add id field to ObjectRef… ([`f5ba98e`](https://github.com/Byron/gitoxide/commit/f5ba98ebd0e1d7d0491871be58476cb6882b8436))
    - [repository #162] Make clear that Objects are actually references… ([`d1e6843`](https://github.com/Byron/gitoxide/commit/d1e68435d0b7d9dcc9e0099be3c0c5723dc08e93))
    - [repository #162] another attempt to find a decent peeling abstraction… ([`716d623`](https://github.com/Byron/gitoxide/commit/716d623fb189eb3002d2137827dbfeb143f6ed12))
    - [repository #162] attach the Object to 'Access' ([`9a12564`](https://github.com/Byron/gitoxide/commit/9a125640da19d5633e51df40dee5332eb9600462))
    - [repository #162] refactor ([`a32d361`](https://github.com/Byron/gitoxide/commit/a32d361fd5cb0eb1a4112d834b53c1625372a7bc))
    - [repository #162] trying new names ([`b3f453b`](https://github.com/Byron/gitoxide/commit/b3f453b33f8cda04526110a82f0e0a46a3bb2e34))
    - [repository #162] put impl for finding object data into the extension trait ([`91b9446`](https://github.com/Byron/gitoxide/commit/91b9446fc7035047ebefaa7907e6a8224b56cf27))
    - [repository #162] experiment with finding objects… ([`312a692`](https://github.com/Byron/gitoxide/commit/312a69256a67a0f9d3f3f5c5f9eaf51b50971c5e))
    - thanks clippy ([`f2fb026`](https://github.com/Byron/gitoxide/commit/f2fb0266ba64d002a9913699bcf5843647843beb))
    - [repository #162] Cannot ever store a RefCell Ref in an object… ([`5c17199`](https://github.com/Byron/gitoxide/commit/5c171995383fa9a3698b6aaf3fbd9537110c0299))
    - [repository #162] experiemnt with optionally keeping data in Object ([`b8a8e08`](https://github.com/Byron/gitoxide/commit/b8a8e08e1d972e5069b136c30407c079825b7e1d))
    - [smart-release #162] Object can be used like a git_hash::ObjectId ([`c7bc730`](https://github.com/Byron/gitoxide/commit/c7bc730836f05fe9d967320a6858443a649a59ce))
    - [smart-release #162] format everything ([`8ff83e5`](https://github.com/Byron/gitoxide/commit/8ff83e5c511ae29979348789bd6e7a2f72b16f1c))
    - [smart-release #162] don't throw away work… ([`b43b780`](https://github.com/Byron/gitoxide/commit/b43b780c0382683edc859e3fbd27739716a47141))
    - [smart-release #162] a demo of attaching and detaching objects… ([`ff2927c`](https://github.com/Byron/gitoxide/commit/ff2927ce3fede654d491559fde1c7b07be6a6979))
    - [smart-release #162] an actual Data type… ([`7fd996f`](https://github.com/Byron/gitoxide/commit/7fd996f5f631f83665e81c0f89c34cc47f270d2b))
    - [smart-release #162] unify 'ext' visibility ([`ca082a7`](https://github.com/Byron/gitoxide/commit/ca082a75ff29de2a471cec4331a80f84477cca56))
    - thanks clippy ([`1f2d458`](https://github.com/Byron/gitoxide/commit/1f2d4584f8b650f7e751c8d2df9a5d27725f4f2f))
    - [smart-release #162] a sketch for accessing objects data… ([`ba27101`](https://github.com/Byron/gitoxide/commit/ba27101e08b2bab5d33b53fedcc0c6aa13b8f35e))
    - [smart-release #162] peeling objects to a certain target kind… ([`5785136`](https://github.com/Byron/gitoxide/commit/57851361f3fc729b964fd0ca5dca9f084fe20f5e))
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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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

 - 40 commits contributed to the release over the course of 63 calendar days.
 - 74 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
    - (cargo-release) version 0.2.0 ([`3286e42`](https://github.com/Byron/gitoxide/commit/3286e42547b59df6365087cbae9ce1c9c959faad))
</details>

## v0.6.0 (2021-05-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release.
 - 49 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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

 - 10 commits contributed to the release over the course of 204 calendar days.
 - 208 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`02df134`](https://github.com/Byron/gitoxide/commit/02df1345a22889a573adfc1be80bda271b2dc9a5))
    - Merge branch 'daniel-levin/main' into main ([`1e727af`](https://github.com/Byron/gitoxide/commit/1e727afd9bce7bc4b33f094ccf5b4b94376dea72))
    - refactor ([`170215d`](https://github.com/Byron/gitoxide/commit/170215dc941af9b6a8f19c1fef91f3b5802e1cc7))
    - Ensured linter checks pass ([`51f2183`](https://github.com/Byron/gitoxide/commit/51f2183357573f9ea30dffbf61af73d5e845f5aa))
    - Ensured output of directory-less git init unchanged ([`539a573`](https://github.com/Byron/gitoxide/commit/539a5737459de10404b6ba6f06a20224b6d534af))
    - Added [directory] argument to init. ([`62f8dc6`](https://github.com/Byron/gitoxide/commit/62f8dc62ec3e76efd7311ced32094035856dbcbb))
    - Spelling fix in error message ([`944d0f4`](https://github.com/Byron/gitoxide/commit/944d0f4ae830c8f2e7eabe3bd58cd023f5674ce1))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - refactor ([`ba1d883`](https://github.com/Byron/gitoxide/commit/ba1d88364424eb60a0874a5726b62740dc348592))
</details>

## v0.4.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 28 calendar days.
 - 30 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`2b1bca8`](https://github.com/Byron/gitoxide/commit/2b1bca83c453544972e370dc0adff57cb7590b42))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
</details>

## v0.3.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 31 calendar days.
 - 31 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

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

