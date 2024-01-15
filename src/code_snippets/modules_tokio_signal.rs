use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("waiting for ctrl-c...");

    signal::ctrl_c().await?;
    // this awaits user input from the user, specifically ctrl-c
    // signal is used to handle OS related signals, like ctrl-c

    println!("received ctrl-c!");

    Ok(())
}