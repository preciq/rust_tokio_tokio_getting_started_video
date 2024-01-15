use log::Level;
use tokio::{fs::File, io::AsyncReadExt, join, task::spawn_blocking, time, spawn};

async fn an_async_function(x: &str) {
    log::info!("This is an async function with param: {}", x);
}

async fn run() {
    spawn(async {
        an_async_function("howdy").await;
    });
    //we can also ensure execution this way too, by spawning a new task
    //this is basically "fire and forget"
    //we don't care about the result, we just want it to execute
    //we can use this to override the lazy nature of futures

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