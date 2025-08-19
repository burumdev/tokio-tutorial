use bytes::Bytes;
use mini_redis::client;

use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}
use Command::*;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut client = client::connect(env!("SERVER_ADDRESS")).await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get { key, resp } => {
                    let response = client.get(&key).await;
                    let _ = resp.send(response);
                }
                Set { key, val, resp } => {
                    let response = client.set(&key, val).await;
                    let _ = resp.send(response);
                }
            }
        }
    });

    let get_task = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let response = resp_rx.await;
        println!("Get task got: {:?}", response);
    });

    let set_task = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();
        let response = resp_rx.await;
        println!("Set task got: {:?}", response);
    });

    get_task.await.unwrap();
    set_task.await.unwrap();
    manager.await.unwrap();
}
