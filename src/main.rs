use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::prelude::*;

mod handler;
use handler::{DirHandler, Handler};

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

async fn handle_connection<H>(
    mut socket: tokio::net::TcpStream,
    handler: &H,
) -> Result<(), Box<dyn std::error::Error>>
where
    H: Handler,
{
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
