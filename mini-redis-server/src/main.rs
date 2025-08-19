use tokio::net::TcpListener;

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

mod socket_handler;
use socket_handler::*;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(env!("SERVER_ADDRESS")).await.unwrap();
    println!(
        "Mini redis server listening on address: {}",
        env!("SERVER_ADDRESS")
    );

    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db_clone = db.clone();

        tokio::spawn(async move {
            process_socket(socket, db_clone).await;
        });
    }
}
