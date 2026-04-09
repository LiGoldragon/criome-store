mod store;
mod error;

pub use store::{Store, ContentHash, MemoryStore, content_hash};
pub use error::Error;

// Re-export arbor's ChunkStore — criome-store's MemoryStore implements it.
pub use arbor::ChunkStore;
