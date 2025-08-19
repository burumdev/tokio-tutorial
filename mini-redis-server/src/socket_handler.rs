use mini_redis::{Connection, Frame};
use tokio::net::TcpStream;

use crate::Db;

pub async fn process_socket(socket: TcpStream, db_map: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db_map
                    .lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());

                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db_map.lock().unwrap().get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Unimplemented command {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
