use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock};
use std::thread;

pub fn lock() {
    let mutex = Arc::new(Mutex::new(0));
    let mut v = vec![];
    for _ in 0..3 {
        let mutex_clone = mutex.clone();
        let child = thread::spawn(move || {
            // 可以直接调用Mutex的方法， 是自动解引用
            let mut mutex_clone = mutex_clone.lock().unwrap();
            // 获取了独占锁，可以安全操纵数据了
            *(mutex_clone) += 1;
        });
        v.push(child);
    }
    for child in v {
        child.join().unwrap();
    }

    println!("mutex lock: {:?}", mutex);
}

// 模拟子线程panic
pub fn lock_poisoning() {
    let mutex = Arc::new(Mutex::new(1));
    let mtx_clone = mutex.clone();
    let _ = thread::spawn(move || {
        let mut data = mtx_clone.lock().unwrap();
        *data = 2;
        panic!("oh no");
    }).join();

    // 子线程panic 导致poisoned
    assert_eq!(mutex.is_poisoned(), true);

    match mutex.lock() {
        Ok(_) => unreachable!(),
        Err(e) => {
            println!("recovered: {}", e.get_ref());
        }
    };
}

// 模拟死锁
pub fn lock_deadlock() {
    // 总的投掷次数
    let total_flips = Arc::new(Mutex::new(0));
    // 所有线程是否已完成
    let completed = Arc::new(Mutex::new(0));

    let runs = 8;  // 线程数
    let target_flips = 10;  // 每个线程至少连续投掷多少次正面才结束
    for _ in 0..runs {
        let total_flips = total_flips.clone();
        let completed = completed.clone();
        thread::spawn(move || {
            flip_simulate(target_flips, total_flips);

            let mut completed = completed.lock().unwrap();
            *completed += 1;
        });
    }

    // 等待所有线程结束
    loop {
        // 在进入下一次循环时， completed离开作用域会自动释放锁，所以不会造成死锁
        let completed = completed.lock().unwrap();
        if *completed == runs {
            let total_flips = total_flips.lock().unwrap();
            println!("Final average: {}", *total_flips / runs);
            break;
        }

        // 以下代码会造成死锁， 因为在当前作用域， completed一直持有锁，造成子线程一直等待
        /*let completed = completed.lock().unwrap();
        while *completed < runs { }  // 一直循环在此处，completed无法释放
        let total_flips = total_flips.lock().unwrap();
        println!("Final average: {}", *total_flips / runs);*/
    }
}
fn flip_simulate(target_flips: u64, total_flips: Arc<Mutex<u64>>) {
    let mut continue_positive = 0;
    let mut iter_counts = 0;
    while continue_positive <= target_flips {
        iter_counts += 1;
        let pro_or_con = rand::random();
        if pro_or_con {
            continue_positive += 1;
        } else {
            continue_positive = 0;
        }
    }
    println!("iter counts: {}", iter_counts);

    // 作用域结束后自动释放锁
    let mut total_flips = total_flips.lock().unwrap();
    *total_flips += iter_counts;
}

// 读写锁
pub fn lock_rwlock() {
    let lock = RwLock::new(5);
    // 读写所不能同时存在， 所以要用显式作用域隔离
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    let mut w = lock.write().unwrap();
    *w += 1;
    assert_eq!(*w, 6);
}

// 屏障
pub fn lock_barrier() {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    for _ in 0..5 {
        let c = Arc::clone(&barrier);
        // The same messages will be printed together.
        // You will NOT see any interleaving.
        handles.push(thread::spawn(move|| {
            println!("before wait");
            c.wait();
            println!("after wait");
        }));
    }
// Wait for other threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}

// 条件变量必须与一个互斥锁配合使用
pub fn lock_condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = pair.clone();
    thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one(); // 唤醒一个阻塞在该条件变量上的线程
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("started: {}", started);
        // 释放锁，阻塞当前线程；
        // 被唤醒时， 重新获取锁， 并检查锁的值是否正确， 以防止假的wakeup
        started = cvar.wait(started).unwrap(); // may spurious wakeup
        println!("started: {}", started);
    }
}

