### v0.9.0 (2021-08-??)

#### New

- `init()`
- `init_bare()`
- `Repository::init(Kind)`
- `open()`
- `Repository::open()`

#### Breaking
- **renames / moves / Signature Changes**
    - `path::Path` to `Path`
    - `init::repository(dir)` -> `path::create::into(dir, **Kind**)`

### v0.8.1 (2021-08-28)

- Introduce `EasyArcExclusive` type, now available thanks to `parking_lot` 0.11.2

### v0.8.0 (2021-08-27)

- Rename `object` to `objs` to be equivalent to `refs` and make space for the new `object` module
- various minor version updates of pre-release dependencies
