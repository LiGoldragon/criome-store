# lojix-store

Content-addressed filesystem — a blake3-hashed analogue to the
nix-store. Holds real unix files and directory trees; a
separate index DB tracks `hash → path + metadata + reachability`.

## Role in the sema ecosystem

Per `mentci-next/docs/architecture.md §3`:

- **sema** (records DB, redb-backed) holds logical records —
  owned by criomed.
- **lojix-store** (this repo) holds **opaque files** —
  compiled binary trees, user attachments — owned by lojixd.
  Any sema record that references big or unstructured payloads
  stores a blake3 hash; lojixd resolves the hash to a
  filesystem path.

A compiled Rust binary lives at a hash-derived path like
`~/.lojix/store/<hash>/bin/<name>` and is directly executable.
This is why architecture.md §8 says "a binary is just a path"
and why there's no `Launch` protocol verb.

## Status

**CANON-MISSING-IMPLEMENTATION.** The repo exists (renamed from
`criome-store` on 2026-04-24); the code inside is an abandoned
byte-map prototype (`MemoryStore` over `HashMap<Hash, Vec<u8>>`)
that does **not** match the filesystem architecture.

**The prototype will be replaced**, not extended. Whoever
implements lojix-store for real starts from:

- `Cargo.toml` + directory layout as scaffolding.
- `flake.nix` for dev env.
- *Not* `source/store.aski` — that's from a superseded design.
- *Not* `ChunkStore`-style byte-map APIs — those don't suit a
  filesystem store.

## Implementation direction

When lojix-store gets a real implementation (per
`mentci-next/reports/030` Phase C onwards — after lojixd
scaffolds and needs a place to write artifacts):

- **Directory layout**: `~/.lojix/store/<hash>/…` — one
  hash-keyed subdirectory per store entry. Subdirectory can
  hold a single file or a tree.
- **Index DB**: `~/.lojix/store/index.redb` (or similar) —
  maps `blake3 → { path_within_store, byte_len, stored_at,
  reachability_state }`.
- **Writes**: in-process only (lojixd owns writes). Reader
  library mmap-friendly.
- **Type**: unknown at this layer — every entry is opaque
  bytes-on-disk. Type is carried by the sema record that
  references the entry's hash (no kind bytes, per
  `mentci-next/reports/017 §3`).
- **Access control**: capability tokens, signed by criomed.

## Not in scope

- Compression. Files live as-is.
- Deduplication beyond content-hash (already given by blake3
  identity).
- Versioning. Post-MVP question.
- Distributed replication. Single-host for MVP.

## Heritage

Renamed from `criome-store` on 2026-04-24. The old repo's
concept (single universal blob store for everything) split
into `sema` (records) + `lojix-store` (files). See
`mentci-next/reports/037 §3` for the naming decision.
Repository was renamed on GitHub; git redirects the old URL.

## VCS

Jujutsu (`jj`) is mandatory. Always pass `-m`.
