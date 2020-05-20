use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::prelude::*;

use structopt::StructOpt;

mod handler;

use handler::{DirHandler, Handler};

#[derive(Debug, StructOpt)]
#[structopt(name = "bodem", about = "Simple gopher server")]
struct Args {
    #[structopt(
        name = "listen",
        short,
        long,
        default_value = "127.0.0.1:7070",
        env = "BODEM_LISTEN"
    )]
    listen_addr: String,
    #[structopt(
        name = "root",
        help = "Directory to serve",
        default_value = "./",
        env = "BODEM_ROOT"
    )]
    root: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();
    let mut listener = TcpListener::bind(&args.listen_addr).await?;
    let handler = DirHandler::new(args.root)?;

    println!("Accepting connections on {}", args.listen_addr);
    loop {
        let (socket, addr) = listener.accept().await?;
        match handle_connection(socket, &handler).await {
            Ok(_) => {}
            Err(error) => {
                dbg!(error, addr);
            }
        };
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
    let response = handler.handle(path).await?;
    socket.write_all(&response).await?;

    Ok(())
}
