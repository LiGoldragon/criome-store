//! Capability-token verification.
//!
//! Every write into arca (deposit, GC, store-entry delete)
//! requires a **criome-signed capability token**. Tokens
//! reference a sema authz record, the target store, the set of
//! permitted operations, and a validity window. arca-daemon
//! verifies the signature against criome's well-known public
//! key — no round-trip back to criome at verification time.
//!
//! Token shape (skeleton; exact fields land with the auth crate
//! in signal):
//!
//! ```text
//! CapabilityToken {
//!     authz_record: Slot,        // sema record describing the grant
//!     target_store: StoreId,     // which arca store this authorises
//!     operations: u32,           // bitmask: deposit / delete / gc
//!     issued_at: u64,            // unix epoch
//!     expires_at: u64,           // unix epoch
//!     issuer_pubkey_id: [u8; 8], // which criome key signed
//!     signature: [u8; 96],       // BLS G1
//! }
//! ```
//!
//! The exact crypto + serialisation lives in
//! [`signal::auth`](https://github.com/LiGoldragon/signal/blob/main/src/auth.rs);
//! arca-daemon re-uses that machinery rather than re-implementing.

use crate::deposit::StoreId;
use crate::Result;

/// A criome-signed capability token, as it arrives in a
/// `signal-arca::Deposit` payload. Opaque to writers; verified
/// by arca-daemon.
#[derive(Clone, Debug)]
pub struct CapabilityToken {
    /// rkyv-serialised token bytes; verified at receive time.
    pub bytes: Vec<u8>,
}

/// Verifier — owns criome's public key(s) and validates incoming
/// tokens.
pub trait TokenVerifier {
    /// Verify a token and return the (target store, permitted
    /// operations) it authorises. Errors on bad signature,
    /// expired window, or unknown issuer key.
    fn verify(
        &self,
        token: &CapabilityToken,
    ) -> Result<VerifiedCapability>;
}

/// Output of successful token verification.
#[derive(Clone, Debug)]
pub struct VerifiedCapability {
    pub target_store: StoreId,
    pub operations: u32,
    pub expires_at: u64,
}
