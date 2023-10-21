use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, _add) = listener.accept().await.unwrap();
    let mut buffer = [0u8; 1024];
    let bytes = socket.read(&mut buffer).await.unwrap();
    socket.write_all(&buffer[..bytes]).await.unwrap();
}
