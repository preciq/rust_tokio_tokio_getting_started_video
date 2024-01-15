use log::Level;
use tokio::time;

async fn run() {
    log::info!("SLEEPING FOR 5 SECONDS"); //this line logs a message
    time::sleep(time::Duration::from_secs(5)).await; 
    //this line sleeps for 5 seconds. the await is added because sleep is an async function and we want to wait for it to finish before continuing
    log::info!("DONE SLEEPING"); //this line logs another message
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap(); //this sets up the logger

    let rt = tokio::runtime::Runtime::new().unwrap();
    //this will create a new runtime for executing our future defined above
    let future = run();
    //this will create a future that will run the run function defined above
    rt.block_on(future);
    //this will run the future on the runtime we created above
}
