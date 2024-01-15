use tokio::task;

fn fib(n: u64) -> u64{
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
//an example of a computer intensive task, a fibonacci calculation

#[tokio::main]
async fn main() {
    let task_one = task::spawn_blocking(|| {
        let n = 40;
        println!("First Fibonacci task started with {}", n);
        println!("First Fibonacci task completed with the result: {}", fib(n));
    }); //spawns a new task (thread) and executes a fibonacci calculation up to 40 in it
    //since this task is in its own thread, it doesn't block execution of anything on the main thread

    let task_two = task::spawn_blocking(|| {
        let n = 25;
        println!("Second Fibonacci task started with {}", n);
        println!("Second Fibonacci task completed with the result: {}", fib(n));
    });
    //same thing as above but for 25

    match tokio::join!(task_one, task_two) {
        (Ok(()), Ok(())) => println!("Successfully completed fibonacci calculations asyncronously."),
        _ => println!("Fibonacci sequence calculation was unsuccessful")
    }
    //match statement handling the result
    //even though we started (asynchronously) the second function after the first, the second function will complete and return its value first
    //this is because the second function does not wait on the first's completion to begin execution
    //fib(25) is also easier to calculate than fib(40), so the second WOULD finish first
    //join delivers us the results of the futures when they are available
}   
