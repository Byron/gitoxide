const SMALL_PACK_INDEX: &str = "packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx";
const SMALL_PACK: &str = "packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.pack";

/// All hardcoded offsets are obtained via `git verify-pack --verbose  tests/fixtures/packs/pack-a2bf8e71d8c18879e499335762dd95119d93d9f1.idx`
mod file;

mod index;
