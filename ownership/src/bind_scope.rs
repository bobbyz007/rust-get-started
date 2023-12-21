/// 创建新的词法作用域时， 在变量作为右值进行赋值操作时，要么转移所有权，要么按位复制，这取决于该变量是复制语义还是移动语义。
/// 可能创建词法作用域：
/// 1. let绑定
/// 2. 花括号
/// 3. match匹配
/// 4. 流程控制、函数或闭包等用到花括号的地方

#[allow(unused_variables, dead_code, path_statements)]
pub fn brace_scope() {
    let outer_val = 1;
    let outer_sp = "hello".to_string();
    {
        let inner_val = 2;
        outer_val;
        drop(outer_sp);
    }

    println!("outer val: {:?}", outer_val);
    // println!("outer sp: {:?}", outer_sp); // error
}

#[allow(dead_code)]
pub fn match_scope() {
    let a = Some("hello".to_string());
    match a {
        Some(s) => println!("{:?}", s),
        _ => println!("nothing"),
    }

    // println!("a: {:?}", a); // error, Option<String>类型是移动语义
}

#[allow(dead_code)]
pub fn loop_scope() {
    let v = vec![1, 2, 3];
    // for, loop, while都是新的作用域，v是移动语义， 进入循环就会转移所有权
    for i in v {
        println!("i: {:?}", i);
        // println!("v: {:?}", v); // error， 绑定v的所有权已经转移到循环中了
    }
}

#[allow(dead_code)]
pub fn let_scope() {
    let a = Some("hello".to_string());
    if let Some(s) = a {
        println!("s: {:?}", s);
    }
    // println!("a: {:?}", a);

    // 复制语义
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("i is {:?}, try again", i);
            optional = Some(i + 1);
        }
    }
}

pub fn func_scope() {
    let s = "hello".to_string();
    foo(s);
    // println!("s: {:?}", s); // error， s是移动语义，转移到函数参数中
}

fn foo(s: String) -> String {
    let w = " world".to_string();
    s + &w
}

pub fn closure_scope() {
    let s = "hello".to_string();
    // 新的作用域， s是移动语义， 转移到闭包中
    let join = |i: &str| {
        s + i
    };
    assert_eq!("hello world", join(" world"));

    // println!("s: {:?}", s); //error
}


