use crate::ContentHash;

#[derive(Debug)]
pub enum Error {
    NotFound(ContentHash),
    Io(std::io::Error),
    HashMismatch { expected: ContentHash, actual: ContentHash },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(h) => write!(f, "chunk not found: {}", hex(h)),
            Error::Io(e) => write!(f, "io: {e}"),
            Error::HashMismatch { expected, actual } => {
                write!(f, "hash mismatch: expected {} got {}", hex(expected), hex(actual))
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

fn hex(h: &ContentHash) -> String {
    h.iter().map(|b| format!("{b:02x}")).collect()
}
