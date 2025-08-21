use tokio::task::JoinSet;
use tokio::time::{Duration, interval};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;

async fn print_ticks(mut stream: IntervalStream, speed_index: usize) {
    while let Some(_) = stream.next().await {
        for _ in 0..speed_index {
            print!("\t");
        }
        print!("Tick\n");
    }
}

const SPEEDS_MILLIS: [u64; 4] = [1024, 512, 256, 128];

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    let mut task_set = JoinSet::new();
    SPEEDS_MILLIS
        .iter()
        .enumerate()
        .for_each(|(index, &speed)| {
            let interval = interval(Duration::from_millis(speed));
            let int_stream = IntervalStream::new(interval);

            task_set.spawn(print_ticks(int_stream, index));
        });

    while let Some(_) = task_set.join_next().await {}

    Ok(())
}
