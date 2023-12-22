use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::mpsc::{channel, Sender, sync_channel};
use std::thread;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

// 只有两个线程的是流通道， 多生产者是共享通道
pub fn channel_async() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(10).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 10);
}

pub fn channel_sync() {
    // 缓冲区大小为1，发送时缓冲区满了则会阻塞
    let (tx, rx) = sync_channel(1);

    tx.send(1).unwrap();

    thread::spawn(move || {
        tx.send(2).unwrap();
    });
    assert_eq!(rx.recv().unwrap(), 1);
    assert_eq!(rx.recv().unwrap(), 2);
}

// 基于channel的一个模拟工作量证明的例子
const BASE: usize = 42;
const THREADS: usize = 8;
static DIFFICULTY: &'static str = "00000";  // 查找以DIFFICULTY开头的hash值
struct Solution(usize, String);  // 寻找的数字 及 对应的hash值
pub fn channel_workload_sample() {
    println!("Find a number, SHA256([the number] * {}) == \"{}.....\"", BASE, DIFFICULTY);
    println!("Started {} threads", THREADS);
    println!("Please wait...");

    let is_solution_found = Arc::new(AtomicBool::new(false));
    let (tx, rx) = channel();
    for i in 0..THREADS {
        let tx = tx.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || {
            find(i, tx, is_solution_found);
        });
    }

    match rx.recv() {
        Ok(Solution(number, hash)) => {
            println!("Found the solution: ");
            println!("The number: {}", number);
            println!("The hash: {}", hash);
        },
        Err(_) => panic!("Worker thread disconnected.")
    }
}

fn verify(number: usize) -> Option<Solution> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(number * BASE).to_string()); // feeds a string
    let hash = hasher.result_str();
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

fn find(start_at: usize, tx: Sender<Solution>, is_solution_found: Arc<AtomicBool>) {
    // 每个线程查找的数字互不干扰
    for number in (start_at..).step_by(THREADS) {
        if is_solution_found.load(Acquire) {   // 此处使用Relaxed也可以
            return;
        }
        if let Some(solution) = verify(number) {
            is_solution_found.store(true, Release);  // 此处使用Relaxed也可以
            tx.send(solution).unwrap();  // 返回结果
            return;
        }
    }
}





