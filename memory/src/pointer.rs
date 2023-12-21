/// 像Box<T>这样的指针被称为智能指针，可以让Rust利用栈来隐式自动释放堆内存，从而避免显式调用free之类的函数去释放内存。
/// 三类指针：
/// 普通指针（引用）： 用&， &mut操作符创建
/// 原生指针（裸指针）：形如*const T 或 *mut T这样的类型
/// 智能指针： 形如 Box<T>，实际上是一个结构体，是对指针的一层封装，添加了额外功能比如自动释放内存。
///
/// 其中，引用和裸指针可以通过as操作符转换： &T as *const T 或 &mut T as *mut T
/// 裸指针可以在unsafe块下任意使用， 而引用必须接受Rust安全规则的限制。

/// 智能指针实现了 Deref和 Drop trait， 可以认为String和Vec也都是一种智能指针。

// 确定性析构，实现Drop trait，当变量离开作用域时，会调用drop方法自动释放内存
// 专业术语叫 RAII(Resource Acquisition Is Initialization) 或 SBRM(Scope-Bound Resource Management)
pub fn smart_pointer() {
    let x = S(1);
    println!("create x: {:?}", x);
    {
        let y = S(2);
        println!("create y: {:?}", y);
        println!("exit inner scope");
    }
    println!("exit main");
}

pub fn smart_pointer_shadowing() {
    println!();

    let x = S(1);
    println!("create x: {:?}", x);
    // 变量遮蔽不会导致析构前面的x， 而是等函数结束后 析构drop 2， drop 1
    let x = S(2);
    println!("create shadowing x: {:?}", x);
}

#[allow(unused_mut)]
pub fn smart_pointer_destruct() {
    println!();

    // 主动析构
    let mut v = vec![1, 2, 3];
    drop(v);

    // error!
    // v.push(4);
}

#[derive(Debug)]
struct S(i32);
impl Drop for S {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

