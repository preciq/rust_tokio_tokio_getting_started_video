use tokio::time;

async fn do_something() -> i32 {
    time::sleep(time::Duration::from_secs(3)).await;
    42
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("do something result: {}", do_something().await);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test] //this can be used to run test cases on asynchronous code, in this case, the "do_something" function
    async fn test_do_something() {
        let result = do_something().await;
        assert_eq!(result, 42);
    }
}