use log::Level;
use tokio::{fs::File, io::AsyncReadExt, join, task::spawn_blocking, time};

fn fibonacci(x: usize) -> usize {
    match x {
        0 => 0,
        1 => 1,
        _ => fibonacci(x - 1) + fibonacci(x - 2),
    }
}
//adding a fibonacci calculation, which is heavy on the computer resources

async fn sleeper() {
    log::info!("SLEEPING FOR 2 SECONDS"); //this line logs a message
    time::sleep(time::Duration::from_secs(2)).await;
    //this line sleeps for 2 seconds. the await is added because sleep is an async function and we want to wait for it to finish before continuing
    log::info!("DONE SLEEPING"); //this line logs another message

    spawn_blocking(move || {
        log::info!("Doing fibonacci calculation");
        let fib_cal = fibonacci(40);
        log::info!("Done with fibonacci calculation: {}", fib_cal);
    });
    //here, we spawn a new tokio thread to run the fibonacci calculation
    //since the operation is heavy, it should be in its own thread
    //this also prevents this task from blocking other tasks running in the main thread, and restores our performance

    //the addition of this fibonacci series calculation will make the program take longer to execute
    //the reason is this is heavier on computer resources than just a wait
    //the program cannot simply fire and forget here, it must ACTIVELY finish the fibonacci calculation
    //and until it does so, it cannot move on to the next function
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

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap(); //this sets up the logger
    run().await;
    //rather than having all the boilerplate we had previously, we can use the tokio main macro, as we did above
    //this turns our main function into an async function, and allows us to execute the run function asynchronously
        //in summary, creates a tokio runtime, executes in main, and handles the future for us, before closing the runtime
}
