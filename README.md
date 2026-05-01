# arca

Content-addressed filesystem — a blake3-hashed analogue to the
[nix-store](https://nix.dev/manual/nix/stable/store/). Holds
real unix files and directory trees under hash-derived paths;
a per-store redb index DB tracks
`hash → { path, metadata, reachability }`.

**One library + one daemon.** The library (`arca`) is the
public reader API + on-disk layout types. The daemon
(`arca-daemon`) is the privileged writer — owns a write-only
staging directory (`~/.arca/_staging/`), manages multiple
stores under `~/.arca/<store>/`, verifies criome-signed
capability tokens, computes blake3 of staged content,
atomically moves deposits into the target store, updates the
per-store redb index.

General-purpose: any data that doesn't fit in sema's record
shape lives in arca.
`forge` is the most
active writer today; future writers (uploads, document store,
others) earn the deposit capability the same way.

See `ARCHITECTURE.md`. Project-wide
context:
criome/ARCHITECTURE.md.

## Status

**Skeleton-as-design.** Hash + layout helpers have real
implementations and tests; reader / writer / bundle / index /
deposit / token / arca-daemon bodies are `todo!()`. Real fills
land alongside forge scaffolding.

## License

[License of Non-Authority](LICENSE.md).
