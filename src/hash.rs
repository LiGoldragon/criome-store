//! Content-hash type used to identify store entries.
//!
//! A `StoreEntryHash` is the blake3 hash of the canonical
//! encoding of a store entry's tree contents. Canonical
//! encoding is deterministic (sorted filenames, normalised
//! timestamps, stable RPATHs); two identical trees hash to
//! the same value regardless of machine or build order.

/// blake3 output width, in bytes.
pub const HASH_LEN: usize = 32;

/// Identity of a store entry: the blake3 of its canonical tree.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StoreEntryHash(pub [u8; HASH_LEN]);

impl StoreEntryHash {
    /// Render as lowercase hex (the on-disk directory-name form).
    pub fn to_hex(&self) -> String {
        todo!("hex-encode {:?}", self.0)
    }

    /// Parse from hex.
    pub fn from_hex(_hex: &str) -> Result<Self, HashParseError> {
        todo!()
    }
}

impl From<blake3::Hash> for StoreEntryHash {
    fn from(h: blake3::Hash) -> Self {
        Self(*h.as_bytes())
    }
}

/// Errors parsing a hex-encoded hash.
#[derive(Debug, thiserror::Error)]
pub enum HashParseError {
    #[error("hex decode failed")]
    InvalidHex,
    #[error("wrong length; expected {HASH_LEN} bytes")]
    WrongLength,
}
