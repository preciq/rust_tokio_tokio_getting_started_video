use log::Level;
use tokio::{fs::File, io::AsyncReadExt, join, task::spawn_blocking, time};

async fn an_async_function(x: &str) {
    log::info!("This is an async function with param: {}", x);
}

async fn run() {
    an_async_function("hi").await;
    //this now executes since we are calling it with await
    an_async_function("there").await;
    an_async_function("friend").await;
    //the await call is key here, because futures in rust are lazy
    //meaning unless we actually call them (with await), they won't execute
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap(); //this sets up the logger
    run().await;
}
