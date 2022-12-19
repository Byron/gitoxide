# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.24.0 (2022-12-19)

### New Features

 - <csr-id-0a2b135d19ce1f1b4b0394befaa3949906322c97/> improve granularity of IO errors for `curl` backends.
   That way it should be possible to tell if `curl` caused
   an IO error due to someting that can be considered spurious.
 - <csr-id-9a2f7cd55c05f2fdb0ae62f0efca9dfa451694c7/> `IsSpuriousError` trait and its implementation.
   That way all transports can tell if the operation failed due to
   an issue that's probably passing, so retrying may be a way to resolve
   the issue.
 - <csr-id-5034544b36994177009ccc8d6c07cb000b429174/> `client::http::Options::no_proxy` to disable a proxy for given hosts.
   This is a curl-first option which can reasonably be implemented for other backends
   as well and thus retains its curl-ish roots in full.
 - <csr-id-e701e7e9cc571108ca210fc0ca23494d6a1c7208/> `client::http::Options::verbose` to see more debug output.
   This means different things depending on the backend, and for
   `curl` it means a lot of debug-output on stderr.

### Bug Fixes

 - <csr-id-ff0332e815c228cc5cdfe58c3598ad261bb2879e/> http transports can now reuse a connection.
   This makes connections more efficient generally and `cargo` relies
   on that behaviour in their tests as well.
 - <csr-id-85dcda81d3fec03ad5687b0e0329cefedd925722/> don't pre-configure curl.
   These settings can interact strangly with other users of the curl package
   within the same dependency tree, so it's paramount to only activate features
   we truly need.
 - <csr-id-0d0eb4aa46b265f97ada7b54d8bcc29decc42e50/> ssh connection remove '=' in port argument
 - <csr-id-4927adf1a57166b581fc293a33f84ef628af70db/> make it possible to parse handshakes without newlines in packetlines #(639)
 - <csr-id-7ab7c2409a47fae587531c0c3b203cd646e32984/> correctly display what's actual and expected when failing to parse capabilities.
 - <csr-id-b0083e38c82829b4d8b81542fc8d1025089e2869/> improve compile-time errors if mutually exclusive http-client features are set.

### New Features (BREAKING)

 - <csr-id-1204bfcaadd31ed198b923df05f19115da3754a4/> Provide support for reading packetlines directly.
   The handshake response itself now provides a `ref` read implementation
   with direct readline support. That way one can avoid having to go
   through `String`.
 - <csr-id-8e158c3f4056f59724fe91587157ef0daa517964/> interpret the FollowRedirects option for the curl HTTP backend.
   This comes with changes to the `HTTP` trait which now requires a base-url
   to be provided as well.

### Bug Fixes (BREAKING)

 - <csr-id-08dcda254bc942dcc36d432cf7130c2ce4c6d54e/> `Capabiltiies::from_lines()` takes a single buffer.
   That way it doesn't have to convert to `String` as intermediary
   which may fail as illformed UTF8 might be present.
   
   Performance wise, reading all lines ahead of time, it is the same
   as it was before as it would collect all lines beforehand anyway.
   
   We are also seeing only a few lines in V2, and in V1 it was
   fully streaming already.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 30 commits contributed to the release over the course of 27 calendar days.
 - 27 days passed between releases.
 - 13 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#602](https://github.com/Byron/gitoxide/issues/602), [#639](https://github.com/Byron/gitoxide/issues/639)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#602](https://github.com/Byron/gitoxide/issues/602)**
    - improve compile-time errors if mutually exclusive http-client features are set. ([`b0083e3`](https://github.com/Byron/gitoxide/commit/b0083e38c82829b4d8b81542fc8d1025089e2869))
 * **[#639](https://github.com/Byron/gitoxide/issues/639)**
    - correctly display what's actual and expected when failing to parse capabilities. ([`7ab7c24`](https://github.com/Byron/gitoxide/commit/7ab7c2409a47fae587531c0c3b203cd646e32984))
 * **Uncategorized**
    - prepare chnagelogs prior to git-repository release ([`7114bbb`](https://github.com/Byron/gitoxide/commit/7114bbb6732aa8571d4ab74f28ed3e26e9fbe4d0))
    - Merge branch 'adjustments-for-cargo' ([`94750e1`](https://github.com/Byron/gitoxide/commit/94750e15831969059551af35d31c21009462084d))
    - improve granularity of IO errors for `curl` backends. ([`0a2b135`](https://github.com/Byron/gitoxide/commit/0a2b135d19ce1f1b4b0394befaa3949906322c97))
    - http transports can now reuse a connection. ([`ff0332e`](https://github.com/Byron/gitoxide/commit/ff0332e815c228cc5cdfe58c3598ad261bb2879e))
    - Merge branch 'adjustments-for-cargo' ([`70ccbb2`](https://github.com/Byron/gitoxide/commit/70ccbb21b1113bdeb20b52d274141a9fdb75f579))
    - Merge branch 'main' into adjustments-for-cargo ([`bb60d3d`](https://github.com/Byron/gitoxide/commit/bb60d3d5cb9dbd7abe61accded6d21e320c624db))
    - adapt to changes in git-repository ([`89230f4`](https://github.com/Byron/gitoxide/commit/89230f4e151056abaa2bce39d9d18f6dd1512d59))
    - Merge branch 'paulyoung/scheme-ext' ([`3e27550`](https://github.com/Byron/gitoxide/commit/3e27550577ea942427a57c902570f0416f540753))
    - `IsSpuriousError` trait and its implementation. ([`9a2f7cd`](https://github.com/Byron/gitoxide/commit/9a2f7cd55c05f2fdb0ae62f0efca9dfa451694c7))
    - don't pre-configure curl. ([`85dcda8`](https://github.com/Byron/gitoxide/commit/85dcda81d3fec03ad5687b0e0329cefedd925722))
    - ssh connection remove '=' in port argument ([`0d0eb4a`](https://github.com/Byron/gitoxide/commit/0d0eb4aa46b265f97ada7b54d8bcc29decc42e50))
    - Merge branch 'fix-638' ([`eb4c5f0`](https://github.com/Byron/gitoxide/commit/eb4c5f051ae2a4eb7178289cfc1437417f265608))
    - make it possible to parse handshakes without newlines in packetlines #(639) ([`4927adf`](https://github.com/Byron/gitoxide/commit/4927adf1a57166b581fc293a33f84ef628af70db))
    - Merge branch 'fixture-async' ([`eca6705`](https://github.com/Byron/gitoxide/commit/eca670585db212985d653cb2c6ec3636ec560905))
    - async version of ref-line parsing now reads line by line. ([`dadd896`](https://github.com/Byron/gitoxide/commit/dadd8964ec551702908055476df10624b266a79f))
    - fix hang when reading packetlines lines directly via HTTP ([`2d033ab`](https://github.com/Byron/gitoxide/commit/2d033ab7c7915d27c86a1e7dfbe9cf70d7ae3320))
    - Merge branch 'remove-lines-parsing' ([`9d8e32d`](https://github.com/Byron/gitoxide/commit/9d8e32d3c276fec34e3fce0feb29de0d24a8d1d2))
    - Provide support for reading packetlines directly. ([`1204bfc`](https://github.com/Byron/gitoxide/commit/1204bfcaadd31ed198b923df05f19115da3754a4))
    - `Capabiltiies::from_lines()` takes a single buffer. ([`08dcda2`](https://github.com/Byron/gitoxide/commit/08dcda254bc942dcc36d432cf7130c2ce4c6d54e))
    - make fmt ([`747008d`](https://github.com/Byron/gitoxide/commit/747008d9d370844574dda94e5bec1648c4deb57e))
    - Merge branch 'main' into http-config ([`6b9632e`](https://github.com/Byron/gitoxide/commit/6b9632e16c416841ffff1b767ee7a6c89b421220))
    - `client::http::Options::no_proxy` to disable a proxy for given hosts. ([`5034544`](https://github.com/Byron/gitoxide/commit/5034544b36994177009ccc8d6c07cb000b429174))
    - `client::http::Options::verbose` to see more debug output. ([`e701e7e`](https://github.com/Byron/gitoxide/commit/e701e7e9cc571108ca210fc0ca23494d6a1c7208))
    - Release git-features v0.24.1, git-actor v0.14.1, git-index v0.9.1 ([`7893502`](https://github.com/Byron/gitoxide/commit/789350208efc9d5fc6f9bc4f113f77f9cb445156))
    - interpret the FollowRedirects option for the curl HTTP backend. ([`8e158c3`](https://github.com/Byron/gitoxide/commit/8e158c3f4056f59724fe91587157ef0daa517964))
    - Merge branch 'http-config' ([`a4ff140`](https://github.com/Byron/gitoxide/commit/a4ff140a0d3607cf282c49228c1248bd36d464fd))
    - Merge branch 'main' into http-config ([`bcd9654`](https://github.com/Byron/gitoxide/commit/bcd9654e56169799eb706646da6ee1f4ef2021a9))
    - make fmt ([`0abab7d`](https://github.com/Byron/gitoxide/commit/0abab7da2ec1b8560e6c1eb009f534c9fc7814fe))
</details>

## 0.23.0 (2022-11-21)

### New Features

 - <csr-id-68ed6d7e2cabc3d3bc78a29003863cd4194549fa/> Support for proxy authentication in http configuration.

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

### Bug Fixes (BREAKING)

 - <csr-id-4308a209dddcbb461c34d45fb9af8b4621d4600a/> `max-pure` now builds without any C build tooling due to lack of `openssl-sys`.
   To make this work, we leave the `reqwest` configuration to downstream crates.
   Note that this means downstream will have to select their TLS settings
   themselves, so builds may fail after upgrade until this is done.
   
   An example for a `reqwest` configuration can be found in the
   `gitoxide` Cargo.toml in the root of the `gitoxide` repository.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#602](https://github.com/Byron/gitoxide/issues/602)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#602](https://github.com/Byron/gitoxide/issues/602)**
    - `max-pure` now builds without any C build tooling due to lack of `openssl-sys`. ([`4308a20`](https://github.com/Byron/gitoxide/commit/4308a209dddcbb461c34d45fb9af8b4621d4600a))
 * **Uncategorized**
    - Release git-hash v0.10.0, git-features v0.24.0, git-date v0.3.0, git-actor v0.14.0, git-glob v0.5.0, git-path v0.6.0, git-quote v0.4.0, git-attributes v0.6.0, git-config-value v0.9.0, git-tempfile v3.0.0, git-lock v3.0.0, git-validate v0.7.0, git-object v0.23.0, git-ref v0.20.0, git-sec v0.5.0, git-config v0.12.0, git-command v0.2.0, git-prompt v0.2.0, git-url v0.11.0, git-credentials v0.7.0, git-diff v0.23.0, git-discover v0.9.0, git-bitmap v0.2.0, git-traverse v0.19.0, git-index v0.9.0, git-mailmap v0.6.0, git-chunk v0.4.0, git-pack v0.27.0, git-odb v0.37.0, git-packetline v0.14.0, git-transport v0.23.0, git-protocol v0.24.0, git-revision v0.7.0, git-refspec v0.4.0, git-worktree v0.9.0, git-repository v0.29.0, git-commitgraph v0.11.0, gitoxide-core v0.21.0, gitoxide v0.19.0, safety bump 28 crates ([`b2c301e`](https://github.com/Byron/gitoxide/commit/b2c301ef131ffe1871314e19f387cf10a8d2ac16))
    - prepare changelogs prior to release ([`e4648f8`](https://github.com/Byron/gitoxide/commit/e4648f827c97e9d13636d1bbdc83dd63436e6e5c))
    - Merge branch 'max-pure' ([`03ff188`](https://github.com/Byron/gitoxide/commit/03ff1882f2982fba38fbbf245eea13ef9df50f33))
    - thanks clippy ([`c7cba33`](https://github.com/Byron/gitoxide/commit/c7cba333dd8654995b367498609b4280fe394402))
    - Merge branch 'version2021' ([`0e4462d`](https://github.com/Byron/gitoxide/commit/0e4462df7a5166fe85c23a779462cdca8ee013e8))
    - upgrade edition to 2021 in most crates. ([`3d8fa8f`](https://github.com/Byron/gitoxide/commit/3d8fa8fef9800b1576beab8a5bc39b821157a5ed))
    - curl can authenticate the proxy now and store or reject credentials. ([`63b9050`](https://github.com/Byron/gitoxide/commit/63b9050240b80c5493dab3e8d0b1c675f83d78d6))
    - Support for proxy authentication in http configuration. ([`68ed6d7`](https://github.com/Byron/gitoxide/commit/68ed6d7e2cabc3d3bc78a29003863cd4194549fa))
    - setup curl proxy authentication method ([`c048990`](https://github.com/Byron/gitoxide/commit/c0489900f4ec385c9033c585c16934ff54941d65))
    - Support for reading `http.proxyAuthMethod` ([`92f88c9`](https://github.com/Byron/gitoxide/commit/92f88c94ff288b5675ca3296c27ffb66e1716c22))
</details>

## 0.22.0 (2022-11-17)

### Changed

 - <csr-id-28615b3bb9acff86d7a5520172513e3cc22aeda1/> `TransportV2Ext::invoke(…,features,…)` can take key-value pairs more flexibly.
   Value can now also be owned, which is useful if the value type is a
   `Cow<'_, String>`.

### New Features

 - <csr-id-42acc88bbc63850c0d38db70bc46b1058875e2a0/> `client::RequestWriter::into_parts()` to obtain a bare write handled along with a buf reader with packetline capabilties.
   That way it's possible to perform interactions akin to a V1 puash.

### Changed (BREAKING)

 - <csr-id-07512db093e62d9b9185368bd3fa561cfcd1d1d2/> `client::TransportWithoutIO::to_url()` returns `Cow<'_, BStr>`.
   That way, it's possible to efficiently return URLs in the right format,
   or return generates ones as needed.
 - <csr-id-fe2042bff9ae38bf76b76cef14986f9f76bded7d/> `client::TransportWithoutIO::to_url()` returns `BString`.
   That way it will not be lossy in case the URL represents a path, which
   is relevant for transports that refer to paths.
   
   Note that this doesn't matter when the url actually is a URL, as it's
   specified to be valid unicode (I think).
 - <csr-id-759b5d482de048deb24d14043a173079914e7ac8/> `client::TransportV2Ext::invoke()` supports owned `capabilities`.
   That way it's easier to pass custom agent strings when invoking.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 8 calendar days.
 - 8 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-glob v0.4.2, git-config-value v0.8.2, git-lock v2.2.0, git-ref v0.19.0, git-config v0.11.0, git-discover v0.8.0, git-index v0.8.0, git-transport v0.22.0, git-protocol v0.23.0, git-worktree v0.8.0, git-repository v0.28.0, gitoxide-core v0.20.0, gitoxide v0.18.0, safety bump 9 crates ([`0c253b1`](https://github.com/Byron/gitoxide/commit/0c253b15143dcedfe4c66d64ab1ea6e097030651))
    - prepare changelogs prior to release ([`fe5721f`](https://github.com/Byron/gitoxide/commit/fe5721f888c64c79fe9a734a9e33b94a282f8d97))
    - Merge branch 'http-config' ([`665b53e`](https://github.com/Byron/gitoxide/commit/665b53e1c2e1de65fafa28b669f58977868bbc81))
    - Merge branch 'push-support' ([`42356ab`](https://github.com/Byron/gitoxide/commit/42356abf9d08dd86ce464fa48e25bbcc98ceefd4))
    - Add test to see if we can correctly read the server response. ([`4d84a20`](https://github.com/Byron/gitoxide/commit/4d84a208e4d0a8bf3dbcfa6912b92ec1bdf4ec05))
    - `client::RequestWriter::into_parts()` to obtain a bare write handled along with a buf reader with packetline capabilties. ([`42acc88`](https://github.com/Byron/gitoxide/commit/42acc88bbc63850c0d38db70bc46b1058875e2a0))
    - Add simple push request/response as obtained through HTTP proxying ([`0573107`](https://github.com/Byron/gitoxide/commit/05731070a5cfb403bb82d8d8baa360ffe8cc45a9))
    - Avoid hardcoding some arbitrary default for connect timeouts, use curl default instead like git ([`1766568`](https://github.com/Byron/gitoxide/commit/17665683efc8e25cbb35737a8c4132b98e5b6b7a))
    - Set TCP keepalive just like `git` does ([`ee0276c`](https://github.com/Byron/gitoxide/commit/ee0276c7659122e32851da8ee6a9e663982bcda5))
    - Set curl `proxy-type` similar to how git does it ([`717b09f`](https://github.com/Byron/gitoxide/commit/717b09fb3cac024b85a885e253c52e1f37bc0590))
    - Merge branch 'main' into http-config ([`f4ff821`](https://github.com/Byron/gitoxide/commit/f4ff821fd4233dd1dc1a449af4d4600becf3b4ac))
    - Merge branch 'async-fetch' ([`0c9c48b`](https://github.com/Byron/gitoxide/commit/0c9c48b3b91a1396eb1796f288a2cb10380d1f14))
    - fix docs ([`0d1a00d`](https://github.com/Byron/gitoxide/commit/0d1a00da4f4e84f989f18d16878cc95ace797b68))
    - Add `Debug` to `client::http::Options` ([`375565f`](https://github.com/Byron/gitoxide/commit/375565f20906b4ca373978b89573cf711c3637a9))
    - `client::TransportWithoutIO::to_url()` returns `Cow<'_, BStr>`. ([`07512db`](https://github.com/Byron/gitoxide/commit/07512db093e62d9b9185368bd3fa561cfcd1d1d2))
    - `client::TransportWithoutIO::to_url()` returns `BString`. ([`fe2042b`](https://github.com/Byron/gitoxide/commit/fe2042bff9ae38bf76b76cef14986f9f76bded7d))
    - add missing `Debug` impl on transport option types ([`7ca4dec`](https://github.com/Byron/gitoxide/commit/7ca4dec2df83ce7763383fb93db5ba0001c2cc27))
    - fix build ([`b9a5eea`](https://github.com/Byron/gitoxide/commit/b9a5eeafb8e86d7f8faaa4ce1884487db24e554d))
    - `client::TransportV2Ext::invoke()` supports owned `capabilities`. ([`759b5d4`](https://github.com/Byron/gitoxide/commit/759b5d482de048deb24d14043a173079914e7ac8))
    - `TransportV2Ext::invoke(…,features,…)` can take key-value pairs more flexibly. ([`28615b3`](https://github.com/Byron/gitoxide/commit/28615b3bb9acff86d7a5520172513e3cc22aeda1))
    - explain how the `user_agent` option is going to work on the transport layer ([`328c069`](https://github.com/Byron/gitoxide/commit/328c0695692ebde5999c941b95c4cd330edb0f04))
    - implement all straightforward curl options, which includes basic proxy settings. ([`0b60097`](https://github.com/Byron/gitoxide/commit/0b60097671fe2d8037fe44271678fe380ecbd543))
    - Add all fields we'd like to implement for the curl transport. ([`cfc1b9c`](https://github.com/Byron/gitoxide/commit/cfc1b9ccea32b2f870427014543196f09cbae9ac))
    - A note about the status of the http `reqwest` backend ([`4724604`](https://github.com/Byron/gitoxide/commit/47246047dc9c476149511086612411d39b257bc6))
    - Remove double-arc construct in `reqwest` backend ([`c6a474c`](https://github.com/Byron/gitoxide/commit/c6a474c9b22e969b040e7ce05f6fba245dc408d0))
    - sketch of reqwest specific backends can be handled ([`a8b3f96`](https://github.com/Byron/gitoxide/commit/a8b3f96110dc9a68c680bad07c52b0204bd539a3))
    - a sketch of passing curl options down to curl… ([`63e24fc`](https://github.com/Byron/gitoxide/commit/63e24fcb0ad66a9fd3de1e4e440bbdbc430fc614))
    - Merge branch 'main' into http-config ([`7c5b37d`](https://github.com/Byron/gitoxide/commit/7c5b37d28e98f59a6847368a0d0166d2dbb4acc1))
</details>

## 0.21.2 (2022-11-08)

### Fixes

- enable clones from `kernel.org` via `https`. This failed as it wouldn't send the service announcement in protocol V2, 
  which was thought to be mandatory at least when judging from `github.com`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 1 calendar day.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-diff v0.22.0, git-index v0.7.1, git-pack v0.26.0, git-odb v0.36.0, git-transport v0.21.2, git-repository v0.27.0, safety bump 6 crates ([`f0cab31`](https://github.com/Byron/gitoxide/commit/f0cab317bb0c2799fa80d16f3ae1b89d6aee4284))
    - prepare changelogs prior to release ([`f5f3a9e`](https://github.com/Byron/gitoxide/commit/f5f3a9edd038a89c8c6c4da02054e5439bcc0071))
    - fix build ([`910d665`](https://github.com/Byron/gitoxide/commit/910d665e66d6e737a7bab3598c0c0ebfdda1a9cc))
    - thanks clippy ([`0dc4c6f`](https://github.com/Byron/gitoxide/commit/0dc4c6fb2199514437acd7629ea2a4c6bc0555c5))
    - Move HTTP service checks up to the HTTP layer (blocking & async) ([`3046646`](https://github.com/Byron/gitoxide/commit/3046646a2a0b06a0ca5c2f28b99b9a2b10c73368))
    - Fix fixture to work with test setup. ([`cf9a53b`](https://github.com/Byron/gitoxide/commit/cf9a53b8bc7265386d3336cff791462011b42d61))
    - fix error when no service annoucment is sent by the server ([`a1d876f`](https://github.com/Byron/gitoxide/commit/a1d876f8474a05aeca2d852ee126f0d0e110c0f9))
</details>

## 0.21.1 (2022-11-06)

### Bug Fixes

 - <csr-id-375051fa97d79f95fa7179b536e616c4aefd88e2/> Allow `client::connect()` to function with `http-client-reqwest` enabled.
 - <csr-id-4b5d6dfb58f325bba692e1e32636c24ba058022f/> `client::Capabilities` lifetimes now point to `'a` instead of `'self`.
   This generally makes returned values longer, and as long as one would
   expect.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 24 calendar days.
 - 27 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - `client::Capabilities` lifetimes now point to `'a` instead of `'self`. ([`4b5d6df`](https://github.com/Byron/gitoxide/commit/4b5d6dfb58f325bba692e1e32636c24ba058022f))
 * **Uncategorized**
    - Release git-features v0.23.1, git-glob v0.4.1, git-config-value v0.8.1, git-tempfile v2.0.6, git-object v0.22.1, git-ref v0.18.0, git-sec v0.4.2, git-config v0.10.0, git-prompt v0.1.1, git-url v0.10.1, git-credentials v0.6.1, git-diff v0.21.0, git-discover v0.7.0, git-index v0.7.0, git-pack v0.25.0, git-odb v0.35.0, git-transport v0.21.1, git-protocol v0.22.0, git-refspec v0.3.1, git-worktree v0.7.0, git-repository v0.26.0, git-commitgraph v0.10.0, gitoxide-core v0.19.0, gitoxide v0.17.0, safety bump 9 crates ([`d071583`](https://github.com/Byron/gitoxide/commit/d071583c5576fdf5f7717765ffed5681792aa81f))
    - prepare changelogs prior to release ([`423af90`](https://github.com/Byron/gitoxide/commit/423af90c8202d62dc1ea4a76a0df6421d1f0aa06))
    - Merge branch 'git_protocol_host' ([`d13c590`](https://github.com/Byron/gitoxide/commit/d13c59070ae6f1661dd6fee056cef1ff75c89222))
    - Avoid invalid invocations of `git` by removing `GIT_CONFIG_COUNT` ([`aa315b4`](https://github.com/Byron/gitoxide/commit/aa315b4a136bec57a1cce4b245c606904adc5c12))
    - fix connections over the git:// protocol not specifying the host ([`8f985d4`](https://github.com/Byron/gitoxide/commit/8f985d44c71a4434c4cf4ae0e360480bc3694a47))
    - Merge branch 'main' into write-sparse-index ([`c4e6849`](https://github.com/Byron/gitoxide/commit/c4e68496c368611ebe17c6693d06c8147c28c717))
    - make fmt ([`ea2136b`](https://github.com/Byron/gitoxide/commit/ea2136b065979cecb3a1fdbf7b20ed7514128d9a))
    - Merge branch 'gix-clone' ([`def53b3`](https://github.com/Byron/gitoxide/commit/def53b36c3dec26fa78939ab0584fe4ff930909c))
    - Merge branch 'main' into gix-clone ([`fa27570`](https://github.com/Byron/gitoxide/commit/fa27570f491388cce6137af44330d76870d07202))
    - Merge branch 'main' into write-sparse-index ([`70963f5`](https://github.com/Byron/gitoxide/commit/70963f5d8e3b59ce6fe8bcc1844218ac717f3390))
    - Merge branch 'paulyoung/reqwest-body' ([`c9c1658`](https://github.com/Byron/gitoxide/commit/c9c1658d6afa6edca60d8072a23c7f4899891af1))
    - Merge branch 'main' into gix-clone ([`64f81d7`](https://github.com/Byron/gitoxide/commit/64f81d78ae75a0e5914f431bbdc385a6d40f8835))
    - Ensure body can be read before configure_request ([`9065a88`](https://github.com/Byron/gitoxide/commit/9065a8824ea80891c4c06ea28e0780e5517a353d))
    - Merge branch 'paulyoung/reqwest-configuration' ([`93f2dd8`](https://github.com/Byron/gitoxide/commit/93f2dd8f7db87afc04a523458faaa46f9b33f21a))
    - turn unwrap into an explicit expectation ([`57d9078`](https://github.com/Byron/gitoxide/commit/57d90783edbb20f9c7fac782e9cb90943d4941d7))
    - Allow `configure_request` to return an error as well. ([`f5e14c3`](https://github.com/Byron/gitoxide/commit/f5e14c3aead5776962c380852e93352364d637e8))
    - remove unused field ([`8a0a5d6`](https://github.com/Byron/gitoxide/commit/8a0a5d6262cc51874fa1d35e40b901333945117c))
    - Configure Request instead of ClientBuilder ([`0885e6e`](https://github.com/Byron/gitoxide/commit/0885e6e5882a133a55d43fc453056564752028ad))
    - Resolve issues with configure function ([`9124583`](https://github.com/Byron/gitoxide/commit/9124583ebaa650e0344598b277a5b529827776c7))
    - Sketch out configuring a reqwest client builder ([`74573df`](https://github.com/Byron/gitoxide/commit/74573dfa0c40e4ad3515e680e5699aa760cac38d))
    - Merge branch 'main' into gix-clone ([`de4fe06`](https://github.com/Byron/gitoxide/commit/de4fe06202906ea5c62e667826b42cf7b57b1ff0))
    - Allow `client::connect()` to function with `http-client-reqwest` enabled. ([`375051f`](https://github.com/Byron/gitoxide/commit/375051fa97d79f95fa7179b536e616c4aefd88e2))
    - refactor ([`d3c6ae1`](https://github.com/Byron/gitoxide/commit/d3c6ae1fa9e1d6a57f32b5831411707fc4cf962e))
    - fix formatting ([`a962eb5`](https://github.com/Byron/gitoxide/commit/a962eb5c263de3730b464a0e89615a04caac11e4))
    - Fix "http not compiled in" w/ http-client-reqwest ([`dff418d`](https://github.com/Byron/gitoxide/commit/dff418dcf2a970929a61c5eea9b4712437582fbc))
</details>

## 0.21.0 (2022-10-10)

<csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/>

### New Features

 - <csr-id-8e17534b0efa7418eabdc36f89bab9f9db7b2c38/> `reqwest` as blocking HTTP backend via `http-client-reqwest` feature toggle.
   Note that despite passing the same tests that `curl` passes, for
   bulletproof HTTP connections to untrusted servers, the `curl` backend
   should be preferred. `reqwest` is known to hang if `content-length:`
   HTTP headers are longer than the actual content, and no timeout is
   kicking in to stop the hanging. `curl` has no trouble with this for
   example.
 - <csr-id-0fd57c6b491a3c8d0127bc1d1f0eb958437edff9/> Add `client::http::Transport::new_http()` constructor.
   This goes along with `client::http::connect_http()` to support
   connections to via custom transports which are passed from calling
   crates, without relying on the implementation to be built-in.
 - <csr-id-e05c1fefeed23dbedf0420e04a2a408510775380/> Allow defaulting `client::Capabilities`.
   This can be useful in conjunction with `std::mem::take()`, even
   though an empty data structure like that doesn't bear any significance
   beyond that.

### Bug Fixes

 - <csr-id-41b0c19e1aca9406015932862058756af2a26dda/> set the protocol version for local git transports as well.
   Previously this was only done for ssh based connections, and requires
   setting an environment variable.
 - <csr-id-6d8b66a9bee901eb8cb869e6e28dbb25988f1fed/> remove `Drop` for `SpawnProcessOnDemand`.
   It is well-intended but is likely to hang the calling process
   if for any reason not all process output was consumed.
   
   Even though not waiting for the process leaves it running, it will
   stop naturally once its output pipe breaks once once
   our handles for it are inevitable dropped at the same time.
 - <csr-id-237682a529dc54e33e4738f34915d872aeb89514/> compare 'Content-Type' header case-insensitively, as required by the http spec.

### Other

 - <csr-id-5bef0a00e8d01110c054a517f6d9696f981a7efc/> try to make the transport configurable after being boxed, but…
   …that would force it to be 'static, which is something we excplicitly
   cannot have. We need references to be contained within, if I remember
   correctly.

### Changed (BREAKING)

 - <csr-id-1cf66c4dbc7a0404701efe4335363c2636ce32f8/> `client::http::connect()` returns `Transport<Impl>` directly.
   It' can't fail, so no need to return `Result<_, Infallible>`.

### New Features (BREAKING)

 - <csr-id-78ad3df64f2c016ba17b158bd9ab1d2341aab399/> add `fetch::Transport::configure` to generically configure any transport.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 20 calendar days.
 - 20 days passed between releases.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - set the protocol version for local git transports as well. ([`41b0c19`](https://github.com/Byron/gitoxide/commit/41b0c19e1aca9406015932862058756af2a26dda))
    - remove `Drop` for `SpawnProcessOnDemand`. ([`6d8b66a`](https://github.com/Byron/gitoxide/commit/6d8b66a9bee901eb8cb869e6e28dbb25988f1fed))
    - Allow defaulting `client::Capabilities`. ([`e05c1fe`](https://github.com/Byron/gitoxide/commit/e05c1fefeed23dbedf0420e04a2a408510775380))
    - Use `&dyn Any` instead of unspecified serialization format, as it's the right way. ([`779eefe`](https://github.com/Byron/gitoxide/commit/779eefed97685300f4cd7b09957d3442c96e5b1f))
    - add `fetch::Transport::configure` to generically configure any transport. ([`78ad3df`](https://github.com/Byron/gitoxide/commit/78ad3df64f2c016ba17b158bd9ab1d2341aab399))
    - `client::http::connect()` returns `Transport<Impl>` directly. ([`1cf66c4`](https://github.com/Byron/gitoxide/commit/1cf66c4dbc7a0404701efe4335363c2636ce32f8))
    - Revert "FAIL: try to make the transport configurable after being boxed, but…" ([`fbb96e4`](https://github.com/Byron/gitoxide/commit/fbb96e4d55e322243bf5500605f72e93b103e308))
    - try to make the transport configurable after being boxed, but… ([`5bef0a0`](https://github.com/Byron/gitoxide/commit/5bef0a00e8d01110c054a517f6d9696f981a7efc))
 * **Uncategorized**
    - Release git-hash v0.9.11, git-features v0.23.0, git-actor v0.13.0, git-attributes v0.5.0, git-object v0.22.0, git-ref v0.17.0, git-sec v0.4.1, git-config v0.9.0, git-url v0.10.0, git-credentials v0.6.0, git-diff v0.20.0, git-discover v0.6.0, git-traverse v0.18.0, git-index v0.6.0, git-mailmap v0.5.0, git-pack v0.24.0, git-odb v0.34.0, git-packetline v0.13.1, git-transport v0.21.0, git-protocol v0.21.0, git-revision v0.6.0, git-refspec v0.3.0, git-worktree v0.6.0, git-repository v0.25.0, safety bump 24 crates ([`104d922`](https://github.com/Byron/gitoxide/commit/104d922add61ab21c534c24ce8ed37cddf3e275a))
    - prepare changelogs for release ([`d232567`](https://github.com/Byron/gitoxide/commit/d23256701a95284857dc8d1cb37c7c94cada973c))
    - Merge branch 'fix-smart-release' ([`aa80b60`](https://github.com/Byron/gitoxide/commit/aa80b606e5570f327660cca42ea81581a6e9d5e3))
    - make fmt ([`7b9c065`](https://github.com/Byron/gitoxide/commit/7b9c06547b75929e3e5bf4240f43c7e9bc7d54e0))
    - `reqwest` as blocking HTTP backend via `http-client-reqwest` feature toggle. ([`8e17534`](https://github.com/Byron/gitoxide/commit/8e17534b0efa7418eabdc36f89bab9f9db7b2c38))
    - re-enable last remaining test for reqwest ([`9ef6a54`](https://github.com/Byron/gitoxide/commit/9ef6a546cac78d335bbe60853046bb23becab623))
    - Merge branch 'main' into new-http-impl ([`702a161`](https://github.com/Byron/gitoxide/commit/702a161ef11fc959611bf44b70e9ffe04561c7ad))
    - and another previously hanging test works - content-length is critical ([`0b0ba03`](https://github.com/Byron/gitoxide/commit/0b0ba03b42a2143787a2d81d287dcc9ca9adb078))
    - clone_v1 now works in reqwest, because… ([`63d5a01`](https://github.com/Byron/gitoxide/commit/63d5a0192ed79f99756930cc9bc9f163119b7a00))
    - make fmt ([`53acf25`](https://github.com/Byron/gitoxide/commit/53acf2565743eff7cead7a42011107b2fc8d7e0e))
    - Merge branch 'fetch-pack' ([`f47c891`](https://github.com/Byron/gitoxide/commit/f47c89129732bcb06fe76a4696fe38ab1151fb0c))
    - Merge branch 'fetch-pack' ([`3c49400`](https://github.com/Byron/gitoxide/commit/3c49400809c7c2120f4ce704c19a0421545b5acd))
    - Merge branch 'main' into fetch-pack ([`93917cb`](https://github.com/Byron/gitoxide/commit/93917cb6ecbb30daf3d20bb5a7c65e12211f084f))
    - Shutdown the stream of the mock server for good measure ([`4e415a8`](https://github.com/Byron/gitoxide/commit/4e415a8414ad0ce2d79bfe29d63d46efa3833218))
    - minor fix to test, but now it hangs. 3 hanging to go… ([`164f5d3`](https://github.com/Byron/gitoxide/commit/164f5d3b0c31afe16ad453e5534284a8e6700c84))
    - one more test works - strangely enough this one doesn't hang. ([`c3150dd`](https://github.com/Byron/gitoxide/commit/c3150dd05bdc158e24adb21d2e678e4008b07740))
    - adjust test expectations to pass one part, but now it hangs like clone_v1 ([`4b731ad`](https://github.com/Byron/gitoxide/commit/4b731adf523c00c8d3fd8dcbecd3b4df9efccbe8))
    - refactor ([`569c6f2`](https://github.com/Byron/gitoxide/commit/569c6f2cbb0e4c12c1547accae7b3160c80a0caf))
    - send error on non-success status codes ([`7ccd4ff`](https://github.com/Byron/gitoxide/commit/7ccd4ffb32d5686d926640ab483381eceb2211de))
    - disable all tests failing under reqwest (except for 1); minor fixes to get it to work ([`ee4eb69`](https://github.com/Byron/gitoxide/commit/ee4eb6991ba95983627b14332ccd5e1fd7b43e07))
    - Properly integrate reqwest to allow testing (even though none of them works yet) ([`250114a`](https://github.com/Byron/gitoxide/commit/250114a856cc9bfeb6f91055a13ca08b6d4b91a7))
    - revise drop of header writer to prevent remote hang ([`2981c4d`](https://github.com/Byron/gitoxide/commit/2981c4d36126f837d15fa6f3cac505a0d7fbb0a6))
    - revise error handling; better docs to explain the 'protocol' ([`fd9634d`](https://github.com/Byron/gitoxide/commit/fd9634d833536e211d89148f4afc1dca49a385dc))
    - in theory, that's what's needed for reqwest to work in blocking mode… ([`ce62f5d`](https://github.com/Byron/gitoxide/commit/ce62f5ddcbb454056ec39458e83aa4fb7774c169))
    - even more safety when dealing with the remote's error state ([`b63d69b`](https://github.com/Byron/gitoxide/commit/b63d69bdeb8006c0da34bd4f1be8999744b669ad))
    - rethink error handling during request creation, completing it ([`1fb0545`](https://github.com/Byron/gitoxide/commit/1fb05451785d2b30124937d3ad53b1e05fbee419))
    - frame for filling in the reqwest implementation ([`943fd15`](https://github.com/Byron/gitoxide/commit/943fd153a2daca25c0e716388c528a83ad43be95))
    - Add 'http-client-reqwest' feature toggle ([`2c9b63c`](https://github.com/Byron/gitoxide/commit/2c9b63cfcc92d4815ddec53d885b6985739f627e))
    - compare 'Content-Type' header case-insensitively, as required by the http spec. ([`237682a`](https://github.com/Byron/gitoxide/commit/237682a529dc54e33e4738f34915d872aeb89514))
    - Merge branch 'paulyoung/git-transport-http-client' ([`c845c16`](https://github.com/Byron/gitoxide/commit/c845c162100ad88f3f7fbc69a5badf80d76d86da))
    - Add `client::http::Transport::new_http()` constructor. ([`0fd57c6`](https://github.com/Byron/gitoxide/commit/0fd57c6b491a3c8d0127bc1d1f0eb958437edff9))
    - Introduce http-client feature to decouple from curl ([`31aa2c9`](https://github.com/Byron/gitoxide/commit/31aa2c92ae4d87071bdb436c0995883f53fb355d))
    - Make Content-Type header check case-insensitive ([`7aa8ab8`](https://github.com/Byron/gitoxide/commit/7aa8ab8b889b278d5f1f49cff570574dfa3146ac))
    - Merge branch 'fix-git-features' ([`82fd251`](https://github.com/Byron/gitoxide/commit/82fd251ac80d07bc9da8a4d36e517aa35580d188))
    - Merge branch 'diff' ([`25a7726`](https://github.com/Byron/gitoxide/commit/25a7726377fbe400ea3c4927d04e9dec99802b7b))
</details>

## 0.20.0 (2022-09-20)

### Changed (BREAKING)

 - <csr-id-99905bacace8aed42b16d43f0f04cae996cb971c/> upgrade `bstr` to `1.0.1`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 22 calendar days.
 - 22 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#450](https://github.com/Byron/gitoxide/issues/450), [#470](https://github.com/Byron/gitoxide/issues/470)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - option to print server information about the connection ([`4720666`](https://github.com/Byron/gitoxide/commit/4720666c8bfdaa3acc5c832b44755d4b4f86e16e))
    - upgrade `bstr` to `1.0.1` ([`99905ba`](https://github.com/Byron/gitoxide/commit/99905bacace8aed42b16d43f0f04cae996cb971c))
 * **[#470](https://github.com/Byron/gitoxide/issues/470)**
    - update changelogs prior to release ([`caa7a1b`](https://github.com/Byron/gitoxide/commit/caa7a1bdef74d7d3166a7e38127a59f5ab3cfbdd))
 * **Uncategorized**
    - Release git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0 ([`f5c36d8`](https://github.com/Byron/gitoxide/commit/f5c36d85755d1f0f503b77d9a565fad6aecf6728))
    - Release git-hash v0.9.10, git-features v0.22.5, git-date v0.2.0, git-actor v0.12.0, git-glob v0.4.0, git-path v0.5.0, git-quote v0.3.0, git-attributes v0.4.0, git-config-value v0.8.0, git-tempfile v2.0.5, git-validate v0.6.0, git-object v0.21.0, git-ref v0.16.0, git-sec v0.4.0, git-config v0.8.0, git-discover v0.5.0, git-traverse v0.17.0, git-index v0.5.0, git-worktree v0.5.0, git-testtools v0.9.0, git-command v0.1.0, git-prompt v0.1.0, git-url v0.9.0, git-credentials v0.5.0, git-diff v0.19.0, git-mailmap v0.4.0, git-chunk v0.3.2, git-pack v0.23.0, git-odb v0.33.0, git-packetline v0.13.0, git-transport v0.20.0, git-protocol v0.20.0, git-revision v0.5.0, git-refspec v0.2.0, git-repository v0.24.0, git-commitgraph v0.9.0, gitoxide-core v0.18.0, gitoxide v0.16.0, safety bump 28 crates ([`29a043b`](https://github.com/Byron/gitoxide/commit/29a043be6808a3e9199a9b26bd076fe843afe4f4))
    - Merge branch 'filter-refs' ([`3773b92`](https://github.com/Byron/gitoxide/commit/3773b92b8372c9a40a74d281149ca65b057a7da9))
    - Merge branch 'filter-refs' ([`fd14489`](https://github.com/Byron/gitoxide/commit/fd14489f729172d615d0fa1e8dbd605e9eacf69d))
    - Release git-features v0.22.6 ([`c9eda72`](https://github.com/Byron/gitoxide/commit/c9eda729d8f8bc266c7516c613d38acfb83a4743))
    - make fmt ([`535e967`](https://github.com/Byron/gitoxide/commit/535e967666c6da657ff1b7eff7c64ab27cafb182))
    - Merge branch 'filter-refs-by-spec' ([`5c05198`](https://github.com/Byron/gitoxide/commit/5c051986bd89590a9287d85d84c713d83dfab83a))
    - Merge branch 'main' into filter-refs-by-spec ([`9aa1d3d`](https://github.com/Byron/gitoxide/commit/9aa1d3dc46d4b1c76af257f573aff3aeef2d3fa8))
    - Release git-features v0.22.4, git-url v0.8.0, safety bump 4 crates ([`1d4600a`](https://github.com/Byron/gitoxide/commit/1d4600ae51475c2e225f96c16c41e2c4a2b3f2aa))
    - Merge branch 'main' into index-from-tree ([`bc64b96`](https://github.com/Byron/gitoxide/commit/bc64b96a2ec781c72d1d4daad38aa7fb8b74f99b))
    - Merge branch 'main' into filter-refs-by-spec ([`cef0b51`](https://github.com/Byron/gitoxide/commit/cef0b51ade2a3301fa09ede7a425aa1fe3527e78))
</details>

## 0.19.3 (2022-08-28)

Maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 4 calendar days.
 - 4 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#XXX](https://github.com/Byron/gitoxide/issues/XXX)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#XXX](https://github.com/Byron/gitoxide/issues/XXX)**
    - prepare changelogs prior to release ([`8c0bca3`](https://github.com/Byron/gitoxide/commit/8c0bca37ff9fbaadbe55561fb2b0d649980c95b1))
 * **Uncategorized**
    - Release git-object v0.20.3, git-ref v0.15.4, git-config v0.7.1, git-diff v0.18.0, git-traverse v0.16.3, git-pack v0.22.0, git-odb v0.32.0, git-url v0.7.3, git-transport v0.19.3, git-protocol v0.19.1, git-refspec v0.1.1, git-repository v0.23.0, safety bump 6 crates ([`85a3bed`](https://github.com/Byron/gitoxide/commit/85a3bedd68d2e5f36592a2f691c977dc55298279))
    - Release git-features v0.22.3, git-revision v0.4.4 ([`c2660e2`](https://github.com/Byron/gitoxide/commit/c2660e2503323531ba02519eaa51124ee22fec51))
    - Merge branch 'main' into filter-refs-by-spec ([`cfa1440`](https://github.com/Byron/gitoxide/commit/cfa144031dbcac2707ab0cec012bc35e78f9c475))
</details>

## 0.19.2 (2022-08-24)

<csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/>
<csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/>

### Chore

 - <csr-id-f7f136dbe4f86e7dee1d54835c420ec07c96cd78/> uniformize deny attributes
 - <csr-id-533e887e80c5f7ede8392884562e1c5ba56fb9a8/> remove default link to cargo doc everywhere

### New Features

 - <csr-id-b1c40b0364ef092cd52d03b34f491b254816b18d/> use docsrs feature in code to show what is feature-gated automatically on docs.rs
 - <csr-id-517677147f1c17304c62cf97a1dd09f232ebf5db/> pass --cfg docsrs when compiling for https://docs.rs

### Bug Fixes

 - <csr-id-5220f9a59fb699e111342b076145a6899d36d433/> Make async `conenct()` signature compatible with the blocking implementation.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - adjust to changes in `git-url` ([`bb83843`](https://github.com/Byron/gitoxide/commit/bb838430ecbaa18921ef60af04ff684809572ce2))
    - Make async `conenct()` signature compatible with the blocking implementation. ([`5220f9a`](https://github.com/Byron/gitoxide/commit/5220f9a59fb699e111342b076145a6899d36d433))
 * **Uncategorized**
    - Release git-date v0.0.5, git-hash v0.9.8, git-features v0.22.2, git-actor v0.11.3, git-glob v0.3.2, git-quote v0.2.1, git-attributes v0.3.2, git-tempfile v2.0.4, git-lock v2.1.1, git-validate v0.5.5, git-object v0.20.2, git-ref v0.15.2, git-sec v0.3.1, git-config v0.7.0, git-credentials v0.4.0, git-diff v0.17.2, git-discover v0.4.1, git-bitmap v0.1.2, git-index v0.4.2, git-mailmap v0.3.2, git-chunk v0.3.1, git-traverse v0.16.2, git-pack v0.21.2, git-odb v0.31.2, git-packetline v0.12.7, git-url v0.7.2, git-transport v0.19.2, git-protocol v0.19.0, git-revision v0.4.2, git-refspec v0.1.0, git-worktree v0.4.2, git-repository v0.22.0, safety bump 4 crates ([`4974eca`](https://github.com/Byron/gitoxide/commit/4974eca96d525d1ee4f8cad79bb713af7a18bf9d))
    - Merge branch 'remote-ls-refs' ([`39d585d`](https://github.com/Byron/gitoxide/commit/39d585d9f9ac6f3ecf51359c8e37f0a50e21ed45))
    - Merge branch 'main' into remote-ls-refs ([`e2ee3de`](https://github.com/Byron/gitoxide/commit/e2ee3ded97e5c449933712883535b30d151c7c78))
    - Merge branch 'docsrs-show-features' ([`31c2351`](https://github.com/Byron/gitoxide/commit/31c235140cad212d16a56195763fbddd971d87ce))
    - use docsrs feature in code to show what is feature-gated automatically on docs.rs ([`b1c40b0`](https://github.com/Byron/gitoxide/commit/b1c40b0364ef092cd52d03b34f491b254816b18d))
    - uniformize deny attributes ([`f7f136d`](https://github.com/Byron/gitoxide/commit/f7f136dbe4f86e7dee1d54835c420ec07c96cd78))
    - pass --cfg docsrs when compiling for https://docs.rs ([`5176771`](https://github.com/Byron/gitoxide/commit/517677147f1c17304c62cf97a1dd09f232ebf5db))
    - remove default link to cargo doc everywhere ([`533e887`](https://github.com/Byron/gitoxide/commit/533e887e80c5f7ede8392884562e1c5ba56fb9a8))
    - Merge branch 'main' into remote-ls-refs ([`bd5f3e8`](https://github.com/Byron/gitoxide/commit/bd5f3e8db7e0bb4abfb7b0f79f585ab82c3a14ab))
</details>

## 0.19.1 (2022-08-17)

A maintenance release without user facing changes.

### Changed (BREAKING)

 - <csr-id-9509ce4faeca8b4e1527bac625370403495bb03c/> `client::connect()` supports anything that parses into a `git_url::Url`; turn http url back to &str
   The http url is always valid UTF-8 and doesn't contain invalid paths,
   thus we should have the type system reflect that.
 - <csr-id-52e8c149ff17ce894cc30d03ead1988f52f0663e/> `client::connect()` now takes a `&BStr` as URL
 - <csr-id-71a43d0bc12661efcb9c94697c704f700a3be488/> use `thiserror` instead of `quickerror`
 - <csr-id-12589cc6f08e4d7aabae30bcdadaa0c2b4850229/> adapt to changes in `git-url` and use `BString` to represent URLs.
   They can contain paths, which is why `String` can't repsent a URL
   losslessly.
   
   For HTTP urls these are ultimately UTF-8 strings though.

### New Features

 - <csr-id-f6a6a499f20e12e2bcca734bdf3c8599d37f6a6f/> `connect()` method is available in when `async-std` feature is set along with `async-client`
   This makes some async support available even trough the base crate,
   which otherwise would require establishing a connection (with a runtime
   of choice) by hand.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 26 calendar days.
 - 26 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#450](https://github.com/Byron/gitoxide/issues/450)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#450](https://github.com/Byron/gitoxide/issues/450)**
    - sketch of simple delegate to collect listed refs ([`1c5f561`](https://github.com/Byron/gitoxide/commit/1c5f5617940efe818a5e2aca5afe2cbd7f4ad940))
    - adjust to changes in `git-transport` ([`94a623b`](https://github.com/Byron/gitoxide/commit/94a623b57e8fa2876731f74224b406ac56838edc))
    - fix async `connect()` to match new signature ([`eb15a79`](https://github.com/Byron/gitoxide/commit/eb15a7971f864bfe02073a21d545ce9ac3dc58e5))
    - `client::connect()` supports anything that parses into a `git_url::Url`; turn http url back to &str ([`9509ce4`](https://github.com/Byron/gitoxide/commit/9509ce4faeca8b4e1527bac625370403495bb03c))
    - `client::connect()` now takes a `&BStr` as URL ([`52e8c14`](https://github.com/Byron/gitoxide/commit/52e8c149ff17ce894cc30d03ead1988f52f0663e))
    - `connect()` method is available in when `async-std` feature is set along with `async-client` ([`f6a6a49`](https://github.com/Byron/gitoxide/commit/f6a6a499f20e12e2bcca734bdf3c8599d37f6a6f))
    - use `thiserror` instead of `quickerror` ([`71a43d0`](https://github.com/Byron/gitoxide/commit/71a43d0bc12661efcb9c94697c704f700a3be488))
    - better docs for `git-transport` ([`30b7b67`](https://github.com/Byron/gitoxide/commit/30b7b67740f80b9954c7fe77d7007722cb95d673))
    - adapt to changes in `git-url` ([`9456146`](https://github.com/Byron/gitoxide/commit/9456146531226e5efc6e1e4e2e89b03683f8a422))
    - adapt to changes in `git-url` and use `BString` to represent URLs. ([`12589cc`](https://github.com/Byron/gitoxide/commit/12589cc6f08e4d7aabae30bcdadaa0c2b4850229))
 * **Uncategorized**
    - Release git-date v0.0.3, git-actor v0.11.1, git-attributes v0.3.1, git-tempfile v2.0.3, git-object v0.20.1, git-ref v0.15.1, git-config v0.6.1, git-diff v0.17.1, git-discover v0.4.0, git-bitmap v0.1.1, git-index v0.4.1, git-mailmap v0.3.1, git-traverse v0.16.1, git-pack v0.21.1, git-odb v0.31.1, git-packetline v0.12.6, git-url v0.7.1, git-transport v0.19.1, git-protocol v0.18.1, git-revision v0.4.0, git-worktree v0.4.1, git-repository v0.21.0, safety bump 5 crates ([`c96473d`](https://github.com/Byron/gitoxide/commit/c96473dce21c3464aacbc0a62d520c1a33172611))
    - prepare changelogs prior to reelase ([`c06ae1c`](https://github.com/Byron/gitoxide/commit/c06ae1c606b6af9c2a12021103d99c2810750d60))
    - Release git-hash v0.9.7, git-features v0.22.1 ([`232784a`](https://github.com/Byron/gitoxide/commit/232784a59ded3e8016e4257c7e146ad385cdd64a))
    - Merge pull request #2 from SidneyDouw/main ([`ce885ad`](https://github.com/Byron/gitoxide/commit/ce885ad4c3324c09c83751c32e014f246c748766))
    - Merge branch 'Byron:main' into main ([`9b9ea02`](https://github.com/Byron/gitoxide/commit/9b9ea0275f8ff5862f24cf5a4ca53bb1cd610709))
    - Merge branch 'main' into rev-parse-delegate ([`6da8250`](https://github.com/Byron/gitoxide/commit/6da82507588d3bc849217c11d9a1d398b67f2ed6))
    - Merge branch 'main' into pathspec ([`7b61506`](https://github.com/Byron/gitoxide/commit/7b615060712565f515515e35a3e8346278ad770c))
    - Merge branch 'kianmeng-fix-typos' ([`4e7b343`](https://github.com/Byron/gitoxide/commit/4e7b34349c0a01ad8686bbb4eb987e9338259d9c))
    - Fix typos ([`e9fcb70`](https://github.com/Byron/gitoxide/commit/e9fcb70e429edb2974afa3f58d181f3ef14c3da3))
</details>

## 0.19.0 (2022-07-22)

This is a maintenance release with no functional changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 33 calendar days.
 - 39 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0 ([`aa639d8`](https://github.com/Byron/gitoxide/commit/aa639d8c43f3098cc4a5b50614c5ae94a8156928))
    - Release git-hash v0.9.6, git-features v0.22.0, git-date v0.0.2, git-actor v0.11.0, git-glob v0.3.1, git-path v0.4.0, git-attributes v0.3.0, git-tempfile v2.0.2, git-object v0.20.0, git-ref v0.15.0, git-sec v0.3.0, git-config v0.6.0, git-credentials v0.3.0, git-diff v0.17.0, git-discover v0.3.0, git-index v0.4.0, git-mailmap v0.3.0, git-traverse v0.16.0, git-pack v0.21.0, git-odb v0.31.0, git-url v0.7.0, git-transport v0.19.0, git-protocol v0.18.0, git-revision v0.3.0, git-worktree v0.4.0, git-repository v0.20.0, git-commitgraph v0.8.0, gitoxide-core v0.15.0, gitoxide v0.13.0, safety bump 22 crates ([`4737b1e`](https://github.com/Byron/gitoxide/commit/4737b1eea1d4c9a8d5a69fb63ecac5aa5d378ae5))
    - prepare changelog prior to release ([`3c50625`](https://github.com/Byron/gitoxide/commit/3c50625fa51350ec885b0f38ec9e92f9444df0f9))
    - Merge pull request #1 from Byron/main ([`085e76b`](https://github.com/Byron/gitoxide/commit/085e76b121291ed9bd324139105d2bd4117bedf8))
    - Merge branch 'main' into pathspec ([`89ea12b`](https://github.com/Byron/gitoxide/commit/89ea12b558bcc056b892193ee8fb44b8664b5da4))
    - Merge branch 'main' into cont_include_if ([`daa71c3`](https://github.com/Byron/gitoxide/commit/daa71c3b753c6d76a3d652c29237906b3e28728f))
    - thanks clippy ([`e1003d5`](https://github.com/Byron/gitoxide/commit/e1003d5fdee5d4439c0cf0286c67dec9b5e34f53))
    - Merge branch 'main' into cont_include_if ([`41ea8ba`](https://github.com/Byron/gitoxide/commit/41ea8ba78e74f5c988148367386a1f4f304cb951))
    - Release git-path v0.3.0, safety bump 14 crates ([`400c9be`](https://github.com/Byron/gitoxide/commit/400c9bec49e4ec5351dc9357b246e7677a63ea35))
</details>

## 0.18.0 (2022-06-13)

A maintenance release without user-facing changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 25 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-date v0.0.1, git-hash v0.9.5, git-features v0.21.1, git-actor v0.10.1, git-path v0.2.0, git-attributes v0.2.0, git-ref v0.14.0, git-sec v0.2.0, git-config v0.5.0, git-credentials v0.2.0, git-discover v0.2.0, git-pack v0.20.0, git-odb v0.30.0, git-url v0.6.0, git-transport v0.18.0, git-protocol v0.17.0, git-revision v0.2.1, git-worktree v0.3.0, git-repository v0.19.0, safety bump 13 crates ([`a417177`](https://github.com/Byron/gitoxide/commit/a41717712578f590f04a33d27adaa63171f25267))
    - update changelogs prior to release ([`bb424f5`](https://github.com/Byron/gitoxide/commit/bb424f51068b8a8e762696890a55ab48900ab980))
    - Merge branch 'main' into SidneyDouw-pathspec ([`a22b1d8`](https://github.com/Byron/gitoxide/commit/a22b1d88a21311d44509018729c3ef1936cf052a))
    - Merge branch 'main' into git_includeif ([`598c853`](https://github.com/Byron/gitoxide/commit/598c853087fcf8f77299aa5b9803bcec705c0cd0))
</details>

## 0.17.0 (2022-05-18)

### New Features (BREAKING)

 - <csr-id-32dc1829a5661f66396d109c8d0a8eaae6b1f532/> use `git-credentials` in `git-protocol`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 33 calendar days.
 - 45 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#301](https://github.com/Byron/gitoxide/issues/301), [#386](https://github.com/Byron/gitoxide/issues/386)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#301](https://github.com/Byron/gitoxide/issues/301)**
    - update changelogs prior to release ([`84cb256`](https://github.com/Byron/gitoxide/commit/84cb25614a5fcddff297c1713eba4efbb6ff1596))
 * **[#386](https://github.com/Byron/gitoxide/issues/386)**
    - adapt to changes in git-sec ([`c5e2346`](https://github.com/Byron/gitoxide/commit/c5e2346cee53019b1b321e45cf080b210e60bb7a))
    - use `git-credentials` in `git-protocol` ([`32dc182`](https://github.com/Byron/gitoxide/commit/32dc1829a5661f66396d109c8d0a8eaae6b1f532))
 * **Uncategorized**
    - Release git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0 ([`349c590`](https://github.com/Byron/gitoxide/commit/349c5904b0dac350838a896759d51576b66880a7))
    - Release git-hash v0.9.4, git-features v0.21.0, git-actor v0.10.0, git-glob v0.3.0, git-path v0.1.1, git-attributes v0.1.0, git-sec v0.1.0, git-config v0.3.0, git-credentials v0.1.0, git-validate v0.5.4, git-object v0.19.0, git-diff v0.16.0, git-lock v2.1.0, git-ref v0.13.0, git-discover v0.1.0, git-index v0.3.0, git-mailmap v0.2.0, git-traverse v0.15.0, git-pack v0.19.0, git-odb v0.29.0, git-packetline v0.12.5, git-url v0.5.0, git-transport v0.17.0, git-protocol v0.16.0, git-revision v0.2.0, git-worktree v0.2.0, git-repository v0.17.0, safety bump 20 crates ([`654cf39`](https://github.com/Byron/gitoxide/commit/654cf39c92d5aa4c8d542a6cadf13d4acef6a78e))
    - Merge branch 'main' into git_includeif ([`05eb340`](https://github.com/Byron/gitoxide/commit/05eb34023933918c51c03cf2afd774db89cc5a33))
    - Merge branch 'main' into msrv-for-windows ([`7cb1972`](https://github.com/Byron/gitoxide/commit/7cb19729133325bdfacedf44cdc0500cbcf36684))
    - make fmt ([`251b6df`](https://github.com/Byron/gitoxide/commit/251b6df5dbdda24b7bdc452085f808f3acef69d8))
    - Merge branch 'main' into repo-status ([`9679d6b`](https://github.com/Byron/gitoxide/commit/9679d6b0e68c28438e22cb65c554d0b31dfaf159))
    - Merge branch 'git-sec' ([`cd723b5`](https://github.com/Byron/gitoxide/commit/cd723b5ae11148e7e9fd07daf28bc04455d5c46f))
</details>

## 0.16.0 (2022-04-03)

### New Features

 - <csr-id-39778fd76191cfdb60df87eab8da59e575e48c78/> in-manifest and in-lib documentation of feature toggles

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 55 calendar days.
 - 69 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#329](https://github.com/Byron/gitoxide/issues/329)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#329](https://github.com/Byron/gitoxide/issues/329)**
    - in-manifest and in-lib documentation of feature toggles ([`39778fd`](https://github.com/Byron/gitoxide/commit/39778fd76191cfdb60df87eab8da59e575e48c78))
    - Document all features related to serde1 ([`72b97f2`](https://github.com/Byron/gitoxide/commit/72b97f2ae4dc7642b160f183c6d5df4502dc186f))
 * **Uncategorized**
    - Release git-diff v0.14.0, git-bitmap v0.1.0, git-index v0.2.0, git-tempfile v2.0.1, git-lock v2.0.0, git-mailmap v0.1.0, git-traverse v0.13.0, git-pack v0.17.0, git-quote v0.2.0, git-odb v0.27.0, git-packetline v0.12.4, git-url v0.4.0, git-transport v0.16.0, git-protocol v0.15.0, git-ref v0.12.0, git-worktree v0.1.0, git-repository v0.15.0, cargo-smart-release v0.9.0, safety bump 5 crates ([`e58dc30`](https://github.com/Byron/gitoxide/commit/e58dc3084cf17a9f618ae3a6554a7323e44428bf))
    - Release git-hash v0.9.3, git-features v0.20.0, git-config v0.2.0, safety bump 12 crates ([`f0cbb24`](https://github.com/Byron/gitoxide/commit/f0cbb24b2e3d8f028be0e773f9da530da2656257))
    - upgrade document-features ([`c35e62e`](https://github.com/Byron/gitoxide/commit/c35e62e0da9ac1f7dcb863f5f9c69108c728d32e))
</details>

## 0.15.0 (2022-01-23)

A maintenance release with no relevant changes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 41 calendar days.
 - 55 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#279](https://github.com/Byron/gitoxide/issues/279)

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#279](https://github.com/Byron/gitoxide/issues/279)**
    - Adjust to changes in git-hash and git-pack ([`0cae25b`](https://github.com/Byron/gitoxide/commit/0cae25b1bb3c902ec323f17a1d9743e42fe213d0))
 * **Uncategorized**
    - Release git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`42ebb53`](https://github.com/Byron/gitoxide/commit/42ebb536cd6086f096b8422291776c9720fa0948))
    - Release git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`1b76119`](https://github.com/Byron/gitoxide/commit/1b76119259b8168aeb99cbbec233f7ddaa2d7d2c))
    - Release git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`8f57c29`](https://github.com/Byron/gitoxide/commit/8f57c297d7d6ed68cf51415ea7ede4bf9263326e))
    - Release git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0 ([`d78aab7`](https://github.com/Byron/gitoxide/commit/d78aab7b9c4b431d437ac70a0ef96263acb64e46))
    - Release git-hash v0.9.1, git-features v0.19.1, git-actor v0.8.0, git-config v0.1.10, git-object v0.17.0, git-diff v0.13.0, git-tempfile v1.0.4, git-chunk v0.3.0, git-traverse v0.12.0, git-pack v0.16.0, git-odb v0.26.0, git-packetline v0.12.3, git-url v0.3.5, git-transport v0.15.0, git-protocol v0.14.0, git-ref v0.11.0, git-repository v0.14.0, cargo-smart-release v0.8.0, safety bump 4 crates ([`373cbc8`](https://github.com/Byron/gitoxide/commit/373cbc877f7ad60dac682e57c52a7b90f108ebe3))
    - prepare changelogs for release ([`674ec73`](https://github.com/Byron/gitoxide/commit/674ec73b0816baa2c63b4ef1b40b7a41849c5e95))
    - prepar changelogs for cargo-smart-release release ([`8900d69`](https://github.com/Byron/gitoxide/commit/8900d699226eb0995be70d66249827ce348261df))
    - Release git-bitmap v0.0.1, git-hash v0.9.0, git-features v0.19.0, git-index v0.1.0, safety bump 9 crates ([`4624725`](https://github.com/Byron/gitoxide/commit/4624725f54a34dd6b35d3632fb3516965922f60a))
    - Merge branch 'sync-db-draft' ([`7d2e20c`](https://github.com/Byron/gitoxide/commit/7d2e20c6fedc2c7e71a307d8d072412fa847a4aa))
    - thanks clippy ([`4ca9e07`](https://github.com/Byron/gitoxide/commit/4ca9e07c7ac062d48d64ad7b516274e32dbc51c6))
</details>

## 0.14.0 (2021-11-29)

A maintenance release, triggered by putting too many adjustments into a single commit.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 12 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0 ([`d3f9227`](https://github.com/Byron/gitoxide/commit/d3f922781a81e8fbb81aa47afdbe9afeb06d666b))
    - Release git-features v0.18.0, git-actor v0.7.0, git-config v0.1.9, git-object v0.16.0, git-diff v0.12.0, git-traverse v0.11.0, git-pack v0.15.0, git-odb v0.25.0, git-packetline v0.12.2, git-transport v0.14.0, git-protocol v0.13.0, git-ref v0.10.0, git-repository v0.13.0, cargo-smart-release v0.7.0, safety bump 12 crates ([`acd3737`](https://github.com/Byron/gitoxide/commit/acd37371dcd92ebac3d1f039224d02f2b4e9fa0b))
    - Adjust changelogs prior to release ([`ec38950`](https://github.com/Byron/gitoxide/commit/ec3895005d141abe79764eaff7c0f04153e38d73))
</details>

## 0.13.1 (2021-11-16)

A maintenance release triggered by changes to git-pack and changelog rewrites.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 25 calendar days.
 - 27 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#254](https://github.com/Byron/gitoxide/issues/254)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#254](https://github.com/Byron/gitoxide/issues/254)**
    - Adjust changelogs prior to git-pack release ([`6776a3f`](https://github.com/Byron/gitoxide/commit/6776a3ff9fa5a283da06c9ec5723d13023a0b267))
 * **Uncategorized**
    - Release git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0 ([`f606fa9`](https://github.com/Byron/gitoxide/commit/f606fa9a0ca338534252df8921cd5e9d3875bf94))
    - better changelog descriptions. ([`f69b2d6`](https://github.com/Byron/gitoxide/commit/f69b2d627099639bc144fd94fde678d84a10d6f7))
    - Adjusting changelogs prior to release of git-config v0.1.8, git-object v0.15.1, git-diff v0.11.1, git-traverse v0.10.1, git-pack v0.14.0, git-odb v0.24.0, git-packetline v0.12.1, git-transport v0.13.1, git-protocol v0.12.1, git-ref v0.9.1, git-repository v0.12.0, cargo-smart-release v0.6.0, safety bump 5 crates ([`39b40c8`](https://github.com/Byron/gitoxide/commit/39b40c8c3691029cc146b893fa0d8d25d56d0819))
    - Fix false positive clippy warning. ([`9b8363b`](https://github.com/Byron/gitoxide/commit/9b8363bbf559223a424c2544610b4520bbd37aeb))
</details>

## v0.13.0 (2021-10-19)

A maintenance release due to properly dealing with previously breaking changes in `git-hash`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#222](https://github.com/Byron/gitoxide/issues/222)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#222](https://github.com/Byron/gitoxide/issues/222)**
    - update changelogs prior to release ([`9a493d0`](https://github.com/Byron/gitoxide/commit/9a493d0651b0b6d71cf230dc510a658be7f8cb19))
 * **Uncategorized**
    - Release git-hash v0.8.0, git-features v0.17.0, git-actor v0.6.0, git-object v0.15.0, git-diff v0.11.0, git-traverse v0.10.0, git-pack v0.13.0, git-odb v0.23.0, git-packetline v0.12.0, git-transport v0.13.0, git-protocol v0.12.0, git-ref v0.9.0, git-repository v0.11.0, git-commitgraph v0.6.0, gitoxide-core v0.12.0, gitoxide v0.10.0, cargo-smart-release v0.5.0, safety bump 16 crates ([`0e02953`](https://github.com/Byron/gitoxide/commit/0e029537a7f6242d02ccf7e63d8d92f5246e6c5e))
</details>

## v0.12.0 (2021-10-15)

### Dependency Upgrade (BREAKING)

* `git-traverse` saw a breaking change moving to v0.9, which triggered this crate to signal a breaking change, too.

### Improvements

* `git-packetline` is now publicly re-exported as `packetline`.
  
  This helps to avoid additional dependencies which in turn may have breaking
  changes. Re-using `packetline` assures it's usable.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 23 commits contributed to the release over the course of 31 calendar days.
 - 47 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#198](https://github.com/Byron/gitoxide/issues/198), [#200](https://github.com/Byron/gitoxide/issues/200)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#198](https://github.com/Byron/gitoxide/issues/198)**
    - Adjust all changelogs to fulfil requirements for publishing ([`04b9ca0`](https://github.com/Byron/gitoxide/commit/04b9ca025a1667529b2221ab4280bd3c8dae01cf))
    - deduplicate conventional message ids ([`e695eda`](https://github.com/Byron/gitoxide/commit/e695eda8cd183f703d9a3e59b7c3c7fa496ea1d2))
    - regenerate all changelogs to get links ([`0c81769`](https://github.com/Byron/gitoxide/commit/0c817690bd444f52bed2936b2b451cafd87dde92))
    - Mention actual issues that where worked on ([`a517e39`](https://github.com/Byron/gitoxide/commit/a517e39a81145b331f6c7a6cc2fc22e25daf42e2))
    - respect release-wide ignore list to allow removing entire conventional headlines ([`145103d`](https://github.com/Byron/gitoxide/commit/145103d4aa715386da9d4953f7f85fadc49fff9a))
    - Rebuild all changelogs to assure properly ordered headlines ([`4a9a05f`](https://github.com/Byron/gitoxide/commit/4a9a05f95930bad5938d4ce9c517ebf0e0b990f1))
    - Sort all commits by time, descending… ([`f536bad`](https://github.com/Byron/gitoxide/commit/f536bad20ffbac4dc353dfeb1a917bb88becbb78))
    - greatly reduce changelog size now that the traversal fix is applied ([`a0bc98c`](https://github.com/Byron/gitoxide/commit/a0bc98c06c349de2fd6e0d4593606e68b98def72))
    - Fixup remaining changelogs… ([`2f75db2`](https://github.com/Byron/gitoxide/commit/2f75db294fcf20c325555822f65629611be52971))
    - Generate changelogs with details ([`e1861ca`](https://github.com/Byron/gitoxide/commit/e1861caa435d312953a9fea7ceff6d2e07b03443))
    - Update all changelogs with details ([`58ab2ae`](https://github.com/Byron/gitoxide/commit/58ab2aee23ba70a536e9487b44fb04c610374d1a))
    - Update changelogs ([`c857d61`](https://github.com/Byron/gitoxide/commit/c857d61ce3ce342012a2c4ba10a8327822aa530e))
    - Avoid adding newlines which make writing unstable ([`6b5c394`](https://github.com/Byron/gitoxide/commit/6b5c394f49282a8d09c2a9ffece840e4683572db))
    - Fix section headline level ([`9d6f263`](https://github.com/Byron/gitoxide/commit/9d6f263beef289d227dec1acc2d4240087cb9be6))
    - Write first version of changlogs thus far… ([`719b6bd`](https://github.com/Byron/gitoxide/commit/719b6bdf543b8269ccafad9ad6b46e0c55efaa38))
    - Parse more user generated section content, adapt existing changelogs to work correctly ([`2f43a54`](https://github.com/Byron/gitoxide/commit/2f43a54298e7ecfff2334627df149fe0882b5d1d))
 * **[#200](https://github.com/Byron/gitoxide/issues/200)**
    - feat: Lift io::Errors to response::Error::UploadPack(…)… ([`f293b63`](https://github.com/Byron/gitoxide/commit/f293b633d16c0f7393d0ede64e12f14e47d0296b))
 * **Uncategorized**
    - Release git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-config v0.1.7, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0 ([`59ffbd9`](https://github.com/Byron/gitoxide/commit/59ffbd9f15583c8248b7f48b3f55ec6faffe7cfe))
    - Adjusting changelogs prior to release of git-hash v0.7.0, git-features v0.16.5, git-actor v0.5.3, git-validate v0.5.3, git-object v0.14.1, git-diff v0.10.0, git-tempfile v1.0.3, git-lock v1.0.1, git-traverse v0.9.0, git-pack v0.12.0, git-odb v0.22.0, git-packetline v0.11.0, git-url v0.3.4, git-transport v0.12.0, git-protocol v0.11.0, git-ref v0.8.0, git-repository v0.10.0, cargo-smart-release v0.4.0, safety bump 3 crates ([`a474395`](https://github.com/Byron/gitoxide/commit/a47439590e36b1cb8b516b6053fd5cbfc42efed7))
    - Update changelogs just for fun ([`21541b3`](https://github.com/Byron/gitoxide/commit/21541b3301de1e053fc0e84373be60d2162fbaae))
    - Merge branch 'changelog-generation' ([`bf0106e`](https://github.com/Byron/gitoxide/commit/bf0106ea21734d4e59d190b424c22743c22da966))
    - Merge branch 'main' into changelog-generation ([`c956f33`](https://github.com/Byron/gitoxide/commit/c956f3351d766c748faf0460780e32ac8dfe8165))
    - Bump git-traverse v0.9.0, safety bump 8 crates ([`d39fabb`](https://github.com/Byron/gitoxide/commit/d39fabb8757369aa19452a457f610fe21dc13a14))
</details>

## v0.11.1 (2021-08-29)

- instruct docs.rs which features to use for more useful documentation

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
    - Release git-transport v0.11.1 ([`0952976`](https://github.com/Byron/gitoxide/commit/0952976eac1dac9b8f351ecc9867746b650377f9))
    - [various #184] configure docs.rs build features ([`cc50249`](https://github.com/Byron/gitoxide/commit/cc502492c512293e93e95610ca80a71896076ded))
</details>

## v0.11.0 (2021-08-27)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 10 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump git-transport v0.11.0 ([`1149f1b`](https://github.com/Byron/gitoxide/commit/1149f1b716624f8f4fdaed20c803530aebc45599))
    - [transport #174] prepare for release ([`f8bc517`](https://github.com/Byron/gitoxide/commit/f8bc51763e96d8d0a97d5f367c943441a98c8e95))
    - Bump git-packetline v0.10.0 ([`b09f391`](https://github.com/Byron/gitoxide/commit/b09f3912e0addd7b4b0ef22bc3a24869d5011646))
    - [packetline #178] refactor ([`23438fd`](https://github.com/Byron/gitoxide/commit/23438fd4a807376c1d4699732ea6c83c0bde574f))
    - [packetline #178] rename PacketLine to PacketLineRef… ([`d4c16a9`](https://github.com/Byron/gitoxide/commit/d4c16a93946244177606b58cc702b81a16424ad4))
</details>

## v0.10.1 (2021-08-17)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-transport v0.10.1 ([`dc74d19`](https://github.com/Byron/gitoxide/commit/dc74d1946b89fb42fa8644db31b1fe1a52a56f05))
    - Apply nightly rustfmt rules. ([`5e0edba`](https://github.com/Byron/gitoxide/commit/5e0edbadb39673d4de640f112fa306349fb11814))
</details>

## v0.10.0 (2021-08-13)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 1 calendar day.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release git-packetline v0.9.0 ([`7ffbd60`](https://github.com/Byron/gitoxide/commit/7ffbd602c08605026b0bb97ab85216907badaf09))
    - remove dev-dependency cycles by removing their version ([`c40faca`](https://github.com/Byron/gitoxide/commit/c40faca41632cd2a226daf4ddf5293b65d1fdc82))
    - Release git-diff v0.8.0, git-odb v0.20.0, git-pack v0.8.0, git-traverse v0.7.0 ([`f123f69`](https://github.com/Byron/gitoxide/commit/f123f69c7a4f9fd1c98bd2f60ebc953a6739fe04))
    - Release git-diff v0.7.0, git-odb v0.19.0, git-pack v0.7.0, git-traverse v0.6.0 ([`c67291f`](https://github.com/Byron/gitoxide/commit/c67291ff9bcdff9a747d87241f6a71015607af05))
    - bump transport version to 0.10 ([`f26a3d3`](https://github.com/Byron/gitoxide/commit/f26a3d3a2745f3eb69d76e0cfd718a90cf74f003))
    - (cargo-release) version 0.8.0 ([`ad6d7f9`](https://github.com/Byron/gitoxide/commit/ad6d7f9c2b4f8879d466e758fc9b51ece6879e96))
    - (cargo-release) version 0.6.0 ([`d704bca`](https://github.com/Byron/gitoxide/commit/d704bca7de0a6591f35345c842d6418b36ecd206))
    - (cargo-release) version 0.7.0 ([`2ef3106`](https://github.com/Byron/gitoxide/commit/2ef3106eb84981e2dabd84f81362b4e44f938ea6))
    - (cargo-release) version 0.5.0 ([`c2f94a5`](https://github.com/Byron/gitoxide/commit/c2f94a51bce287be301090450cb00cde57e92f76))
    - (cargo-release) version 0.4.0 ([`d69d0ac`](https://github.com/Byron/gitoxide/commit/d69d0ac21989243fdafa514fa41579fd51bc2558))
    - [transport] A much better name for 'is_stateful()` ([`f15f1e8`](https://github.com/Byron/gitoxide/commit/f15f1e85fda76eef72c3754d625cf51e3c454eea))
</details>

## v0.9.0 (2021-08-10)

<csr-id-2f3725efcaa439db4e10ade1b9fbeb1258fd93c1/>

### Other

 - <csr-id-2f3725efcaa439db4e10ade1b9fbeb1258fd93c1/> make capabilities parsing public

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 115 commits contributed to the release over the course of 90 calendar days.
 - 93 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`0e9c73a`](https://github.com/Byron/gitoxide/commit/0e9c73abd17e0dd21952275077ae53ad7e7aa1af))
    - (cargo-release) version 0.16.0 ([`1231dbd`](https://github.com/Byron/gitoxide/commit/1231dbd16dacefb39adec8e067c312d313a82e3c))
    - clippy on tests and thanks clippy ([`a77a71c`](https://github.com/Byron/gitoxide/commit/a77a71cf02d328a2a964388928d6b2a235a0aa85))
    - thanks clippy ([`e1964e4`](https://github.com/Byron/gitoxide/commit/e1964e43979b3e32a5d4bfbe377a842d2c0b10ea))
    - Bump async-trait from 0.1.50 to 0.1.51 ([`ce0b81e`](https://github.com/Byron/gitoxide/commit/ce0b81e8f5c652d389ff876844bc42bcfa687921))
    - [transport] more convenient check for available capabilities ([`e9ed952`](https://github.com/Byron/gitoxide/commit/e9ed952d35fa9ffa142f941d75c385abec3997ef))
    - Bump futures-io from 0.3.15 to 0.3.16 ([`3c23820`](https://github.com/Byron/gitoxide/commit/3c23820d3f0d3567f44215cdb0ad13ab675a201f))
    - Remove unnecessary pub(crate) exports ([`3d2456e`](https://github.com/Byron/gitoxide/commit/3d2456e11709f0461b37c6df55ecc3861ca4cab5))
    - Bump thiserror from 1.0.25 to 1.0.26 ([`9682590`](https://github.com/Byron/gitoxide/commit/9682590095dc3a502b0c84ccd206ca4797635092))
    - [transport] remove Transport::close()… ([`4268a9b`](https://github.com/Byron/gitoxide/commit/4268a9bcf733413f7326be7af487a8fcdec1f71c))
    - [transport] implement Transport for &mut T: Transport as well ([`372fb81`](https://github.com/Byron/gitoxide/commit/372fb8183aff19bd0f2d17ea74409b2ca3a08511))
    - [transport] tests for extra parameters ([`fffd926`](https://github.com/Byron/gitoxide/commit/fffd926a3d5c6abfa732aa2305a4a05fdd06254d))
    - [protocol] extra_parameters are forwarded from delegate to handshake ([`03e3db3`](https://github.com/Byron/gitoxide/commit/03e3db3809bd031d7d0c151ada2542214d7e32c0))
    - [transport] allow setting a custom URL in git::Connection ([`f7437e0`](https://github.com/Byron/gitoxide/commit/f7437e041b2f3a8d51012972bd443d3c4b0a9252))
    - [transport] async transports support extra params ([`a0d6756`](https://github.com/Byron/gitoxide/commit/a0d67569b5c947d8158177e308f0919df2f182a3))
    - [transport] extra_headers for http ([`6026dcc`](https://github.com/Byron/gitoxide/commit/6026dcc07674ee9ea79503aab07491dd395e51a4))
    - [transport] extra-parameters for the http protocol ([`d30bcf1`](https://github.com/Byron/gitoxide/commit/d30bcf18afda19d89b8cb020e193405bfd2d3787))
    - [transport] git::Connection handles extra-parameters ([`961b6a4`](https://github.com/Byron/gitoxide/commit/961b6a40aaa003497abbd17fd53485c6cb2b2857))
    - [transport]  File implementation doesn't need to inherit git::Connection's… ([`951b1e2`](https://github.com/Byron/gitoxide/commit/951b1e26a1dca054aa5af6a565fde3c733b43ffd))
    - [transport] unsupported protocol versions now abort the fetch operation ([`812aa3b`](https://github.com/Byron/gitoxide/commit/812aa3bc02a823cb9277847db905e76a50ee7413))
    - [transport] flexible version of version support check doesn't actually work :D ([`2b220f0`](https://github.com/Byron/gitoxide/commit/2b220f0758cb7a96a66b256552f13a020cdee3fc))
    - [transport] improve docs for `is_stateful()` ([`22f7e67`](https://github.com/Byron/gitoxide/commit/22f7e6719d2e3931f8cd4bb4e94c23c1f9f84189))
    - Merge branch 'pubcap' ([`292f8ff`](https://github.com/Byron/gitoxide/commit/292f8ff2851dff846eda1b80943de718f08e65be))
    - Add missing docs ([`a6cbbde`](https://github.com/Byron/gitoxide/commit/a6cbbdeecbfe459556579a2d991bd546452c04c3))
    - make capabilities parsing public ([`2f3725e`](https://github.com/Byron/gitoxide/commit/2f3725efcaa439db4e10ade1b9fbeb1258fd93c1))
    - thanks clippy ([`6200ed9`](https://github.com/Byron/gitoxide/commit/6200ed9ac5609c74de4254ab663c19cfe3591402))
    - [async-client] unblock the async delegate in the cheapest possible way… ([`a3b5d75`](https://github.com/Byron/gitoxide/commit/a3b5d75d387dc5d6c44f695f63df8803613637a2))
    - Revert "[async-client] Try to bring 'Send' back but…" ([`52eb953`](https://github.com/Byron/gitoxide/commit/52eb953fcc44cce19604b1df6a600237b8c81392))
    - [async-client] Try to bring 'Send' back but… ([`3a06adb`](https://github.com/Byron/gitoxide/commit/3a06adb41f6b2946f78044e4ab1385e6441fc40f))
    - refactor ([`2a406d6`](https://github.com/Byron/gitoxide/commit/2a406d62c02db24c39980f9ae636b87e2d707faf))
    - [async-client] frame for async connect ([`9ada080`](https://github.com/Byron/gitoxide/commit/9ada0805fc5896f8ef1a31dc821b789b7f0438a6))
    - Prevent selecting mutually exclusive features ([`7f5da18`](https://github.com/Byron/gitoxide/commit/7f5da18c39b84af788ea1366ccca2c8b9d09f755))
    - [git-transport] Fix http build ([`3469e99`](https://github.com/Byron/gitoxide/commit/3469e99afeaf354db4020d5a363b05a031894096))
    - [git-protocol] fetch in sync and async… ([`4776039`](https://github.com/Byron/gitoxide/commit/47760399bffd030c848e0ef6df52a4765d8fb566))
    - Bump maybe-async from 0.2.4 to 0.2.6 ([`d99a1a8`](https://github.com/Byron/gitoxide/commit/d99a1a815809d22c7384c6ecb1275e39fb911d91))
    - fix docs ([`bca7594`](https://github.com/Byron/gitoxide/commit/bca7594713d623e0f0a4b82b658c26ee9a041eaa))
    - [git-protocol] fix build ([`4cce648`](https://github.com/Byron/gitoxide/commit/4cce6487d6d514541afee1a9aa92043f186136d3))
    - [git-transport] refactor ([`d09153f`](https://github.com/Byron/gitoxide/commit/d09153f34135609224929f77175f3e0ac04ea12e))
    - [git-transport] Properly implement Transport for Boxed types ([`47b10c9`](https://github.com/Byron/gitoxide/commit/47b10c9b4ccba5a0928819c92eda472bbfda0c50))
    - [git-transport] refactor ([`3b0baee`](https://github.com/Byron/gitoxide/commit/3b0baee6b856b510ea839a1e294e9a99aafaa3ac))
    - refactor ([`2eefe17`](https://github.com/Byron/gitoxide/commit/2eefe1712131a69298be02e94df8b6ba844afcd9))
    - refactor ([`14c9093`](https://github.com/Byron/gitoxide/commit/14c909341d243ca3dcc42d343aeee65d28045b65))
    - [git-protocol] async capabilities and arguments abstractions ([`aa3eacb`](https://github.com/Byron/gitoxide/commit/aa3eacbd53665d6b76bd9706d801d1189a970261))
    - [git-transport] see how things can be moved to a different thread… ([`c271d32`](https://github.com/Byron/gitoxide/commit/c271d323086486a9d1dbe004b33fdb7d9eec45ed))
    - [git-transport] partial transfer to thread doesn't work in test… ([`4a6dfd4`](https://github.com/Byron/gitoxide/commit/4a6dfd40e465226d0f9c7eb02cfb721b55bbff41))
    - [git-transport] allow fetch processing to be offloading to another thread ([`a1302e0`](https://github.com/Byron/gitoxide/commit/a1302e0ff549e96362c441d8eecec56f1ef4ca43))
    - Revert "[git-transport] async-executor (Local) hangs…" ([`ec8bcd0`](https://github.com/Byron/gitoxide/commit/ec8bcd0f36b46ff319ad6d6da9ffcb80dd5b0429))
    - [git-transport] async-executor (Local) hangs… ([`68ac51b`](https://github.com/Byron/gitoxide/commit/68ac51b384cae11f5dca103160bf3d519305364e))
    - Revert "[git-transport] attempt to mix 'blocking' but realize that now things need to be static." ([`e367753`](https://github.com/Byron/gitoxide/commit/e3677537d7858a113008849ac8ace136f5d5c4d2))
    - [git-transport] attempt to mix 'blocking' but realize that now things need to be static. ([`3d296fa`](https://github.com/Byron/gitoxide/commit/3d296fae08ce0d4c2625008ab1fdcd7ede8dac54))
    - [git-transport] V2 transport tests work on async ([`e04a1c9`](https://github.com/Byron/gitoxide/commit/e04a1c98a1ee0ba58fa326c9a68bd36230e229da))
    - [git-transport] first V2 test ([`f9da975`](https://github.com/Byron/gitoxide/commit/f9da975ca777d2345e8c2842771b16a17af79cd3))
    - [git-transport] adapt extension trait in blocking code to match async version ([`95eee30`](https://github.com/Byron/gitoxide/commit/95eee30191aed9421f69dcd6e1587c0b5a1f2dd2))
    - [git-transport] extension trait working ([`28fbd28`](https://github.com/Byron/gitoxide/commit/28fbd284d108d5db3f13c8cede5772e065e5f8fb))
    - [git-transport] a first step towards getting the extension trait to compile ([`b692979`](https://github.com/Byron/gitoxide/commit/b6929792c3d2ef9a73eb049eb88b06c8c763d899))
    - [git-transport] no warnings when building without any choice of client ([`3dc568a`](https://github.com/Byron/gitoxide/commit/3dc568a3a29dfea6ff311cf965ecce7f7eddbf63))
    - [git-transport] upgrade to futures-lite 1.12 with BufRead support ([`ee01c79`](https://github.com/Byron/gitoxide/commit/ee01c79887a892e001787bbefa93f75d9c4f1cfc))
    - [git-transport] Show how to use blocking git-pack code in non-blocking transports ([`de2ba3c`](https://github.com/Byron/gitoxide/commit/de2ba3c4919d454894911c54fd4bb0e0a4665723))
    - [git-transport] handshakeV1 tests run in async! ([`d1c0e35`](https://github.com/Byron/gitoxide/commit/d1c0e35817d183982a5b1eb7e545bfe83edb141e))
    - [git-transport] And a chance to have V1 working in async ([`2bf93fc`](https://github.com/Byron/gitoxide/commit/2bf93fc72b3f9dcb63f8b24c77c95d518072431f))
    - [git-transport] refactor ([`64bb8b3`](https://github.com/Byron/gitoxide/commit/64bb8b3937fcf7f14034ccfb6a72a24bf05f0320))
    - [git-transport] improve error handling considerably… ([`7b7d314`](https://github.com/Byron/gitoxide/commit/7b7d314851b8db230228c28fb38a5a6541ec865c))
    - [git-transport] Add remaninig git connection method… ([`73fcf38`](https://github.com/Byron/gitoxide/commit/73fcf38f9c334813572a6aeb7691758a524cac07))
    - [git-transport] refactor ([`db83600`](https://github.com/Byron/gitoxide/commit/db83600179b3f27770b989f5f8ae1dd459749354))
    - [git-transport] the first part of async transport for git connections ([`d94fbf8`](https://github.com/Byron/gitoxide/commit/d94fbf83d3e57c54dead3fb849e63b1d37343cb2))
    - [git-transport] Split git connection into shared and blocking parts ([`0bfe693`](https://github.com/Byron/gitoxide/commit/0bfe69385932698b99871717d823fe645e4eabb8))
    - [git-transport] refactor ([`2342e8a`](https://github.com/Byron/gitoxide/commit/2342e8a6a4ceca12603cb5e2c350edc2d4e71580))
    - [git-transport] refactor ([`957403e`](https://github.com/Byron/gitoxide/commit/957403e11e0f7b3c97aa1996b2e936bbdb7ee12c))
    - [git-transport] refactor ([`e580354`](https://github.com/Byron/gitoxide/commit/e58035452be10328c3e5e1991bafbab3f71d3353))
    - [git-transport] re-enable `request()` method of main trait… ([`3adbade`](https://github.com/Byron/gitoxide/commit/3adbade31afa163a499fc2946f5af5ef3f367387))
    - [git-transport] RequestWriter complete ([`a05fff3`](https://github.com/Byron/gitoxide/commit/a05fff3b2d987a5750e11945306bfa3731ed5ca3))
    - [git-transport] refactor ([`03a3aed`](https://github.com/Byron/gitoxide/commit/03a3aedf17a91465279800d8028cc7435326534a))
    - [git-transport] ARGH: PIN!!! ([`71379ac`](https://github.com/Byron/gitoxide/commit/71379ac25c44bc744ed0e93d2b126d4959bc4469))
    - [git-transport] naive attempt to make Request async… ([`b819546`](https://github.com/Byron/gitoxide/commit/b819546b0096a4abfbe5ada25a1ac661a084cfc9))
    - [git-transport] ExtendedBufRead for Async… ([`d4e56c8`](https://github.com/Byron/gitoxide/commit/d4e56c8efd586b571445e0085ce518c5efb8f5e6))
    - [git-transport] First stab at ExtendedBufRead, but… ([`13f73d2`](https://github.com/Byron/gitoxide/commit/13f73d2f9b65d5ea829185af669532c3797cf90b))
    - [git-transport] put request writer into general spot… ([`af07ebf`](https://github.com/Byron/gitoxide/commit/af07ebf44fbb3386cd5176441fd707cc820b71d0))
    - [git-transport] refactor ([`5f98ac1`](https://github.com/Byron/gitoxide/commit/5f98ac140f9f3260d3d5a784d1aa1e1ac8c37114))
    - [git-transport] fix docs ([`fbfc827`](https://github.com/Byron/gitoxide/commit/fbfc8271431f7c19adbed5e095d7c2ee10dda5e5))
    - [git-transport] refactor ([`011ece0`](https://github.com/Byron/gitoxide/commit/011ece04827d75aa6d93e9fcae449aaba4167f80))
    - [git-transport] the first async trait ([`2abac2a`](https://github.com/Byron/gitoxide/commit/2abac2a2df8033c6d2578e5afb88bb34aab86988))
    - [git-transport] refactor ([`73df129`](https://github.com/Byron/gitoxide/commit/73df12987a9255efc4724e1761f335a072d3bcaf))
    - [git-transport] the first async-only type ([`88109a5`](https://github.com/Byron/gitoxide/commit/88109a54ad594df6d18cf6b66a9c89a76fc0cdf5))
    - [git-transport] all non-IO types are now shared ([`209c780`](https://github.com/Byron/gitoxide/commit/209c780efff32d63ce7edc8b1f92fac0cd1a396d))
    - [git-transport] feature toggle for async-client; prepare for test ([`95e6801`](https://github.com/Byron/gitoxide/commit/95e6801b121a0744908552090b855cb3dbe99e64))
    - [git-transport] refactor ([`592d9ac`](https://github.com/Byron/gitoxide/commit/592d9ac03b0d26848435a43480c526a4a0e0efb8))
    - [git-transport] remove maybe_async from dependencies, add async-client feature ([`e57aad3`](https://github.com/Byron/gitoxide/commit/e57aad3a19ec89fd0aa4d8670430434f0dc4c826))
    - (cargo-release) version 0.15.0 ([`d69d9fb`](https://github.com/Byron/gitoxide/commit/d69d9fb0931f8257cef96ef14a89da9340ad9738))
    - [git-packetline] Use io::(Result|Error) everywhere ([`374f129`](https://github.com/Byron/gitoxide/commit/374f129e0d1473db9a2107c408f655da032efe89))
    - [git-packetline] refactor ([`f038ca1`](https://github.com/Byron/gitoxide/commit/f038ca1e1c6d99bfcedb0387abc4151b188750c6))
    - [git-packetline] document feature toggle ([`8b8a1aa`](https://github.com/Byron/gitoxide/commit/8b8a1aafb04b2e305cf2674c15530b430dad4969))
    - [git-packetline] refactor ([`1328c5b`](https://github.com/Byron/gitoxide/commit/1328c5b4001f380936beff73e1f822f14e41e98b))
    - (cargo-release) version 0.6.0 ([`ec5a54e`](https://github.com/Byron/gitoxide/commit/ec5a54e9f3543afddc9f972f16135edc6ef6ff5b))
    - [git-packetline] refactor ([`e5769d1`](https://github.com/Byron/gitoxide/commit/e5769d1e7668ae54c667d2593c0c22e7723710c0))
    - [git-packetline] refactor ([`fef3c9f`](https://github.com/Byron/gitoxide/commit/fef3c9f0aed3f6a509a71e8ff20050c6ea660f56))
    - (cargo-release) version 0.9.0 ([`18f6d01`](https://github.com/Byron/gitoxide/commit/18f6d011043203523f1d0dacf657704ed3f9cf89))
    - [git-transport] simplify parsing capabilities from lines ([`401af09`](https://github.com/Byron/gitoxide/commit/401af0974742f10c8b9b3c9752e9d30205e96c16))
    - refactor ([`8ce28e7`](https://github.com/Byron/gitoxide/commit/8ce28e74545ded2909417df9091da866fb343710))
    - [git-transport] test capabilities in blocking and async mode ([`66eb2a5`](https://github.com/Byron/gitoxide/commit/66eb2a5c803d3365c8d9522f24843a8b73dff76d))
    - refactor ([`558b208`](https://github.com/Byron/gitoxide/commit/558b208f5055ab0562d3704e4fb62693eaab94fe))
    - [git-transport] first round of getting capabilities into 'dual' mode… ([`3af353b`](https://github.com/Byron/gitoxide/commit/3af353b8b3ed0ee608cbc96d1cd45a3165907a12))
    - [git-transport] remove default features to force being explicit everywhere ([`d1b39f8`](https://github.com/Byron/gitoxide/commit/d1b39f8093c032a172237a584c9208479611a866))
    - [git-transport] A first async test, right now there is nothing to test though ([`9741ae1`](https://github.com/Byron/gitoxide/commit/9741ae1ee0fcf65c144d87cd17d8fe547b288b12))
    - Tests follow crate structure closely (again) ([`8d6e46a`](https://github.com/Byron/gitoxide/commit/8d6e46a84c41d7f04f2dbbb1a4602159b1a96c8b))
    - Make the blocking client the default… ([`9d62ca3`](https://github.com/Byron/gitoxide/commit/9d62ca338927139708246ce0934f1cb317f14784))
    - Revert "Remove maybe-async for now" ([`ebd5701`](https://github.com/Byron/gitoxide/commit/ebd57017f80d0c12f6fe9a8d236843323c638311))
    - refactor ([`84d1509`](https://github.com/Byron/gitoxide/commit/84d150952d9cd72a05d83419d4fc013c75d7b2dc))
    - refactor ([`1412282`](https://github.com/Byron/gitoxide/commit/141228219d33e8056489514f91221d803888edd8))
    - refactor ([`f16d057`](https://github.com/Byron/gitoxide/commit/f16d057054ad6fabc664bbcb00f75e5974f05db9))
    - refactor ([`976da51`](https://github.com/Byron/gitoxide/commit/976da51efc5720300dfd3093e377284d1c4ccf3c))
    - refactor ([`7ac6a05`](https://github.com/Byron/gitoxide/commit/7ac6a05442b0d7e97c886f8f57b8695a14761027))
    - refactor ([`cd02749`](https://github.com/Byron/gitoxide/commit/cd027498944abde8339c421b5527abb9c3495de3))
    - Remove maybe-async for now ([`97e96f4`](https://github.com/Byron/gitoxide/commit/97e96f49ce9e4ac325faebe112cec9c11cdc715c))
    - refactor ([`6e6f4ac`](https://github.com/Byron/gitoxide/commit/6e6f4acf4b3c704989928347f10f1725e6a866bd))
    - refactor git-transport test in preparation for async testing ([`42d5bf7`](https://github.com/Byron/gitoxide/commit/42d5bf7f5e8f4d88d3f849febec0c6d2678e0d06))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 2 times to make code idiomatic. 

## v0.8.0 (2021-05-09)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 21 calendar days.
 - 30 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.8.0 ([`411a05e`](https://github.com/Byron/gitoxide/commit/411a05ead1546c76fe51f359fbcb961a1140535e))
    - (cargo-release) version 0.5.0 ([`8c4cc3f`](https://github.com/Byron/gitoxide/commit/8c4cc3fb5922d1a761463bbbad65e59f91cce4cb))
    - [async-transport] Cargo.toml and traits to be more 'realistic' ([`9a617a5`](https://github.com/Byron/gitoxide/commit/9a617a5e68f032dbaba9a902b558666992683701))
    - [async-transport] The very first step ([`b9e5559`](https://github.com/Byron/gitoxide/commit/b9e5559afa776dc1a7eecc90cc219da5ff911d79))
    - (cargo-release) version 0.14.0 ([`a760f8c`](https://github.com/Byron/gitoxide/commit/a760f8c013e13ba82daa1acf1a4a57e0818a008d))
    - (cargo-release) version 0.13.0 ([`ac2eddb`](https://github.com/Byron/gitoxide/commit/ac2eddb06eb3d8a9a3dcdcd796eb54a7e45ab935))
</details>

## v0.7.0 (2021-04-08)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 13 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.7.0 ([`334b7e1`](https://github.com/Byron/gitoxide/commit/334b7e1b838b5201f2484be42dee3c4d2fd789d7))
    - (cargo-release) version 0.12.0 ([`3b71e7e`](https://github.com/Byron/gitoxide/commit/3b71e7e8416e550b47e5aed2259c1181497ac9e8))
</details>

## v0.6.0 (2021-03-26)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 70 calendar days.
 - 79 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.6.0 ([`50fb6f2`](https://github.com/Byron/gitoxide/commit/50fb6f25e9afa900ac1c3cfb88d7ca0d5a9a95f7))
    - (cargo-release) version 0.3.0 ([`d5c6643`](https://github.com/Byron/gitoxide/commit/d5c6643a41d295eaf7aabb84eab435e42a11dd42))
    - thanks clippy ([`749ceba`](https://github.com/Byron/gitoxide/commit/749ceba246fb8a4cb8d48fa86184619fef500108))
    - Merge pull request #50 from Byron/edward-shen/odb-zlib-ng ([`acb90d7`](https://github.com/Byron/gitoxide/commit/acb90d755fb02c37f8a5a431778abcbe143fb5e5))
    - [gix] Add optional zlib feature ([`f1f9665`](https://github.com/Byron/gitoxide/commit/f1f96658a6cd6165ba9c9d7acb809fcaf2c46f9c))
    - (cargo-release) version 0.11.0 ([`1aa1f5e`](https://github.com/Byron/gitoxide/commit/1aa1f5e84a07427d5d7f3231735fe9c1923f506f))
    - (cargo-release) version 0.2.0 ([`0c39373`](https://github.com/Byron/gitoxide/commit/0c39373de5aba0acc4aaa330bf51b6abd4f50474))
    - support for radicle urls ([`2c5b955`](https://github.com/Byron/gitoxide/commit/2c5b955b07073c5ef0e7bbe3ab20f0047770440b))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.5.1 (2021-01-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 7 calendar days.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.1 ([`0cf1d06`](https://github.com/Byron/gitoxide/commit/0cf1d06f5e289b541a842af03b59229fec833ca7))
    - silence so far unknown clippy lints ([`b5f2a4b`](https://github.com/Byron/gitoxide/commit/b5f2a4b079665daa8b9e0228acc59d1eddd603b2))
    - thanks clippy ([`343ab9a`](https://github.com/Byron/gitoxide/commit/343ab9adb62da1dde495fc209c179137bbe59a10))
    - complete git-transport docs ([`fa2dc9d`](https://github.com/Byron/gitoxide/commit/fa2dc9d65100f0f3f97358746a37dc722bae12c3))
    - documentation for capabilities in git-transport ([`5ec79fa`](https://github.com/Byron/gitoxide/commit/5ec79faaa2568cee9333b4bb0c96e8f0ee5a2433))
    - more docs for git-transport ([`3a867e9`](https://github.com/Byron/gitoxide/commit/3a867e945edad05dd65b75111628b99fa955c03f))
    - more git-transport docs ([`6cd69b9`](https://github.com/Byron/gitoxide/commit/6cd69b9a6b79ba8f9297cf2bb6e36dd8f63845a2))
</details>

## v0.5.0 (2020-12-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.5.0 ([`28df5e9`](https://github.com/Byron/gitoxide/commit/28df5e9131aec3efb2b68db204662b92b232b33c))
    - use git-hash in git-features ([`5b307e0`](https://github.com/Byron/gitoxide/commit/5b307e076f6f5975592c8b177c122c91c1d809c6))
</details>

## v0.4.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.4.0 ([`32aefc0`](https://github.com/Byron/gitoxide/commit/32aefc051c7ad9d1a160f77db070df7fa4843dbc))
    - (cargo-release) version 0.4.0 ([`72eaece`](https://github.com/Byron/gitoxide/commit/72eaeceed135e4cc5c943685f4c902d03597c4d2))
    - (cargo-release) version 0.9.0 ([`a89fdb9`](https://github.com/Byron/gitoxide/commit/a89fdb98f64bb0ca070fa79a1f58f1232bb14090))
</details>

## v0.3.0 (2020-12-15)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 90 calendar days.
 - 92 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.3.0 ([`d19ee35`](https://github.com/Byron/gitoxide/commit/d19ee35cc6683c63e0eabd717e4758075faeaa71))
    - (cargo-release) version 0.3.0 ([`eade7d1`](https://github.com/Byron/gitoxide/commit/eade7d101e071153055b07d9c6ae3c1452493a21))
    - thanks clippy ([`ba9b3c2`](https://github.com/Byron/gitoxide/commit/ba9b3c2345887353e02fc081be80733f1c5e22d9))
    - uograde everything else ([`0cd79d0`](https://github.com/Byron/gitoxide/commit/0cd79d00bce3f042b5cc849cf48739e29f95fcb0))
    - (cargo-release) version 0.8.0 ([`47c00c2`](https://github.com/Byron/gitoxide/commit/47c00c2228cf25c79e1fa3eb4229c7ab24de91e5))
    - refactor ([`b3a8bb5`](https://github.com/Byron/gitoxide/commit/b3a8bb5f7f0c6e80259922546928c2739c24f7b5))
    - refactor ([`f9e8d29`](https://github.com/Byron/gitoxide/commit/f9e8d2932c02c22bf57acd39fb0a9e6d521070bd))
    - cargo clippy Rust 1.48 ([`475a68c`](https://github.com/Byron/gitoxide/commit/475a68ce33b895de911939c51afa159df534f7b8))
    - (cargo-release) version 0.7.0 ([`7fa7bae`](https://github.com/Byron/gitoxide/commit/7fa7baeb3e7d008a25e4d714eff908e2516c828b))
    - remove dash in all repository links ([`98c1360`](https://github.com/Byron/gitoxide/commit/98c1360ba4d2fb3443602b7da8775906224feb1d))
    - Merge from main. ([`b59bd5e`](https://github.com/Byron/gitoxide/commit/b59bd5e0b0895c7d1d585816cec8be4dea78c278))
    - refactor ([`ba1d883`](https://github.com/Byron/gitoxide/commit/ba1d88364424eb60a0874a5726b62740dc348592))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

## v0.2.1 (2020-09-14)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.1 ([`ebf3419`](https://github.com/Byron/gitoxide/commit/ebf341936b22e899de88293ef377b438353d1821))
    - (cargo-release) version 0.6.0 ([`9ef184e`](https://github.com/Byron/gitoxide/commit/9ef184e35712f938fb4f9f6da7390a8777a9284e))
</details>

## v0.2.0 (2020-09-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 189 commits contributed to the release over the course of 28 calendar days.
 - 62 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 0.2.0 ([`779e9d0`](https://github.com/Byron/gitoxide/commit/779e9d0ad67c20fa9cec14359e87774ca2d74ee4))
    - (cargo-release) version 0.2.0 ([`da830de`](https://github.com/Byron/gitoxide/commit/da830defc9cfa81ce159f6d908da828227760845))
    - (cargo-release) version 0.5.0 ([`82b7313`](https://github.com/Byron/gitoxide/commit/82b73131b79ec3c42a712dad1c0766a72209d737))
    - thanks clippy ([`e5d80b1`](https://github.com/Byron/gitoxide/commit/e5d80b19b83dc03d49606b7ccba20ff0c39bc5d9))
    - [clone] make cloning the linux kernel work ([`e780526`](https://github.com/Byron/gitoxide/commit/e78052649c734f16f4d154edcbf54f4cc4484f5e))
    - [clone] Assure we don't hang due to unprocessed headers when peeking lines! ([`d9ced27`](https://github.com/Byron/gitoxide/commit/d9ced2711dba702d73b28f0e1b9399cd7eab5183))
    - [clone] none the wiser - it really looks like everything is alright… ([`3b8d613`](https://github.com/Byron/gitoxide/commit/3b8d613c6de349defce9af06d56f73ac2c0d0d25))
    - [clone] reassure ourselves that ERR lines are handled, always ([`925370b`](https://github.com/Byron/gitoxide/commit/925370b3f1d701d3376bdc80a4876e407b54c400))
    - [clone] Response parsing up to (optional) pack ([`24064c7`](https://github.com/Byron/gitoxide/commit/24064c77f2969380fb92ea66109df86e84060324))
    - [clone] properly handle V2 response parsing ([`0d7d768`](https://github.com/Byron/gitoxide/commit/0d7d768278234824e03c5e74dacaafca3ee65713))
    - [ref-ls] A way to abort on multiple delimiters; first tests work ([`8d44912`](https://github.com/Byron/gitoxide/commit/8d44912e7215b85c6931b7b829bd73ac38584424))
    - refactor ([`feec5be`](https://github.com/Byron/gitoxide/commit/feec5be335a99a4c47ba98f93803863044575838))
    - [ref-ls] Allow multiple delimiters at the same time ([`cfae63a`](https://github.com/Byron/gitoxide/commit/cfae63a5f7d2d99560dd857f7220980d70c4c4d8))
    - [ref-ls] It would be practical to simply have access to the line provider… ([`5fba787`](https://github.com/Byron/gitoxide/commit/5fba78796d3bcc16f812dc3202d521ee057e86f9))
    - thanks clippy ([`27f30df`](https://github.com/Byron/gitoxide/commit/27f30df9a8046fe4e872837e36dd497096660282))
    - [ref-ls] don't leak the PacketLine error type in Transport interface ([`58ddd29`](https://github.com/Byron/gitoxide/commit/58ddd2955a407efb6a46249810b916c6035eb5ff))
    - [ref-ls] support for line peeking in packet line readers ([`0c0c575`](https://github.com/Byron/gitoxide/commit/0c0c57522972f2a49ed5261474114da062e6ab15))
    - [ref-ls] don't enforce V1 for local interactions ([`7b33336`](https://github.com/Byron/gitoxide/commit/7b333369de1221f9bfbbe03a3a13e9a09bc1c907))
    - [ref-ls] don't do anything on drop ([`9f18d9b`](https://github.com/Byron/gitoxide/commit/9f18d9b9062d61d6da6e2bb7564fe5edbb1528c4))
    - [ref-ls] Transport layer knows whether it's stateful or not ([`22c3640`](https://github.com/Byron/gitoxide/commit/22c3640b70bb6925d72794eeaeda48b0687f2047))
    - [ref-ls] Always send a flush before closing the connection ([`918f19f`](https://github.com/Byron/gitoxide/commit/918f19f0c2dc202ed2014e30b7247e63a0f6a51e))
    - [ref-ls] git protocol now supports user expansion… ([`d88e9da`](https://github.com/Byron/gitoxide/commit/d88e9da15068aede3a587cbc857702fcfbdc6c6e))
    - refactor ([`e07fbd6`](https://github.com/Byron/gitoxide/commit/e07fbd63db297cd9f70f8b86b1f1f56b15e270a8))
    - refactor ([`7b5ce69`](https://github.com/Byron/gitoxide/commit/7b5ce695a05630c20ab461c63ff1f9b5fc662958))
    - [ref-ls] allow ssh to work with tildes in paths ([`301ae81`](https://github.com/Byron/gitoxide/commit/301ae81d1062fb002b080e8cdbb0bec134dd4de6))
    - [ref-ls] first stab at leaving path resolution to upload pack ([`51dad09`](https://github.com/Byron/gitoxide/commit/51dad09221c118884db7e52d1337eb2ab476e744))
    - [ref-ls] verify also ssh works ([`1ef39ae`](https://github.com/Byron/gitoxide/commit/1ef39aed71a684157363e27d0ae092a616782d41))
    - [ref-ls] tune request to actually work in all cases, particularly for github ([`6bab2f3`](https://github.com/Byron/gitoxide/commit/6bab2f343347dca45288a9f17f1b05fc62611080))
    - [ref-ls] Make credentials helper truly work ([`7f3c3a7`](https://github.com/Byron/gitoxide/commit/7f3c3a71db7eeba1d37481ba1b522d5ded654237))
    - [ref-ls] Finally fix http content encoding (and fixtures to go with it) ([`49b7ad9`](https://github.com/Byron/gitoxide/commit/49b7ad97075474ced5232345c7afac9d657c72b4))
    - [ref-ls] This actually makes things work in real-life ([`24ebc59`](https://github.com/Byron/gitoxide/commit/24ebc59d669dd22c68efa76eb3bcd66b6b59a3dd))
    - [ref-ls] provide blanket impl at least to be less specific ([`0223e7f`](https://github.com/Byron/gitoxide/commit/0223e7f4bf3eb5b3d3f3f430d82ce3386a6a566e))
    - [ref-ls] Make things compile ([`b6506a4`](https://github.com/Byron/gitoxide/commit/b6506a46ef59d8e25b245fa8caac5b4de4fdaa3d))
    - refactor ([`b38290e`](https://github.com/Byron/gitoxide/commit/b38290e4a8fcabd758f26a15407710ab2abcdc07))
    - refactor ([`202383a`](https://github.com/Byron/gitoxide/commit/202383add69e7667fb2043d55e17f8064bc658c9))
    - thanks clippy ([`b060f42`](https://github.com/Byron/gitoxide/commit/b060f42d097e52b5891e3b774ebd8ea8b076d011))
    - [clone] support automatic downgrade to protocol version 1 ([`4cf3643`](https://github.com/Byron/gitoxide/commit/4cf36436f11eb95d420c1147a1ec8adb618ea5fb))
    - [clone] transport provides desired protocol version ([`c39b645`](https://github.com/Byron/gitoxide/commit/c39b64598a4119f0cf3c8aaf30e32996632fb51c))
    - [clone] update README, improve delegate docs ([`dc7908f`](https://github.com/Byron/gitoxide/commit/dc7908f1546239ade71f4147a389a001769311f5))
    - [clone] features for V1 fetch ([`5b24a55`](https://github.com/Byron/gitoxide/commit/5b24a559dfb03c99ee360e9997650c443fd30077))
    - [clone] Support explicitly closing (v2) connections ([`41e4cb2`](https://github.com/Byron/gitoxide/commit/41e4cb22de9e06bfc5aa93246f931f483335fa69))
    - refactor ([`dda62fc`](https://github.com/Byron/gitoxide/commit/dda62fc1a170793768ef1791db85a0c3cca0fad1))
    - [clone] Prevent accidental leakage by transforming back to the 'right' type ([`2d469c6`](https://github.com/Byron/gitoxide/commit/2d469c66ec47be2e1bc3e0b1f3d17dfea5050970))
    - refactor ([`88ecda1`](https://github.com/Byron/gitoxide/commit/88ecda11dc1d97a7460a449350945dcac2f13752))
    - [clone] support for git-credentials helper ([`a6546da`](https://github.com/Byron/gitoxide/commit/a6546dab8d6d0dc4453052b77278cf5bb96aaade))
    - [clone] make URL available in transport layer ([`6778447`](https://github.com/Byron/gitoxide/commit/67784478b96f8afd142e52982e2161a1f05d2ec9))
    - [clone] http basic auth support for all kinds of calls ([`572fb54`](https://github.com/Byron/gitoxide/commit/572fb54b1445d25d55713ca3d68e19bede2b3cff))
    - [clone] first sketch of basic authentication support for http ([`c5b2d04`](https://github.com/Byron/gitoxide/commit/c5b2d046f58a8cd3b66097c48ea9399ac34246d7))
    - [clone] sketch for identity handling ([`b23f470`](https://github.com/Byron/gitoxide/commit/b23f47029fba50c7bba23a6ebe135e129ee9392a))
    - refactor ([`22cb37d`](https://github.com/Byron/gitoxide/commit/22cb37d26f18653b81f52e23f58b5797c763f883))
    - [clone] Add (hardcoded) timeout when connecting via TCP ([`02c195b`](https://github.com/Byron/gitoxide/commit/02c195ba312b174ada5733321c08a8294f360cdd))
    - thanks clippy ([`712527f`](https://github.com/Byron/gitoxide/commit/712527f2adfad18c2527ee7bf9bb8841897db4e0))
    - [clone] Finish implementing ssh access, which might even work ([`8b843f2`](https://github.com/Byron/gitoxide/commit/8b843f2f08c3b070db427a3a8f2c08f4d778914c))
    - [clone] Add support for SSH prefixes in otherwise local service invocations ([`a1db217`](https://github.com/Byron/gitoxide/commit/a1db2172dc691765dec907a226e0790c36358c1f))
    - [clone] once again, for SSH we need to delay calling the actual service ([`2c70275`](https://github.com/Byron/gitoxide/commit/2c70275e45b487a966d1772cf1e7a90e96cbbaad))
    - [clone] Support for the probably very unnkown VIRTUAL_HOST env var ([`36fe20c`](https://github.com/Byron/gitoxide/commit/36fe20cb3a82fe6fa78cc18f7d71d25b9022397c))
    - [clone] Allow connecting via the protocol ([`eb7be2b`](https://github.com/Byron/gitoxide/commit/eb7be2bd5f75ac3a04fb1b6afdc9377478f7818e))
    - [clone] be sure to wait on the spawned child on drop to prevent resource depletion ([`768d7f2`](https://github.com/Byron/gitoxide/commit/768d7f2b704f569e117a63a690c3b3769a2b1442))
    - thanks clippy ([`2528c82`](https://github.com/Byron/gitoxide/commit/2528c82f4708a0258985d3ebfde6cba10a72c9c6))
    - [clone] implement file based protocol using git-<service> processes ([`be254a9`](https://github.com/Byron/gitoxide/commit/be254a9316c08e17702eeb4c65b4dde8e6bb2e6e))
    - [clone] add 'Process' mode for git connection ([`e38c7bf`](https://github.com/Byron/gitoxide/commit/e38c7bf1a2a9409762e891b7bb50f509d9b0d03d))
    - refactor ([`2ecb975`](https://github.com/Byron/gitoxide/commit/2ecb9759f6a916c0a887800df625480c123aa5f6))
    - [clone] first steps towards launching git-upload-pack while… ([`41f05f1`](https://github.com/Byron/gitoxide/commit/41f05f13a1fac078b694e6f4a9c8f52eeaff4191))
    - [clone] Http fetch test for V2 ([`81618ae`](https://github.com/Byron/gitoxide/commit/81618ae8f4e60fbfbb424de6e42bf796dabb47f8))
    - [clone] http test for ls-refs V2 ([`3ef1e47`](https://github.com/Byron/gitoxide/commit/3ef1e47d7e3781bdc52daac0d266dcbaa3dfb07a))
    - [clone] finish test for git-based V2 command invocation ([`9384f32`](https://github.com/Byron/gitoxide/commit/9384f327e6c9bf5d6fc6f17d5d0ed573213fc5d8))
    - [clone] support for V2 arguments ([`8d56e79`](https://github.com/Byron/gitoxide/commit/8d56e7961c7de15197d1127617194fde028cc2aa))
    - refactor ([`f46c89d`](https://github.com/Byron/gitoxide/commit/f46c89d087e387702d2f887946ff0da1fdb19117))
    - refactor ([`9ed859a`](https://github.com/Byron/gitoxide/commit/9ed859a5ebd8f4a1654107c9909f99739c73435d))
    - [clone] Using a normal writer, we can't produce delimiter packets ([`1877b5f`](https://github.com/Byron/gitoxide/commit/1877b5f09d65fac6963b75afdf22afa938c7aac8))
    - [clone] first sketch of extension trait to invoke V2 commands ([`90eed9d`](https://github.com/Byron/gitoxide/commit/90eed9d5a8a672fe6c899c07b98b51e1e783b656))
    - [clone] Finally, HTTP requests are properly handled, it all works out! ([`a6121d9`](https://github.com/Byron/gitoxide/commit/a6121d93eec50406cbd0b1b8a8f1fbbbabec0f53))
    - [clone] Helper now integrates with Http trait, neat ([`b462bc7`](https://github.com/Byron/gitoxide/commit/b462bc7abbb8468063d85d08d656b820fdac903e))
    - [clone] first sketch of 'HeaderAndBody' helper ([`226f096`](https://github.com/Byron/gitoxide/commit/226f0967b557f1186ce5a1ca6f6e80e6926d3210))
    - refactor ([`f2ff90d`](https://github.com/Byron/gitoxide/commit/f2ff90d65edd91c4f6dc6baaf1242df31ef0ef2e))
    - [clone] a way to change progress handling on the fly ([`c1bcc0a`](https://github.com/Byron/gitoxide/commit/c1bcc0adf04a32e9332fae047fba066d4cff6538))
    - [clone] first steps towards more flexible sideband switching ([`3d959e6`](https://github.com/Byron/gitoxide/commit/3d959e68f1b4906ac143e26eb14d65bdf03ef62a))
    - [clone] Issue: shoehorning header handling into the body reader ([`4c304f1`](https://github.com/Byron/gitoxide/commit/4c304f1520fdc06c03aceb389049154cc53edea9))
    - thanks clippy ([`bdcaf36`](https://github.com/Byron/gitoxide/commit/bdcaf3680a13fde84ff7f9cbe3f49b09c8e55d8f))
    - [clone] Now we get to the point where uploads start, but… ([`8bd6182`](https://github.com/Byron/gitoxide/commit/8bd618262c963b23f84ad5214b25efe474abb851))
    - [clone] first steps towards testing posting via http… ([`b6a7e75`](https://github.com/Byron/gitoxide/commit/b6a7e752053254a3547c9afcb6254201fd9f69a8))
    - refactor ([`a810f9f`](https://github.com/Byron/gitoxide/commit/a810f9f354a5e532b819d542ed2a2fdf7a42eb17))
    - refactor ([`5c2bd5f`](https://github.com/Byron/gitoxide/commit/5c2bd5f244bacc776be6f5f45dfd79f03b3a3093))
    - [clone] make on-drop messages do the right thing ([`5a39d70`](https://github.com/Byron/gitoxide/commit/5a39d70c00dbe04598137f479dca54c1ec2dd98e))
    - [clone] first test for request - ideally we manage to add a lifetime to the closure box… ([`db1a5b8`](https://github.com/Byron/gitoxide/commit/db1a5b83338b12a14c3cc53886f5362feef1370f))
    - thanks clippy ([`913e55d`](https://github.com/Byron/gitoxide/commit/913e55de74d3cfecba78345945f1b965b47d407d))
    - refactor ([`de22323`](https://github.com/Byron/gitoxide/commit/de22323c075de5bcb2957418410876101a05c9da))
    - refactor ([`bad8361`](https://github.com/Byron/gitoxide/commit/bad836163c101f74e1eea862698edcbf730a05d5))
    - refactor ([`466557c`](https://github.com/Byron/gitoxide/commit/466557c6c0e4c7b5afe94bee4a4ab07a714089b2))
    - [clone] on-demand line writer, it's cheap ([`8ddd0fa`](https://github.com/Byron/gitoxide/commit/8ddd0fa9f74c9ac44e305d7fb0e53fb816bea0b6))
    - [clone] it shows that the packetline writer is better to be owned ([`f2c6e9f`](https://github.com/Byron/gitoxide/commit/f2c6e9fa763d10f7c72ad4ff29f273f8a6c6872f))
    - refactor ([`aceaaed`](https://github.com/Byron/gitoxide/commit/aceaaed45be5d523c9b4c1f98444b84619cbc13f))
    - refactor ([`2cdda7a`](https://github.com/Byron/gitoxide/commit/2cdda7af8ae884b5efde8861f13d85b07d643b94))
    - refactor ([`521516f`](https://github.com/Byron/gitoxide/commit/521516f8c5713070ca393bd3b54994cb162d7523))
    - refactor ([`3738897`](https://github.com/Byron/gitoxide/commit/3738897a0347694458717e0abbb052ed79c3546d))
    - refactor ([`2e68315`](https://github.com/Byron/gitoxide/commit/2e68315a7a1e47d2659eb3c047ba6015475bd9bf))
    - [clone] first sketch of http request ([`8b4befb`](https://github.com/Byron/gitoxide/commit/8b4befb9ef589d02f9232d72229731b646614fcf))
    - refactor ([`23af7e1`](https://github.com/Byron/gitoxide/commit/23af7e1e7c19d6e0db43540016ef9d851d0a048a))
    - [clone] support writing multiple messages on drop for the 'clone' case ([`9266442`](https://github.com/Byron/gitoxide/commit/92664425cb70d3d646dab97a714efc7f7b99b96d))
    - thanks clippy ([`2ed10de`](https://github.com/Byron/gitoxide/commit/2ed10de5f6213ca5ed68f072b9984bff32a5d67c))
    - [clone] Sketch 'request()' implementation for git protocol ([`fd0e0e9`](https://github.com/Byron/gitoxide/commit/fd0e0e9e49f5481c14e17a462f9e507663fd5e6a))
    - [clone] Allow progress callback to know if it's an error line ([`0c41844`](https://github.com/Byron/gitoxide/commit/0c41844dbad5182ad3ea8d15dcfd0af92263936c))
    - [clone] sketch for generic request/response pattern suitable for clone/fetch ([`e0fd5a6`](https://github.com/Byron/gitoxide/commit/e0fd5a60960e0768a68878f38c034be9c44c6039))
    - thanks clippy (what would I do without you <3) ([`631af04`](https://github.com/Byron/gitoxide/commit/631af04c87f0b6b22c3ac1ef0d300a02bbdcd217))
    - [clone] Capabilities now can have multiple values (per command) for V2 ([`44dcea6`](https://github.com/Byron/gitoxide/commit/44dcea6ed33549e97cfbdd006f9f8fb3ce2e8597))
    - [clone] First step towards http V2 handshake shows capabilities are… ([`f58a785`](https://github.com/Byron/gitoxide/commit/f58a78595658e30cac216c0ddade891cda745eae))
    - [clone] remaining handshake V2 assertions ([`1a58955`](https://github.com/Byron/gitoxide/commit/1a58955f70a945e94d9846424c35c341a8661549))
    - [clone] first sketch of git handshake, V2 ([`bf1f05b`](https://github.com/Byron/gitoxide/commit/bf1f05bbb229f98cfa636c2b974b687042168d20))
    - [clone] git protocol sends in packet line format, which is now enforced ([`4ce5916`](https://github.com/Byron/gitoxide/commit/4ce591646c2c6958bc3cf67a7dc6f792d507c30b))
    - refactor ([`44b06a7`](https://github.com/Byron/gitoxide/commit/44b06a7d4b11494271e59ef3069f0fe326c9eadf))
    - thanks clippy ([`ee5abfc`](https://github.com/Byron/gitoxide/commit/ee5abfcf85889a56495844e4403bc40ebaa01a29))
    - [clone] Configure http timeouts, just so that it is done ([`070855a`](https://github.com/Byron/gitoxide/commit/070855a5bda314c5843ea9091351e8f8540d7d05))
    - refactor ([`8b1dc48`](https://github.com/Byron/gitoxide/commit/8b1dc4846c54be78b2df0f3d02c4efa53c7f79a6))
    - [clone] Allow differentiating HTTP permission errors ([`4c9c413`](https://github.com/Byron/gitoxide/commit/4c9c413c1cdd24dea59274931b335be3daf3653d))
    - [clone] abort early on HTTP status errors ([`e829c0a`](https://github.com/Byron/gitoxide/commit/e829c0ab9923ff38c36196cbe196d158cb3a1ea7))
    - refactor ([`791c05e`](https://github.com/Byron/gitoxide/commit/791c05e08d9c6148fa0f89894c9b2abdadf3f503))
    - [clone] more http test validations ([`e697b8c`](https://github.com/Byron/gitoxide/commit/e697b8cbc236783938fca70c0a4b46ccdf10c084))
    - Revert "[clone] FAIL: try to communicate error codes after request" ([`350de7c`](https://github.com/Byron/gitoxide/commit/350de7cd517f8b94e263312f5dda1515ae6297a9))
    - [clone] FAIL: try to communicate error codes after request ([`2501ddd`](https://github.com/Byron/gitoxide/commit/2501ddd9f377cc869817fb32be213d943a9666a0))
    - [clone] Check for 'smart' protcols ([`2960645`](https://github.com/Byron/gitoxide/commit/2960645ffb160745cac82b2e7267dcff10286420))
    - [clone] validate expected http service announcement ([`a224a2c`](https://github.com/Byron/gitoxide/commit/a224a2c8190b1d45a61eef4cdef620e7f3bea659))
    - [clone] Keep line reader around in http transport ([`feb2596`](https://github.com/Byron/gitoxide/commit/feb259645651309b31df91b18ab247d6955f9a7f))
    - thanks clippy (I really tried) ([`e8880fb`](https://github.com/Byron/gitoxide/commit/e8880fb6e69c15bd44c21edb1ff6de4d8738e036))
    - [clone] unbelievable, but it now seems to work as expected ([`88dbbf5`](https://github.com/Byron/gitoxide/commit/88dbbf5294446bfad1534d1c0635e9a5876ef769))
    - [clone] quick hack to finish http set service, but something is seriously wrong… ([`dd93504`](https://github.com/Byron/gitoxide/commit/dd93504624f57e16b71119a8aadbe3aa8b971a01))
    - [clone] non-deterministic behaviour when parsing HTML, despite ignoring the encoding ([`bab3ec3`](https://github.com/Byron/gitoxide/commit/bab3ec324ddbee50cfddfef76fd7715286fc9caf))
    - [clone] It definitely doesn't want to read the data to the end with 'chunked' ([`49f1aca`](https://github.com/Byron/gitoxide/commit/49f1acac1817c02b76014965f0d26cdfaa4f062e))
    - [clone] for good measure: configure posts (more) correctly ([`e491e58`](https://github.com/Byron/gitoxide/commit/e491e588eb2d3785ac7cc5a226733836e6552309))
    - [clone] Disabling transfer decoding makes it better, but… ([`3a1b8bc`](https://github.com/Byron/gitoxide/commit/3a1b8bcfad37f8f5a0587d7f1a40a499f0c32131))
    - [clone] It looks like curl is stumbling over the 'chunked' header ([`279a386`](https://github.com/Byron/gitoxide/commit/279a3868d1d8ac56bfb7db57a8fb9853bf992f7b))
    - [clone] Fix deadlock - classic, and obvious ([`72a165e`](https://github.com/Byron/gitoxide/commit/72a165ea23b4ba119eb97bbe30912aa25b7502fe))
    - [clone] possibly correct impl of Handler; still hangs though :D ([`aefd8d4`](https://github.com/Byron/gitoxide/commit/aefd8d459030ce3b88c7629894a36ace4ba0fb31))
    - [clone] Fair enough - it locks up somewhere, let's see :D ([`33a1a22`](https://github.com/Byron/gitoxide/commit/33a1a2217e27121c8061ef9547c2589fed39e015))
    - [clone] Improve usability of posts… ([`e1b944e`](https://github.com/Byron/gitoxide/commit/e1b944ef7d79294f56193431abade939e307c19a))
    - [clone] Actually use that abstraction ([`d0bdbe4`](https://github.com/Byron/gitoxide/commit/d0bdbe42ed8a62f23643904f9d2284e31fdbabea))
    - [clone] generalization of get and post ([`e62adc9`](https://github.com/Byron/gitoxide/commit/e62adc937d2c0256c15358e5ee6f76fc2cc5318f))
    - [clone] Curl can now use remote to perform operations (get only for now) ([`a82f028`](https://github.com/Byron/gitoxide/commit/a82f0280f4240bb29801b1beabc96e6f7fa451d7))
    - [clone] try sending curl error even harder… ([`b450bfc`](https://github.com/Byron/gitoxide/commit/b450bfc3d917d13dde9f32427dd905fb35d51063))
    - [clone] first sketch of remote-curl, a way to transform curl into Read/Write ([`22b4b39`](https://github.com/Byron/gitoxide/commit/22b4b39a3e009d25bcfb18c33be1ca4dfcd76f2d))
    - [clone] Send headers with BufReaders ([`6a95aaa`](https://github.com/Byron/gitoxide/commit/6a95aaab582941c6d1697dde6982c0aa8896c73d))
    - refactor ([`d427671`](https://github.com/Byron/gitoxide/commit/d4276719ac5e54bb70ee5a88c534acbf94e4a817))
    - [clone] Fixed shortcomings of http error handling, with thiserror ([`424e159`](https://github.com/Byron/gitoxide/commit/424e1598a2b37f7b67250ad5b7ec62abdfe75f58))
    - [clone] Allow to use specific HttpErrors, at the expense of source ([`b16a8c5`](https://github.com/Byron/gitoxide/commit/b16a8c5c153e7368f17c0a7110157f5a545a35ba))
    - [clone] Fix 'small' compile (without http) ([`29ca5e8`](https://github.com/Byron/gitoxide/commit/29ca5e8df83beede03f966de2aeae5e30638f6bc))
    - [clone] First step towards 'remote http executor' ([`f1e48d7`](https://github.com/Byron/gitoxide/commit/f1e48d7c178b15f0a0b9ba41fb4f94e4c6f0c74a))
    - [clone] things get more complicated…take a step back maybe? ([`f778637`](https://github.com/Byron/gitoxide/commit/f77863726a5b7001e47f8f6b48eb0709a1cd0856))
    - [clone] Right before actually performing the http call… ([`5bf9e6a`](https://github.com/Byron/gitoxide/commit/5bf9e6a21c1bdab3010e362604e324bcebbb45b0))
    - [clone] better user agent header ([`4396587`](https://github.com/Byron/gitoxide/commit/4396587a2d1a0ca50119947048252709cafa277d))
    - [clone] in small steps to getting the http 'interface' right ([`43f2a92`](https://github.com/Byron/gitoxide/commit/43f2a92c6f51a097f6f707953fe215c28c95fa04))
    - [clone] A utility to respond with mock replies on a socket ([`1bf7ef7`](https://github.com/Byron/gitoxide/commit/1bf7ef739dc4c94b4be8a0748333c4486b6555b7))
    - [clone] improvements to Http trait; prep for curl tests ([`9f69d6a`](https://github.com/Byron/gitoxide/commit/9f69d6ab3964b2cd566b5a6c4b739804d7cf3f7f))
    - [clone] a piped iterator ([`5148c85`](https://github.com/Byron/gitoxide/commit/5148c85efc70c0ec06be3ebce267ce727c8ee4e1))
    - thanks clippy ([`c4f570f`](https://github.com/Byron/gitoxide/commit/c4f570fcae7e21745a37a4265b05d21e6149157b))
    - [clone] frame for implementing 'pipe' support ([`c555681`](https://github.com/Byron/gitoxide/commit/c55568127ff943cc6749dba5054d7b3e93c049eb))
    - refactor ([`bfda633`](https://github.com/Byron/gitoxide/commit/bfda633234fa5661a34a98f9f8071570f6cea10c))
    - [clone] sketch for http infrastructure to get going with curl ([`8351299`](https://github.com/Byron/gitoxide/commit/835129968b5414c980f24dc73aa89b43ac39bcaa))
    - [clone] an easy way to get a few HTTP replies for consumption by the client ([`8b082d0`](https://github.com/Byron/gitoxide/commit/8b082d068f1adc0224ab5e0c646a91742dffaa5f))
    - refactor ([`0bbd87e`](https://github.com/Byron/gitoxide/commit/0bbd87e023da4e2cf0412e4c017816be11fc4174))
    - refactor ([`bbce340`](https://github.com/Byron/gitoxide/commit/bbce340e89bbc8b53b6632897ba5bf6dbdeafe11))
    - thanks clippy ([`73a6868`](https://github.com/Byron/gitoxide/commit/73a6868963993a3328e7d8fe94e5a6ac5078a944))
    - [clone] Make it optional to abort the packet line reader on 'ERR <e>' ([`abf9c3b`](https://github.com/Byron/gitoxide/commit/abf9c3b3c9fe757a7418626cd985960f58718357))
    - [clone] Finally it all works exactly as desired… ([`c5bbb57`](https://github.com/Byron/gitoxide/commit/c5bbb57ad7069c839757f72432d23c43de0b61da))
    - [clone] Most of the V1 handshake works, but… ([`318024b`](https://github.com/Byron/gitoxide/commit/318024bc14b0bc88e7728bdacaa0c32265618f4d))
    - [clone] YES! Boxes with dyn traits and lifetimes… ([`5e35d0a`](https://github.com/Byron/gitoxide/commit/5e35d0acf9efcb787eec0591c6f02735ae417e60))
    - [clone] FAIL: Right, need a box after all ([`6e57927`](https://github.com/Byron/gitoxide/commit/6e5792746d164696a1b3bcb64f100328380e19df))
    - [clone] FAIL: can't pass line reader as box ([`633341d`](https://github.com/Byron/gitoxide/commit/633341dd5f3fbd7b910c545e203e0bd734b5f989))
    - [clone] sketching how to possibly return Line readers while keeping it sane… ([`4ba123b`](https://github.com/Byron/gitoxide/commit/4ba123b8e543a2ef3ba07aaf467b208047db0e1d))
    - thanks clippy ([`81c0185`](https://github.com/Byron/gitoxide/commit/81c0185bdb7f483021db1ab85064863c17f33571))
    - refactor ([`f8ff1c7`](https://github.com/Byron/gitoxide/commit/f8ff1c753300c3af7d328440a3591343cbe5f040))
    - [clone] capability parsing ([`5b019af`](https://github.com/Byron/gitoxide/commit/5b019afe49fe1b35f72b91e4919f7a31e87f4f94))
    - refactor ([`2b40961`](https://github.com/Byron/gitoxide/commit/2b40961a2c653d1f5be7d31337e4eed8f08f900a))
    - [clone] a little closer to handling the client handshake ([`1a4f84d`](https://github.com/Byron/gitoxide/commit/1a4f84dd698d89b83d251f81797cb4cf8c3ceb34))
    - [clone] first frame for testing transport layer interactions ([`e1100c8`](https://github.com/Byron/gitoxide/commit/e1100c817bdf49d960aff7b7bee99302583d599c))
    - refactor ([`f3c5c05`](https://github.com/Byron/gitoxide/commit/f3c5c059169e9cc998ec0c80baf637142eb200ef))
    - bump git-features to 0.4 to allow publishes after breaking changes ([`9d6b879`](https://github.com/Byron/gitoxide/commit/9d6b8790e2edd7fa01b3239adff86a7cd2393f10))
    - [clone] move packet-line code into own crate ([`879af67`](https://github.com/Byron/gitoxide/commit/879af671fcde405d3d08ddbc07ea70d0bee23ef1))
    - [clone] http protocol is now optional ([`06c0816`](https://github.com/Byron/gitoxide/commit/06c0816bd0ed0ae0f125c1fa93ffe1b89e7e7eb1))
    - [clone] (very) First stab at http protocol connection ([`218a5eb`](https://github.com/Byron/gitoxide/commit/218a5ebbd5c7b466406fd488ade6c60bde3f78a6))
    - [clone] Better error handling for generalized `connect(…)` ([`713808c`](https://github.com/Byron/gitoxide/commit/713808cd8bd326b632c2b8f0cfbe7f147b1fa0aa))
    - [clone] fix git-transport crate size ([`720f444`](https://github.com/Byron/gitoxide/commit/720f4442b06702b9efc1c257d2b54d4a22d3649d))
    - [clone] enable git-transport tests ([`8e07be4`](https://github.com/Byron/gitoxide/commit/8e07be44ae38830f4894ed80ea9faab567593256))
    - refactor ([`104b7fe`](https://github.com/Byron/gitoxide/commit/104b7fef720751ba7306cee4010e90a20bd6955d))
    - thanks clippy ([`c62bfa2`](https://github.com/Byron/gitoxide/commit/c62bfa277b854ee21c6774ce88f086a2926dd858))
    - [clone] expand-path should be server-side ([`8a38856`](https://github.com/Byron/gitoxide/commit/8a38856a811078d1d453db9c0e0ad7b6baaaed3c))
    - [clone] the return of actually parsing remote progress ([`c465fde`](https://github.com/Byron/gitoxide/commit/c465fdecd4840627a3c6943af014999f5c9cc3e1))
    - [clone] move packet-lint into transport layer ([`c0dd831`](https://github.com/Byron/gitoxide/commit/c0dd8315089243164d82c444499a459756a0337b))
    - [clone] sample on how SSH connection fits in ([`a562059`](https://github.com/Byron/gitoxide/commit/a56205935ead562a8388e86565919081261dea2a))
    - [clone] first sketch of transport layer's connection logic ([`f10cee5`](https://github.com/Byron/gitoxide/commit/f10cee5638a220fff629af274baebbcc0f4f0f61))
    - Allow dual-licensing with Apache 2.0 ([`ea353eb`](https://github.com/Byron/gitoxide/commit/ea353eb02fd4f75508600cc5676107bc7e627f1e))
</details>

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 15 times to make code idiomatic. 

## v0.0.0 (2020-07-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 17 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add missing license description ([`2b80181`](https://github.com/Byron/gitoxide/commit/2b80181ad428a9bf267a9660886f347a850fc76f))
    - Make crates publishable ([`5688a34`](https://github.com/Byron/gitoxide/commit/5688a3427ff3673e1422d43106f4d685fa837aed))
    - \#[forbid(unsafe)] for all crates ([`afda803`](https://github.com/Byron/gitoxide/commit/afda8039259b7a30cfed5dbcdd9caf4773b4c234))
    - cleanup - don't build and run tests while there is nothing to test ([`4a153da`](https://github.com/Byron/gitoxide/commit/4a153da0d60a30615fc402cfecb977f0d771594a))
    - prepare git-transport just so that we don't forget to take the name ([`2c3ad7d`](https://github.com/Byron/gitoxide/commit/2c3ad7d916ca513cc9dff26ff2150bae0dcb93e1))
</details>

