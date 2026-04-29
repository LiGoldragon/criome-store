//! Write-only staging + atomic-move-to-store.
//!
//! Writers (forge being the most active today) deposit content
//! into `~/.arca/_staging/<deposit-id>/`. The staging directory
//! is **write-only** to writers — once content is dropped in,
//! the writer cannot read or modify it. arca-daemon owns the
//! staging directory; only it has read+move permission.
//!
//! Per-deposit flow:
//!
//! 1. Writer asks arca-daemon for a `StagingId` (or mints one
//!    locally per the chosen mechanism).
//! 2. Writer atomically writes the canonicalised tree under
//!    `~/.arca/_staging/<deposit-id>/`.
//! 3. Writer sends `signal-arca::Deposit { staging_id,
//!    target_store, capability_token }` to arca-daemon.
//! 4. arca-daemon verifies the capability token (see
//!    [`crate::token`]), computes the blake3 of the staged
//!    tree (the hash of exactly what's there — no TOCTOU race
//!    because the writer can't modify after deposit),
//!    atomically moves the tree into the target store at
//!    `~/.arca/<store>/<blake3>/`, updates the per-store redb
//!    index, and replies with `DepositOk { blake3 }`.
//!
//! The exact write-only mechanism (chmod 1733 + per-deposit
//! subdirs / SCM_RIGHTS over UDS / Linux namespace isolation)
//! is open until arca-daemon's body lands.

use crate::hash::StoreEntryHash;
use crate::Result;

/// Unique identifier for one deposit-in-flight under
/// `~/.arca/_staging/`. Uniqueness is per-arca-daemon-instance.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct StagingId(pub [u8; 16]);

/// Identifier of an arca store (e.g. `system`, `user-foo`,
/// `project-bar`). Maps to a directory under `~/.arca/`.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StoreId(pub String);

/// One deposit's lifecycle, owned by arca-daemon.
pub trait Deposit {
    /// Compute the canonical blake3 of the staged tree at
    /// `staging_id`, atomically move it into `store` at
    /// `<store>/<blake3>/`, update the per-store index, and
    /// return the canonical hash.
    fn finalize(
        &mut self,
        staging_id: StagingId,
        store: StoreId,
    ) -> Result<StoreEntryHash>;

    /// Discard a staged deposit (writer cancelled or token
    /// rejected).
    fn discard(&mut self, staging_id: StagingId) -> Result<()>;
}
