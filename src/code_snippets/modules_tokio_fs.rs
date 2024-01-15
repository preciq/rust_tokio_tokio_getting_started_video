use tokio::{fs, io::AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //added a return type to the main of Result so we could use the question mark operator

    let mut file = fs::File::open("test.txt").await?;
    //this reads a file asynchronously

    let mut contents = String::new();
    //an empty string created to save the contents of the file

    file.read_to_string(&mut contents).await?;
    //reads the file and saves its contents to "contents", asynchronously

    println!("File contents: {}", contents);

    Ok(())
    //returns "Ok" if everything runs successfully
}