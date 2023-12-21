use std::fmt::Debug;

/// 标注生命周期参数并不能改变任何引用的生命周期长短，只用于编译器的借用检查，消除悬垂指针
///
/// 1. 函数签名的生命周期参数限制条件： 输出（借用方）的生命周期长度 不能超过 输入（出借方）的生命周期长度。
/// 2. 禁止在没有任何输入参数的情况下 返回引用， 明显造成悬垂引用， 以下函数报错。 但可以返回String
/*pub fn return_str<'a>() -> &'a str {
    let mut s = "rust".to_string();
    for i in 0..3 {
        s.push_str(" good");
    }
    &s[..]
}*/
/// 3. 输出（借用方）的生命周期参数 必须与函数的 输入参数的生命周期相匹配，不然标注生命周期毫无意义
/*fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    let result = String::from("really long string");
    return &result;
}*/

#[allow(unused_variables)]
pub fn return_str() -> String {
    let mut s = "rust".to_string();
    for i in 0..3 {
        s.push_str(" good");
    }
    s
}

pub fn lifetime_func() {
    let s1 = String::from("rust");
    let s1_r = &s1;
    {
        let s2 = String::from("c");
        let res = longest_mul(s1_r, &s2);
        println!("{} is the longest", res);
    }
}

// 函数参数或输出中有引用类型，需考虑(非必须)标注生命周期参数
#[allow(dead_code)]
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 多个生命周期参数定义： 'b 长于 'a。 原则就是输出的'a 不能长于输入参数的'a和'b
#[allow(dead_code)]
fn longest_mul<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// 结构体含有引用类型成员时需标注生命周期参数
// 结构体本身的生命周期 应不长于 任意一个成员的生命周期
#[derive(Debug)]
struct Foo<'a> {
    part: &'a str,
}
pub fn lifetime_struct() {
    let words = String::from("sometimes think, others");
    let first = words.split(",").next().expect("could not find a ,");
    // first  生命周期更长， f会先被析构
    let f = Foo {
        part: first
    };
    assert_eq!("sometimes think", f.part);
}

/// 生命周期省略规则如下，编译器会根据这些规则自动补齐生命周期参数
/// 1. 每个输入位置上省略的生命周期都将成为一个不同的生命周期参数
/// 2. 如果只有一个输入生命周期的位置，则该生命周期将分配给输出生命周期
/// 3. 如果有多个输入生命周期的位置，且其中包含这&self，&mut self，则self的生命周期将分配给输出生命周期
pub fn lifetime_omit_rules(){
    let foo = Foo::new("sometimes think, others");
    let foo2 = Foo::new2("sometimes think, others");

    println!("part: {:?}", foo.get_part());
    println!("part2: {:?}", foo2.get_part());
}

#[allow(dead_code)]
impl<'a> Foo<'a> {
    // 符合省略规则
    fn get_part(&self) -> &str {
        self.part
    }

    // 符合省略规则
    fn split_first(s: &str) -> &str {
        s.split(",").next().expect("could not find a ,")
    }

    // 符合省略规则
    fn new(s: & str) -> Foo {
        Foo { part: Foo::split_first(s) }
    }

    // 因为Self代表 Foo<'a>，所以参数也需要标注'a
    fn new2(s: &'a str) -> Self {
        Foo { part: Foo::split_first(s) }
    }
}

/// 生命周期限定
/// Lifetime bounds can be applied to types or to other lifetimes.
/// The bound 'a: 'b is usually read as 'a outlives 'b. 'a: 'b means that 'a lasts at least as long as 'b,
/// so a reference &'a () is valid whenever &'b () is valid.
#[allow(dead_code, unused_assignments, unused_variables)]
fn f<'a, 'b>(x: &'a i32, y: &'b mut i32) where 'a: 'b {
    *y = *x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
}

pub fn lifetime_bound() {
    let mut b =10;
    f(&20, &mut b);
    println!("b: {:?}", b);

    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print_ref2(&ref_x);

    print(ref_x);
}

/// T: 'a means that all lifetime parameters of T outlive 'a.
#[derive(Debug)]
struct Ref<'a, T>(&'a T);  // 从参数 &'a T 编译器可以自动推导出 T:'a，因为T的生命周期肯定长于&T
#[allow(dead_code)]
struct Ref2<'a, T: 'a>(&'a T); // 忽略编译器隐式推导的完整写法


fn print<T>(t: T) where T: Debug{
    println!("t: {:?}", t);
}
// 符合生命周期省略规则以及生命周期自动推导， 以下两个方法等价
fn print_ref2<'a, T>(t: &'a T) where T: Debug+'a {
    println!("ref: {:?}", t);
}
fn print_ref<T>(t: &T) where T: Debug {
    println!("ref: {:?}", t);
}

/// trait对象生命周期
trait T1 {}
#[allow(dead_code)]
struct Bar<'a> {
    x: &'a i32,
}
impl<'a> T1 for Bar<'a> {}
#[allow(unused_variables)]
pub fn lifetime_trait_object() {
    let num = 5;
    let bar = Box::new(Bar { x: &num });
    // 可以自动推导trait对象的生命周期就是'a，因为实现类型中有只有唯一的生命周期'a
    let obj = bar as Box<dyn T1>;
}

// 生命周期也是类型的一部分
pub trait T2 {}
#[allow(dead_code)]
struct S2<'a> {
    s: &'a [u32],
}
impl<'a> T2 for S2<'a> {
}
// trait对象默认的生命周期是'static, 此时显式指定trait对象的声明周期是'a, 不然导致 'a outlives 'static
pub fn lifetime_trait_object_explicit<'a>(s: &'a [u32]) -> Box<dyn T2 + 'a> {
    Box::new(S2 { s })
}


