#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    // this creates a new 1 way channel that can be used to communicate between threads

    tokio::spawn(async move {
        for i in 0..20 {
            tx.send(i).await.unwrap();
        }
    });
    // this creates a new thread, in which the sender from the channel in the first line is moved
    // it then uses the sender to send values to the receiver (still in the main thread) iteratively with a for loop

    while let Some(value) = rx.recv().await {
        println!("Value received from other thread - {}", value);
    }
    //the receiver is processed inside of a while let loop
    //while the sender from the other thread is transmitting, the receiver will continue to get values (kind of like a stream) until the sender terminates
}

//works essentially the same way as a regular channel, but with an asynchronous twist