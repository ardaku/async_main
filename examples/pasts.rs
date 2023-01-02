use async_main::async_main;
use pasts::prelude::*;

#[async_main(pasts)]
async fn main(_executor: Executor) {
    println!("Hello, world!");
}
