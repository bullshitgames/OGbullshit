use tokio::net::TcpListener;    
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};    

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();


    loop {
        
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let (reader_half, mut writer_half) = socket.split();
    
        let mut reader = BufReader::new(reader_half);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await.unwrap();

            if bytes_read == 0 { break }

            writer_half.write_all(line.as_bytes()).await.unwrap();
            line.clear();
        }
    }
}
