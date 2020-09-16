## Potential for improving performance

### NLL/Borrowcheck limitation in git-odb

* Once polonius is available with production-ready performance, we have to [make this code less wasteful](https://github.com/Byron/gitoxide/blob/b125c763c5d628c397dce9a5d085fbf123ce1f29/git-odb/src/compound.rs#L42)
 * See https://github.com/rust-lang/rust/issues/45402 for a discussion and more links
 * Here is [a commit](https://github.com/Byron/gitoxide/commit/8c5bd095539042d7db0e611460803cdbf172beb0) that sets up polonius, which greatly degrades borrow check times (as of 2020-09-15)

### Pack Decoding

* [ ] @joshtriplett writes: "Regarding decompression performance, try replacing miniz_oxide with a better zlib decoder. Build with libz-sys, and then try substituting zlib-ng built with --zlib-compat. (I'm working on making that easier.) That should substantially improve decompression."
  * @joshtriplett writes: "As far as I know, I'm not aware of flate2 adding any significant overhead, and it provides fairly low-level interfaces in addition to high-level ones. If there's a good reason to, you could use libz-sys directly, but that's a less safe interface. Either way, if you port to libz-sys or to a crate like flate2 that's based on libz-sys, that'll make it trivial to switch to zlib-ng later, as well as making it easy to test zlib-ng now via LD_LIBRARY_PATH."
  * potential [savings: MASSIVE](https://github.com/Byron/gitoxide/issues/1#issuecomment-672626465) 
  * Note that this should only be feature toggled. Using any of the above would replace a pure Rust implementation, which we would always like to keep as an option for those who want maximum safety.
* [ ] Add more control over the amount of memory used for the `less-memory` algorithm of `pack-verify` to increase cache hit rate at the cost of memory.
  Note that depending on this setting, it might not be needed anymore to iterated over sorted offsets, freeing 150MB of memory in the process
  that could be used for the improved cache. With the current cache and no sorted offsets, the time nearly triples.
* [ ] _progress measuring costs when using 96 cores_ (see [this comment][josh-aug-12])
  * potential savings: low
* [ ] Add '--chunk|batch-size' flag to `pack-verify` and `pack-index-from-data` to allow tuning sizes for large amounts of cores
  * @joshtriplett write: "I did find that algorithm when I was looking for the chunk size, though I didn't dig into the details. As a quick hack, I tried dropping the upper number from 1000 to 250, which made no apparent difference in performance."
  * potential savings: ~~medium~~ unclear
* [ ] On 96 core machines, it takes visible time until all threads are started and have work. Is it because starting 100 threads takes so long? Or is it contention to get work?
* [ ] Improve cache hit rate of `lookup` pack traversal by using partial DAGs build with help of the index
  * @joshtriplett writes: "Would it be possible, with some care, to use the index to figure out in advance which objects will be needed again and which ones won't? Could you compute a small DAG of objects you need for deltas (without storing the objects themselves), and use that to decide the order you process objects in?"
  * Note that there is tension between adding more latency to build such tree and the algorithms ability to (otherwise) start instantly.
  * potential savings: unknown
  
  ### Miniz Oxide
  * [x] Wait for release so we can use the new reset capability introduced by [this PR](https://github.com/Frommi/miniz_oxide/pull/91)
