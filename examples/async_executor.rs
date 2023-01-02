use std::sync::Arc;

use async_main::async_main;
use async_executor::LocalExecutor;

#[async_main(async_executor)]
async fn main(_executor: Arc<LocalExecutor<'_>>) {
    println!("Hello, world!");
}
