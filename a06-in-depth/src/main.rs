mod mainfuture;
use std::time::{Duration, Instant};

use mainfuture::*;

mod delay;
use delay::*;

mod mini_tokio;
use mini_tokio::*;

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(4000);
    let delay = Delay::new(when);

    let delay_output = delay.await;
    println!("Delay future returned output: {}", delay_output);
    assert_eq!(delay_output, "done");

    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(1000);
        let delay = Delay::new(when);

        let delay_output = delay.await;
        println!("Mini tokio delay future returned output: {}", delay_output);
        assert_eq!(delay_output, "done");
    });

    mini_tokio.run();
}
