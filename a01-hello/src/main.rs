use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:9999").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("Got value from the server; result={:?}", result);

    let mut another_client = client::connect("127.0.0.1:9999").await?;
    let another_result = another_client.get("hello").await?;

    println!("Another connection to see if you persisted the HashMap db.");

    if let Some(result) = another_result {
        println!("You did. The result is: {:?}", result);
    } else {
        eprintln!("No you didn't :(");
    }

    Ok(())
}
