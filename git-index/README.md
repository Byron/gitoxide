
#### Test fixtures

Most of the test indices are snatched directly from the unit test suite of `git` itself, usually by running something like the following

```shell
 ./t1700-split-index.sh -r 2 --debug 
```

Then one finds all test state and the index in particular in `trash directory/t1700-split-index/.git/index` and can possibly copy it over and use as fixture.
The preferred way is to find a test of interest, and use its setup code within one of our own fixture scripts that are executed once to generate the file of interest.
