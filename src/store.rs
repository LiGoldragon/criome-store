use crate::Error;
use arbor::ChunkStore;

/// blake3 hash — the content address
pub type ContentHash = [u8; 32];

/// Content-addressed store. Immutable: same bytes always produce the same
/// hash. Writes are idempotent. Values are never updated or deleted.
///
/// The `kind` byte sorts objects into typed namespaces:
/// strings, sema objects per struct type, arbor nodes, manifests, commits.
/// All live in one store, content-addressed within each kind.
pub trait Store {
    /// Store bytes, return their content hash. Skips if already present.
    fn put(&mut self, kind: u8, data: &[u8]) -> Result<ContentHash, Error>;

    /// Retrieve bytes by content hash.
    fn get(&self, hash: &ContentHash) -> Result<&[u8], Error>;

    /// Retrieve bytes and kind by content hash.
    fn get_typed(&self, hash: &ContentHash) -> Result<(u8, &[u8]), Error>;

    /// Check if a hash exists without loading the data.
    fn contains(&self, hash: &ContentHash) -> bool;

    /// Iterate all entries of a given kind.
    fn scan(&self, kind: u8) -> Vec<(ContentHash, &[u8])>;
}

/// Hash bytes with blake3, returning the content address.
pub fn content_hash(data: &[u8]) -> ContentHash {
    *blake3::hash(data).as_bytes()
}

/// In-memory store for tests and bootstrap.
pub struct MemoryStore {
    /// hash → (kind, data)
    entries: std::collections::HashMap<ContentHash, (u8, Vec<u8>)>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Store for MemoryStore {
    fn put(&mut self, kind: u8, data: &[u8]) -> Result<ContentHash, Error> {
        let hash = content_hash(data);
        self.entries
            .entry(hash)
            .or_insert_with(|| (kind, data.to_vec()));
        Ok(hash)
    }

    fn get(&self, hash: &ContentHash) -> Result<&[u8], Error> {
        self.entries
            .get(hash)
            .map(|(_, data)| data.as_slice())
            .ok_or(Error::NotFound(*hash))
    }

    fn get_typed(&self, hash: &ContentHash) -> Result<(u8, &[u8]), Error> {
        self.entries
            .get(hash)
            .map(|(kind, data)| (*kind, data.as_slice()))
            .ok_or(Error::NotFound(*hash))
    }

    fn contains(&self, hash: &ContentHash) -> bool {
        self.entries.contains_key(hash)
    }

    fn scan(&self, kind: u8) -> Vec<(ContentHash, &[u8])> {
        self.entries
            .iter()
            .filter(|(_, (k, _))| *k == kind)
            .map(|(hash, (_, data))| (*hash, data.as_slice()))
            .collect()
    }
}

/// Kind byte for arbor tree node chunks.
pub const KIND_ARBOR_NODE: u8 = 0xA0;

/// arbor's ChunkStore implemented over criome-store's MemoryStore.
/// Arbor chunks are stored with KIND_ARBOR_NODE as the type tag.
impl ChunkStore for MemoryStore {
    fn get(&self, hash: &arbor::Hash) -> Option<Vec<u8>> {
        self.entries.get(hash).map(|(_, data)| data.clone())
    }

    fn put(&mut self, hash: arbor::Hash, data: Vec<u8>) {
        self.entries
            .entry(hash)
            .or_insert_with(|| (KIND_ARBOR_NODE, data));
    }

    fn contains(&self, hash: &arbor::Hash) -> bool {
        self.entries.contains_key(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(s: &mut MemoryStore, kind: u8, data: &[u8]) -> ContentHash {
        Store::put(s, kind, data).unwrap()
    }

    fn get<'a>(s: &'a MemoryStore, hash: &ContentHash) -> &'a [u8] {
        Store::get(s, hash).unwrap()
    }

    #[test]
    fn put_get_round_trip() {
        let mut store = MemoryStore::new();
        let hash = put(&mut store, 0, b"hello world");
        assert_eq!(get(&store, &hash), b"hello world");
    }

    #[test]
    fn content_addressing_is_deterministic() {
        let mut store = MemoryStore::new();
        let h1 = put(&mut store, 0, b"same bytes");
        let h2 = put(&mut store, 0, b"same bytes");
        assert_eq!(h1, h2);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn different_content_different_hash() {
        let mut store = MemoryStore::new();
        let h1 = put(&mut store, 0, b"alpha");
        let h2 = put(&mut store, 0, b"beta");
        assert_ne!(h1, h2);
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn not_found() {
        let store = MemoryStore::new();
        let hash = content_hash(b"missing");
        assert!(!Store::contains(&store, &hash));
        assert!(Store::get(&store, &hash).is_err());
    }

    #[test]
    fn idempotent_put() {
        let mut store = MemoryStore::new();
        let h1 = put(&mut store, 0, b"data");
        let h2 = put(&mut store, 1, b"data");
        assert_eq!(h1, h2);
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn hash_matches_blake3() {
        let expected = *blake3::hash(b"verify hash").as_bytes();
        assert_eq!(expected, content_hash(b"verify hash"));
    }

    #[test]
    fn get_typed_returns_kind() {
        let mut store = MemoryStore::new();
        let hash = put(&mut store, 7, b"typed data");
        let (kind, data) = Store::get_typed(&store, &hash).unwrap();
        assert_eq!(kind, 7);
        assert_eq!(data, b"typed data");
    }

    #[test]
    fn arbor_chunk_store_compat() {
        let mut store = MemoryStore::new();

        // Put via criome-store Store trait
        let hash = put(&mut store, 0, b"string data");

        // Get via arbor ChunkStore trait
        let data = ChunkStore::get(&store, &hash).unwrap();
        assert_eq!(data, b"string data");

        // Put via arbor ChunkStore trait
        let arbor_hash = content_hash(b"tree node");
        ChunkStore::put(&mut store, arbor_hash, b"tree node".to_vec());

        // Verify it got KIND_ARBOR_NODE
        let (kind, _) = Store::get_typed(&store, &arbor_hash).unwrap();
        assert_eq!(kind, KIND_ARBOR_NODE);

        assert_eq!(store.len(), 2);
    }

    #[test]
    fn scan_filters_by_kind() {
        let mut store = MemoryStore::new();
        put(&mut store, 1, b"thought alpha");
        put(&mut store, 1, b"thought beta");
        put(&mut store, 2, b"rule gamma");
        put(&mut store, 0, b"string delta");

        assert_eq!(store.scan(1).len(), 2);
        assert_eq!(store.scan(2).len(), 1);
        assert_eq!(store.scan(0).len(), 1);
        assert!(store.scan(99).is_empty());
    }
}
