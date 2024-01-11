use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send("tx1").await.unwrap();
        tx2.send("tx2").await.unwrap();
        tx3.send("tx3").await.unwrap();
    });

    loop {
        // The select! macro runs all branches concurrently on the same task.
        // Because all branches of the select! macro are executed on the same task,
        // they will never run simultaneously.
        // The select! macro multiplexes asynchronous operations on a single task.
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}