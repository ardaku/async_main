use async_main::{async_main, LocalSpawner};
use pasts::prelude::*;

#[async_main(pasts)]
async fn main(_spawner: &LocalSpawner) {
    println!("Hello, world!");
}
