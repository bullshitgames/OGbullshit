mod server;
mod filehandler;
mod logger;

use crate::server::*;

#[tokio::main]
async fn main() {

    let mut server = ChatServer::new("192.168.1.10:7888").await;

    server.run().await;

}
