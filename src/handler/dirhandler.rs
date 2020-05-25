use async_trait::async_trait;
use snafu::ResultExt;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::prelude::*;

use super::*;

pub struct DirHandler {
    root: PathBuf,
    host: String,
    port: u16,
}

impl DirHandler {
    pub fn new<P, S>(root: P, host: S, port: u16) -> Result<Self>
    where
        P: Into<PathBuf> + Clone,
        S: Into<String>,
    {
        let root_debug = root.clone();
        Ok(Self {
            root: root
                .into()
                .canonicalize()
                .context(PathLookup { path: root_debug })?,
            host: host.into(),
            port: port,
        })
    }

    fn list_dir<P>(&self, path: P) -> Result<String>
    where
        P: AsRef<std::path::Path>,
    {
        let full_path = &self.root.join(path.as_ref());
        let dir = full_path.read_dir().context(ReadDir {
            path: full_path.clone(),
        })?;
        let paths = dir.filter_map(|entry| entry.ok()).map(|entry| entry.path());

        let mut result = String::new();
        for full_path in paths {
            let user_path = full_path.strip_prefix(&self.root).context(ValidatePath)?;
            let file_name = user_path.file_name().unwrap_or(user_path.as_ref());
            let file_type = if full_path.is_dir() { 1 } else { 0 };
            let entry_line = format!(
                "{}{}\t{}\t{}\t{}\r\n",
                file_type, // The file type indicator according to the gopher spec.
                file_name.to_string_lossy(), // path to the file
                user_path.to_string_lossy(), // The 'user presentation' of the file (e.g. a name)
                self.host, // hostname where the file can be retrieved
                self.port, // port on the host where the file can be retrieved
            );
            result.push_str(&entry_line);
        }
        Ok(result)
    }
}

#[async_trait]
impl Handler for DirHandler {
    async fn handle(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = self
            .root
            .join(path)
            .canonicalize()
            .context(PathLookup { path: path })?;
        if !full_path.starts_with(&self.root) {
            return Err(Error::RootEscape);
        }

        if full_path.is_dir() {
            Ok(self.list_dir(path)?.into_bytes())
        } else {
            let mut response: Vec<u8> = vec![];
            let mut file = File::open(&full_path).await.context(OpenFile {
                path: full_path.clone(),
            })?;
            file.read_to_end(&mut response).await.context(ReadFile {
                path: full_path.clone(),
            })?;
            Ok(response)
        }
    }
}
