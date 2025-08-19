use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file_foo = File::open("foo.txt").await?;
    let mut buffer = [0; 24];

    let index_size = file_foo.read(&mut buffer[..]).await?;

    println!("The bytes read are: {:?}", &buffer[..index_size]);

    print!("\nThe human readable version is: ");
    buffer.iter().for_each(|byte| {
        print!("{}", *byte as char);
    });
    print!("\n");

    let mut file_ayodhya = File::options()
        .read(true)
        .write(true)
        .open("ayodhya.txt")
        .await?;

    let mut buffer = Vec::new();

    let index_size = file_ayodhya.read_to_end(&mut buffer).await?;
    println!("Read until pos: {}", index_size);

    print!("\nAyodhya was:\n");
    buffer.iter().for_each(|byte| {
        print!("{}", *byte as char);
    });
    print!("\n");

    file_ayodhya
        .write_all(
            b"\nEight-wheeled, nine-doored, is the impregnable stronghold of the gods; in that is a golden vessel, heaven-going (swarga), covered with light\n",
        )
        .await?;

    file_ayodhya.sync_all().await?;

    buffer.clear();

    let mut file_ayodhya = File::open("ayodhya.txt").await?;
    let index_size = file_ayodhya.read_to_end(&mut buffer).await?;

    println!("Ayodhya read until pos: {}", index_size);
    buffer.iter().for_each(|byte| {
        print!("{}", *byte as char);
    });
    print!("\n");

    Ok(())
}
