This file is for tracking features that are less well implemented or less powerful than their `git` counterparts for one reason or another.

#### `git-repository`

### git-protocol
* **fetches using protocol V1 and stateful connections, i.e. ssh, git, file, may hang**
    * This can be fixed by making response parsing.
    * Note that this does not affect cloning, which works fine.

### `git-pack`
* **Packfiles use memory maps**
    * Even though they are comfortable to use and fast, they squelch IO errors.
    * _potential remedy_: We could generalize the Pack to make it possible to work on in-memory buffers directly. That way, one
      would initialize a Pack by reading the whole file into memory, thus not squelching IO errors at the expense of latency as well
      as memory efficiency.
* **Packfiles cannot load files bigger than 2^31 or 2^32 on 32 bit systems**
    * As these systems cannot address more memory than that.
    * _potential remedy_: implement a sliding window to map and unmap portions of the file as needed.
        * However, those who need to access big packs on these systems would rather resort to `git` itself, allowing
          our implementation to be simpler and potentially more performant.
* **Objects larger than 32 bits cannot be loaded on 32 bit systems**
    * in-memory representations objects cannot handle objects greater than the amount of addressable memory.
    * This will not affect git LFS though.

### `git-url`

* **git-url** _might_ be more restrictive than what git allows as for the most part, it uses a browser grade URL parser.
    * Thus far there is no proof for this, and as _potential remedy_ we could certainly re-implement exactly what git does
      to handle its URLs.

### `git-features`

* **local time** is currently impeded by [this issue](https://github.com/time-rs/time/issues/293#issuecomment-909158529) but it's planned to resolve it eventually.
