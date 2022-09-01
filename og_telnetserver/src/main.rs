use tokio::net::TcpListener;    
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};   
use tokio::net::tcp::WriteHalf;
use tokio::sync::broadcast::{self, Sender}; 
use std::net::SocketAddr; 

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("192.168.1.10:7888").await.unwrap();
    listener.set_ttl(100).unwrap();
    let welcome_msg = "Welcome to the OG BS prototype chat server!\n\r";
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);   
    let (sv_tx, _sv_rx) = broadcast::channel(10);

    loop {
        
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        let mut sv_rx = sv_tx.subscribe();

        let announcement = format!("{}", _addr.ip().to_string() + " has joined us!");
        sv_tx.send(announcement).unwrap();

        tokio::spawn(async move {
            let (reader_half, mut writer_half) = socket.split();
            writer_half.write_all(welcome_msg.as_bytes()).await.unwrap();

            let mut reader = BufReader::new(reader_half);
            let mut line = String::new();
    
            loop {
                tokio::select!{
                    bytes_read = reader.read_line(&mut line) => {
                        if bytes_read.unwrap() == 0 { break; }

                        handle_client_input(&mut line, &tx, _addr);
                    }

                    msg = rx.recv() => {
                        let (mes , other_addr) = msg.unwrap();

                        handle_broadcast_recv(_addr, other_addr, mes, &mut writer_half).await;
                    }

                    msg = sv_rx.recv() => {
                        let msg = msg.unwrap();

                        handle_announcement_recv(_addr, msg, &mut writer_half).await;
                    }
                }
                
            }
        });

    }
}


pub fn handle_client_input(line: &mut String, tx: &Sender<(String, SocketAddr)>, _addr : SocketAddr){
    tx.send( (line.clone(), _addr) ).unwrap();
    print!("{}", _addr.ip().to_string() + ":" + &_addr.port().to_string() + "> " + &line);
    line.clear();
}


async fn handle_broadcast_recv(_addr : SocketAddr, _recved_addr : SocketAddr, mes : String, writer_half : &mut WriteHalf<'_>) {
    if _recved_addr != _addr {
        let mes = _recved_addr.ip().to_string() + ":" + &_recved_addr.port().to_string() + "> " + &mes;
        writer_half.write_all(mes.as_bytes()).await.unwrap();
    }
}


async fn handle_announcement_recv(_addr : SocketAddr, mes : String, writer_half : &mut WriteHalf<'_>) {
    let mes = "SERVER > ".to_string() + &mes;
    writer_half.write_all(mes.as_bytes()).await.unwrap();
}