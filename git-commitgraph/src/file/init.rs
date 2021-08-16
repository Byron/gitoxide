use std::{
    convert::{TryFrom, TryInto},
    ops::Range,
    path::Path,
};

use bstr::ByteSlice;
use byteorder::{BigEndian, ByteOrder};
use filebuffer::FileBuffer;
use git_hash::SIZE_OF_SHA1_DIGEST as SHA1_SIZE;

use crate::file::{
    ChunkId, File, BASE_GRAPHS_LIST_CHUNK_ID, CHUNK_LOOKUP_SIZE, COMMIT_DATA_CHUNK_ID, COMMIT_DATA_ENTRY_SIZE,
    EXTENDED_EDGES_LIST_CHUNK_ID, FAN_LEN, HEADER_LEN, OID_FAN_CHUNK_ID, OID_LOOKUP_CHUNK_ID, OID_LOOKUP_ENTRY_SIZE,
    SENTINEL_CHUNK_ID, SIGNATURE,
};

/// The error used in [`File::at()`].
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Commit-graph {:?} chunk contains {from_chunk} base graphs, but commit-graph file header claims {from_header} base graphs", BASE_GRAPHS_LIST_CHUNK_ID.as_bstr())]
    BaseGraphMismatch { from_header: u8, from_chunk: u32 },
    #[error("Commit-graph {:?} chunk contains {chunk1_commits} commits, but {:?} chunk contains {chunk2_commits} commits", .chunk1_id.as_bstr(), .chunk2_id.as_bstr())]
    CommitCountMismatch {
        chunk1_id: ChunkId,
        chunk1_commits: u32,
        chunk2_id: ChunkId,
        chunk2_commits: u32,
    },
    #[error("{0}")]
    Corrupt(String),
    #[error("Commit-graph file contains multiple {:?} chunks", .0.as_bstr())]
    DuplicateChunk(ChunkId),
    // This error case is disabled, as git allows extra garbage in the extra edges list?
    // #[error("The last entry in commit-graph's extended edges list does is not marked as being terminal")]
    // ExtraEdgesOverflow,
    #[error("Commit-graph chunk {:?} has invalid size: {msg}", .id.as_bstr())]
    InvalidChunkSize { id: ChunkId, msg: String },
    #[error("Could not open commit-graph file at '{}'", .path.display())]
    Io {
        #[source]
        err: std::io::Error,
        path: std::path::PathBuf,
    },
    #[error("Missing required chunk {:?}", .0.as_bstr())]
    MissingChunk(ChunkId),
    #[error("{0}")]
    Trailer(String),
    #[error("Commit-graph file uses unsupported hash version: {0}")]
    UnsupportedHashVersion(u8),
    #[error("Unsupported commit-graph file version: {0}")]
    UnsupportedVersion(u8),
}

const TRAILER_LEN: usize = SHA1_SIZE;
const MIN_FILE_SIZE: usize = HEADER_LEN + ((MIN_CHUNKS + 1) * CHUNK_LOOKUP_SIZE) + TRAILER_LEN;

// Required chunks: OIDF, OIDL, CDAT
const MIN_CHUNKS: usize = 3;

impl File {
    /// Try to parse the commit graph file at `path`.
    pub fn at(path: impl AsRef<Path>) -> Result<File, Error> {
        Self::try_from(path.as_ref())
    }
}

impl TryFrom<&Path> for File {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let data = FileBuffer::open(path).map_err(|e| Error::Io {
            err: e,
            path: path.to_owned(),
        })?;
        let data_size = data.len();
        if data_size < MIN_FILE_SIZE {
            return Err(Error::Corrupt(
                "Commit-graph file too small even for an empty graph".to_owned(),
            ));
        }

        let mut ofs = 0;
        if &data[ofs..ofs + SIGNATURE.len()] != SIGNATURE {
            return Err(Error::Corrupt(
                "Commit-graph file does not start with expected signature".to_owned(),
            ));
        }
        ofs += SIGNATURE.len();

        match data[ofs] {
            1 => (),
            x => {
                return Err(Error::UnsupportedVersion(x));
            }
        };
        ofs += 1;

        match data[ofs] {
            1 => (),
            x => {
                return Err(Error::UnsupportedHashVersion(x));
            }
        };
        ofs += 1;

        let chunk_count = data[ofs];
        // Can assert chunk_count >= MIN_CHUNKS here, but later OIDF+OIDL+CDAT presence checks make
        // it redundant.
        ofs += 1;

        let base_graph_count = data[ofs];
        ofs += 1;

        let chunk_lookup_end = ofs + ((usize::from(chunk_count) + 1) * CHUNK_LOOKUP_SIZE);
        if chunk_lookup_end > data_size {
            return Err(Error::Corrupt(format!(
                "Commit-graph file is too small to hold {} chunks",
                chunk_count
            )));
        }

        let mut base_graphs_list_offset: Option<usize> = None;
        let mut commit_data_offset: Option<usize> = None;
        let mut commit_data_count = 0u32;
        let mut extra_edges_list_range: Option<Range<usize>> = None;
        let mut fan_offset: Option<usize> = None;
        let mut oid_lookup_offset: Option<usize> = None;
        let mut oid_lookup_count = 0u32;

        let mut chunk_id: ChunkId = data[ofs..ofs + 4].try_into().expect("ChunkId to accept 4 bytes");
        ofs += 4;

        let mut chunk_offset: usize = BigEndian::read_u64(&data[ofs..ofs + 8])
            .try_into()
            .expect("an offset small enough to fit a usize");
        if chunk_offset < chunk_lookup_end {
            return Err(Error::Corrupt(format!(
                "Commit-graph chunk 0 has invalid offset {} (must be at least {})",
                chunk_offset, chunk_lookup_end
            )));
        }
        ofs += 8;

        for _ in 0..chunk_count {
            let next_chunk_id: ChunkId = data[ofs..ofs + 4].try_into().expect("ChunkId to accept 4 bytes");
            ofs += 4;

            let next_chunk_offset: usize = BigEndian::read_u64(&data[ofs..ofs + 8])
                .try_into()
                .expect("an offset small enough to fit a usize");
            ofs += 8;

            let chunk_size: usize =
                next_chunk_offset
                    .checked_sub(chunk_offset)
                    .ok_or_else(|| Error::InvalidChunkSize {
                        id: chunk_id,
                        msg: "size is negative".to_string(),
                    })?;
            if next_chunk_offset >= data_size {
                return Err(Error::InvalidChunkSize {
                    id: chunk_id,
                    msg: "chunk extends beyond end of file".to_string(),
                });
            }

            match chunk_id {
                BASE_GRAPHS_LIST_CHUNK_ID => {
                    if base_graphs_list_offset.is_some() {
                        return Err(Error::DuplicateChunk(chunk_id));
                    }
                    if chunk_size % SHA1_SIZE != 0 {
                        return Err(Error::InvalidChunkSize {
                            id: chunk_id,
                            msg: format!("chunk size {} is not a multiple of {}", chunk_size, SHA1_SIZE),
                        });
                    }
                    let chunk_base_graph_count: u32 = (chunk_size / SHA1_SIZE)
                        .try_into()
                        .expect("base graph count to fit in 32-bits");
                    if chunk_base_graph_count != u32::from(base_graph_count) {
                        return Err(Error::BaseGraphMismatch {
                            from_chunk: chunk_base_graph_count,
                            from_header: base_graph_count,
                        });
                    }
                    base_graphs_list_offset = Some(chunk_offset);
                }
                COMMIT_DATA_CHUNK_ID => {
                    if commit_data_offset.is_some() {
                        return Err(Error::DuplicateChunk(chunk_id));
                    }
                    if chunk_size % COMMIT_DATA_ENTRY_SIZE != 0 {
                        return Err(Error::InvalidChunkSize {
                            id: chunk_id,
                            msg: format!(
                                "chunk size {} is not a multiple of {}",
                                chunk_size, COMMIT_DATA_ENTRY_SIZE
                            ),
                        });
                    }
                    commit_data_offset = Some(chunk_offset);
                    commit_data_count = (chunk_size / COMMIT_DATA_ENTRY_SIZE)
                        .try_into()
                        .expect("number of commits in CDAT chunk to fit in 32 bits");
                }
                EXTENDED_EDGES_LIST_CHUNK_ID => {
                    if extra_edges_list_range.is_some() {
                        return Err(Error::DuplicateChunk(chunk_id));
                    }

                    extra_edges_list_range = Some(Range {
                        start: chunk_offset,
                        end: next_chunk_offset,
                    })
                }
                OID_FAN_CHUNK_ID => {
                    if fan_offset.is_some() {
                        return Err(Error::DuplicateChunk(chunk_id));
                    }
                    let expected_size = 4 * FAN_LEN;
                    if chunk_size != expected_size {
                        return Err(Error::InvalidChunkSize {
                            id: chunk_id,
                            msg: format!("expected chunk length {}, got {}", expected_size, chunk_size),
                        });
                    }
                    fan_offset = Some(chunk_offset);
                }
                OID_LOOKUP_CHUNK_ID => {
                    if oid_lookup_offset.is_some() {
                        return Err(Error::DuplicateChunk(chunk_id));
                    }
                    if chunk_size % OID_LOOKUP_ENTRY_SIZE != 0 {
                        return Err(Error::InvalidChunkSize {
                            id: chunk_id,
                            msg: format!(
                                "chunk size {} is not a multiple of {}",
                                chunk_size, OID_LOOKUP_ENTRY_SIZE
                            ),
                        });
                    }
                    oid_lookup_offset = Some(chunk_offset);
                    oid_lookup_count = (chunk_size / OID_LOOKUP_ENTRY_SIZE)
                        .try_into()
                        .expect("number of commits in OIDL chunk to fit in 32 bits");
                    // TODO(ST): Figure out how to handle this. Don't know what to do with the commented code.
                    // git allows extra garbage in the extra edges list chunk?
                    // if oid_lookup_count > 0 {
                    //     let last_edge = &data[next_chunk_offset - 4..next_chunk_offset];
                    //     let last_edge = BigEndian::read_u32(last_edge);
                    //     if let ExtraEdge::Internal(_) = ExtraEdge::from_raw(last_edge) {
                    //         return Err(Error::ExtraEdgesListOverflow);
                    //     }
                    // }
                }
                _ => {}
            }

            chunk_id = next_chunk_id;
            chunk_offset = next_chunk_offset;
        }
        if chunk_id != SENTINEL_CHUNK_ID {
            return Err(Error::Corrupt(format!(
                "Commit-graph file has invalid last chunk ID: {:?}",
                chunk_id.as_bstr()
            )));
        }

        let actual_trailer_len = data_size.saturating_sub(chunk_offset);
        if actual_trailer_len != TRAILER_LEN {
            return Err(Error::Trailer(format!(
                "Expected commit-graph trailer to contain {} bytes, got {}",
                TRAILER_LEN, actual_trailer_len
            )));
        }

        let fan_offset = fan_offset.ok_or(Error::MissingChunk(OID_FAN_CHUNK_ID))?;
        let oid_lookup_offset = oid_lookup_offset.ok_or(Error::MissingChunk(OID_LOOKUP_CHUNK_ID))?;
        let commit_data_offset = commit_data_offset.ok_or(Error::MissingChunk(COMMIT_DATA_CHUNK_ID))?;
        if base_graph_count > 0 && base_graphs_list_offset == None {
            return Err(Error::MissingChunk(BASE_GRAPHS_LIST_CHUNK_ID));
        }

        let (fan, _) = read_fan(&data[fan_offset..]);
        if oid_lookup_count != fan[255] {
            return Err(Error::CommitCountMismatch {
                chunk1_id: OID_FAN_CHUNK_ID,
                chunk1_commits: fan[255],
                chunk2_id: OID_LOOKUP_CHUNK_ID,
                chunk2_commits: oid_lookup_count,
            });
        }
        if commit_data_count != fan[255] {
            return Err(Error::CommitCountMismatch {
                chunk1_id: OID_FAN_CHUNK_ID,
                chunk1_commits: fan[255],
                chunk2_id: COMMIT_DATA_CHUNK_ID,
                chunk2_commits: commit_data_count,
            });
        }
        Ok(File {
            base_graph_count,
            base_graphs_list_offset,
            commit_data_offset,
            data,
            extra_edges_list_range,
            fan,
            oid_lookup_offset,
            path: path.to_owned(),
        })
    }
}

// Copied from git-odb/pack/index/init.rs
fn read_fan(d: &[u8]) -> ([u32; FAN_LEN], usize) {
    let mut fan = [0; FAN_LEN];
    for (c, f) in d.chunks(4).zip(fan.iter_mut()) {
        *f = BigEndian::read_u32(c);
    }
    (fan, FAN_LEN * 4)
}
