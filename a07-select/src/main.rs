use std::time::Duration;

use tokio::sync::oneshot;

async fn some_operation() -> String {
    tokio::time::sleep(Duration::from_millis(500)).await;

    String::from("Some operation finished.")
}

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    let task1 = tokio::spawn(async {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            // Can't get this to print
            _ = tx1.closed() => {
                    println!("Some operation cancelled");
                }
        }
    });

    let task2 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(500)).await;
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }

    task1.await.unwrap();
    task2.await.unwrap();
}
