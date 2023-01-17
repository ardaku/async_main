use async_main::{async_main, LocalSpawner};

#[async_main]
async fn main(_spawner: LocalSpawner) {
    println!("Hello, world!");
}
