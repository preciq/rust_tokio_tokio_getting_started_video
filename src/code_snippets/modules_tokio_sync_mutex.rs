use tokio::sync;

struct SomeStruct {
    some_field: String,
}
//creating a sample struct to demonstrate use of tokios mutex across multiple threads

#[tokio::main]
async fn main() {
    let lock = std::sync::Arc::new(sync::Mutex::new(SomeStruct {
        some_field: "hello".to_string(),
    }));

    let lock1 = lock.clone();
    let lock2 = lock.clone();
    // creates 2 copies of the lock, which will be passed to 2 spawned threads, where the value within will be mutated
    // possible due to the multiple ownership that Arc provides

    let a = tokio::spawn(async move {
        let mut val = lock1.lock().await;
        // asynchronously waits for the lock to be released if another thread holds it, then acquires the lock when it is available
        val.some_field.push_str(" how are");
        // mutates the value within the lock

        //the lock is dropped at the end of this thread
    });

    let b = tokio::spawn(async move {
        let mut val = lock2.lock().await;
        val.some_field.push_str(" you?");
        //same as above but in a second thread
    });
    
    match tokio::join!(a, b) {
        (Ok(_), Ok(_)) => {
            let val = lock.lock().await;
            println!("{}", val.some_field);
        }
        _ => println!("Error"),
    }
    //the above checks the result of a and b once they complete (asynchronously)
    //if either of them give an error, we go to the second match arm, which prints out "Error"
    //otherwise, we take the lock now in the main thread and print the message within, which should be "hello how are you?"
}