mod client;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    // 在多个task之间共享
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        println!("Accepted");

        // Clone the handle to the hash map.
        let db = db.clone();

        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
// 使用tokio::sync::Mutex如果需要跨await范围锁定。否则std::sync::Mutex
type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// 如果锁竞争太激烈，可以用mutex sharding，多个锁分担压力
#[allow(dead_code)]
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
#[allow(dead_code)]
fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
async fn process(socket: TcpStream, db: Db) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}