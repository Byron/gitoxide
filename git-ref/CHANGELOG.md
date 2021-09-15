# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.7.3

- Compatibility with Rust 1.55. It informed about the incorrect usage of `std::io::ErrorKind::Other` which this crate also dependent on causing
  runtime bugs. The latter were detected by tests and this release contains the fix to not rely on certain behaviour anymore.

## v0.7.0

### Breaking

* Replace `transaction::Create` with `transaction::PreviousValue` and remove `transaction::Create`
* Remove `file::Reference` in favor of `Reference`
* Move `file::log::Line` to `log::Line`
* `TargetRef::Symbolic(&BStr)` -> `TargetRef::Symbolic(FullNameRef)`
* replace `Transaction::namespacce()` with `file::Store::namespace`

## v0.6.1

### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

## v0.6.0

### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`
