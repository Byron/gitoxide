# Development Guide

See the [contribution guide] for requirements to meet before beginning implementation.
Follow the [repository-wide development rules](AGENTS.md); this guide provides additional workflow, tooling, and design background.

[contribution guide]: https://github.com/GitoxideLabs/gitoxide/blob/main/CONTRIBUTING.md

## Common commands

Run `just` to browse commands grouped by purpose, with everyday development tasks
first. Use `just --groups` to list the groups, or filter the overview with, for
example, `just --list --group 'Dependencies and SBOMs'`.

## Release source paths

The GitHub release workflow builds `gix` and `ein` through
`etc/scripts/remap-release-paths.sh`. It appends rustc's `--remap-path-prefix`
flags for all target crates, including dependencies, using stable virtual roots:
`/gitoxide` for the checkout, `/cargo` for Cargo sources, `/rust` for the selected
Rust toolchain, and `/generated` for build output. Home, Rustup, and temporary
paths are also remapped. Native host paths, canonical aliases, Windows separator
variants, and `cross` container mounts are covered. Normal
local development builds are unchanged.

To opt into the same policy locally, run these commands from the workspace root:

```sh
bash etc/scripts/remap-release-paths.sh build cargo build --release --locked -p gitoxide --bins
bash etc/scripts/remap-release-paths.sh check target/release/gix target/release/ein
```

## Practices

 * **fuzzing**
   * fuzz parsers, algorithms, and other code that processes untrusted input.
   * add a regression test for every issue found before fixing it.
 * **benchmarks**
   * benchmark performance-sensitive changes before and after implementation.
   * use representative inputs and compare against git when applicable.
 * Adhere to the [stability guide](https://github.com/GitoxideLabs/gitoxide/blob/main/STABILITY.md)

## Commit History

We use feature branches and pull requests to take advantage of CI and GitHub review tools, and merge with merge commits
to make individual efforts stand out. There is no requirement to squash commits or linearize history.
However, commits may be squashed when the history is too messy to review, and commit titles may receive minor edits.

Use Tix to organize topic-oriented commit stacks. See the [Tix workflow](.agents/skills/tix/SKILL.md) for history edits
and preservation of review state.

## Configuration and overrides

As a general rule, respect and implement all applicable [git-config](https://git-scm.com/docs/git-config) by default, but allow the
caller to set overrides. How overrides work depends on the goals of the particular API so it can be done on the main call path,
forcing a choice, or more typically, as a side-lane where overrides can be done on demand.

Note that it should be possible to obtain the current configuration for modification by the user for selective overrides, either
by calling methods or by obtaining a data structure that can be set as a whole using a `get -> modify -> set` cycle.

Note that without any of that, one should document that with `config_snapshot_mut()` any of the relevant configuration can be
changed in memory before invoking a method in order to affect it.

Parameters which are not available in git or specific to `gitoxide` or the needs of the caller can be passed as parameters or via
`Options` or `Context` structures as needed.

## Software bills of materials (SBOMs)

Install the pinned Rust tools once with `just sbom-install`, then run `just sbom` to
write CycloneDX 1.5 and SPDX 2.3 JSON inventories of all workspace members, with all
features and platforms enabled. `cargo deny` remains the audit tool; it does not
export these SBOM formats. `cargo-cyclonedx` generates the inventory and
`sbom-tools` converts that same inventory to SPDX, reporting any metadata loss
(such as CycloneDX properties) on standard error.

Use `--package` to select a workspace crate with its default features on the host
platform. The `gitoxide` package builds the `gix` and `ein` binaries; the `gix`
package is the library. For example:

```sh
just sbom --package gitoxide --no-default-features --features small
just sbom --package gitoxide --no-default-features --features max-pure
just sbom --package gix --no-default-features --features sha1
just sbom --package gix --features blocking-http-transport-reqwest-rust-tls
just sbom --package gix --no-default-features --features sha1,blocking-http-transport-curl-openssl
```

`--features` accepts comma- or space-separated named crate features and can be
repeated. `--all-features` enables every feature of the selected crate. Use
`--target TRIPLE` for another platform or `--target all` for all platforms.

Outputs default to `<Cargo target directory>/sbom/<workspace-or-package>.cdx.json`
and `.spdx.json`. Use `--output-dir PATH` to keep inventories for different feature
or target selections alongside each other. Each run replaces the existing files
for that package or workspace only after both tools succeed.

Inventories include runtime and build dependencies and exclude dev-only dependencies.
Every workspace member is a root in workspace mode, including internal tooling.
Package mode resolves in a temporary workspace so unrelated workspace members
cannot enable extra HTTP or TLS backends. Resolution uses a copy of `Cargo.lock`
and rejects changed package versions; source manifests and lockfiles stay untouched.
The selected graph comes from `cargo tree`, which resolves platform-specific features
and host build dependencies that `cargo metadata --filter-platform` alone cannot isolate.
These are Cargo dependency inventories, not scans of native libraries installed
on the system or of the contents of a compiled binary.

Run `just sbom-test` to exercise generation using an isolated offline Cargo fixture,
including feature selection and matching inventories in both formats.

## General

* **async**
  * **library client-side**
    * Provide `async` network clients as opt-in using feature toggles, and document supported transports and runtime requirements.
  * **User Interfaces**
    * User interfaces can greatly benefit from using async as it's much easier to maintain a responsive UI thread that way thanks
      to the wonderful future combinators.
    * Offload blocking work, including pack processing in async APIs, to an execution context that tolerates blocking rather than the UI thread.
    * Ignoring a result does not stop background work or roll back filesystem changes. Inspect the operation's documented interruption outcome.
  * **server-side**
    * Keep async integration independent of a particular runtime where feasible; account for CPU-intensive and blocking work.
  * **shared blocking and async code**
    * Use async-shaped source with the `keep`, `discard`, and `sync` attributes from `gix-macros`. Alias them in a local `bisync` module at each
      integration point so both variants can coexist while being generated from one implementation that cannot drift.
* **`Default` trait implementations**
  * These can change only if the effect is contained within the callers process.
    This means **changing the default of a file version** is a **breaking change**.
* **Using the `Progress` trait**
  * When receiving a `Progress` implementation
     * without calling `add_child(…)` then use it directly to communicate progress, leaving
       control of the name to the caller. However, call `.init(…)` to configure the iteration.
     * and when calling `add_child(…)` don't use the parent progress instance for anything else.
* **interruption of long-running operations**
  * Use `gix_utils::interrupt::*` to implement cooperative interruption by polling the operation's interrupt flag.
  * The process-global `gix::interrupt::trigger()` flag affects only operations that explicitly poll it. Reset it with
    `gix::interrupt::reset()` only after the affected work has finished, not after an arbitrary delay.
  * **this is a must if…**
    * …temporary resources like files might otherwise be leaked.
  * **this is optional but desirable if…**
    * …there is no leakage otherwise to support user interfaces. They background long-running operations and need them to be cancellable.

* **when to use interior mutability**
  - Optional abstractions can manage caller-owned caches and buffers using **interior mutability only if part of the mutable state has to be returned as borrow**
    or if otherwise it wouldn't be possible to borrowcheck. Or in other words: start without interior mutability and try to do it the standard way, but switch when needed.
  - The preferred way of using it is to start out as upgradable reader, and upgrading to write if needed, keeping contention to a minimum.
  - Shared ownership alone does not require interior mutability; mutation through shared references does. Locally stored caches may still use an API requiring `&mut self`.
  - Types that are not thread-local must be `Sync`, but only if the `gix-parallel/parallel` is enabled due to the usage of `gix_parallel::…` primitives which won't
    be thread-safe without the feature.
* **when to use shared ownership**
  - Use `gix_parallel::OwnShared` for resources shared by thread-local handles. It selects `Arc` or `Rc` according to the `parallel` feature.
    Measure ownership and synchronization overhead in the relevant workload.

## Architecture background

For background on repository-backed platforms and caches, see [this PR](https://github.com/Canop/bacon/pull/98) and
[this discussion](https://github.com/GitoxideLabs/gitoxide/discussions/675).

## Examples, Porcelain CLI and Plumbing CLI - which does what?

### Plumbing vs Porcelain

Both terms are coming from the `git` implementation itself, even though it won't necessarily point out which commands are plumbing and which
are porcelain.
The term *plumbing* refers to lower-level, more rarely used commands that complement porcelain by being invoked by it or by hand for certain use
cases.
The term *porcelain* refers to commands that prioritize an approachable user experience. They are primarily designed for humans,
but may also be used by scripts and agents.

In any case, both types of programs must self-document their capabilities using through the `--help` flag.

From there, we can derive a few rules to adhere to unless there are good reasons not to:

#### Plumbing

* does not show any progress or logging output by default
* if supported and logging is enabled, it will show timestamps in UTC
* tracing is buffered independently of progress and written to stderr with terminal-aware colors after the command finishes
* `corpus` uses the same trace display while persisting full per-run trees independently of the display level
* repository-independent operations accept explicit inputs; repository operations may discover or open a repository

#### Porcelain

* Provides progress information to stderr during long-running operations; allow it to be disabled with `--quiet`.
* If timestamps are shown, they are in localtime.
* Non-progress information goes to stdout.

#### Summary

Here is the hierarchy of programs - each level requires more polish and generally work to be done.
_Examples_ are materialized ideas that others can learn from but that don't quite have the polish (or the potential) to move up to
_plumbing_ or _porcelain_. _Plumbing_ prioritizes composition, whereas _porcelain_ prioritizes an approachable user experience.

* **Examples**
  * An implementation of ideas for actual occasional use and the first step towards getting integrated into Porcelain or Plumbing CLIs.
  * Proper command-line parsing with Clap.
  * Examples must compile; add focused tests where behavior or correctness warrants them. Elaborate test suites are not required.
  * No progress.
  * High quality Rust code along with idiomatic `gitoxide` usage so people can learn from it.
* **Plumbing CLI (like `gix`)**
  * Use Clap for command-line parsing, including small builds.
  * Journey tests only if it's proven valuable
  * Progress can be turned on using the `--verbose` flag, quiet by default.
  * Examples can be turned into plumbing by adding journey tests and optional progress.
* **Porcelain CLI (like `ein`)**
  * Use Clap for command-line parsing for the best quality CLI experience - it's for the user.
  * Journey tests
  * Support for `--quiet`; line progress is shown by default for long-running operations.
  * Verbose by default.
  * Examples can be turned into porcelain by adding journey tests and progress.

# Maintenance Guide

Run `just --list --group 'Maintenance'` to browse maintenance tasks. The `Makefile` retains specific build and fixture-generation targets used by CI.

## Creating a release

Use `just release` to publish crates with the repository's release defaults. Additional release-tool arguments can be passed to this recipe.

`just roll-release <cargo-smart-release-args>` runs the release command and then triggers the GitHub binary workflow for the unique `v*` tag at `HEAD`.
To trigger only that workflow, use `just run-release-workflow`. Workflow commands require an authenticated `gh` CLI.

## Git compatibility checks

Git-version detection and fixture support live in [`gix-testtools`](tests/tools/src/lib.rs).
`should_skip_as_git_version_is_smaller_than()` lets tests skip locally when the installed Git is too old, but deliberately
never skips on CI. Check a failing test's required Git version when reproducing CI-only failures.

For fixtures requiring a particular Git version, `scripted_fixture_read_only_with_git_version()` and its writable counterpart
run the fixture script only with a compatible Git. Otherwise they use a matching archive, or return `None` if none is available.
