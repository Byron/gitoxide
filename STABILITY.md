# Stability

Even though this project adheres to [semver], this guide aims to explain how semantic versioning is used exactly and how we aim to provide stability within an
ever-changing codebase.

Please note that this guide isn't stable itself and may be adjusted to fit changes in requirements or new ways are discovered.

## Terminology

* _dependent crate_
  - A crate which depends on a crate in this workspace directly.
* _downstream crate_
  - A crate which directly or indirectly depends on a crate in this workspace.
* _workspace crate_
  - A crate which is a member of this workspace and hence is stored in this repository
* _breaking change_
  - A change in code that requires a `dependent crate` to adjust their code to fix compile errors.
* _release_
  - A new version of a crate is published to crates.io
* _development version_ 
  - A crate version whose _major_ version is 0.
* _release version_
  - A crate version whose _major_ version is 1 or higher.

## Tiers

The project uses three stability tiers for all of its crates, and all crates use [semver] for their version numbers.
Tiers differ primarily in the time between breaking changes, which always have to be announced with `PRs` as per
our [collaboration guide].

The following schematic helps to visualize what follows.

```
                Release Software v1.X              
    Stability Tier 1 ═════════════════════════════╗
    ║                                             ║
    ║    gixp─────────────┐ gix──────────────┐    ║
    ║    │  plumbing app  │ │  porcelain app │    ║
    ║    └────────────────┘ └────────────────┘    ║
    ║             │                  │            ║
    ║             ▼                  ▼            ║
    ║    gitoxide-core───────────────────────┐    ║
    ║    │        application functionality  │    ║
    ║    └───────────────────────────────────┘    ║
    ║                      │                      ║
    ║                      ▼                      ║
    ║    git-repository──────────────────────┐    ║
    ║    │                application crate  │    ║
    ║    └───────────────────────────────────┘    ║
    ║             │                       │       ║
    ║             ▼                               ║
    ║    Foundation Crates─────────────┐  │       ║
    ║    │┌──────────┐ ┌──────────────┐│          ║
    ║    ││ git-hash │ │ git-ref-base ││  │       ║
    ║    │└──────────┘ └──────────────┘│          ║
    ║    │     ┌─────────────────┐     │  │       ║
    ║    │     │ git-config-base │     │          ║
    ║    │     └─────────────────┘     │  │       ║
    ║    └─────────────────────────────┘          ║
    ║                                     │       ║
    ╚═════════════════════════════════════════════╝
                            ─ ─ ─ ─ ─ ─ ─ ┘        
    Stability Tier 2 ──────┼──────────────────────┐
    │                      ▼                      │
    │    Plumbing Crates─────────────────────┐    │
    │    │ ┌───────────┐       ┌───────────┐ │    │
    │    │ │  git-odb  │       │  git-ref  │ │    │
    │    │ └───────────┘       └───────────┘ │    │
    │    │ ┌───────────┐       ┌───────────┐ │    │
    │    │ │git-config │       │ git-pack  │ │    │
    │    │ └───────────┘       └───────────┘ │    │
    │    │ ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐ │    │
    │    │            …many more…            │    │
    │    │ └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘ │    │
    │    └───────────────────────────────────┘    │
    └─────────────────────────────────────────────┘
 ```

### Tier 3: pre-release crates

Pre-release crates is marked with major version number zero, for example `0.1.4` and lives in stability tier 3 _(->ST3)_.

It's acceptable to let each breaking change be immediately followed by a minor version release.

### Tier 2: released plumbing crates

Released plumbing crates are marked with major version number 1 or above, for example `1.2.4` and live in stability tier 2 _(->ST2)_.

Breaking changes are collected and may be released no more often than every 4 weeks by incrementing the major version number.

For example, `git-odb` and `git-ref` both have breaking changes, where `git-odb`'s change is on August 1st and `git-ref`'s dependent change
is on September 10th. `git-odb`'s breaking change can be released earliest on September 1st, whereas `git-ref` can be released earliest at October 10th.

If there are additional breaking changes without a release, these push back the earliest release date accordingly.

### Tier 1: released apps and application crates

Released apps and application crates are marked with major version number 1 or above, like `2.3.0+21.06` and live in tier 1 _(->ST1)_, 
with the build identifiers for year (`21`) and and month `06` appended, based on the actual release year and month.

Breaking changes are collected and may be released no more often than every 6 months by incrementing the major version number. If there are additional breaking changes,
these push bac the release date so that they can be tested at least for 3 months. For example, a breaking change happens in January 01, and another breaking change in February 15.
The earliest release date is July 1st. Had the second breaking change happened in April 01, the release date would have to be pushed to August 1st.

Intermediate pre-releases may be created at most every 4 weeks by appending `-alpha.X` where `X` is the sequential release number. These should help testing
breaking changes or new features without forcing the use of `git` sources for dependencies in cargo manifests. Pre-release releases must pin all the pre-release
crates they depend on to prevent automatic upgrades. Dependent external crates are advised to pin their `alpha` dependencies with `=<version>` version requirements to avoid
automatic updates which may be breaking.

Once breaking changes are known to be planned, deprecation warnings should be provided in intermediate pre-releases.

Minor version updates for new features can be released when needed assuming there are no other breaking changes, updating the build identifiers for year and month accordingly.

## The _Minimal Stable Rust Version_ (->MSRV)

The MSRV is automatically assumed to be the latest stable version.

Increasing the MSRV is not considered a breaking change and doesn't warrant a major version bump itself.

Please let us know if you have other requirement and we see if we can provide stability guarantees for it or reduce the MSRV to a given version.

## Transitioning from pre-release to release crates

How do we avoid staying in pre-release mode forever?

There is only two questions to ask and answer positively:

- _Does the crate fulfill its intended purpose well enough?_
- _Do the dependent workspace crates fulfill their intended purposes well enough?_

For plumbing crates, the intended purpose is narrow which would allow them to transition earlier. For plumbing crates, if in doubt or fear of future requirements
especially if dependent crates are still early in development, prefer to release them anyway and live with requirements of _ST2_.

Apps and application crates may take longer as they are larger in scope. A good indicator for them to get to a release may be maturing plumbing crates they
use. Their scope shoud also be narrowed to a minimal viable product.

[semver]: https://semver.org
[collaboration guide]: https://github.com/Byron/gitoxide/blob/main/COLLABORATING.md
