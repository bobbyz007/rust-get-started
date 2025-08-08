use std::time::Duration;
use utils::trpl;

fn main() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {} from the first task!", i);
                trpl::sleep(Duration::from_millis(500)).await
            }
        });

        for i in 1..5 {
            println!("hi number {} from the second task!", i);
            trpl::sleep(Duration::from_millis(500)).await;
        }
        
        handle.await.unwrap();
    })
}