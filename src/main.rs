use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    print!("We connected");
    let (mut socket, _add) = listener.accept().await.unwrap();
    let(read, mut write) = socket.split();
    let mut reader = BufReader::new(read);
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut  line).await.unwrap();
        if bytes_read == 0 { break }
        write.write_all(line.as_bytes()).await.unwrap();
        line.clear()
    }
}
