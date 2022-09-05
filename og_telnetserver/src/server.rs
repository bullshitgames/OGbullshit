use std::net::SocketAddr;

use tokio::net::TcpListener;    
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};   
use tokio::net::tcp::WriteHalf;
use tokio::sync::broadcast::{self};

use crate::logger::{WatchLogger};

pub trait Server{
    fn start();
    fn shutdown();
}

#[derive(Clone)]
struct ChatServerCommunicators{
    pub client_tx : broadcast::Sender<(String, SocketAddr)>,
    pub server_tx : broadcast::Sender<String>,

    pub logger_tx : broadcast::Sender<String>,
}

pub struct ChatServer{
    listener : TcpListener,
    welcome_message : String,
    
    _communicator : ChatServerCommunicators,
    _alive : bool,

    //join_handles : Vec<JoinHandle<()>>,

}





impl ChatServer{

    pub async fn new(addr : &str) -> ChatServer{
        let listener = TcpListener::bind(addr).await
            .expect(&format!("Cannot bind to {}.", addr));
        
        let (c_tx, _) = broadcast::channel::<(String, SocketAddr)>(10);
        let (s_tx, _) = broadcast::channel::<String>(10);

        let (logger_tx, _) = broadcast::channel::<String>(10);

        let new_server = ChatServer{

            welcome_message : "Welcome to the OG BS prototype chat server!\n\r".to_string(),
            listener : listener,

            _communicator : ChatServerCommunicators{
                client_tx : c_tx,
                server_tx : s_tx,
                logger_tx : logger_tx
            },

            _alive : false,
            //join_handles : Vec::new(),
        };

        new_server
    }


    pub async fn run(&mut self){

        

        /* 
        tokio::spawn( async move {
            ChatServer::get_admin_commands()
        });*/

        self.run_logger().await;
        self._alive = true;
        self.server_loop().await;

    }

    async fn get_admin_commands(){

    }

    async fn server_loop(&self) {
        loop {
            let (mut socket, _addr) = self.listener.accept().await.unwrap();

            let comm = self._communicator.clone();
            let mut client_rx = comm.client_tx.subscribe();
            let mut server_rx = comm.server_tx.subscribe();
    
            let announcement = format!("{}", _addr.ip().to_string() + " has joined us!\n\r");
            comm.server_tx.send(announcement.clone()).unwrap();
            comm.logger_tx.send(announcement).unwrap();

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
    
                            ChatServer::handle_client_input(&mut line, &comm, _addr);
                        }
    
                        msg = client_rx.recv() => {
                            let (mes , other_addr) = msg.unwrap();
    
                            ChatServer::handle_broadcast_recv(_addr, other_addr, mes, &mut writer_half).await;
                        }
    
                        msg = server_rx.recv() => {
                            let msg = msg.unwrap();
    
                            ChatServer::handle_announcement_recv(_addr, msg, &mut writer_half).await;
                        }
                    }
                    
                }
    
                let announcement = format!("{}", _addr.ip().to_string() + " has left!\n\r");
                comm.server_tx.send(announcement.clone()).unwrap();
                comm.logger_tx.send(announcement).unwrap();
            });
    
        }
    }

    async fn run_logger(&self){
        let mut rx = self._communicator.logger_tx.subscribe();
        tokio::spawn( async move {
            let mut logger = WatchLogger::new();
            logger.start(None);

            loop{

                let msg = rx.recv().await;
                if msg.is_ok() {
                    logger.log(msg.unwrap());
                }

            }
        });
    }

    fn handle_client_input(line: &mut String, comminicator: &ChatServerCommunicators, _addr : SocketAddr){
        comminicator.client_tx.send( (line.clone(), _addr) ).unwrap();
        let log_msg = format!("{}", _addr.ip().to_string() + ":" + &_addr.port().to_string() + "> " + &line);
        comminicator.logger_tx.send(log_msg).unwrap();
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