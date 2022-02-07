## Tracking issues 

* [repository clone](https://github.com/Byron/gitoxide/issues/303)
* [repository FSCK](https://github.com/Byron/gitoxide/issues/304)
* [Show changes in various forms](https://github.com/Byron/gitoxide/issues/305)
* [Client side push (client to server)](https://github.com/Byron/gitoxide/issues/306)
* [Server fetch/pull (server to client)](https://github.com/Byron/gitoxide/issues/307)

## Smaller Tasks

…to not forget. Might get reorganized.

### gix organize

* [ ] Add journey test to cover case with non-bare repository. Try to only read `non-bare` git config files and see the journey test fail.

### gix cat

* A program to cat objects and pretty-print them, similar to git cat-file. Useful to get a feel for
  'locate(…)' performance and stress test it a little.
* Be sure to escape terminal escape codes  
