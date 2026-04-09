# criome-store

Content-addressed persistence for sema worlds. blake3 hash → bytes.
The substrate that sema objects, arbor chunks, and all media live on.

## What It Stores

Everything that has bytes and needs persistence:
- Arbor prolly tree chunks (sema objects, index nodes)
- String content (transitional — until sema enumerates it)
- Media (images, audio, video — typed sema compositions eventually)
- Manifests and commits (content-addressed version history)

Content-addressing makes writes idempotent and deduplication automatic.
The same bytes always produce the same hash. Structural sharing between
versions is free.

## Two Eras

**String era (current):** string fields in sema objects are blake3 hashes
pointing to text blobs stored here. Bodies, descriptions, URLs.

**Domain era (target):** as sema enumerates meaning into typed domain
compositions, string content shrinks toward zero. The store evolves to
hold typed domain trees and media — not text. Text becomes a projection
rendered by per-language translation tables. The store persists what
sema can't enumerate yet.

## Target: Append-Only File Store

Content-addressed records are write-once. No updates, no deletes, no
ordering, no transactions. All the machinery of B-trees, WAL, MVCC
exists to handle mutation. We don't mutate.

### On-disk layout

```
~/.criome/store/
  store.bin     append-only data file (all objects)
  store.idx     hash→offset index (rebuildable from store.bin)
```

### Object format in store.bin

```
byte 0       [blake3_hash: 32]    content address (self-verifying)
byte 32      [kind: u8]           caller-defined type tag
byte 33      [length: u32 LE]     length of payload
byte 37      [padding: 0–15]      align payload to 16 bytes (for rkyv)
byte 37+pad  [payload: length bytes]
```

### API surface

```rust
Store::open(path)                  → Result<Store>
Store::put(kind: u8, data: &[u8]) → Result<ContentHash>
Store::get(hash)                   → Result<&[u8]>
Store::contains(hash)              → bool
Store::rebuild_index()             → Result<()>
```

### Crash safety

Append-only. Interrupted writes detected by hash verification on next
index rebuild. No WAL, no journal. The store.bin file only grows.
GC (future): walk live roots, mark reachable, compact to new file.

## VCS

Jujutsu (`jj`) is mandatory. Always pass `-m`.
