use std::pin::{pin, Pin};
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

        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>>  = vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        let pin_tx1_fut = pin!(tx1_fut);
        let pin_rx_fut = pin!(rx_fut);
        let pin_tx_fut = pin!(tx_fut);
        // Pin提供了一个统一的类型，不需要Box
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![pin_tx1_fut, pin_rx_fut, pin_tx_fut];
        trpl::join_all(futures).await;
    })
}