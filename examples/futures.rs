use async_main::async_main;
use futures::executor::LocalSpawner;

#[async_main(futures)]
async fn main(_executor: LocalSpawner) {
    println!("Hello, world!");
}
