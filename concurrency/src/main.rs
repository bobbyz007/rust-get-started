mod thread;
mod lock;
mod channel;

use utils::println_format_marker;
use crate::channel::{channel_async, channel_sync, channel_workload_sample};
use crate::lock::{lock, lock_barrier, lock_condvar, lock_deadlock, lock_poisoning, lock_rwlock};
use crate::thread::{thread_create, thread_create_builder, thread_local, thread_park, thread_send, thread_send_sync};

fn main() {
    println_format_marker("thread");
    thread_create();
    thread_create_builder();
    thread_local();
    thread_park();
    thread_send();
    thread_send_sync();

    println_format_marker("lock");
    lock();
    lock_poisoning(); //锁中毒: 子线程panic
    lock_deadlock();
    lock_rwlock();  // 读写锁
    lock_barrier(); // 屏障
    lock_condvar(); // 条件变量

    println_format_marker("channel");
    channel_async();
    channel_sync();
    channel_workload_sample();

}
