### v0.10.0 (2021-??-??)

- Change return value of `prelude::RepositoryAccessExt::committer()` from `git_actor::Signature` to `Result<git_actor::Signature, easy::borrow:repo::Error>`
- Change return value of `prelude::ReferenceAccessExt` from `Result<Vec<RefEdit>>, _>` to `Result<easy::Reference, _>`.

### v0.9.1 (2021-09-10)

- Remove `max-performance` feature from default set until the `msvc` build issue is fixed. Otherwise it will surprisingly break windows builds.

### v0.9.0 (2021-09-08)

- rename `prelude::ConfigAccessExt` to `prelude::RepositoryAccessExt`
- `prelude::ObjectAccessExt::commit()` signature change
- cargo feature changed in incompatible ways. `network` was replaced by more finegrained options for _blocking_ and _async_ networking, as well as optional http transport
- 
#### New

- `init()`
- `init_bare()`
- `Repository::init(Kind)`
- `open()`
- `Repository::open()`
- `easy::Head`
- `easy::ext::ReferenceAccessExt::head()`
- `ext::ReferenceExt` trait

#### Breaking
- **renames / moves / Signature Changes**
    - `path::Path` to `Path`
    - `init::repository(dir)` -> `path::create::into(dir, **Kind**)`

### v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies
