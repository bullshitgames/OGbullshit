use tokio::net::TcpListener;    
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};   
use tokio::sync::broadcast; 
use std::net::SocketAddr; 

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("192.168.1.10:7888").await.unwrap();
    let welcome_msg = "Welcome to the OG BS prototype chat server.\n\r";
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);   

    loop {
        
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader_half, mut writer_half) = socket.split();
            
            writer_half.write_all(welcome_msg.as_bytes()).await.unwrap();

            let mut reader = BufReader::new(reader_half);
            let mut line = String::new();
    
            loop {
                tokio::select!{
                    read = reader.read_line(&mut line) => {
                        if read.unwrap() == 0 { break; }

                        tx.send( (line.clone(), _addr) ).unwrap();
                        print!("{}", _addr.ip().to_string() + ":" + &_addr.port().to_string() + "> " + &line);
                        line.clear();
                    }
                    
                    msg = rx.recv() => {
                        let (mut mes , other_addr) = msg.unwrap();

                        if other_addr != _addr {
                            mes = other_addr.ip().to_string() + ":" + &other_addr.port().to_string() + "> " + &mes;
                            writer_half.write_all(mes.as_bytes()).await.unwrap();
                        }
                    }
                }
                
            }
        });

    }
}
