# criome-store

Content-addressed blob store engine. Generic — stores bytes addressed by
blake3 hash. Knows nothing about World, Commit, or any typed caller.

## Current implementation

redb index + one-file-per-blob with two-level fan-out + optional zstd.
This is the old-stack design (string era). Works, but carries dependencies
(redb, zstd, base64) that the binary-native stack eliminates.

## Target: append-only file store

Replace redb + blob files with a single append-only data file.
Content-addressed immutable records have no updates, no deletes, no
ordering, no transactions — all the machinery of B-trees, WAL, page
splits, MVCC exists to handle mutation. We don't mutate.

### On-disk layout

```
~/.criome/store/
  store.bin     append-only data file (all objects)
  store.idx     append-only index (hash→offset, rebuildable from store.bin)
```

### Object format in store.bin

```
byte 0       [blake3_hash: 32]    content address (self-verifying)
byte 32      [kind: u8]           caller-defined type tag
byte 33      [length: u32 LE]     length of payload
byte 37      [padding: 0–15]      align payload to 16 bytes (for rkyv)
byte 37+pad  [payload: length bytes]
```

blake3(payload) == hash. Content-addressing makes writes idempotent.

### API surface

```rust
Store::open(path)                → Result<Store>
Store::put(kind: u8, data: &[u8]) → Result<ContentHash>  // append, skip if exists
Store::get(hash)                 → Result<&[u8]>          // mmap, zero-copy
Store::contains(hash)            → bool                    // index check
Store::rebuild_index()           → Result<()>              // scan store.bin
```

### In-memory index

`HashMap<[u8; 32], (u64, u32)>` — hash → (file_offset, length).
Loaded from store.idx on startup. Rebuilt by scanning store.bin if
store.idx is missing or corrupt. At agent scale (thousands of objects),
fits in a few KB of RAM.

### Crash safety

Append-only. If the last write is interrupted, trailing bytes fail hash
verification — detected on next index rebuild, truncated. No WAL, no
journal, no two-phase commit. The store.bin file only grows.

### Why not keep redb

redb solves the general case: mutable key-value pairs with ordered
traversal and ACID transactions. Our keys are blake3 hashes (point
lookup only), our values are immutable (no updates), and we have one
writer (the agent process). redb's COW B-tree and transaction machinery
are for problems we don't have.

The append-only file matches redb's read performance (both can mmap)
with less mechanism. Dependencies drop from redb+zstd+base64 to just
blake3.

### When this stops working

If the index outgrows RAM (billions of objects) or we need secondary
indexes that can't be in-memory. At single-author agent scale this is
not a concern. Escape hatch: LMDB+rkyv or hash table on pages. The
append file migrates trivially — scan and re-insert.

## VCS

Jujutsu (`jj`) is mandatory. Git is the backend only. Always pass `-m` to
`jj` commands.
