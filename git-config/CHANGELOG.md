# Changelog

## `git-config` 0.1.2 (2021-08-06)

### Added

 - Added the following methods to `GitConfig`:
   - `is_empty`
   - `len`
   - `from_env`
   - `open`

### Changed

 - `parse_from_path` now accepts a `AsRef<Path>` instead of a `&Path`.
 - `parse_from_path` now returns an `ParserOrIoError<'static>` instead, from
   `ParserFromIoError`

### Fixed

 - _None._