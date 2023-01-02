use async_main::async_main;

#[async_main(async_std)]
async fn main(_executor: ()) {
    println!("Hello, world!");
}
