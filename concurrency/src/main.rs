mod thread;
mod lock;
mod channel;
mod utils;
mod async_await;

use ::utils::println_format_marker;
use crate::async_await::{async_await, async_future, async_pin_heap, async_pin_stack, async_unpin};
use crate::channel::{channel_async, channel_sync, channel_workload_sample};
use crate::lock::{lock, lock_barrier, lock_condvar, lock_deadlock, lock_poisoning, lock_rwlock};
use crate::thread::{thread_create, thread_create_builder, thread_local, thread_park, thread_send, thread_send_sync};
use crate::utils::{rayon_join, rayon_on_off, rayon_par_iter};

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

    println_format_marker("rayon crate");
    rayon_par_iter();
    rayon_on_off(1_000_000);
    rayon_join(12);
    // rayon_on_off(100_000_000);
    // rayon_join(42);

    println_format_marker("async");
    async_await();
    async_future();
    async_unpin();
    async_pin_stack();
    async_pin_heap();

}
