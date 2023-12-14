use std::thread;

#[derive(Copy, Clone)]
struct MyStruct;

pub fn send_sync_safe() {
    let mut x = vec![1, 2, 3, 4];
    // 默认闭包中的x是借用, 但move阻止了数据竞争， x实现了Send和Sync才可以move传递所有权。
    let t = thread::spawn(move || {
        x.push(5);
        println!("new thread: {:?}", x)
    });

    t.join().expect("panic message");

    /*
    如下两种情况都会报错，阻止了数据竞争。 通过Send，Sync以及所有权机制，编译器即可以检测出线程安全问题，保证了无数据竞争的并发安全
    thread::spawn(move || {x.push(5);});
    x.push(2);

    thread::spawn(|| {x.push(5);});
    x.push(2);
    */
}