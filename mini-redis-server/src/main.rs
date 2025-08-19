use tokio::net::TcpListener;

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

mod socket_handler;
use socket_handler::*;

const SERVER_ADDRESS: &'static str = "127.0.0.1:9999";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SERVER_ADDRESS).await.unwrap();
    println!("Mini redis server listening on address: {}", SERVER_ADDRESS);

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db_clone = db.clone();

        tokio::spawn(async move {
            process_socket(socket, db_clone).await;
        });
    }
}
