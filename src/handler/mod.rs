use async_trait::async_trait;
use snafu::{ResultExt, Snafu};
use std::path::PathBuf;

mod dirhandler;
pub use dirhandler::DirHandler;

#[derive(Debug, Snafu)]
pub enum Error {
    ReadDir {
        source: std::io::Error,
        path: PathBuf,
    },
    OpenFile {
        source: std::io::Error,
        path: PathBuf,
    },
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },
    ValidatePath {
        source: std::path::StripPrefixError,
    },
    ReadFileMetadata {
        source: std::io::Error,
    },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait Handler {
    // TODO: Use a prettier type than `&str`.
    async fn handle(&self, req: &str) -> Result<Vec<u8>>;
}
