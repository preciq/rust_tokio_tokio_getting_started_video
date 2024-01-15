use log::Level;
use tokio::{fs::File, io::AsyncReadExt, join, time};

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
    join!(
        sleeper(),
        reader(),
        //the change done here (using the join macro) forces rust/tokio to execute the sleeper and reader functions concurrently
        //what that means is the execution fires the sleeper function first, but doesn't wait for it to finish
        //it then fires off the reader function and doesn't wait for it to finish either
        //essentially, it fires off both functions in the order we specified BUT it doesn't wait for one to finish before starting the other
        //we can extract the total result from this in a future

        //for reference, we created several sleepers below. Upon seeing the output, we can see that rather than waiting for each sleeper to finish before starting the next one, the program fires off all the sleepers at once and then waits for them to finish
        //which they will all do at the same time, since they all sleep for the same amount of time
        sleeper(),
        sleeper(),
        sleeper(),
        sleeper(),
        sleeper(),
        sleeper(),
        sleeper(),
        sleeper(),
    );
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap(); //this sets up the logger

    let rt = tokio::runtime::Runtime::new().unwrap(); //this sets up the tokio runtime
    //note that there is a tokio macro that actually does the same thing, shown later
    let future = run();
    //this will create a future that will run the "run" function defined above, which is executed asynchronously

    rt.block_on(future); //this will run the future on the runtime we created above
}
