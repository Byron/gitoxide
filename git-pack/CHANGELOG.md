# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

### Unreleased

#### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 17 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 240 commits contributed to the release over the course of 12 calendar days.
 - 54 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on

### v0.11.0 (2021-09-08)

- manual bump for safety as its dependencies have breaking changes



#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.10.0 (2021-09-07)

- **renames**
   - `data::Object::into_commit_iter()` -> `data::Object::try_into_commit_iter()`
   - `data::Object::into_tree_iter()` -> `data::Object::try_into_tree_iter()`
   - `data::Object::into_tag_iter()` -> `data::Object::try_into_tag_iter()`



#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 86 commits contributed to the release over the course of 2 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.9.0 (2021-08-27)

- **renames / moves / visibility**
   - `find::Find`  and `find::FindExt` only in `Find` and `FindExt` (not in `find` anymore)
   - `data::output::count::Count` -> `data::output::Count`
   - `data::output::entry::Entry` -> `data::output::Entry`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find()-> `Find::try_find()`
   - `bundle::Bundle` -> `Bundle`
   - `bundle::Error` -> `bundle::init::Error`
   - `pub tree::` -> `pub(crate) cache::delta::`
   - `data::object::Object` -> `data::Object`
   - `data::entry::Entry` -> `data::Entry`

* **new methods**
   - `Find::find_tag_iter()`
#### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 4 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 185 commits contributed to the release over the course of 5 calendar days.
 - 1 commit where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.8.2 (2021-08-17)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 2 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.8.1 (2021-08-13)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.8.0 (2021-08-12)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 4 commits contributed to the release over the course of 1 calendar day.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.6.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 4 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.5.0 (2021-08-11)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 9 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.3.1 (2021-08-10)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 3 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.3.0 (2021-08-10)

#### Thanks Clippy

<csr-read-only-do-not-edit/>
[Clippy](https://github.com/rust-lang/rust-clippy) helped 7 times to make code idiomatic. 

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 142 commits contributed to the release over the course of 76 calendar days.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.2.0 (2021-05-25)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 13 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

### v0.1.0 (2021-05-24)

#### Commit Statistics

<csr-read-only-do-not-edit/>
 - 2 commits contributed to the release.
 - 0 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 unique issues were worked on

