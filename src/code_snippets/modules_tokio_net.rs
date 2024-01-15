use tokio::{net, io::{AsyncReadExt, AsyncWriteExt}};
use std::str;

#[tokio::main]
async fn main() {
    let host = "localhost:8080";
    //this line specifies the hosts address as a string literal

    let server = net::TcpListener::bind(host).await.unwrap();
    //this line starts up a listener (server instance). as this might take some time due to internet connectivity, this is done asynchronously
    //the below code proceeds AFTER this is completed and the future is retrieved due to the await keyword
    //the listener listens to anything that is sent from port 8080 of the local machine

    loop {
        //continually loops, waiting for a new connection to the server. once it gets one, the second thread is spawned

        let (mut socket, _) = server.accept().await.unwrap();
        //as explained above, this waits for a connection to be accepted. 
        //until it is, the below code will not execute due to the "await"

        //we ignore the second value returned by server.accept, the SocketAddr, because we don't need it for this program
        //the socket address contains the clients (whoever connects with localhost:8080) IP address and port number
        //this is sometimes used for debugging/logging, security (for verification of the client) or customization (performing custom behaviour based on the client)

        tokio::spawn(async move {
            //spawns a new asynchronous task (a new thread); because of "move", the task will take ownership of anything outside that is passed to it
            let mut buf = [0; 1024];
            //creates an array of length 1024 representing a buffer that can hold up to 1024 bytes of data
            //it is mutable
        
            let n = socket.read(&mut buf).await.unwrap();
            //this reads the data from the socket and stores it inside the buffer created above
            //again, since it may take a while to read the data, this is done in a future to be non-blocking

            socket.write_all(&buf[0..n]).await.unwrap();
            //this writes the data that was read back into the socket
            //basically rerwrites the exact data that was read
            //this is an echo server after all...

            let data = str::from_utf8(&buf[0..n]).unwrap();
            //the above that was read (and rewritten back to the socket) is converted from a byte array to a string, to be used later

            println!("Echoed: {:?}", data);
            //print out the data read from the server and converted to a string from a byte array

            socket.shutdown().await.unwrap();
            //shuts down the listener/connection
            //again, since this might take a while due to the internet connection, this is done asynchronously
        });
    }
}
