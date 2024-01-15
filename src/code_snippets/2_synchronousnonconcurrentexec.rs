use log::Level;
use tokio::{fs::File, io::AsyncReadExt, time};

async fn sleeper() {
    log::info!("SLEEPING FOR 2 SECONDS"); //this line logs a message
    time::sleep(time::Duration::from_secs(2)).await;
    //this line sleeps for 2 seconds. the await is added because sleep is an async function and we want to wait for it to finish before continuing
    log::info!("DONE SLEEPING"); //this line logs another message
}

async fn reader() {
    log::info!("Reading from a file");
    let mut file = File::open("test.txt").await.unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).await.unwrap();
    log::info!("Read file! Length: {}", contents.len());
    //the above reads a file and logs the length of the file
}

async fn run() {
    sleeper().await;
    reader().await;
    //the above forces rust/tokio to execute the sleeper and reader functions in order
    //so the execution first sleeps for 2 seconds
    //then reads from the file
    sleeper().await;
    sleeper().await;
    sleeper().await;
    sleeper().await;
    sleeper().await;
    sleeper().await;
    sleeper().await;
    sleeper().await;

    //this will execute each of the sleepers in order, waiting for each one to finish before starting the next one
    //so NOT concurrently
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap(); //this sets up the logger

    let rt = tokio::runtime::Runtime::new().unwrap(); //this sets up the tokio runtime
                                                      //note that there is a tokio macro that actually does the same thing, shown later
    let future = run();
    //this will create a future that will run the "run" function defined above, which is executed asynchronously

    rt.block_on(future); //this will run the future on the runtime we created above
}
