use std::rc::Rc;

//隐式类型转换 implicit type conversion，由编译器或解释器自动来完成，用来简化编程
pub fn implicit_type_conversion_string() {
    let a = "hello".to_string();
    let b = " world".to_string();

    // 如下语句报错，因为隐式类型转换是针对引用
    // let c = a + b;

    // &String 隐式转换为 &str，因为String实现了Deref trait: Deref<Target=str>，可以自动隐式转换
    let c = a + &b;
    println!("{:?}", c);


    // Vector<T>实现了Deref trait: Deref<Target=[T]>，所以引用 &v 会自动隐式转换为 &[T]
    let v = vec![1, 2, 3];
    foo(&v);
}

fn foo(s: &[i32]) {
    println!("{:?}", s[0]);
}

// 手动解引用
pub fn manual_type_conversion() {
    let x = Rc::new("hello");

    // 调用的是Rc的clone方法，返回 Rc<&str>
    let _y = x.clone();

    // 因为Rc类型和解引用的目标类型都含有clone方法，此时需手动解引用，返回&str
    let _z = *x;

    let s = "hello".to_string();

    // 写成"match s" 报错， *String 解引用为 str， 再添加引用变成 &str
    match &*s {
        "hello" => println!("hello"),
        _ => {}
    }
}

/// as operation
pub fn as_op() {
    // 用于基本类型
    let a = u32::MAX;
    let b = a as u16;
    assert_eq!(b, 65535);
    
    // 结构体实现多个trait，出现同名方法
    let s = S(1);
    <S as A>::test(&s, 1);
    <S as B>::test(&s, 2);
}

/// from/into trait
/// 一般情况下实现From或Into其中之一即可，因为它们是互逆的
pub fn from_into() {
    let person1 = Person::new("Alex");
    let person2 = Person::new("Sander".to_string());
    println!("{:?}", person1);
    println!("{:?}", person2);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String
}
impl Person {
    // String和&str都实现了Into trait，两种类型参数都可以接受
    fn new<T: Into<String>>(name: T) -> Person {
        Person{name: name.into()}
    }
}

struct S(i32);
trait A {
    fn test(&self, i: i32);
}
trait B {
    fn test(&self, i: i32);
}
impl A for S {
    fn test(&self, i: i32) {
        println!("from A: {:?}", i);
    }
}
impl B for S {
    fn test(&self, i: i32) {
        println!("from B: {:?}", i)
    }
}
