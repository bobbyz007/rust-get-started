use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    sp_owner();
    sp_rc_weak();
    sp_cell_refcell();
    sp_cow();
}

/// 智能指针与普通引用的区别之一就是所有权的不同， 智能指针拥有资源的所有权，而普通引用只是对所有权的借用。
#[allow(dead_code, unused_variables)]
fn sp_owner() {
    let x = Box::new("hello");
    let y = x;
    // ERR, x moved
    // println!("x: {:?}", x);

    let b = Box::new("rust".to_string());
    let c = *y;  // 解引用，是因为Box实现了Deref trait
    let d = *b;
    println!("y: {:?}", y);
    // ERR, Box包含的T是String， 是移动语义， 所以b是解引用移动
    // println!("b: {:?}", b);

    // Box是唯一支持解引用移动的智能指针，Rc或Arc都不在支持解引用移动
    let r = Rc::new("rust".to_string());
    // Err, 智能指针除了Box，都不支持 解引用移动
    // let x = *r;
}

/// Rc<T> 主要用于共享堆上分配的数据 可以供程序的多个部分读取， 是单线程引用计数指针，不是线程安全类型，不允许共享给别的线程。
#[allow(unused_variables, dead_code)]
fn sp_rc_weak() {
    let x = Rc::new(45);
    let y1 = x.clone();  // 强引用计数，非深复制，只是共享所有权的计数
    let y2 = x.clone();
    println!("rc strong count: {:?}", Rc::strong_count(&x));

    // 解决强引用循环引用问题
    let w = Rc::downgrade(&x);  // 弱引用计数， 无所有权，不会阻止数据的销毁
    let z = Rc::downgrade(&x);
    println!("rc weak count: {:?}", Rc::weak_count(&x));
    let y3 = &*x;  // 不增加计数
}

/// 内部可变性是对struct的一种封装，表面不可变，内部可以通过某种方案改变里面的值
/// Cell<T> : T实现了copy类型，才能调用get，执行按位复制；否则调用get_mut返回可变借用。 依然遵循rust借用检查规则
///           避免包裹大的结构体，因为get会执行一次按位复制。
///           无运行时开销。
/// RefCell<T>: 适用范围更广，对类型T没有copy实现的限制。 但有运行时开销，其内部维护着运行时借用检查器，比如如果持有多个可变借用则panic
fn sp_cell_refcell() {
    let foo = Foo {
        x: 1,
        y: Cell::new(3),
    };
    foo.y.set(5);
    println!("cell new y: {:?}", foo.y.get());

    let x = RefCell::new(vec![1, 2, 3, 4]);
    println!("refcell: {:?}", x.borrow());
    x.borrow_mut().push(5);
    // ERR: 内部调用会在内部作用域范围内自动释放mut引用，而显式获取mute引用则会持续到程序结束
    // let mut m = x.borrow_mut();
    // m.push(5);
    println!("refcell: {:?}", x.borrow());
}
#[allow(dead_code)]
struct Foo {
    x: u32,
    y: Cell<u32>,
}

/// 写时复制： copy on write
/// Cow<T>: 可能是Borrowed 或 Owned（本身拥有所有权，无需克隆） 类型
fn sp_cow() {
    // 没有可变需求，不会克隆
    let s1 = [1, 2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("no write: {:?} == {:?}", s1.as_ptr(), i1.as_ptr());

    // 有可变需求，会克隆
    let s2 = [1, 2, 3, -45, 5];
    let mut i2 = Cow::from(&s2[..]);
    abs_all(&mut i2);
    println!("write clone: {:?} != {:?}", s2.as_ptr(), i2.as_ptr());
    println!("write clone: {:?} != {:?}", s2, i2);


    // from传参是非引用， 此时是Owned类型， 本身拥有所有权，不会克隆
    let mut i3 = Cow::from(vec![1, 2, -3, 4]);
    abs_all(&mut i3);

}
fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0 .. input.len() {
        let v = input[i];  // Cow实现了Deref
        if v < 0 {
            // to_mut方法第一次调用时克隆一个新的对象，后续调用使用新的克隆对象； 如果本身就拥有所有权，则不会发生克隆。
            input.to_mut()[i] = -v;
        }
    }
}


