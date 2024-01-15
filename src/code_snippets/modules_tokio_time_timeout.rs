use tokio::time;

async fn sleepy_function() {
    println!("future running");
    time::sleep(time::Duration::from_secs(3)).await;
    println!("future completed");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    //creates a logger instance to use

    if let Err(_) = time::timeout(
        time::Duration::from_secs(2), sleepy_function())
        //this sets up a timeout for the sleepy function of 2 seconds, asynchronously
        //if the 2 seconds elapses and the function is still running, it will print "future timed out"
        //we can also define custom behaviour below as well
        .await {
        println!("future timed out");
    }
    Ok(())
}
