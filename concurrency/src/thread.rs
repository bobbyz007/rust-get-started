use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{Builder, current};
use std::time::Duration;

// thead::spawn创建线程
pub fn thread_create() {
    let mut v = vec![];
    for id in 0..5 {
        let child = thread::spawn(move || {
            println!("in child: {}", id);
        });
        v.push(child);
    }
    println!("in main: join before");
    for child in v {
        let _ = child.join();
    }
    println!("in main: join after");
    println!();
}

// builder创建，指定线程名以及栈大小
pub fn thread_create_builder() {
    let mut v = vec![];
    for id in 0..5 {
        let thread_name = format!("child-{}", id);
        let size = 3 * 1024 * 1024;
        let builder = Builder::new().name(thread_name).stack_size(size);
        let child = builder.spawn(move || {
            println!("in child: {}", id);
            if id == 3 {
                // 模拟捕获panic
                /*panic::catch_unwind(|| {
                    panic!("oh no!");
                }).expect("panic message");*/
                println!("in {} do sth", current().name().unwrap());
            }
        }).unwrap();
        v.push(child);
    }
    for child in v {
        let _ = child.join();
    }
}

// thread local变量：各个线程独立的存储
pub fn thread_local() {
    thread_local! {static FOO: RefCell<u32> = RefCell::new(1)}

    // 主线程中操作FOO
    FOO.with(|f| {
        assert_eq!(*(f.borrow()), 1);
        *(f.borrow_mut()) = 2;
    });

    // 子线程拥有独立的副本
    thread::spawn(|| {
        FOO.with(|f| {
            assert_eq!(*(f.borrow()), 1);
            *(f.borrow_mut()) = 3;
        })
    });

    FOO.with(|f| {
        assert_eq!(*(f.borrow()), 2);
    });

    println!();
}

// 阻塞原语
pub fn thread_park() {
    let parked_thread = Builder::new().spawn(|| {
        println!("Parking thread");
        thread::park();
        println!("Thread unparked");
    }).unwrap();
    // 主线程睡眠10ms
    thread::sleep(Duration::from_millis(10));
    println!("Unpark the thread");

    parked_thread.thread().unpark();
    parked_thread.join().unwrap();

    println!();
}

// 闭包（其他复合类型也类似）的类型是和捕获变量相关的，如果捕获变量的类型实现了Send, 那么闭包也实现了Send， Sync也类似。
// rust默认为所有类型都实现了 Send和Sync。
// 官方文档里提到的，a type T is Sync if and only if &T is Send.
// 实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)

// 没有实现Send的类型（无法跨线程移动）：
// *const T
// *mut T
// Rc<T>

// 没有实现Sync的类型（无法跨线程共享）：
// *const T
// *mut T
// Cell<T>
// RefCell<T>
// Rc<T>

// 更多理解参考： https://www.zhihu.com/question/303273488
pub fn thread_send() {
    // 指针类型无法在线程间共享，可以包装成NewType
    let p = MyBox(5 as *mut u8);
    // 传递所有权
    let t = thread::spawn(move || {
        println!("{:?}",p);
    });

    t.join().unwrap();
}

pub fn thread_send_sync() {
    // 指针类型无法在线程间共享，可以包装成NewType
    let b = &MySendSyncBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || {
        let _v1 =  v.lock().unwrap();
    });

    t.join().unwrap();
}
#[derive(Debug)]
struct MyBox(*mut u8);
unsafe impl Send for MyBox {}

struct MySendSyncBox(*const u8);
unsafe impl Send for MySendSyncBox {}
unsafe impl Sync for MySendSyncBox {}


