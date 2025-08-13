use concurrency::lock::*;

fn main() {
    lock();
    lock_poisoning(); //锁中毒: 子线程panic
    lock_deadlock();
    lock_rwlock();  // 读写锁
    lock_barrier(); // 屏障
    lock_condvar(); // 条件变量
}