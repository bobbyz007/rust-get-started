use tokio::net::TcpStream;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    // Spawn a task that sends a message over the oneshot
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });

    // <pattern> = <async expression> => <handler>,
    // The select! macro runs all branches concurrently on the same task.
    // Because all branches of the select! macro are executed on the same task,
    // they will never run simultaneously.
    // The select! macro multiplexes asynchronous operations on a single task.
    tokio::select! {
        socket = TcpStream::connect("localhost:3465") => {
            println!("Socket connected {:?}", socket);
        }
        msg = rx => {
            println!("received message first {:?}", msg);
        }
    }
}