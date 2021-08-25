Use tempfiles to minimize the risk of resource leakage when preparing to overwrite or create a file with new content
in a signal-safe way, making the change atomic.

Tempfiles can also be used as locks as only one tempfile can exist at a given path at a time.

* [x] registered temporary files which are deleted automatically as the process terminates or on drop
    * [x] write to temorary file and persist it under new name
    * [x] close temporary files to convert them into a marker while saving system resources
    * [x] mark paths with a closed temporary file
* [x] persist temporary files to prevent them from perishing.
* [x] signal-handler integration with `git-repository` to clean lockfiles before the process is aborted.
* [x] use a temporary file transparently due thanks to implementations of `std::io` traits
