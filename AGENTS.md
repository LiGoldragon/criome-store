# Agent instructions — arca

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

## Repo role

Content-addressed filesystem — blake3-hashed analogue to the nix-store. **One library + one daemon**: the `arca` library is the public reader API + on-disk layout; `arca-daemon` is the privileged writer (write-only staging, multi-store, capability-token-gated).

Skeleton-as-design today; bodies are `todo!()`.

## Carve-outs worth knowing

- Store-entry identity is `StoreEntryHash` (blake3 of canonical tree encoding); paths are typed (`StorePath` vs bare `PathBuf`).
- `StoreReader` (public) and `StoreWriter` (arca-daemon-only) split the reader/writer authority. Writes carry a criome-signed capability token referencing a sema access-control record + target store.
- `BundlePolicy` (in `bundle.rs`) makes determinism controls explicit — `normalise_timestamps`, `strip_build_id`, `rewrite_rpath`.
- The skeleton **is** the design doc — modifying the interface means modifying this code (per skeleton-as-design).
