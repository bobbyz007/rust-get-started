use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;
use futures::executor::block_on;
use futures::future::BoxFuture;
use futures::FutureExt;
use futures::task::{ArcWake, waker_ref};

/// 有大量 IO 任务需要并发运行时，选 async 模型
/// 有部分 IO 任务需要并发运行时，选多线程，如果想要降低线程创建和销毁的开销，可以使用线程池
/// 有大量 CPU 密集任务需要并行运行时，例如并行计算，选多线程模型，且让线程数等于或者稍大于 CPU 核心数
/// 无所谓时，统一选多线程

// 基本用法
pub fn async_await() {
    // 返回一个Future, 因此不会打印任何输出
    let future = do_something();
    // 执行`Future`并等待其运行完成，此时会被打印输出
    block_on(future);

    block_on(hello_world());

}
async fn do_something() {
    println!("go go go");
}
async fn hello_world() {
    hello_cat().await;
    println!("hello, world!");
}
async fn hello_cat() {
    println!("hello, kitty!");
}

pub fn async_future() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务，并发送到channel中
    spawner.spawn(async {
        println!("Async Task poll: {:?}", thread::current());
        println!("howdy!");
        // 创建定时器Future，并等待它完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);

    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}
/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!();
        println!("TimerFuture poll: {:?}", thread::current());

        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            println!("TimerFuture pending: {:?}", cx.waker());

            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        // 创建新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!();
            println!("New thread will sleep a while: {:?}", thread::current());

            // 睡眠指定时间实现计时功能
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                println!("Thread waker starts to wake the task: {:?}", &waker);
                println!();

                waker.wake()  // 触发为Task实现的ArcWake方法wake_by_ref
            }
        });

        TimerFuture { shared_state }
    }
}

/// `Spawner`负责创建新的`Future`然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,  // channel sender
}
impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

/// 一个Future，它可以调度自己(将自己放入任务通道中)，然后等待执行器去`poll`
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("Waked by new thread and start to send cloned task again");
        // 通过发送任务到任务管道的方式来实现`wake`，这样`wake`后，任务就能被执行器`poll`
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, task_rx) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { task_rx }, Spawner { task_sender })
}

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    task_rx: Receiver<Arc<Task>>,  // channel receiver
}
impl Executor {
    fn run(&self) {
        println!("Executor thread: {:?}", thread::current());

        while let Ok(task) = self.task_rx.recv(){
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于任务自身创建一个 `LocalWaker`
                let waker = waker_ref(&task);
                println!("Executor get task: {:?}", &*waker);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                if future.as_mut().poll(context).is_pending() {
                    // Future还没执行完，因此将它放回任务中，等待下次被poll
                    println!("Executor task pending, need to be waked");
                    *future_slot = Some(future);
                }
            }
        }
    }
}

// pin
pub fn async_unpin() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2);
    // 本应该打印 test1 test1, 而实际上是打印 test1, test2(交换后test2.b指针是test1.a的地址, 交换后test.a值是test2了）
    println!("a: {}, b: {}", test2.a(), test2.b());
}

// 固定在运行栈上
pub fn async_pin_stack() {
    // 此时的`test1_raw`可以被安全的移动
    let mut test1_raw = TestPined::new("test1");
    // 新的`test1`由于使用了`Pin`，因此无法再被移动，这里的声明会将之前的`test1`遮蔽掉(shadow)
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1_raw) };
    TestPined::init(test1.as_mut());

    let mut test2_raw = TestPined::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2_raw) };
    TestPined::init(test2.as_mut());

    println!();
    println!("a: {}, b: {}", TestPined::a(test1.as_ref()), TestPined::b(test1.as_ref()));
    // 放开以下语句会报错， pin后结构体对象不允许移动
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!("a: {}, b: {}", TestPined::a(test2.as_ref()), TestPined::b(test2.as_ref()));
}

// Test 提供了方法用于获取字段 a 和 b 的值的引用。这里b 是 a 的一个引用，
// 但是我们并没有使用引用类型而是用了裸指针，原因是：Rust 的借用规则不允许我们这样用，因为不符合生命周期的要求。
#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

#[derive(Debug)]
struct TestPined {
    a: String,
    b: *const String,
    _marker: PhantomPinned,//标记类型 PhantomPinned 将自定义结构体 Test 变成了 !Unpin (编译器会自动帮我们实现)，因此该结构体无法再被移动。
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}
impl TestPined {
    fn new(txt: &str) -> Self {
        TestPined {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}
// pin在堆上
pub fn async_pin_heap() {
    let test1 = TestPinedHeap::new("test1");
    let test2 = TestPinedHeap::new("test2");

    println!();
    println!("a: {}, b: {}",test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}",test2.as_ref().a(), test2.as_ref().b());
}
#[derive(Debug)]
struct TestPinedHeap {
    a: String,
    b: *const String,
    _marker: PhantomPinned,//标记类型 PhantomPinned 将自定义结构体 Test 变成了 !Unpin (编译器会自动帮我们实现)，因此该结构体无法再被移动。
}
impl TestPinedHeap {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = TestPinedHeap {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned
        };
        let mut boxed = Box::pin(t);  // t在heap中不能被移动
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}


