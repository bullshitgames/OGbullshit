use tokio::net::TcpListener;    
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};   
use tokio::net::tcp::WriteHalf;
use tokio::sync::broadcast::{self, Sender};
use tokio::task::JoinHandle; 
use std::net::SocketAddr; 

use crate::filehandler::FileHandler;

pub trait Server{
    fn start();
    fn shutdown();
}

pub struct ChatServer{
    listener : TcpListener,
    welcome_message : String,
    
    client_tx : Sender<(String, SocketAddr)>,

    server_tx : Sender<String>,

    _alive : bool,

    //join_handles : Vec<JoinHandle<()>>,

    log_path : String,

}





impl ChatServer{

    pub async fn new(addr : &str) -> ChatServer{
        let listener = TcpListener::bind(addr).await
            .expect("Cannot ");
        
        let (c_tx, _) = broadcast::channel::<(String, SocketAddr)>(10);
        let (s_tx, _) = broadcast::channel::<String>(10);
        
        let mut new_server = ChatServer{

            welcome_message : "Welcome to the OG BS prototype chat server!\n\r".to_string(),
            listener : listener,

            client_tx : c_tx,
            server_tx : s_tx,

            _alive : false,
            log_path: "~/.server_logs/log.txt".to_string(),

            //join_handles : Vec::new(),
        };

        let path = new_server.log_path.clone();

        new_server
    }


    pub async fn run(&mut self){

        tokio::spawn( async move {
            ChatServer::get_admin_commands()
        });

        self._alive = true;
        self.server_loop().await;

    }

    async fn get_admin_commands(){

    }

    async fn server_loop(&self) {
        loop {
            let (mut socket, _addr) = self.listener.accept().await.unwrap();
    
            let tx = self.client_tx.clone();
            let mut rx = tx.subscribe();
    
            let sv_tx = self.server_tx.clone();
            let mut sv_rx = sv_tx.subscribe();
    
            let announcement = format!("{}", _addr.ip().to_string() + " has joined us!\n\r");
            sv_tx.send(announcement.clone()).unwrap();
            print!("{}", announcement);

            let welcome_message = self.welcome_message.clone();

            tokio::spawn(async move {
                let (reader_half, mut writer_half) = socket.split();
                writer_half.write_all(welcome_message.as_bytes()).await.unwrap();
    
                let mut reader = BufReader::new(reader_half);
                let mut line = String::new();
        
                loop {
                    tokio::select!{
                        bytes_read = reader.read_line(&mut line) => {
                            if bytes_read.unwrap() == 0 || line.trim_end() == "QUIT" { break; }
    
                            ChatServer::handle_client_input(&mut line, &tx, _addr);
                        }
    
                        msg = rx.recv() => {
                            let (mes , other_addr) = msg.unwrap();
    
                            ChatServer::handle_broadcast_recv(_addr, other_addr, mes, &mut writer_half).await;
                        }
    
                        msg = sv_rx.recv() => {
                            let msg = msg.unwrap();
    
                            ChatServer::handle_announcement_recv(_addr, msg, &mut writer_half).await;
                        }
                    }
                    
                }
    
                let announcement = format!("{}", _addr.ip().to_string() + " has left!\n\r");
                sv_tx.send(announcement.clone()).unwrap();
                print!("{}", announcement);
            });
    
        }
    }


    fn handle_client_input(line: &mut String, tx: &Sender<(String, SocketAddr)>, _addr : SocketAddr){
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

}