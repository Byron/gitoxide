# Stability

Even though this project adheres to [semver], this guide aims to explain how semantic versioning is used exactly and how we aim to provide stability within an
ever-changing codebase.

## Project Structure

The project consists of _many_ _plumbing_ crates, a single _application-level crate_ as well as _plumbing_ and _porcelain apps. The released application crate 
as well as both apps get the tier 1 stability (->S1) and plumbing crates are assigned to the tier 2 stability (->S2) when released.

```
    S1════════════════════════════════════════════╗
    ║                                             ║
    ║    gixp─────────────┐ gix──────────────┐    ║
    ║    │  plumbing app  │ │  porcelain app │    ║
    ║    └────────────────┘ └────────────────┘    ║
    ║             │                  │            ║
    ║             ▼                  ▼            ║
    ║    git-repository──────────────────────┐    ║
    ║    │                application crate  │    ║
    ║    └───────────────────────────────────┘    ║
    ║                      │                      ║
    ╚═════════════════════════════════════════════╝
                           │                       
    S2────────────────────────────────────────────┐
    │                      │                      │
    │                      ▼                      │
    │    plumbing crates─────────────────────┐    │
    │    │ ┌───────────┐       ┌───────────┐ │    │
    │    │ │  git-odb  │       │  git-ref  │ │    │
    │    │ └───────────┘       └───────────┘ │    │
    │    │ ┌───────────┐       ┌───────────┐ │    │
    │    │ │git-config │       │…many more…│ │    │
    │    │ └───────────┘       └───────────┘ │    │
    │    └───────────────────────────────────┘    │
    │                                             │
    └─────────────────────────────────────────────┘```      
```

Pre-release crates and applications fall under tier 3 stability (S3).

## How to proceed 
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
