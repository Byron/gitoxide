## Potential for improving performance

### NLL/Borrowcheck limitation git-odb::CompoundDb cause half-of-possible performance during object lookup

* Once polonius is available with production-ready performance, we have to [make this code less wasteful](https://github.com/Byron/gitoxide/blob/b125c763c5d628c397dce9a5d085fbf123ce1f29/git-odb/src/compound.rs#L42)
 * See https://github.com/rust-lang/rust/issues/45402 for a discussion and more links
 * Here is [a commit](https://github.com/Byron/gitoxide/commit/8c5bd095539042d7db0e611460803cdbf172beb0) that sets up polonius, which greatly degrades borrow check times (as of 2020-09-15)

### Pack Decoding

* [ ] On **ARM64 on MacOS** the SHA1 implementation of the [`sha-1` crate](https://github.com/RustCrypto/hashes) is capped at about 550MB/s, half the speed of what I saw on Intel and about 50% slower than what's implemented in `libcorecrypto.dylib`. Get that fast and the decoding stage will be able
      to beat git on fewer cores. [See this comment for more](https://github.com/Byron/gitoxide/discussions/46#discussioncomment-511268). Right now we only do when scaling beyond what `git` can do due to lock contention.
      * This should work once the `asm` feature can be enabled in the `sha-1` crate, which currently fails but is tracked [in this issue](https://github.com/RustCrypto/asm-hashes/issues/28).
        * If it's not fast enough, one might hope that ARM8 instructions can improve performance, but right now they [aren't available](https://github.com/rust-lang/stdarch/issues/1055#issuecomment-803737796).
        * Maybe the path forward for that crate is to [use system or openssl dylibs](https://github.com/RustCrypto/asm-hashes/issues/5).
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
    
[josh-aug-12]: https://github.com/Byron/gitoxide/issues/1#issuecomment-672566602
