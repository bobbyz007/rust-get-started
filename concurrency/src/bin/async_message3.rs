use std::time::Duration;
use utils::trpl;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();
        let tx1_fut = async move{
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut = async move{
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(val) = rx.recv().await {
                println!("received '{val}'")
            }
        };

        trpl::join3(tx_fut, tx1_fut, rx_fut).await;
    })
}