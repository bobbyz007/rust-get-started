/// 对于实现Copy trait的类型，即复制语义的类型，在作为右值赋值操作时候，默认会进行按位复制（通常是栈内存上）
/// 对于未实现Copy trait的类型来说，如果也支持按位复制，则可能出现栈内存上的多个引用指向堆上同一个内存块， 会导致内存安全问题
/// 所以对于未实现Copy trait的类型来说，作为右值的变量会执行移动语义来转移所有权，保证每个值都有唯一的所有者，保证内存安全。

/// 即使结构和枚举的成员都实现了Copy，自身不会自动实现Copy，需要手工添加。
/// 数组、Option，元组：如果其元素都实现了Copy，则它们也自动实现了Copy
///
/// 默认以下类型都实现了Copy
/// 1. 基础类型，比如char，fn()
/// 2. 不可变引用类型，比如 &String， *const String， *mut String。 比如 &mut i32没有实现
/// 3. 切片类型，比如 &str， &[i32],

#[allow(dead_code, unused_variables)]
pub fn copy_semantic() {
    let a = A { a: 1, b: 2 };
    let b = a;
    // 如果A不添加Copy语义， 会报错
    println!("a: {:?}", a);
}

#[allow(unused_variables)]
pub fn copy_semantic_auto() {
    let a = ("a".to_string(), "b".to_string());
    let b = a;
    // 报错：a没有实现copy， 因为其元素没有实现copy
    // println!("a: {:?}", a);

    // c是元组，且元素都实现了copy
    let c = (1, 2, 3);
    let d = c;
    println!("c: {:?}", c);
}

#[allow(unused_variables, dead_code)]
pub fn copy_semantic_borrow() {
    let a = "hello".to_string();
    let b = & a;
    // error:因为b是借用，本来就不拥有变量a的所有权，不能将其借用的数据解引用转移所有权到另一个变量中去，如果允许，&a就无效了，a就成为了野指针
    // let c = *b;

    let mut a = "hello".to_string();
    let b = &mut a;
    // error:因为b是借用，本来就不拥有变量a的所有权，不能将其借用的数据解引用转移所有权到另一个变量中去，如果允许，&a就无效了，a就成为了野指针
    // let c = *b;

    // 但可以解引用操作
    (*b).push('d');
    println!("b: {:?}", a)
}

pub fn is_copy<T: Copy>() {}
pub fn is_copy_semantic() {
    // 基本类型
    is_copy::<bool>();
    is_copy::<char>();
    is_copy::<i8>();
    is_copy::<i16>();
    is_copy::<i32>();
    is_copy::<i64>();
    is_copy::<isize>();
    is_copy::<usize>();
    is_copy::<f32>();
    is_copy::<f64>();
    is_copy::<fn()>();
    // is_copy::<String>();  // ERR

    // 不可变引用。 如果可变引用允许Copy，则可能出现多个修改指针指向同一个数据， 引发内存安全问题
    is_copy::<&i32>();
    // is_copy::<&mut i32>(); // ERR
    is_copy::<&str>();
    is_copy::<& String>();
    // is_copy::<&mut String>(); // ERR
    is_copy::<*const String>();
    is_copy::<*mut String>();

    // 切片类型
    is_copy::<&str>();
    is_copy::<&[i32]>();

    // 数组，元组, Option
    is_copy::<[i32; 1]>();
    is_copy::<(i32, i32)>();
    is_copy::<Option<i32>>();
    is_copy::<Option<&str>>();
    is_copy::<Option<&String>>();
    // is_copy::<Option<String>>(); // ERR
    // is_copy::<[Vec<i32>; 1]>();  // ERR
    // is_copy::<(Vec<i32>, Vec<i32>)>();  // ERR
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, unused_variables)]
struct A {
    a: i32,
    b: u32,
}