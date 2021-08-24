# Stability

Even though this project adheres to [semver], this guide aims to explain how semantic versioning is used exactly and how we aim to provide stability within an
ever-changing codebase.

## Terminology

* `dependent crate`
  - A crate which depends on a crate in this workspace directly.
* `downstream crate`
  - A crate which directly or indirectly depends on a crate in this workspace.
* `breaking change`
  - A change in code that requires a `dependent crate` to adjust their code to fix compile errors.
* `release`
  - A new version of a crate is published to crates.io 

## Tiers

The project uses three stability tiers for all of its crates, and all crates use [semver] for their version numbers.
Tiers differ primarily in the time between breaking changes, which always have to be announced with `PRs` as per
our [collaboration guide]

### Tier 3: pre-release crates

Pre-release crates is marked with major version number zero, for example `0.1.4` and lives in stability tier 3 (->ST3).

It's acceptable to let each breaking change be followed by a minor version release.
Dependent crates are advised to use `=<version>` version requirements to avoid automatic updates of patch levels which
may be breaking if they also depend on other `plumbing crates`.

### Tier 2: released plumbing crates

Released plumbing crates are marked with major version number 1 or above, for example `1.2.4` and live in stability tier 2 (->ST2).

Breaking changes are collected and may be released no more often than every 6 weeks by incrementing the major version number.

For example, `git-odb` and `git-ref` both have breaking changes, where `git-odb`'s change is on August 1st and `git-ref`'s dependent change
is on September 10th. `git-odb`'s breaking change can be released earliest on September 15th, whereas `git-ref` can be released earliest at October 22nd.

## Project Structure

The project consists of _many_ _plumbing_ crates, a single _application-level crate_ called `git-repository` as well as a _plumbing_ and a _porcelain apps. 

### Released crates and apps
```
                Release Software v1.X              
    ST1═══════════════════════════════════════════╗
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
    ║                      │                      ║
    ╚═════════════════════════════════════════════╝
                           │                       
    ST2───────────────────────────────────────────┐
    │                      ▼                      │
    │    plumbing crates─────────────────────┐    │
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

## TODO
* **strive for an MVP and version 1.0 fast...**
    * ...even if that includes only the most common usecases.
* **Prefer to increment major version rapidly...**
    * ...instead of keeping major version zero for longer than needed.
* **stability**
    * we adhere to semantic versioning
    * while below 1.0, expect a greater amount of breaking changes, which are announced with minor versions
    * From 1.0, we will try hardest to keep the API and user interface non-breaking the closer to the user a library is. Thus the CLI should remain at version
      1 for a long time. However, crates that make it up can change more rapidly and may see more major version changes over time.

[semver]: https://semver.org
[collaboration guide]: https://github.com/Byron/gitoxide/blob/main/COLLABORATING.md
