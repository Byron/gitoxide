### v0.6.1

#### Bugfixes

* splits of edits to symbolic references will now 'move' the desired previous values down to the
  referents while resorting to not having any requirements in the symbolic ref instead.

### v0.6.0

#### BREAKING

- rename `file::Store::packed()` to `file::Store::packed_buffer()`
