use async_trait::async_trait;

use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Make listen address and root configurable.
    let mut listener = TcpListener::bind("127.0.0.1:7070").await?;
    let handler = DirHandler::new("/tmp");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Incoming from {}!", addr);
        handle_connection(socket, &handler).await?;
    }
}

async fn handle_connection<H>(mut socket: tokio::net::TcpStream, handler: &H) -> Result<(), Box<dyn std::error::Error>> where H: Handler {
    let mut buf = vec![];
    let (reader, _writer) = socket.split();

    let mut buf_reader = BufReader::new(reader);
    buf_reader.read_until(b'\n', &mut buf).await?;

    let path = String::from_utf8_lossy(&buf);
    let path = path.trim_end();
    dbg!("requested path: {}", path);
    let response = handler.handle(path).await;
    socket.write_all(&response).await?;

    Ok(())
}

#[async_trait]
pub trait Handler {
    // TODO: Return `Result` and write a `Handler` that wraps a `Handler` and does pretty errors.
    // TODO: Use a prettier type than `&str`.
    async fn handle(&self, req: &str) -> Vec<u8>;
}

pub struct DirHandler {
    root: std::path::PathBuf,
}

impl DirHandler {
    pub fn new<P>(root: P) -> Self where P: Into<std::path::PathBuf> {
        Self {
            root: root.into()
        }
    }
}

#[async_trait]
impl Handler for DirHandler {
    async fn handle(&self, path: &str) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];

        // TODO: check if the full_path is really a child of our `root` (e.g. to prevent
        // `../` escapes).
        let full_path = self.root.join(path);

        if full_path.is_dir() {
            let dir = match full_path.read_dir() {
                Ok(dir) => dir,
                Err(_e) => { return vec![b'3', b'\t', b'e', b'r', b'r', b'o', b'r', b'\r', b'\n'] }
            };
            for p in dir {
                if let Ok(p) = p {
                    let path = p.path();
                    let path = path.strip_prefix(&self.root).unwrap().to_string_lossy();
                    let entry = format!("{}\t{}\r\n",
                        if p.metadata().unwrap().is_dir() { 1 } else { 0 },
                        path);

                    response.extend_from_slice(entry.as_bytes());
                }
            }
        } else {
            let mut file = match File::open(full_path).await {
                Ok(f) => f,
                Err(_e) => { return vec![b'3', b'\t', b'e', b'r', b'r', b'o', b'r', b'\r', b'\n'] },
            };
            match file.read_to_end(&mut response).await {
                Ok(_) => {},
                Err(_e) => { return vec![b'3', b'\t', b'e', b'r', b'r', b'o', b'r', b'\r', b'\n'] },
            }
        }
        response
    }
}
