use std::sync::Arc;

use async_main::async_main;
use tokio::runtime::Runtime;

#[async_main(tokio)]
async fn main(_executor: Arc<Runtime>) {
    println!("Hello, world!");
}
