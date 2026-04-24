# criome-store

> **⚠ Context: renamed in the canonical architecture**
>
> Per `mentci-next/docs/architecture.md` and
> `mentci-next/reports/019`, the MVP splits this slot into two:
>
> - **sema** — records database (redb-backed; logical code
>   records, owned by criomed).
> - **lojix-store** — content-addressed filesystem (nix-store
>   analogue; hash-keyed directory of real unix files and
>   directory trees; separate index DB for metadata;
>   owned by lojixd).
>
> This repo is the **predecessor** of `lojix-store` and still
> contains a `MemoryStore` + `ChunkStore` prototype that
> operates on opaque byte arrays. That API does NOT match the
> canonical architecture: lojix-store holds real *files*, not
> byte arrays. The prototype survives only for tests; a fresh
> `lojix-store` implementation will look much more like nix's
> daemon-and-store combo than like the byte-map below.

Earlier framing (kept for historical reference):

A content-addressed store for the lojix family. Under the
canonical architecture, its job is to hold compiled binaries
and user file attachments as *real unix files* under hash-
derived paths — a blake3-hashed analogue to `/nix/store/`.

## Planned shape (renaming to lojix-store)

**Hash-keyed filesystem directory + separate index DB.** The
filesystem layer holds real files; the index DB maps
`blake3 → path + metadata + reachability`. A compiled binary
tree lives at e.g. `~/.lojix/store/<hash>/bin/<name>` and is
directly executable — the same reason architecture.md §8 says
"A binary is just a path."

No kind bytes (per `mentci-next/reports/017`): blob types are
known only through the sema records that reference them.

## Current Implementation (prototype)

`MemoryStore` — in-memory `HashMap<ContentHash, (u8, Vec<u8>)>`. 9 tests.
Good for development; does not reflect the terminal filesystem
architecture. When lojix-store gets a real implementation it
will not carry this shape forward unchanged.

## Target: nix-store-like filesystem

```
~/.lojix/store/<hash>/…                        # hash-keyed tree of real files
~/.lojix/store/index.redb  (or similar)        # path + metadata + reachability
```

Directory renames from `~/.criome/store/` to `~/.lojix/store/`
when the lojix-store repo consolidates.

## Dependency on arbor

Shelved for MVP. When arbor returns (post-self-hosting), the
`ChunkStore` trait is how it plugs in.

## VCS

Jujutsu (`jj`) is mandatory. Always pass `-m`.
