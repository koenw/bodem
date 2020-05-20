use async_trait::async_trait;

use tokio::fs::File;
use tokio::prelude::*;

use std::path::PathBuf;

use snafu::{ResultExt, Snafu};

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

pub struct DirHandler {
    root: PathBuf,
}

impl DirHandler {
    pub fn new<P>(root: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self { root: root.into() }
    }
}

#[async_trait]
impl Handler for DirHandler {
    async fn handle(&self, path: &str) -> Result<Vec<u8>> {
        let mut response: Vec<u8> = vec![];

        // TODO: check if the full_path is really a child of our `root` (e.g. to prevent
        // `../` escapes).
        let full_path = self.root.join(path);

        if full_path.is_dir() {
            let dir = full_path.read_dir().context(ReadDir {
                path: full_path.clone(),
            })?;
            for p in dir {
                if let Ok(p) = p {
                    let path = p.path();
                    let path = path
                        .strip_prefix(&self.root)
                        .context(ValidatePath)?
                        .to_string_lossy();
                    let entry = format!(
                        "{}\t{}\r\n",
                        if p.metadata().context(ReadFileMetadata)?.is_dir() {
                            1
                        } else {
                            0
                        },
                        path
                    );

                    response.extend_from_slice(entry.as_bytes());
                }
            }
        } else {
            let mut file = File::open(&full_path).await.context(OpenFile {
                path: full_path.clone(),
            })?;
            file.read_to_end(&mut response).await.context(ReadFile {
                path: full_path.clone(),
            })?;
        }
        Ok(response)
    }
}
