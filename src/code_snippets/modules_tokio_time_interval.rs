use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let duration = time::Duration::from_secs(1);
    let mut when = time::interval(duration);
    //uses the time module to create a duration of 1 second
    //we use the interval command to asynchronously wait for the duration to pass

    when.tick().await;
    //this awaits the duration to pass
    println!("1 second has elapsed");
    when.tick().await;
    println!("2 seconds have elapsed");
    when.tick().await;
    println!("3 seconds have elapsed");

    Ok(())
}