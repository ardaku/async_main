use std::sync::Arc;

use async_executor::LocalExecutor;
use async_main::async_main;

#[async_main(async_executor)]
async fn main(_executor: Arc<LocalExecutor<'_>>) {
    println!("Hello, world!");
}
