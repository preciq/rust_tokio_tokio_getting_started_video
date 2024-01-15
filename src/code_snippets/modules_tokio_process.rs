//yeah I'll be honest I have no idea what this does or means but here is the code, with copilot comments

// Import the necessary modules from the tokio crate
use tokio::{process, io::AsyncWriteExt};

// This is the main function of the program. It's asynchronous, which means it can perform
// operations that might take some time without blocking the rest of the program.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Create a new Command that, when spawned, will run the "sort" command
    let mut cmd = process::Command::new("sort");

    // Set up the command to capture its output
    cmd.stdout(std::process::Stdio::piped());
    // Set up the command to accept input
    cmd.stdin(std::process::Stdio::piped());

    // Spawn the command, which starts it running and returns a Child process
    let mut child = cmd.spawn()?;

    // This is the list of animals that we're going to sort
    let animals: &[&str] = &["dog", "cat", "pig", "hamster", "cow"];

    // Take the stdin of the child process so we can write to it
    let mut stdin = child.stdin.take().expect("child did not have a handle in stdin");

    // Write our animals to the child process. We join the animals with "\n" to create a
    // newline-separated string, convert that string to bytes, and then write those bytes
    // to the child's stdin.
    stdin.write(animals.join("\n").as_bytes()).await.expect("failed to write to stdin");
    
    // Drop the stdin, which closes it. This signals to the child process that we're done
    // providing input.
    drop(stdin);

    // Wait for the child process to finish and collect its output. This returns a
    // Output struct that contains the child's stdout, stderr, and exit status.
    let op = child.wait_with_output().await?;

    // Print the sorted animals. We convert the stdout (which is a Vec<u8>) to a str
    // for printing.
    println!("sorted:\n\n{}", std::str::from_utf8(&op.stdout)?);

    // Return Ok to indicate that the program finished successfully
    Ok(())
}