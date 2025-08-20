use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;

use rand::random;

struct Response(&'static [u8]);

const PING: &'static [u8] = b"ping!";
const PONG: &'static [u8] = b"pong!";
const PANG: &'static [u8] = b"pang!";
const PUNG: &'static [u8] = b"pung!";

impl Response {
    pub fn get_str(&self) -> &'static [u8] {
        self.0
    }
}

impl From<&[u8]> for Response {
    fn from(value: &[u8]) -> Self {
        match value {
            PING => Self(PONG),
            PANG => Self(PUNG),
            _ => Self(b"Unrecognized request command. Say what?"),
        }
    }
}

async fn client_task() {
    println!("Hello from client_task.");

    let mut buf = vec![0; 1024];

    loop {
        if let Ok(mut socket) = TcpStream::connect("127.0.0.1:3000").await {
            let message = if random() { PING } else { PANG };
            if let Err(e) = socket.write_all(message).await {
                eprintln!("CLIENT: Failed to write to socket: {e}");
            } else {
                println!("CLIENT: Sent a message...");
                'response: while let Ok(index) = socket.read(&mut buf).await {
                    match index {
                        i if i > 0 => {
                            println!(
                                "CLIENT: And got a response back: {}",
                                String::from_utf8_lossy(&buf[..index])
                            );
                            break 'response;
                        }
                        0 => {
                            break 'response;
                        }
                        _ => (),
                    }
                }
            }
        }
        sleep(Duration::from_millis(512)).await;
    }
}

async fn server_task() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("TCP listener listening on 127.0.0.1:3000");
    loop {
        match listener.accept().await {
            Ok((mut socket, address)) => {
                println!("SERVER: New client connected from {address}");

                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];
                    loop {
                        match socket.read(&mut buf).await {
                            Ok(0) => {
                                println!("SERVER: Client from {address} disconnected.");
                                return;
                            }
                            Ok(n) => {
                                let slice = &buf[..n];
                                let response: Response = Response::from(slice);
                                println!("SERVER: Got {}", String::from_utf8_lossy(slice));
                                socket.write_all(response.get_str()).await.unwrap();
                            }
                            Err(err) => {
                                eprintln!("SERVER: Error reading bytes from {address}: {:?}", err);
                                return;
                            }
                        }
                    }
                });
            }
            Err(err) => eprintln!("SERVER: Accept error: {:?}", err),
        };
    }
}

#[tokio::main]
async fn main() {
    let server_task = tokio::spawn(server_task());
    let client_task = tokio::spawn(client_task());

    let _ = tokio::join!(server_task, client_task);
}
