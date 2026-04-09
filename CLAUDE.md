# criome-store

The universal content-addressed store for the sema ecosystem. Every
object — strings, sema objects, arbor tree nodes, manifests, commits —
lives here, sorted by kind, addressed by blake3 hash.

## Dependency

criome-store depends on arbor. It implements arbor's `ChunkStore` trait,
bridging arbor trees to the content-addressed store. arbor chunks get
`KIND_ARBOR_NODE` (0xA0) as their type tag.

## Two Traits

**`Store`** — the typed layer. `put(kind, data) → hash`, `get(hash) → bytes`,
`get_typed(hash) → (kind, bytes)`, `scan(kind) → entries`. The `kind` byte
sorts objects into typed namespaces.

**`ChunkStore`** (from arbor) — the raw layer. `put(hash, bytes)`,
`get(hash) → bytes`, `contains(hash)`. arbor trees use this interface
without knowing about kinds. criome-store's `MemoryStore` implements both.

## Kind Bytes

```
0x00..0x0F    strings (transitional — until sema enumerates them)
0x10..0x1F    sema objects per struct type
0xA0          arbor tree nodes
0xF0          manifests
0xF1          commits
```

## Current Implementation

`MemoryStore` — in-memory `HashMap<ContentHash, (u8, Vec<u8>)>`. 9 tests.

## Target: Append-Only File Store

```
~/.criome/store/
  store.bin     append-only data file (all objects)
  store.idx     hash→(offset, length, kind) cache (rebuildable)
```

Not yet built. `MemoryStore` is sufficient for development and testing.
`FileStore` drops in behind the same `Store` + `ChunkStore` traits.

## VCS

Jujutsu (`jj`) is mandatory. Always pass `-m`.
