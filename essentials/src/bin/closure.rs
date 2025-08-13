use std::fmt::Debug;
fn main() {
    closure_fn();
    closure_fnmut();
    closure_fnonce();
    closure_move();
    let _ = closure_return(1);
    closure_vector();
    closure_higher_ranked_lifetime();
}

/// 本质： 是一个语法糖，闭包表达式会由编译器自动翻译为结构体实例，并为其实现Fn，FnMut，FnOnce三个trait中的一个。
/// 继承关系： Fn（复制语义环境变量，不可变借用捕获）: FnMut（可变绑定修改环境变量，可变借用捕获）: FnOnce（移动语义环境变量，转移所有权来捕获）
///
/// 一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们。
/// move 本身强调的就是后者，闭包如何捕获变量，但是实际上使用了 move 的闭包依然可能实现了 Fn 或 FnMut 特征。
/// 1. 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
/// 2. 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
/// 3. 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
///
/// 注意： 就算签名一样的闭包，类型也是不同的； rust中的函数都默认实现了Fn，FnMut，FnOnce这三个trait

// 复制语义类型自动实现Fn，以不可变借用捕获环境中的自由变量
pub fn closure_fn() {
    let s = "hello";
    // 不需要对捕获变量进行改变
    let c = || { println!("Fn: {:?}", s);};
    c();
    c();
    exec(c);
    println!("Fn: {:?}", s);
}

// 移动语义类型自动实现FnOnce
pub fn closure_fnonce() {
    let s = "hello".to_string();
    // 移出所捕获变量的所有权，因此只实现了FnOnce
    let c = || s;
    println!("FnOnce: {:?}", c());
    // Err，c 转移了s的所有权
    // c();
    // Err, s被转移到闭包中
    // println!("{:?}", s);
}

pub fn closure_fnmut() {
    let mut s = String::new();

    // 没有移出所捕获变量的所有权
    let mut update_string =  |str| s.push_str(str);

    update_string("a");
    update_string("b");

    println!("FnMut: {:?}",s);
}

pub fn closure_move() {
    let s = "hello".to_string();
    // 不需要对捕获变量进行改变
    let c =  move || { println!("Fn with move: {:?}", s) };
    c();
    c();
    exec(c); // 实现Fn
    // Err, move把s已经转移到闭包中
    // println!("move {:?}", s);
}
fn exec<F: Fn()>(f: F)  {
    f()
}

// 闭包是大小不定， 但rust编译时要求大小固定，用Box指针包装
pub fn closure_return(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}

// 每一个闭包都是不同的类型，需要Box包装
pub fn closure_vector() {
    let mut c = vec![];
    boxed_closure(&mut c);
    for f in c {
        f();
    }
}
fn boxed_closure(c: &mut Vec<Box<dyn Fn()>>) {
    let s = "second";
    c.push(Box::new(|| println!("first")));
    c.push(Box::new(move || println!("{:?}", s)));  // 必须使用move转移所有权（但因为是复制语义，转移实际是副本），因为稍后会调用这个闭包
    c.push(Box::new(|| println!("third")));
}

/// 高阶生命周期：解决闭包参数中涉及引用的生命周期参数的标注
pub fn closure_higher_ranked_lifetime() {
    let x = Box::new(&2usize);
    foo(x);
}
trait DoSomething<T> {
    fn do_sth(&self, value: T);
}
impl<T: Debug> DoSomething<T> for &usize {
    fn do_sth(&self, value: T) {
        println!("value: {:?}", value);
    }
}
// 高阶生命周期参数for<'f> 表示只针对后面跟着的参数DoSomething<&'f usize>，foo函数的生命周期和DoSomething<&'f usize>没有直接关系
// 不然去掉 for<'f> 会报错， 因为foo函数的生命周期和临时变量s关联起来了，导致悬垂指针
fn foo(b: Box<dyn for<'f> DoSomething<&'f usize>>) {
    let s = 10;
    b.do_sth(&s);
}