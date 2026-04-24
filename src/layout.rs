//! Filesystem layout conventions for a lojix-store directory.
//!
//! Default root: `$HOME/.lojix/store/`.
//!
//! Layout:
//!
//! ```text
//! ~/.lojix/store/
//!   <hex-hash>/                  # one subdirectory per entry
//!     bin/<name>                 # executables (rpath into sibling /lib)
//!     lib/<libX>.so              # shared libs (rpath absolute into lojix-store)
//!     share/...                  # data files
//!   index.redb                   # hash → { path, metadata, reachability }
//! ```
//!
//! Paths inside an entry are normal unix; the entry as a whole
//! is addressed by its blake3. Cross-entry RPATHs use absolute
//! paths into the store, so artifacts work regardless of cwd.

use std::path::{Path, PathBuf};

use crate::hash::StoreEntryHash;

/// Root of a lojix-store directory.
#[derive(Clone, Debug)]
pub struct StoreRoot(pub PathBuf);

impl StoreRoot {
    /// The default root: `$HOME/.lojix/store/`.
    pub fn default_for_user() -> Self {
        todo!()
    }

    /// Path to the subdirectory that holds a given entry's tree.
    pub fn entry_tree(&self, _hash: StoreEntryHash) -> PathBuf {
        todo!()
    }

    /// Path to the index DB file.
    pub fn index_db_path(&self) -> PathBuf {
        self.0.join("index.redb")
    }

    /// Does this store root exist and look valid?
    pub fn exists(&self) -> bool {
        todo!()
    }
}

/// A resolved filesystem path inside the store (entry root, bin,
/// lib, or leaf). Kept distinct from bare `PathBuf` so the type
/// surface distinguishes store-resolved paths from arbitrary ones.
#[derive(Clone, Debug)]
pub struct StorePath(pub PathBuf);

impl StorePath {
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}
