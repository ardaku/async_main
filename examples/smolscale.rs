use async_main::async_main;

#[async_main(smolscale)]
async fn main(_executor: ()) {
    println!("Hello, world!");
}
