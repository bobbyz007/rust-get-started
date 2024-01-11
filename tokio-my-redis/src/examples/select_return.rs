async fn computation1() -> String {
    // .. computation
    "computation1".to_string()
}

async fn computation2() -> String {
    // .. computation
    "computation2".to_string()
}

#[tokio::main]
async fn main() {
    // <pattern> = <async expression> => <handler>,
    // The select! macro runs all branches concurrently on the same task.
    // Because all branches of the select! macro are executed on the same task,
    // they will never run simultaneously.
    // The select! macro multiplexes asynchronous operations on a single task.
    let out = tokio::select! {
        res1 = computation1() => res1,
        res2 = computation2() => res2,
    };

    // 结果随机
    println!("Got = {}", out);
}